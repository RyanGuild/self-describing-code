//! Mermaid diagram bodies (no fences).

use std::fmt::Write as _; // write! / writeln!

use crate::util::alias;
use crate::view::{
    ClassMember, ClassStereotype, CouplingFn, CouplingGraph, CouplingModule, HierarchyView,
    LinkGraph, NodeMeta, PackageNode, PackageStructure, ProcessEvent, ProcessView, RelationKind,
    SiblingGraph, SourceSlice, TypeModel,
};

use super::Renderer;

#[derive(Debug, Default, Clone, Copy)]
pub struct MermaidRenderer;

impl Renderer for MermaidRenderer {
    fn meta(&self, _meta: &NodeMeta) -> String {
        String::new()
    }

    fn process(&self, view: &ProcessView) -> String {
        let mut out = String::from("sequenceDiagram\n");
        out.push_str("    autonumber\n");

        for participant in &view.participants {
            let _ = writeln!(
                out,
                "    participant {} as {}",
                alias(&participant.id),
                crate::util::text(&participant.label)
            );
        }

        for event in &view.events {
            match event {
                ProcessEvent::End => out.push_str("    end\n"),
                ProcessEvent::Else { label } => {
                    let _ = writeln!(out, "    else {}", crate::util::text(label));
                }
                ProcessEvent::Begin { keyword, label } => {
                    let _ = writeln!(out, "    {keyword} {}", crate::util::text(label));
                }
                ProcessEvent::Interaction {
                    from,
                    to,
                    label,
                    observed,
                    ..
                } => {
                    let arrow = if *observed { "->>" } else { "-->>" };
                    let _ = writeln!(
                        out,
                        "    {}{}{}: {}",
                        alias(from),
                        arrow,
                        alias(to),
                        crate::util::text(label)
                    );
                }
            }
        }

        out
    }

    fn types(&self, view: &TypeModel) -> String {
        let mut out = String::from("classDiagram\n");
        for class in &view.classes {
            let _ = writeln!(out, "    class {} {{", class.id);
            match class.stereotype {
                Some(ClassStereotype::Interface) => out.push_str("        <<interface>>\n"),
                Some(ClassStereotype::Enumeration) => out.push_str("        <<enumeration>>\n"),
                None => {}
            }
            for member in &class.members {
                match member {
                    ClassMember::Field(name) => {
                        let _ = writeln!(out, "        +{name}");
                    }
                    ClassMember::Method(name) => {
                        let _ = writeln!(out, "        +{name}()");
                    }
                    ClassMember::Variant(name) => {
                        let _ = writeln!(out, "        {name}");
                    }
                }
            }
            out.push_str("    }\n");
        }
        for rel in &view.relations {
            match rel.kind {
                RelationKind::Implements => {
                    let _ = writeln!(out, "    {} ..|> {} : implements", rel.from_id, rel.to_id);
                }
                RelationKind::Extends => {
                    let _ = writeln!(out, "    {} --|> {} : extends", rel.from_id, rel.to_id);
                }
            }
        }
        out
    }

    fn package_deps(&self, view: &PackageStructure) -> String {
        // Nested source-structure packages + external stadiums; unweighted arrows.
        let mut out = String::from("flowchart TB\n");
        if view.is_empty() {
            return out;
        }

        emit_package_subgraph(&mut out, &view.root, 1);
        for ext in &view.externals {
            let _ = writeln!(out, "    {}([\"{}\"])", ext.stable_id, ext.name);
        }
        for edge in &view.edges {
            let arrow = if edge.in_cycle { "-.->" } else { "-->" };
            let _ = writeln!(out, "    {} {arrow} {}", edge.from, edge.to);
        }
        out
    }

    fn coupling_deps(&self, view: &CouplingGraph) -> String {
        // Modules as subgraphs; functions (or workspace modules) as nodes; Calls.
        let mut out = String::from("flowchart TB\n");
        if view.is_empty() {
            return out;
        }

        if view.functions.is_empty() {
            // Workspace: crates as subgraphs, modules as leaf nodes.
            emit_workspace_coupling(&mut out, &view.root, 1);
        } else {
            let by_parent: std::collections::BTreeMap<&str, Vec<&CouplingFn>> = {
                let mut map: std::collections::BTreeMap<&str, Vec<&CouplingFn>> =
                    std::collections::BTreeMap::new();
                for leaf in &view.functions {
                    map.entry(leaf.parent_stable_id.as_str())
                        .or_default()
                        .push(leaf);
                }
                map
            };
            emit_coupling_subgraph(&mut out, &view.root, &by_parent, 1);
        }

        for edge in &view.edges {
            if edge.in_cycle {
                let _ = writeln!(
                    out,
                    "    {} -.->|\"{} circular\"| {}",
                    edge.from, edge.weight, edge.to
                );
            } else if edge.weight > 1 {
                let _ = writeln!(
                    out,
                    "    {} -->|\"{}\"| {}",
                    edge.from, edge.weight, edge.to
                );
            } else {
                let _ = writeln!(out, "    {} --> {}", edge.from, edge.to);
            }
        }
        out
    }

    fn dependency_matrix(&self, view: &PackageStructure) -> String {
        // Mermaid has no matrix; DSM is rendered as Markdown/ASCII by other renderers.
        let _ = view;
        String::new()
    }

    fn hierarchy(&self, _view: &HierarchyView) -> String {
        String::new()
    }

    fn externals(&self, _view: &crate::view::ExternalsView) -> String {
        String::new()
    }

    fn comments(&self, _view: &crate::view::CommentsView) -> String {
        String::new()
    }

    fn links(&self, _view: &LinkGraph) -> String {
        String::new()
    }

