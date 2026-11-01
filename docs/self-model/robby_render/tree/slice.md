# `robby_render::tree::slice`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn slice<'a>(sources: &'a BTreeMap<String, String>, span: &robby_ir::Span) -> Option<&'a str>` |
| location | [`crates/robby-render/src/tree.rs:624`](../../../../crates/robby-render/src/tree.rs#L624) |
| provenance | extractor · confidence 1.00 |

```rust
fn slice<'a>(sources: &'a BTreeMap<String, String>, span: &robby_ir::Span) -> Option<&'a str> {
    sources
        .get(&span.file)?
        .get(span.start.byte as usize..span.end.byte as usize)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant slice as slice()
    participant get as get
    slice-->>get: sources .get(&span.file)? .get(span.start.byte as usize..span.end.byt...
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)

