# `robby_render::test_markdown::report_is_byte_stable`

[Self-model](../../README.md) · [test_markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn report_is_byte_stable()` |
| location | [`crates/robby-render/tests/markdown.rs:47`](../../../../crates/robby-render/tests/markdown.rs#L47) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn report_is_byte_stable() {
    let (doc, sources) = fixture();
    let first = report(&doc, None, Some(&sources));
    let second = report(&doc, None, Some(&sources));
    assert_eq!(first, second);
}
```

```mermaid
sequenceDiagram
    autonumber
    participant report_is_byte_stable as report_is_byte_stable()
    participant fixture as fixture
    participant report as report
    report_is_byte_stable-->>fixture: fixture()
    report_is_byte_stable-->>report: report(&doc, None, Some(&sources))
    report_is_byte_stable-->>report: report(&doc, None, Some(&sources))
    report_is_byte_stable-->>report_is_byte_stable: assert_eq!(first, second)
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::test_markdown::fixture`](fixture.md)
- calls → [`robby_cli::report`](../../robby_cli/report.md)
- calls → [`robby_cli::report`](../../robby_cli/report.md)

