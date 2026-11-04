# `robby_ir::test_coverage::nested_spans_are_unioned_not_double_counted`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn nested_spans_are_unioned_not_double_counted()` |
| location | [`crates/robby-ir/tests/coverage.rs:31`](../../../../crates/robby-ir/tests/coverage.rs#L31) |
| provenance | extractor · confidence 1.00 |

```rust
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
```

```mermaid
sequenceDiagram
    autonumber
    participant nested_spans_are_unioned_not_double_counted as nested_spans_are_unioned_not_double_counted()
    participant len as len
    participant span as span
    participant file_coverage as file_coverage
    nested_spans_are_unioned_not_double_counted-->>len: source.len()
    nested_spans_are_unioned_not_double_counted-->>span: span(source, 0, source.len() as u32)
    nested_spans_are_unioned_not_double_counted-->>span: span(source, 3, 8)
    nested_spans_are_unioned_not_double_counted-->>span: span(source, 4, 6)
    nested_spans_are_unioned_not_double_counted-->>file_coverage: file_coverage(FILE, source, &spans)
    nested_spans_are_unioned_not_double_counted-->>nested_spans_are_unioned_not_double_counted: assert!(report.is_complete())
    nested_spans_are_unioned_not_double_counted-->>nested_spans_are_unioned_not_double_counted: assert_eq!(report.covered_bytes, source.len() as u32)
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::test_coverage::span`](span.md)
- calls → [`robby_ir::test_coverage::span`](span.md)
- calls → [`robby_ir::test_coverage::span`](span.md)
- calls → [`robby_ir::coverage::file_coverage`](../coverage/file-coverage.md)

## References

- calls → [`robby_ir::coverage`](../coverage/index.md) — file_coverage
- calls → [`robby_ir::test_coverage`](index.md) — span

