# `robby_render::view::types::ClassView`

[Self-model](../../../README.md) · [view::types](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/types.rs:15`](../../../../../crates/robby-render/src/view/types.rs#L15) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassView {
    /// Sanitized identifier safe for Mermaid class names.
    pub id: String,
    pub name: String,
    pub stereotype: Option<ClassStereotype>,
    pub members: Vec<ClassMember>,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/types.rs:17:5`
- `name` — `crates/robby-render/src/view/types.rs:19:5`
- `stereotype` — `crates/robby-render/src/view/types.rs:20:5`
- `members` — `crates/robby-render/src/view/types.rs:21:5`

