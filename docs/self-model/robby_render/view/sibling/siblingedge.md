# `robby_render::view::sibling::SiblingEdge`

[Self-model](../../../README.md) · [view::sibling](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/sibling.rs:24`](../../../../../crates/robby-render/src/view/sibling.rs#L24) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingEdge {
    pub from: String,
    pub to: String,
    pub label: &'static str,
    pub resolved: bool,
    pub kind: EdgeKind,
}
```

**Fields and variants**

- `from` — `crates/robby-render/src/view/sibling.rs:26:5`
- `to` — `crates/robby-render/src/view/sibling.rs:27:5`
- `label` — `crates/robby-render/src/view/sibling.rs:28:5`
- `resolved` — `crates/robby-render/src/view/sibling.rs:29:5`
- `kind` — `crates/robby-render/src/view/sibling.rs:30:5`

