//! Tests for the coverage invariant.
//!
//! This is the highest-risk component in the project: the entire trust
//! argument rests on "nothing is silently dropped". These tests are written
//! against exact byte offsets so a regression is unambiguous.

use std::collections::BTreeMap;

use robby_ir::coverage::{document_coverage, file_coverage};
use robby_ir::model::{Document, Node, NodeId};
use robby_ir::span::Span;
use robby_ir::{GapReason, kind};

const FILE: &str = "src/demo.rs";

fn span(source: &str, start: u32, end: u32) -> Span {
    Span::new(FILE, source, start, end)
}

#[test]
fn fully_covered_file_has_no_gaps() {
    let source = "fn add(a: i32, b: i32) -> i32 { a + b }";
    let report = file_coverage(FILE, source, &[span(source, 0, source.len() as u32)]);

    assert!(report.is_complete());
    assert!(report.gaps.is_empty());
    assert_eq!(report.covered_bytes, source.len() as u32);
    assert_eq!(report.ratio(), 1.0);
}

#[test]
fn nested_spans_are_unioned_not_double_counted() {
    let source = "fn add() { 1 }";
    let spans = [
        span(source, 0, source.len() as u32),
        span(source, 3, 8),  // "add()"
        span(source, 4, 6),  // "dd"
    ];

    let report = file_coverage(FILE, source, &spans);
    assert!(report.is_complete());
    assert_eq!(report.covered_bytes, source.len() as u32);
}

#[test]
fn whitespace_and_comments_are_classified_not_failed() {
    let source = "// leading comment\n\nfn a() {}\n";
    let start = source.find("fn a").unwrap() as u32;
    let report = file_coverage(FILE, source, &[span(source, start, source.len() as u32)]);

    assert!(report.is_complete());
    assert_eq!(report.gaps.len(), 1);
    assert!(matches!(report.gaps[0].reason, GapReason::Comment));
}

#[test]
fn attributes_are_classified() {
    let source = "#[derive(Debug)]\nfn a() {}\n";
    let start = source.find("fn a").unwrap() as u32;
    let report = file_coverage(FILE, source, &[span(source, start, source.len() as u32)]);

    assert!(report.is_complete());
    assert_eq!(report.gaps.len(), 1);
    assert!(matches!(report.gaps[0].reason, GapReason::Attribute));
}

#[test]
fn unclaimed_code_fails_with_the_exact_byte() {
    let source = "fn a() {}\nlet x = 1;\n";
    let end = source.find('\n').unwrap() as u32 + 1;
    let report = file_coverage(FILE, source, &[span(source, 0, end)]);

    assert!(!report.is_complete(), "unclaimed code must fail the gate");

    let failure = report
        .gaps
        .iter()
        .find(|g| g.reason.is_failure())
        .expect("a failing gap");

    match failure.reason {
        GapReason::Unknown { at } => {
            assert_eq!(at.byte, 10);
            assert_eq!(at.line, 2);
            assert_eq!(at.column, 1);
        }
        ref other => panic!("expected Unknown, got {other:?}"),
    }
    assert_eq!(report.uncovered_bytes(), source.len() as u32 - end);
}

#[test]
fn empty_file_is_trivially_complete() {
    let report = file_coverage(FILE, "", &[]);
    assert!(report.is_complete());
    assert_eq!(report.ratio(), 1.0);
}

#[test]
fn spans_beyond_eof_are_clamped_not_panicking() {
    let source = "fn a() {}";
    let report = file_coverage(FILE, source, &[span(source, 0, 10_000)]);
    assert!(report.is_complete());
    assert_eq!(report.covered_bytes, source.len() as u32);
}

#[test]
fn spans_from_other_files_are_ignored() {
    let source = "fn a() {}";
    let other = Span::new("src/other.rs", source, 0, source.len() as u32);
    let report = file_coverage(FILE, source, &[other]);

    assert!(!report.is_complete());
    assert_eq!(report.covered_bytes, 0);
}

#[test]
fn unreferenced_files_appear_fully_uncovered() {
    let mut doc = Document::new("test");
    let mut node = Node::new(
        NodeId::symbol("rust", "demo::a"),
        kind::code::FUNCTION,
        "demo::a",
    );
    node.span = Some(Span::new(FILE, "fn a() {}", 0, 9));
    doc.insert(node);

    let mut sources = BTreeMap::new();
    sources.insert(FILE.to_owned(), "fn a() {}".to_owned());
    sources.insert("src/invisible.rs".to_owned(), "fn b() {}".to_owned());

    let report = document_coverage(&doc, &sources);

    assert_eq!(report.files.len(), 2);
    assert!(!report.is_complete());

    let worst = report.worst_first();
    assert_eq!(worst[0].file, "src/invisible.rs");
    assert_eq!(worst[0].ratio(), 0.0);
}

#[test]
fn report_orders_worst_files_first() {
    let mut sources = BTreeMap::new();
    sources.insert("a.rs".to_owned(), "fn a() {}".to_owned());
    sources.insert("b.rs".to_owned(), "fn b() {}".to_owned());

    let mut doc = Document::new("test");
    let mut a = Node::new(NodeId::symbol("rust", "a"), kind::code::FUNCTION, "a");
    a.span = Some(Span::new("a.rs", "fn a() {}", 0, 9));
    doc.insert(a);

    let report = document_coverage(&doc, &sources);
    let order: Vec<&str> = report.worst_first().iter().map(|f| f.file.as_str()).collect();
    assert_eq!(order, vec!["b.rs", "a.rs"]);
}

#[test]
fn gap_preview_is_whitespace_collapsed() {
    let source = "fn a() {}\n    let   x = 1;\n";
    let end = source.find('\n').unwrap() as u32 + 1;
    let report = file_coverage(FILE, source, &[span(source, 0, end)]);

    let gap = report.gaps.iter().find(|g| g.reason.is_failure()).unwrap();
    assert_eq!(gap.preview, "let x = 1;");
}

#[test]
fn parent_spans_do_not_cover_their_children() {
    // A structural parent span must never satisfy the gate on its own,
    // otherwise every file with one top-level item is trivially 100%.
    let source = "fn a() {}\nfn b() {}\n";

    let mut doc = Document::new("test");
    let mut parent = Node::new(NodeId::from("rust://demo"), kind::code::MODULE, "demo");
    parent.span = Some(Span::new(FILE, source, 0, source.len() as u32));
    doc.insert(parent);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), kind::code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.span = Some(Span::new(FILE, source, 0, 9));
    doc.insert(a);

    let mut sources = BTreeMap::new();
    sources.insert(FILE.to_owned(), source.to_owned());

    let report = document_coverage(&doc, &sources);
    assert!(
        !report.is_complete(),
        "fn b is unclaimed and must fail even though the module span covers it"
    );

    // Add the missing child: now the frontier covers the file.
    let mut b = Node::new(NodeId::symbol("rust", "demo::b"), kind::code::FUNCTION, "demo::b");
    b.parent = Some(NodeId::from("rust://demo"));
    b.span = Some(Span::new(FILE, source, 10, source.len() as u32));
    doc.insert(b);

    let report = document_coverage(&doc, &sources);
    assert!(report.is_complete());
}
