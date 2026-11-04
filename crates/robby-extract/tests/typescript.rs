//! TypeScript extractor fixture tests (`tests/fixtures/ts-mini`).

use std::collections::BTreeMap;
use std::path::PathBuf;

use robby_ir::{EdgeKind, Lang, kind};

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/ts-mini")
}

#[test]
fn extracts_typescript_package_with_lang_tag() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let pkg = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::PACKAGE && n.name == "ts-mini")
        .expect("package node");
    assert_eq!(pkg.lang, Lang::TypeScript);

    assert!(
        doc.nodes
            .values()
            .any(|n| n.kind.as_str() == kind::code::CLASS && n.name == "Point"),
        "expected Point class"
    );
    assert!(
        doc.nodes
            .values()
            .any(|n| n.kind.as_str() == kind::code::INTERFACE && n.name == "Drawable"),
        "expected Drawable interface"
    );
}

#[test]
fn resolves_calls_and_imports() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let draw = doc
        .nodes
        .values()
        .find(|n| n.fqn.ends_with("/Point/draw") || n.fqn.ends_with("Point/draw"))
        .expect("Point.draw");
    assert!(
        draw.edges.iter().any(|e| e.kind == EdgeKind::Calls),
        "draw should call helper: {:?}",
        draw.edges
    );

    let shapes = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::MODULE && n.fqn.contains("shapes"))
        .expect("shapes module");
    let import = doc
        .nodes
        .values()
        .find(|n| {
            n.kind.as_str() == kind::code::IMPORT
                && n.parent.as_ref() == Some(&shapes.id)
                && n.fqn.contains("import:./util")
        })
        .expect("relative import node");
    assert!(
        import.edges.iter().any(|e| e.kind == EdgeKind::Imports),
        "import should resolve: {:?}",
        import.edges
    );

    let point = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::CLASS && n.name == "Point")
        .expect("Point");
    assert!(
        point.edges.iter().any(|e| e.kind == EdgeKind::Implements),
        "Point implements Drawable"
    );
}

#[test]
fn bare_import_creates_external_npm_stub() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let stub = doc
        .nodes
        .values()
        .find(|n| n.fqn == "npm/demo-ext")
        .expect("npm/demo-ext stub");
    assert_eq!(stub.kind.as_str(), kind::code::PACKAGE);
    assert_eq!(stub.lang, Lang::TypeScript);
    assert!(stub.tags.iter().any(|t| t == "external"));
    assert_eq!(
        stub.annotations
            .get("external")
            .map(std::string::String::as_str),
        Some("true")
    );

    let bare = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::IMPORT && n.fqn.contains("import:demo-ext"))
        .expect("bare import");
    assert!(
        bare.edges
            .iter()
            .any(|e| { e.kind == EdgeKind::Imports && e.to == stub.id }),
        "bare import should edge to stub: {:?}",
        bare.edges
    );
}

#[test]
fn emits_comment_nodes_and_attaches_docs() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let comments: Vec<_> = doc
        .nodes
        .values()
        .filter(|n| n.kind.as_str() == kind::code::COMMENT)
        .collect();
    assert!(
        comments.len() >= 2,
        "expected line + block comments, got {comments:?}"
    );
    assert!(
        comments.iter().any(|c| {
            c.annotations
                .get("comment_kind")
                .map(std::string::String::as_str)
                == Some("block")
                && c.docs.as_deref().is_some_and(|d| d.contains("drawable"))
        }),
        "missing JSDoc block: {comments:?}"
    );
    assert!(
        comments.iter().any(|c| {
            c.annotations
                .get("comment_kind")
                .map(std::string::String::as_str)
                == Some("line")
                && c.docs.as_deref().is_some_and(|d| d.contains("canvas"))
        }),
        "missing line comment: {comments:?}"
    );

    let drawable = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::INTERFACE && n.name == "Drawable")
        .expect("Drawable");
    assert!(
        drawable
            .docs
            .as_deref()
            .is_some_and(|d| d.contains("drawable")),
        "JSDoc should attach to Drawable: {:?}",
        drawable.docs
    );
}

#[test]
fn typescript_fixture_is_fully_covered() {
    let root = fixture();
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
        .map(|(file, gap)| format!("{}:{} {:?}", file.file, gap.span.location(), gap.preview))
        .collect();

    assert!(report.is_complete(), "unclaimed TypeScript: {failures:#?}");
}
