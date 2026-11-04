# `robby_ir::test_coverage::unclaimed_code_fails_with_the_exact_byte`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn unclaimed_code_fails_with_the_exact_byte()` |
| location | [`crates/robby-ir/tests/coverage.rs:68`](../../../../crates/robby-ir/tests/coverage.rs#L68) |
| provenance | extractor · confidence 1.00 |

```rust
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
```

```mermaid
sequenceDiagram
    autonumber
    participant unclaimed_code_fails_with_the_exact_byte as unclaimed_code_fails_with_the_exact_byte()
    participant find as find
    participant unwrap as unwrap
    participant span as span
    participant file_coverage as file_coverage
    participant iter as iter
    participant is_failure as is_failure
    participant expect as expect
    unclaimed_code_fails_with_the_exact_byte-->>find: source.find('\n')
    unclaimed_code_fails_with_the_exact_byte-->>unwrap: source.find('\n').unwrap()
    unclaimed_code_fails_with_the_exact_byte-->>span: span(source, 0, end)
    unclaimed_code_fails_with_the_exact_byte-->>file_coverage: file_coverage(FILE, source, &[span(source, 0, end)])
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: assert!(!report.is_complete(), 'unclaimed code must fail the gate')
    unclaimed_code_fails_with_the_exact_byte-->>iter: report .gaps .iter()
    unclaimed_code_fails_with_the_exact_byte-->>is_failure: g.reason.is_failure()
    unclaimed_code_fails_with_the_exact_byte-->>find: report .gaps .iter() .find(|g| g.reason.is_failure())
    unclaimed_code_fails_with_the_exact_byte-->>expect: report .gaps .iter() .find(|g| g.reason.is_failure()) .expect('a fail...
    alt match failure.reason
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: assert_eq!(at.byte, 10)
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: assert_eq!(at.line, 2)
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: assert_eq!(at.column, 1)
    else ref other
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: panic!('expected Unknown, got {other:?}')
    end
    unclaimed_code_fails_with_the_exact_byte-->>unclaimed_code_fails_with_the_exact_byte: assert_eq!(report.uncovered_bytes(), source.len() as u32 - end)
```

14 steps · 0 observed · 14 possible only

**Edges**

- calls → [`robby_ir::test_coverage::span`](span.md)
- calls → [`robby_ir::coverage::file_coverage`](../coverage/file-coverage.md)
- calls → [`robby_ir::coverage::impl GapReason::is_failure`](../coverage/index.md)

## References

- calls → [`robby_ir::coverage`](../coverage/index.md) — file_coverage, is_failure
- calls → [`robby_ir::test_coverage`](index.md) — span

