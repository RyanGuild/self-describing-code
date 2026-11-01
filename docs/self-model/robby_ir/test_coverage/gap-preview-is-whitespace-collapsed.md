# `robby_ir::test_coverage::gap_preview_is_whitespace_collapsed`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn gap_preview_is_whitespace_collapsed()` |
| location | [`crates/robby-ir/tests/coverage.rs:158`](../../../../crates/robby-ir/tests/coverage.rs#L158) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn gap_preview_is_whitespace_collapsed() {
    let source = "fn a() {}\n    let   x = 1;\n";
    let end = source.find('\n').unwrap() as u32 + 1;
    let report = file_coverage(FILE, source, &[span(source, 0, end)]);

    let gap = report.gaps.iter().find(|g| g.reason.is_failure()).unwrap();
    assert_eq!(gap.preview, "let x = 1;");
}
```

```mermaid
sequenceDiagram
    autonumber
    participant gap_preview_is_whitespace_collapsed as gap_preview_is_whitespace_collapsed()
    participant file_coverage as file_coverage
    participant unwrap as unwrap
    gap_preview_is_whitespace_collapsed-->>gap_preview_is_whitespace_collapsed: 'fn a() {}\n let x = 1 \n'
    gap_preview_is_whitespace_collapsed-->>gap_preview_is_whitespace_collapsed: source.find('\n').unwrap() as u32 + 1
    gap_preview_is_whitespace_collapsed-->>file_coverage: file_coverage(FILE, source, &[span(source, 0, end)])
    gap_preview_is_whitespace_collapsed-->>unwrap: report.gaps.iter().find(|g| g.reason.is_failure()).unwrap()
    gap_preview_is_whitespace_collapsed-->>gap_preview_is_whitespace_collapsed: assert_eq!(gap.preview, 'let x = 1 ')
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_ir::coverage::file_coverage`](../coverage/file-coverage.md)

