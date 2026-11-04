# `robby_render::view::comments::CommentEntry`

[Self-model](../../../README.md) · [view::comments](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/comments.rs:12`](../../../../../crates/robby-render/src/view/comments.rs#L12) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentEntry {
    pub location: Option<String>,
    pub kind: CommentKind,
    pub text: String,
    pub owner_fqn: Option<String>,
    pub code: Option<String>,
    pub verdict: Option<Verdict>,
    pub note_kind: Option<NoteKind>,
    pub note: Option<String>,
    /// Byte offsets into `code`.
    pub highlights: Vec<(u32, u32)>,
}
```

**Fields and variants**

- `location` — `crates/robby-render/src/view/comments.rs:14:5`
- `kind` — `crates/robby-render/src/view/comments.rs:15:5`
- `text` — `crates/robby-render/src/view/comments.rs:16:5`
- `owner_fqn` — `crates/robby-render/src/view/comments.rs:17:5`
- `code` — `crates/robby-render/src/view/comments.rs:18:5`
- `verdict` — `crates/robby-render/src/view/comments.rs:19:5`
- `note_kind` — `crates/robby-render/src/view/comments.rs:20:5`
- `note` — `crates/robby-render/src/view/comments.rs:21:5`
- `highlights` — `crates/robby-render/src/view/comments.rs:22:5`

