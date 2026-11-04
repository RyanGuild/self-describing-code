# `robby_extract::test_typescript::typescript_fixture_is_fully_covered`

[Self-model](../../README.md) · [test_typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn typescript_fixture_is_fully_covered()` |
| location | [`crates/robby-extract/tests/typescript.rs:162`](../../../../crates/robby-extract/tests/typescript.rs#L162) |
| provenance | extractor · confidence 1.00 |

```rust
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

    assert!(
        report.is_complete(),
        "unclaimed TypeScript: {failures:#?}"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant typescript_fixture_is_fully_covered as typescript_fixture_is_fully_covered()
    participant fixture as fixture
    participant robby_extract__extract as robby_extract::extract
    participant expect as expect
    participant BTreeMap__new as BTreeMap::new
    participant join as join
    participant std__fs__read_to_string as std::fs::read_to_string
    participant unwrap_or_else as unwrap_or_else
    participant clone as clone
    participant insert as insert
    participant robby_ir__coverage__document_coverage as robby_ir::coverage::document_coverage
    participant failures as failures
    participant map as map
    participant collect as collect
    typescript_fixture_is_fully_covered-->>fixture: fixture()
    typescript_fixture_is_fully_covered-->>robby_extract__extract: robby_extract::extract(&root)
    typescript_fixture_is_fully_covered-->>expect: robby_extract::extract(&root).expect('extract')
    typescript_fixture_is_fully_covered-->>BTreeMap__new: BTreeMap::new()
    loop &doc.files
    typescript_fixture_is_fully_covered-->>join: root.join(file)
    typescript_fixture_is_fully_covered-->>std__fs__read_to_string: std::fs::read_to_string(root.join(file))
    typescript_fixture_is_fully_covered-->>typescript_fixture_is_fully_covered: panic!('reading {file}: {err}')
    typescript_fixture_is_fully_covered-->>unwrap_or_else: std::fs::read_to_string(root.join(file)) .unwrap_or_else(|err| panic!...
    typescript_fixture_is_fully_covered-->>clone: file.clone()
    typescript_fixture_is_fully_covered-->>insert: sources.insert(file.clone(), text)
    end
    typescript_fixture_is_fully_covered-->>robby_ir__coverage__document_coverage: robby_ir::coverage::document_coverage(&doc, &sources)
    typescript_fixture_is_fully_covered-->>failures: report .failures()
    typescript_fixture_is_fully_covered-->>typescript_fixture_is_fully_covered: format!('{}:{} {:?}', file.file, gap.span.location(), gap.preview)
    typescript_fixture_is_fully_covered-->>map: report .failures() .map(|(file, gap)| format!('{}:{} {:?}', file.file...
    typescript_fixture_is_fully_covered-->>collect: report .failures() .map(|(file, gap)| format!('{}:{} {:?}', file.file...
    typescript_fixture_is_fully_covered-->>typescript_fixture_is_fully_covered: assert!( report.is_complete(), 'unclaimed TypeScript: {failures: ?}' )
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_extract::test_typescript::fixture`](fixture.md)
- calls → [`robby_extract::extract`](../extract.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::coverage::document_coverage`](../../robby_ir/coverage/document-coverage.md)
- calls → [`robby_ir::coverage::impl CoverageReport::failures`](../../robby_ir/coverage/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_extract`](../index.md) — extract
- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::test_typescript`](index.md) — fixture
- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — document_coverage, failures
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

