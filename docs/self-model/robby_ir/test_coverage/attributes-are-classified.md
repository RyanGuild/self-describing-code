# `robby_ir::test_coverage::attributes_are_classified`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn attributes_are_classified()` |
| location | [`crates/robby-ir/tests/coverage.rs:57`](../../../../crates/robby-ir/tests/coverage.rs#L57) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn attributes_are_classified() {
    let source = "#[derive(Debug)]\nfn a() {}\n";
    let start = source.find("fn a").unwrap() as u32;
    let report = file_coverage(FILE, source, &[span(source, start, source.len() as u32)]);

    assert!(report.is_complete());
    assert_eq!(report.gaps.len(), 1);
    assert!(matches!(report.gaps[0].reason, GapReason::Attribute));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant attributes_are_classified as attributes_are_classified()
    participant find as find
    participant unwrap as unwrap
    participant len as len
    participant span as span
    participant file_coverage as file_coverage
    attributes_are_classified-->>find: source.find('fn a')
    attributes_are_classified-->>unwrap: source.find('fn a').unwrap()
    attributes_are_classified-->>len: source.len()
    attributes_are_classified-->>span: span(source, start, source.len() as u32)
    attributes_are_classified-->>file_coverage: file_coverage(FILE, source, &[span(source, start, source.len() as u32)])
    attributes_are_classified-->>attributes_are_classified: assert!(report.is_complete())
    attributes_are_classified-->>attributes_are_classified: assert_eq!(report.gaps.len(), 1)
    attributes_are_classified-->>attributes_are_classified: assert!(matches!(report.gaps[0].reason, GapReason::Attribute))
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_ir::test_coverage::span`](span.md)
- calls → [`robby_ir::coverage::file_coverage`](../coverage/file-coverage.md)

## References

- calls → [`robby_ir::coverage`](../coverage/index.md) — file_coverage
- calls → [`robby_ir::test_coverage`](index.md) — span

