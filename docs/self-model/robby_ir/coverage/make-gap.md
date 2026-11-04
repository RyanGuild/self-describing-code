# `robby_ir::coverage::make_gap`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn make_gap(file: &str, source: &str, bytes: &[u8], start: u32, end: u32) -> Gap` |
| location | [`crates/robby-ir/src/coverage.rs:253`](../../../../crates/robby-ir/src/coverage.rs#L253) |
| provenance | extractor · confidence 1.00 |

```rust
fn make_gap(file: &str, source: &str, bytes: &[u8], start: u32, end: u32) -> Gap {
    let mode = TriviaMode::from_path(file);
    let reason = match scan_trivia(bytes, start, end, mode) {
        Ok(flags) => flags.classify(),
        Err(at) => GapReason::Unknown {
            at: Position::at(source, at),
        },
    };
    Gap {
        span: Span::new(file, source, start, end),
        reason,
        preview: preview(source, start, end),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant make_gap as make_gap()
    participant TriviaMode__from_path as TriviaMode::from_path
    participant scan_trivia as scan_trivia
    participant classify as classify
    participant Position__at as Position::at
    participant Span__new as Span::new
    participant preview as preview
    make_gap-->>TriviaMode__from_path: TriviaMode::from_path(file)
    make_gap-->>scan_trivia: scan_trivia(bytes, start, end, mode)
    alt match scan_trivia(bytes, start, end, mode)
    make_gap-->>classify: flags.classify()
    else Err(at)
    make_gap-->>Position__at: Position::at(source, at)
    end
    make_gap-->>Span__new: Span::new(file, source, start, end)
    make_gap-->>preview: preview(source, start, end)
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::coverage::impl TriviaMode::from_path`](index.md)
- calls → [`robby_ir::coverage::scan_trivia`](scan-trivia.md)
- calls → [`robby_ir::coverage::impl TriviaFlags::classify`](index.md)
- calls → [`robby_ir::span::impl Position::at`](../span/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::coverage::preview`](preview.md)

## References

- calls → [`robby_ir::coverage`](index.md) — classify, from_path, preview, scan_trivia
- calls → [`robby_ir::kind`](../kind/index.md) — new
- calls → [`robby_ir::span`](../span/index.md) — at

## Referenced by

- calls ← [`robby_ir::coverage::file_coverage`](file-coverage.md)

