//! The Markdown report is a committed artifact, so it must be stable and
//! self-contained.

use std::collections::BTreeMap;

use robby_ir::kind::code;
use robby_ir::{Document, Kind, Node, NodeId, Participant, Process, Source, Step, StepKind};
use robby_render::markdown::report;

const SOURCE: &str = "pub fn a() {\n    b();\n}\n";

fn fixture() -> (Document, BTreeMap<String, String>) {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    doc.insert(module);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.span = Some(robby_ir::Span::new("src/demo.rs", SOURCE, 0, 28));
    a.signature = Some("fn a()".to_owned());
    a.docs = Some("Does a thing.".to_owned());
    a.provenance.source = Source::Extractor;

    let mut process = Process::new();
    process.participant("caller", "a", Kind::from(code::FUNCTION));
    process.participant("b", "b", Kind::from(code::FUNCTION));
    process.step(Step {
        from: "caller".to_owned(),
        to: "b".to_owned(),
        label: "b()".to_owned(),
        kind: StepKind::Call,
        observed: true,
        span: None,
        blocks: Vec::new(),
    });
    a.process = Some(process);
    doc.insert(a);

    let mut sources = BTreeMap::new();
    sources.insert("src/demo.rs".to_owned(), SOURCE.to_owned());

    (doc, sources)
}

#[test]
fn report_is_byte_stable() {
    let (doc, sources) = fixture();
    let first = report(&doc, None, Some(&sources));
    let second = report(&doc, None, Some(&sources));
    assert_eq!(first, second);
}

#[test]
fn report_embeds_source_and_diagram_together() {
    let (doc, sources) = fixture();
    let out = report(&doc, None, Some(&sources));

    assert!(out.contains("demo::a"));
    assert!(out.contains("pub fn a() {"), "source slice is embedded");
    assert!(out.contains("sequenceDiagram"), "diagram is embedded");
    assert!(out.contains("caller->>b: b()"));
    assert!(out.contains("> Does a thing."));
}

#[test]
fn fences_are_balanced() {
    let (doc, sources) = fixture();
    let out = report(&doc, None, Some(&sources));
    let fences = out.matches("```").count();
    assert_eq!(fences % 2, 0, "unbalanced code fences:\n{out}");
}

#[test]
fn report_without_sources_still_renders() {
    let (doc, _) = fixture();
    let out = report(&doc, None, None);
    assert!(out.contains("demo::a"));
    assert!(!out.contains("pub fn a() {"));
}

#[test]
fn unused_participant_import_is_referenced() {
    // Keeps the public API surface honest in this test crate.
    let _ = Participant {
        id: "x".to_owned(),
        label: "x".to_owned(),
        kind: Kind::from(code::FUNCTION),
    };
}
