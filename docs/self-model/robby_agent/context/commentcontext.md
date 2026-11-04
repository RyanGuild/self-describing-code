# `robby_agent::context::CommentContext`

[Self-model](../../README.md) · [context](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/context.rs:7`](../../../../crates/robby-agent/src/context.rs#L7) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentContext {
    pub parent_id: NodeId,
    pub comment_text: String,
    pub code_window: String,
    pub owner_fqn: Option<String>,
    pub file: String,
    pub location: Option<String>,
}
```

**Fields and variants**

- `parent_id` — `crates/robby-agent/src/context.rs:9:5`
- `comment_text` — `crates/robby-agent/src/context.rs:10:5`
- `code_window` — `crates/robby-agent/src/context.rs:11:5`
- `owner_fqn` — `crates/robby-agent/src/context.rs:12:5`
- `file` — `crates/robby-agent/src/context.rs:13:5`
- `location` — `crates/robby-agent/src/context.rs:14:5`

