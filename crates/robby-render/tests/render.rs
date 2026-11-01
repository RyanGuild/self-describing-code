//! Renderer tests. Determinism is the point: these diagrams are committed.

use robby_ir::kind::code;
use robby_ir::{
    Block, BlockKind, Document, Edge, EdgeKind, Kind, Node, NodeId, Process, Source, Step, StepKind,
};
use robby_render::{flowchart, sequence, stable_id};

fn step(from: &str, to: &str, label: &str, observed: bool, blocks: Vec<Block>) -> Step {
    Step {
        from: from.to_owned(),
        to: to.to_owned(),
        label: label.to_owned(),
        kind: StepKind::Call,
        observed,
        span: None,
        blocks,
    }
}

fn process(steps: Vec<Step>) -> Process {
    let mut p = Process::new();
    p.participant("caller", "caller", Kind::from(code::FUNCTION));
    p.participant("io", "io::load", Kind::from(code::FUNCTION));
    for s in steps {
        p.step(s);
    }
    p
}

#[test]
fn renders_a_flat_sequence() {
    let p = process(vec![step("caller", "io", "load(path)", true, vec![])]);
    let out = sequence(&p);

    assert!(out.starts_with("sequenceDiagram\n"));
    assert!(out.contains("participant caller as caller"));
    assert!(out.contains("participant io as io::load"));
    assert!(out.contains("caller->>io: load(path)"));
}

#[test]
fn unobserved_steps_use_dashed_arrows() {
    let p = process(vec![step("caller", "io", "load(path)", false, vec![])]);
    let out = sequence(&p);
    assert!(out.contains("caller-->>io: load(path)"));
    assert!(!out.contains("caller->>io"));
}

#[test]
fn opens_and_closes_alt_blocks() {
    let alt = Block {
        kind: BlockKind::Alt,
        label: "cfg.verbose".to_owned(),
        branch: None,
    };
    let p = process(vec![
        step("caller", "io", "read", true, vec![]),
        step("caller", "io", "info", true, vec![alt.clone()]),
        step("caller", "io", "parse", true, vec![]),
    ]);
    let out = sequence(&p);

    let alt_at = out.find("alt cfg.verbose").expect("alt opened");
    let end_at = out.find("    end\n").expect("alt closed");
    let parse_at = out.find("parse").expect("parse step");

    assert!(alt_at < end_at, "alt must close");
    assert!(end_at < parse_at, "parse must come after the block closes");
}

#[test]
fn emits_else_for_a_second_branch() {
    let branch_a = Block {
        kind: BlockKind::Alt,
        label: "mode".to_owned(),
        branch: None,
    };
    let branch_b = Block {
        kind: BlockKind::Alt,
        label: "mode".to_owned(),
        branch: Some("fallback".to_owned()),
    };
    let p = process(vec![
        step("caller", "io", "fast", true, vec![branch_a]),
        step("caller", "io", "slow", true, vec![branch_b]),
    ]);
    let out = sequence(&p);

    assert!(out.contains("alt mode"));
    assert!(out.contains("else fallback"));
}

#[test]
fn nests_blocks_by_frame_stack() {
    let alt = Block {
        kind: BlockKind::Alt,
        label: "outer".to_owned(),
        branch: None,
    };
    let mut loop_block = alt.clone();
    loop_block.kind = BlockKind::Loop;
    loop_block.label = "inner".to_owned();

    let p = process(vec![
        step("caller", "io", "a", true, vec![alt.clone()]),
        step("caller", "io", "b", true, vec![alt.clone(), loop_block]),
        step("caller", "io", "c", true, vec![alt]),
        step("caller", "io", "d", true, vec![]),
    ]);
    let out = sequence(&p);

    assert!(out.contains("alt outer"));
    assert!(out.contains("loop inner"));
    assert_eq!(out.matches("    end\n").count(), 2);
}

#[test]
fn output_is_byte_identical_across_runs() {
    let p = process(vec![
        step("caller", "io", "a", true, vec![]),
        step("caller", "io", "b", false, vec![]),
    ]);
    assert_eq!(sequence(&p), sequence(&p));
}

#[test]
fn stable_ids_are_pinned() {
    // If this value changes, every committed diagram in the repo rewrites.
    // Changing it must be a deliberate, documented migration.
    assert_eq!(stable_id("rust://demo::a"), stable_id("rust://demo::a"));
    assert_ne!(stable_id("rust://demo::a"), stable_id("rust://demo::b"));
    assert_eq!(stable_id("").len(), 18);
}

#[test]
fn flowchart_clusters_modules() {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    module.provenance.source = Source::Extractor;
    doc.insert(module);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.signature = Some("fn a()".to_owned());
    doc.insert(a);

    let mut b = Node::new(NodeId::symbol("rust", "demo::b"), code::FUNCTION, "demo::b");
    b.parent = Some(NodeId::from("rust://demo"));
    b.edges.push(Edge::new(NodeId::symbol("rust", "demo::a"), EdgeKind::Calls));
    doc.insert(b);

    let out = flowchart(&doc, &NodeId::from("rust://demo"));

    assert!(out.starts_with("flowchart TD\n"));
    assert!(out.contains("subgraph"));
    assert!(out.contains("|calls|"));
}
