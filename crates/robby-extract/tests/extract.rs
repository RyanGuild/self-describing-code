//! The extractor's job is to make the workspace visible. These tests are the
//! guard rails: they fail if a file, or the depth of a process, silently
//! disappears.

use std::collections::BTreeMap;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn every_source_file_is_declared() {
    let root = root();
    let doc = robby_extract::extract(&root).expect("extract");
    let expected = robby_extract::source_file_count(&root).expect("count");

    assert!(expected > 0, "workspace should contain Rust sources");
    assert_eq!(
        doc.files.len(),
        expected,
        "a source file was skipped: {:?}",
        doc.files
    );
}

#[test]
fn the_workspace_is_fully_covered() {
    let root = root();
    let doc = robby_extract::extract(&root).expect("extract");

    let mut sources = BTreeMap::new();
    for file in &doc.files {
        let text = std::fs::read_to_string(root.join(file))
            .unwrap_or_else(|err| panic!("reading {file}: {err}"));
        sources.insert(file.clone(), text);
    }

    let report = robby_ir::coverage::document_coverage(&doc, &sources);
    let failures: Vec<String> = report
        .failures()
        .map(|(file, gap)| format!("{}:{}", file.file, gap.span.location()))
        .collect();

    assert!(report.is_complete(), "unclaimed code: {failures:#?}");
}

#[test]
fn functions_produce_process_steps() {
    let root = root();
    let doc = robby_extract::extract(&root).expect("extract");

    let extract_fn = doc
        .nodes
        .values()
        .find(|node| node.fqn == "robby_extract::extract")
        .expect("extract fn");

    let process = extract_fn.process.as_ref().expect("process");
    assert!(
        process.steps.len() >= 3,
        "expected the body to produce steps, got {}",
        process.steps.len()
    );
    assert!(process.participants.len() >= 2);
}

#[test]
fn spans_point_at_the_right_bytes() {
    let root = root();
    let doc = robby_extract::extract(&root).expect("extract");

    let node = doc
        .nodes
        .values()
        .find(|n| n.fqn == "robby_ir::coverage::file_coverage")
        .expect("file_coverage");

    let span = node.span.as_ref().expect("span");
    let source = std::fs::read_to_string(root.join(&span.file)).expect("source");
    let slice = &source[span.start.byte as usize..span.end.byte as usize];

    // The span includes the doc comment: attributes belong to the item, and
    // that is what makes coverage close over `///` lines without special cases.
    assert!(
        slice.contains("pub fn file_coverage"),
        "span did not land on the function: {slice:?}"
    );
    assert!(slice.trim_end().ends_with('}'), "span is truncated");
}
