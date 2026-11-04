# `robby_render::view::sibling::SiblingGraph`

[Self-model](../../../README.md) · [view::sibling](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/sibling.rs:7`](../../../../../crates/robby-render/src/view/sibling.rs#L7) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingGraph {
    pub root_id: String,
    pub root_label: String,
    pub nodes: Vec<SiblingNode>,
    pub edges: Vec<SiblingEdge>,
}
```

**Fields and variants**

- `root_id` — `crates/robby-render/src/view/sibling.rs:9:5`
- `root_label` — `crates/robby-render/src/view/sibling.rs:10:5`
- `nodes` — `crates/robby-render/src/view/sibling.rs:11:5`
- `edges` — `crates/robby-render/src/view/sibling.rs:12:5`

