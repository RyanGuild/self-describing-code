# `robby_render::view::deps::PackageNode`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:48`](../../../../../crates/robby-render/src/view/deps.rs#L48) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageNode {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub children: Vec<PackageNode>,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/deps.rs:50:5`
- `stable_id` — `crates/robby-render/src/view/deps.rs:51:5`
- `name` — `crates/robby-render/src/view/deps.rs:52:5`
- `fqn` — `crates/robby-render/src/view/deps.rs:53:5`
- `children` — `crates/robby-render/src/view/deps.rs:54:5`

