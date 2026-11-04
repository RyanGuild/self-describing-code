# `robby_extract::test_extract::the_workspace_is_fully_covered`

[Self-model](../../README.md) · [test_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn the_workspace_is_fully_covered()` |
| location | [`crates/robby-extract/tests/extract.rs:27`](../../../../crates/robby-extract/tests/extract.rs#L27) |
| provenance | extractor · confidence 1.00 |

```rust
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

    assert!(
        report.is_complete(),
        "unclaimed code: {failures:#?}"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant the_workspace_is_fully_covered as the_workspace_is_fully_covered()
    participant root as root
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
    the_workspace_is_fully_covered-->>root: root()
    the_workspace_is_fully_covered-->>robby_extract__extract: robby_extract::extract(&root)
    the_workspace_is_fully_covered-->>expect: robby_extract::extract(&root).expect('extract')
    the_workspace_is_fully_covered-->>BTreeMap__new: BTreeMap::new()
    loop &doc.files
    the_workspace_is_fully_covered-->>join: root.join(file)
    the_workspace_is_fully_covered-->>std__fs__read_to_string: std::fs::read_to_string(root.join(file))
    the_workspace_is_fully_covered-->>the_workspace_is_fully_covered: panic!('reading {file}: {err}')
    the_workspace_is_fully_covered-->>unwrap_or_else: std::fs::read_to_string(root.join(file)) .unwrap_or_else(|err| panic!...
    the_workspace_is_fully_covered-->>clone: file.clone()
    the_workspace_is_fully_covered-->>insert: sources.insert(file.clone(), text)
    end
    the_workspace_is_fully_covered-->>robby_ir__coverage__document_coverage: robby_ir::coverage::document_coverage(&doc, &sources)
    the_workspace_is_fully_covered-->>failures: report .failures()
    the_workspace_is_fully_covered-->>the_workspace_is_fully_covered: format!('{}:{}', file.file, gap.span.location())
    the_workspace_is_fully_covered-->>map: report .failures() .map(|(file, gap)| format!('{}:{}', file.file, gap...
    the_workspace_is_fully_covered-->>collect: report .failures() .map(|(file, gap)| format!('{}:{}', file.file, gap...
    the_workspace_is_fully_covered-->>the_workspace_is_fully_covered: assert!( report.is_complete(), 'unclaimed code: {failures: ?}' )
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_extract::test_extract::root`](root.md)
- calls → [`robby_extract::extract`](../extract.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::coverage::document_coverage`](../../robby_ir/coverage/document-coverage.md)
- calls → [`robby_ir::coverage::impl CoverageReport::failures`](../../robby_ir/coverage/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_extract`](../index.md) — extract
- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::test_extract`](index.md) — root
- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — document_coverage, failures
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

