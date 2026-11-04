//! Execution sequence: code-path Calls as a [`Process`] (Mermaid views).
//!
//! Participants are callables on the path plus owning types for methods
//! (structs/enums/traits parsed from `impl … for Type`), so the diagram
//! connects type lifelines and function nodes that describe the run.

use robby_ir::{
    CodePath, Document, Kind, Node, NodeId, Process, Step, StepKind, DEFAULT_MAX_DEPTH, kind,
};

use crate::util::alias;

/// Cap diagram size so Mermaid stays readable in the Prezi detail panel.
const MAX_EDGES: usize = 48;

/// Build an execution [`Process`] rooted at `root` (BFS Calls, depth-capped).
pub fn execution_process(doc: &Document, root: &NodeId) -> Option<Process> {
    let path = CodePath::from_entry(doc, root, DEFAULT_MAX_DEPTH)?;
    from_code_path(doc, &path)
}

pub fn from_code_path(doc: &Document, path: &CodePath) -> Option<Process> {
    let entry = doc.get(&path.entry)?;
    let mut process = Process::new();

    add_callable(&mut process, entry);

    let edges: Vec<_> = path.edges.iter().take(MAX_EDGES).collect();
    if edges.is_empty() {
        // No outbound path — fall back to the node's own process if any.
        return entry.process.clone().filter(|p| !p.steps.is_empty());
    }

    for edge in edges {
        let Some(from) = doc.get(&edge.from) else {
            continue;
        };
        let Some(to) = doc.get(&edge.to) else {
            continue;
        };
        add_callable(&mut process, from);

        let from_id = participant_id(from);
        if let Some(type_label) = owning_type_label(doc, to) {
            let type_id = alias(&type_label);
            process.participant(
                type_id.clone(),
                type_label,
                Kind::from(kind::code::STRUCT),
            );
            process.step(Step {
                from: from_id,
                to: type_id,
                label: to.name.clone(),
                kind: StepKind::Call,
                observed: false,
                span: None,
                blocks: Vec::new(),
            });
        } else {
            add_callable(&mut process, to);
            process.step(Step {
                from: from_id,
                to: participant_id(to),
                label: to.name.clone(),
                kind: StepKind::Call,
                observed: false,
                span: None,
                blocks: Vec::new(),
            });
        }
    }

    if process.steps.is_empty() {
        return None;
    }
    Some(process)
}

fn add_callable(process: &mut Process, node: &Node) {
    process.participant(
        participant_id(node),
        node.fqn.clone(),
        node.kind.clone(),
    );
}

fn participant_id(node: &Node) -> String {
    alias(node.fqn.as_str())
}

/// From a method under `impl … for Type`, recover the type name for a lifeline.
fn owning_type_label(doc: &Document, method: &Node) -> Option<String> {
    if method.kind.as_str() != kind::code::METHOD {
        return None;
    }
    let parent = method.parent.as_ref().and_then(|id| doc.get(id))?;
    if parent.kind.as_str() != kind::code::IMPL {
        return None;
    }
    let name = parent.name.trim();
    if let Some((_, ty)) = name.rsplit_once(" for ") {
        let ty = ty.trim();
        if !ty.is_empty() {
            return Some(ty.to_owned());
        }
    }
    name.strip_prefix("impl")
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.contains(' '))
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use robby_ir::{Edge, EdgeKind};

    fn sample() -> Document {
        let mut doc = Document::new("test");
        let mut main = Node::new(
            NodeId::symbol("rust", "demo::main"),
            kind::code::FUNCTION,
            "demo::main",
        );
        main.edges
            .push(Edge::new(NodeId::symbol("rust", "demo::run"), EdgeKind::Calls));
        doc.insert(main);

        let mut run = Node::new(
            NodeId::symbol("rust", "demo::run"),
            kind::code::FUNCTION,
            "demo::run",
        );
        run.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::impl Foo::bar"),
            EdgeKind::Calls,
        ));
        doc.insert(run);

        let mut impl_node = Node::new(
            NodeId::symbol("rust", "demo::impl Foo"),
            kind::code::IMPL,
            "impl Foo",
        );
        impl_node.name = "impl Foo".into();
        doc.insert(impl_node);

        let mut method = Node::new(
            NodeId::symbol("rust", "demo::impl Foo::bar"),
            kind::code::METHOD,
            "demo::impl Foo::bar",
        );
        method.name = "bar".into();
        method.parent = Some(NodeId::symbol("rust", "demo::impl Foo"));
        doc.insert(method);

        doc
    }

    #[test]
    fn sequence_includes_call_and_type_lifeline() {
        let doc = sample();
        let process =
            execution_process(&doc, &NodeId::symbol("rust", "demo::main")).expect("process");
        assert!(process.participants.iter().any(|p| p.label.contains("main")));
        assert!(process.participants.iter().any(|p| p.label.contains("run")));
        assert!(
            process.participants.iter().any(|p| p.label == "Foo"),
            "expected Foo type participant, got {:?}",
            process.participants
        );
        assert!(process.steps.len() >= 2);
    }
}
