# `robby_agent::model::format_context`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn format_context(ctx: &CommentContext) -> String` |
| location | [`crates/robby-agent/src/model.rs:177`](../../../../crates/robby-agent/src/model.rs#L177) |
| provenance | extractor · confidence 1.00 |

```rust
fn format_context(ctx: &CommentContext) -> String {
    let owner = ctx.owner_fqn.as_deref().unwrap_or("(none)");
    let location = ctx.location.as_deref().unwrap_or("(unknown)");
    format!(
        "file: {}\nlocation: {}\nowner: {}\n\ncomment:\n{}\n\ncode:\n{}",
        ctx.file, location, owner, ctx.comment_text, ctx.code_window
    )
}
```

```mermaid
sequenceDiagram
    autonumber
    participant format_context as format_context()
    participant as_deref as as_deref
    participant unwrap_or as unwrap_or
    format_context-->>as_deref: ctx.owner_fqn.as_deref()
    format_context-->>unwrap_or: ctx.owner_fqn.as_deref().unwrap_or('(none)')
    format_context-->>as_deref: ctx.location.as_deref()
    format_context-->>unwrap_or: ctx.location.as_deref().unwrap_or('(unknown)')
    format_context-->>format_context: format!( 'file: {}\nlocation: {}\nowner: {}\n\ncomment:\n{}\n\ncode:\...
```

5 steps · 0 observed · 5 possible only

## Referenced by

- calls ← [`robby_agent::model::impl DecisionModel<'a, C>::decide`](index.md)
- calls ← [`robby_agent::model::impl NoteModel<'a, C>::write_note`](index.md)

