# `robby_render::view::deps::PackageStructure`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:36`](../../../../../crates/robby-render/src/view/deps.rs#L36) |
| provenance | extractor · confidence 1.00 |

> Nested source-structure packages + inter-package deps (and externals).

```rust
/// Nested source-structure packages + inter-package deps (and externals).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageStructure {
    pub root: PackageNode,
    pub externals: Vec<DepModule>,
    pub edges: Vec<WeightedEdge>,
    pub layers: Vec<Vec<String>>,
    pub cycles: Vec<Vec<String>>,
    /// Flat index (tree packages + externals) for DSM / lookups.
    pub modules: Vec<DepModule>,
}
```

**Fields and variants**

- `root` — `crates/robby-render/src/view/deps.rs:39:5`
- `externals` — `crates/robby-render/src/view/deps.rs:40:5`
- `edges` — `crates/robby-render/src/view/deps.rs:41:5`
- `layers` — `crates/robby-render/src/view/deps.rs:42:5`
- `cycles` — `crates/robby-render/src/view/deps.rs:43:5`
- `modules` — `crates/robby-render/src/view/deps.rs:44:5`

