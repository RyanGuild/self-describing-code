//! Execution sequence: code-path Calls as a [`Process`] (Mermaid views).
//!
//! Participants are callables on the path plus owning types for methods
//! (structs/enums/traits parsed from `impl … for Type`), so the diagram
//! connects type lifelines and function nodes that describe the run.
//!
//! This is intentionally distinct from a node's definition [`Process`]:
//! empty call paths return [`None`] rather than cloning the body process
//! (which previously made Execution and Process Mermaid charts identical).

use robby_ir::{
    CodePath, DEFAULT_MAX_DEPTH, Document, Kind, Node, NodeId, Process, Step, StepKind, kind,
};

use crate::util::alias;

/// Cap diagram size so Mermaid stays readable in the Prezi detail panel.
const MAX_EDGES: usize = 48;

/// Build an execution [`Process`] rooted at `root` (BFS Calls, depth-capped).
///
/// Returns [`None`] when there is no outbound call path — callers should not
/// substitute the node's static body process (that belongs under Process).
pub fn execution_process(doc: &Document, root: &NodeId) -> Option<Process> {
    let path = CodePath::from_entry(doc, root, DEFAULT_MAX_DEPTH)?;
    from_code_path(doc, &path)
}

/// Observed-only slice of a (possibly absorbed) definition process.
///
/// Used when there is no call-path execution yet, but runtime has stamped
/// `observed` steps via [`Process::absorb`].
pub fn observed_process(process: &Process) -> Option<Process> {
    let steps: Vec<Step> = process
        .steps
        .iter()
        .filter(|s| s.observed)
        .cloned()
        .collect();
    if steps.is_empty() {
        return None;
    }
    // Fully-observed absorb of the same body is still the definition graph —
    // Process already shows it (solid arrows). Only emit when observation is a
    // proper subset so Execution is visually distinct.
    if steps.len() == process.steps.len() {
        return None;
    }

    let used: std::collections::BTreeSet<&str> = steps
        .iter()
        .flat_map(|s| [s.from.as_str(), s.to.as_str()])
        .collect();

    let mut out = Process::new();
    for p in &process.participants {
        if used.contains(p.id.as_str()) {
            out.participant(p.id.clone(), p.label.clone(), p.kind.clone());
        }
    }
    for step in steps {
        out.step(step);
    }
    Some(out)
}

pub fn from_code_path(doc: &Document, path: &CodePath) -> Option<Process> {
    let entry = doc.get(&path.entry)?;
    let mut process = Process::new();

    add_callable(&mut process, entry);

    let edges: Vec<_> = path.edges.iter().take(MAX_EDGES).collect();
    if edges.is_empty() {
        // No outbound Calls — do not reuse the definition process.
        return None;
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
            process.participant(type_id.clone(), type_label, Kind::from(kind::code::STRUCT));
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
    process.participant(participant_id(node), node.fqn.clone(), node.kind.clone());
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
        main.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::run"),
            EdgeKind::Calls,
        ));
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
        assert!(
            process
                .participants
                .iter()
                .any(|p| p.label.contains("main"))
        );
        assert!(process.participants.iter().any(|p| p.label.contains("run")));
        assert!(
            process.participants.iter().any(|p| p.label == "Foo"),
            "expected Foo type participant, got {:?}",
            process.participants
        );
        assert!(process.steps.len() >= 2);
    }

    #[test]
    fn empty_call_path_does_not_clone_definition_process() {
        let mut doc = Document::new("test");
        let mut leaf = Node::new(
            NodeId::symbol("rust", "demo::leaf"),
            kind::code::FUNCTION,
            "demo::leaf",
        );
        let mut body = Process::new();
        body.participant("leaf", "leaf()", Kind::from(kind::code::FUNCTION));
        body.step(Step {
            from: "leaf".into(),
            to: "leaf".into(),
            label: "body".into(),
            kind: StepKind::Note,
            observed: false,
            span: None,
            blocks: Vec::new(),
        });
        leaf.process = Some(body);
        doc.insert(leaf);

        assert!(
            execution_process(&doc, &NodeId::symbol("rust", "demo::leaf")).is_none(),
            "leaf with definition process but no Calls must not yield execution"
        );
    }

    #[test]
    fn observed_process_is_proper_subset_only() {
        let mut process = Process::new();
        process.participant("a", "a", Kind::from(kind::code::FUNCTION));
        process.participant("b", "b", Kind::from(kind::code::FUNCTION));
        process.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "hit".into(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: Vec::new(),
        });
        process.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "miss".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: Vec::new(),
        });

        let obs = observed_process(&process).expect("subset");
        assert_eq!(obs.steps.len(), 1);
        assert_eq!(obs.steps[0].label, "hit");
        assert!(obs.steps[0].observed);

        // All observed → None (Process already shows the solid graph).
        process.steps[1].observed = true;
        assert!(observed_process(&process).is_none());
    }
}
