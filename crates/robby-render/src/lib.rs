//! Deterministic Mermaid views over Robby IR.
//!
//! Every function here is a pure function of the IR. There is no clock, no
//! filesystem, no hash map iteration order. Running the renderer twice on the
//! same document must produce byte-identical output, because the generated
//! diagrams are committed and diffed in CI.

pub mod markdown;

use std::fmt::Write as _;

use robby_ir::{
    Block, BlockKind, Document, EdgeKind, Node, NodeId, Process, Step, StepKind, kind,
};

/// Render a process as a Mermaid `sequenceDiagram`.
///
/// Observed steps use solid arrows; steps that are statically possible but were
/// never observed use dashed arrows.
pub fn sequence(process: &Process) -> String {
    let mut out = String::from("sequenceDiagram\n");
    out.push_str("    autonumber\n");

    for participant in &process.participants {
        let _ = writeln!(
            out,
            "    participant {} as {}",
            alias(&participant.id),
            text(&participant.label)
        );
    }

    let mut stack: Vec<Block> = Vec::new();

    for step in &process.steps {
        let common = common_prefix(&stack, &step.blocks);

        // Close frames that ended before this step.
        for _ in common..stack.len() {
            out.push_str("    end\n");
        }

        // Same frame, different branch: emit `else` instead of reopening.
        let mut opened = common;
        if common == stack.len()
            && common > 0
            && step.blocks.len() >= common
            && step.blocks[common - 1].branch != stack[common - 1].branch
        {
            let branch = step.blocks[common - 1].branch.as_deref().unwrap_or("");
            let _ = writeln!(out, "    else {}", text(branch));
        }

        // Open frames that started at this step.
        while opened < step.blocks.len() {
            let block = &step.blocks[opened];
            let _ = writeln!(out, "    {} {}", keyword(block.kind), text(&block.label));
            opened += 1;
        }

        stack = step.blocks.clone();

        let arrow = if step.observed { "->>" } else { "-->>" };
        let _ = writeln!(
            out,
            "    {}{}{}: {}",
            alias(&step.from),
            arrow,
            alias(&step.to),
            text(&step.label)
        );
    }

    for _ in 0..stack.len() {
        out.push_str("    end\n");
    }

    out
}

fn common_prefix(a: &[Block], b: &[Block]) -> usize {
    let mut i = 0;
    while i < a.len() && i < b.len() && a[i].kind == b[i].kind && a[i].label == b[i].label {
        i += 1;
    }
    i
}

fn keyword(kind: BlockKind) -> &'static str {
    match kind {
        BlockKind::Alt => "alt",
        BlockKind::Opt => "opt",
        BlockKind::Loop => "loop",
        BlockKind::Par => "par",
        BlockKind::Break => "break",
    }
}

/// Render a subtree as a Mermaid `flowchart`, clustering modules.
pub fn flowchart(doc: &Document, root: &NodeId) -> String {
    let nodes = doc.subtree(root);
    let ids: Vec<&NodeId> = nodes.iter().map(|n| &n.id).collect();

    let mut out = String::from("flowchart TD\n");

    for node in &nodes {
        if node.kind.as_str() == kind::code::MODULE || node.kind.as_str() == kind::code::CRATE {
            let _ = writeln!(
                out,
                "    subgraph {}[\"{}\"]",
                stable_id(node.id.as_str()),
                text(&label(node))
            );
            out.push_str("        direction TB\n");
            for child in &node.children {
                if let Some(child_node) = doc.get(child) {
                    let _ = writeln!(
                        out,
                        "        {}[\"{}\"]",
                        stable_id(child_node.id.as_str()),
                        text(&label(child_node))
                    );
                }
            }
            out.push_str("    end\n");
        }
    }

    // Nodes outside a module group, plus any node not already emitted as a child.
    for node in &nodes {
        let in_group = node
            .parent
            .as_ref()
            .and_then(|p| doc.get(p))
            .is_some_and(|p| {
                p.kind.as_str() == kind::code::MODULE || p.kind.as_str() == kind::code::CRATE
            });
        if !in_group
            && node.kind.as_str() != kind::code::MODULE
            && node.kind.as_str() != kind::code::CRATE
        {
            let _ = writeln!(
                out,
                "    {}[\"{}\"]",
                stable_id(node.id.as_str()),
                text(&label(node))
            );
        }
    }

    let mut edges: Vec<(String, String, EdgeKind, bool)> = Vec::new();
    for node in &nodes {
        for edge in &node.edges {
            if !ids.contains(&&edge.to) {
                continue;
            }
            edges.push((
                stable_id(node.id.as_str()),
                stable_id(edge.to.as_str()),
                edge.kind,
                edge.resolved,
            ));
        }
    }
    edges.sort();
    edges.dedup();

    for (from, to, edge_kind, resolved) in edges {
        let arrow = if resolved { "-->" } else { "-.->" };
        let _ = writeln!(out, "    {from} {arrow}|{}| {to}", edge_label(edge_kind));
    }

    out
}

fn edge_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Calls => "calls",
        EdgeKind::Uses => "uses",
        EdgeKind::Imports => "imports",
        EdgeKind::Implements => "implements",
        EdgeKind::Extends => "extends",
        EdgeKind::Returns => "returns",
        EdgeKind::Field => "field",
        EdgeKind::References => "refs",
    }
}

fn label(node: &Node) -> String {
    let sig = node.signature.as_deref().unwrap_or(&node.name);
    format!("{}  [{}]", sig, short_kind(&node.kind))
}

fn short_kind(kind: &robby_ir::Kind) -> &str {
    kind.as_str().rsplit('.').next().unwrap_or(kind.as_str())
}

/// A stable, order-independent identifier derived from a node id.
///
/// FNV-1a, not `DefaultHasher`: `DefaultHasher` is explicitly allowed to change
/// between releases, which would rewrite every diagram in the repository on a
/// toolchain bump.
pub fn stable_id(input: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("n_{hash:016x}")
}

/// Mermaid participant and node ids allow only a restricted alphabet.
fn alias(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'p');
    }
    if out.is_empty() {
        out.push_str("p_empty");
    }
    out
}

/// Sanitize text for a Mermaid label or message.
fn text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\n' | '\r' | '\t' => out.push(' '),
            ';' | '#' => out.push(' '),
            '"' => out.push('\''),
            _ => out.push(ch),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Convenience: render every process in a document, keyed by node id.
pub fn all_sequences(doc: &Document) -> Vec<(NodeId, String)> {
    let mut out: Vec<(NodeId, String)> = doc
        .nodes
        .values()
        .filter_map(|node| {
            node.process
                .as_ref()
                .map(|p| (node.id.clone(), sequence(p)))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Render a step as a compact one-liner, used by the site's step list.
pub fn step_summary(step: &Step) -> String {
    let kind = match step.kind {
        StepKind::Call => "call",
        StepKind::Return => "return",
        StepKind::Signal => "signal",
        StepKind::Decision => "decision",
        StepKind::Note => "note",
    };
    format!("{kind} {} -> {}", step.from, step.to)
}
