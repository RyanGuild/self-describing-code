# `robby_render::view::hierarchy::HierarchyNode`

[Self-model](../../../README.md) · [view::hierarchy](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/hierarchy.rs:10`](../../../../../crates/robby-render/src/view/hierarchy.rs#L10) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyNode {
    pub id: NodeId,
    pub name: String,
    pub kind_tag: String,
    pub children: Vec<HierarchyNode>,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/hierarchy.rs:12:5`
- `name` — `crates/robby-render/src/view/hierarchy.rs:13:5`
- `kind_tag` — `crates/robby-render/src/view/hierarchy.rs:14:5`
- `children` — `crates/robby-render/src/view/hierarchy.rs:15:5`