    fn sibling_graph(&self, view: &SiblingGraph) -> String {
        let mut out = String::from("flowchart TD\n");
        let _ = writeln!(
            out,
            "    subgraph {}[\"{}\"]",
            view.root_id, view.root_label
        );
        out.push_str("        direction TB\n");
        for node in &view.nodes {
            let _ = writeln!(out, "        {}[\"{}\"]", node.stable_id, node.label);
        }
        out.push_str("    end\n");
        for edge in &view.edges {
            let arrow = if edge.resolved { "-->" } else { "-.->" };
            let _ = writeln!(
                out,
                "    {} {arrow}|{}| {}",
                edge.from, edge.label, edge.to
            );
        }
        out
    }

    fn source(&self, _view: &SourceSlice) -> String {
        String::new()
    }
}

fn emit_package_subgraph(out: &mut String, node: &PackageNode, depth: usize) {
    let indent = "    ".repeat(depth);
    if node.children.is_empty() {
        // Leaf package: draw as a node so edges have an endpoint.
        let _ = writeln!(out, "{indent}{}[\"{}\"]", node.stable_id, node.name);
        return;
    }
    let _ = writeln!(
        out,
        "{indent}subgraph {}[\"{}\"]",
        node.stable_id, node.name
    );
    let _ = writeln!(out, "{indent}    direction TB");
    for child in &node.children {
        emit_package_subgraph(out, child, depth + 1);
    }
    let _ = writeln!(out, "{indent}end");
}

fn emit_workspace_coupling(out: &mut String, node: &CouplingModule, depth: usize) {
    let indent = "    ".repeat(depth);
    if node.children.is_empty() {
        let _ = writeln!(out, "{indent}{}[\"{}\"]", node.stable_id, node.name);
        return;
    }
    let _ = writeln!(
        out,
        "{indent}subgraph {}[\"{}\"]",
        node.stable_id, node.name
    );
    let _ = writeln!(out, "{indent}    direction TB");
    for child in &node.children {
        emit_workspace_coupling(out, child, depth + 1);
    }
    let _ = writeln!(out, "{indent}end");
}

fn emit_coupling_subgraph(
    out: &mut String,
    node: &CouplingModule,
    by_parent: &std::collections::BTreeMap<&str, Vec<&CouplingFn>>,
    depth: usize,
) {
    let indent = "    ".repeat(depth);
    let leaves = by_parent.get(node.stable_id.as_str()).cloned().unwrap_or_default();
    if node.children.is_empty() && leaves.is_empty() {
        return;
    }

    let _ = writeln!(
        out,
        "{indent}subgraph {}[\"{}\"]",
        node.stable_id, node.name
    );
    let _ = writeln!(out, "{indent}    direction TB");
    for child in &node.children {
        emit_coupling_subgraph(out, child, by_parent, depth + 1);
    }
    for leaf in &leaves {
        let _ = writeln!(
            out,
            "{indent}    {}[\"{}\"]",
            leaf.stable_id, leaf.name
        );
    }
    let _ = writeln!(out, "{indent}end");
}

/// Markdown DSM table (row depends on column). Shared by Markdown/TUI.
pub fn dsm_markdown_table(view: &PackageStructure) -> String {
    if view.modules.is_empty() {
        return String::new();
    }

    let order: Vec<&crate::view::DepModule> = {
        // Prefer layer order for headers, then any leftovers by name.
        let mut seen = std::collections::BTreeSet::new();
        let mut ordered = Vec::new();
        for layer in &view.layers {
            for sid in layer {
                if let Some(m) = view.module_by_stable(sid) {
                    if seen.insert(sid.clone()) {
                        ordered.push(m);
                    }
                }
            }
        }
        for m in &view.modules {
            if seen.insert(m.stable_id.clone()) {
                ordered.push(m);
            }
        }
        ordered
    };

    let index: std::collections::BTreeMap<&str, usize> = order
        .iter()
        .enumerate()
        .map(|(i, m)| (m.stable_id.as_str(), i))
        .collect();

    let n = order.len();
    let mut matrix = vec![vec![0usize; n]; n];
    let mut cyclic = vec![vec![false; n]; n];
    for edge in &view.edges {
        let Some(&i) = index.get(edge.from.as_str()) else {
            continue;
        };
        let Some(&j) = index.get(edge.to.as_str()) else {
            continue;
        };
        matrix[i][j] = edge.weight;
        cyclic[i][j] = edge.in_cycle;
    }

    let mut out = String::new();
    out.push_str("| |");
    for m in &order {
        let _ = write!(out, " {} |", m.name);
    }
    out.push('\n');
    out.push_str("|---|");
    for _ in &order {
        out.push_str("---:|");
    }
    out.push('\n');

    for (i, row) in order.iter().enumerate() {
        let _ = write!(out, "| **{}** |", row.name);
        for (j, _) in order.iter().enumerate() {
            if i == j {
                out.push_str(" · |");
            } else if matrix[i][j] == 0 {
                out.push_str(" |");
            } else if cyclic[i][j] {
                let _ = write!(out, " {}⇄ |", matrix[i][j]);
            } else {
                let _ = write!(out, " {} |", matrix[i][j]);
            }
        }
        out.push('\n');
    }

    if !view.cycles.is_empty() {
        out.push_str("\nCycles: ");
        let names: Vec<String> = view
            .cycles
            .iter()
            .map(|scc| {
                scc.iter()
                    .filter_map(|sid| view.module_by_stable(sid).map(|m| m.name.as_str()))
                    .collect::<Vec<_>>()
                    .join(" ↔ ")
            })
            .collect();
        out.push_str(&names.join("; "));
        out.push('\n');
    }

    out
}
