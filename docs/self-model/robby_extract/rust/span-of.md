# `robby_extract::rust::span_of`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn span_of<T: Spanned>(node: &T, ctx: &Ctx<'_>) -> Span` |
| location | [`crates/robby-extract/src/rust.rs:505`](../../../../crates/robby-extract/src/rust.rs#L505) |
| provenance | extractor · confidence 1.00 |

```rust
fn span_of<T: Spanned>(node: &T, ctx: &Ctx<'_>) -> Span {
    let span = node.span();
    let start = ctx.index.byte(span.start());
    let end = ctx.index.byte(span.end());
    Span::new(ctx.file, ctx.source, start, end.max(start))
}
```

```mermaid
sequenceDiagram
    autonumber
    participant span_of as span_of()
    participant span as span
    participant start as start
    participant byte as byte
    participant end as end
    participant max as max
    participant Span__new as Span::new
    span_of-->>span: node.span()
    span_of-->>start: span.start()
    span_of-->>byte: ctx.index.byte(span.start())
    span_of-->>end: span.end()
    span_of-->>byte: ctx.index.byte(span.end())
    span_of-->>max: end.max(start)
    span_of-->>Span__new: Span::new(ctx.file, ctx.source, start, end.max(start))
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::test_coverage::span`](../../robby_ir/test_coverage/span.md)
- calls → [`robby_extract::rust::impl LineIndex::byte`](index.md)
- calls → [`robby_extract::rust::impl LineIndex::byte`](index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)

## References

- calls → [`robby_extract::rust`](index.md) — byte, new
- calls → [`robby_ir::test_coverage`](../../robby_ir/test_coverage/index.md) — span

## Referenced by

- calls ← [`robby_extract::rust::signature_of`](signature-of.md)
- calls ← [`robby_extract::rust::walk_expr`](walk-expr.md)
- calls ← [`robby_extract::rust::walk_items`](walk-items.md)
- calls ← [`robby_extract::rust::walk_stmts`](walk-stmts.md)

