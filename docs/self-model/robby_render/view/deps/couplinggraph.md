# `robby_render::view::deps::CouplingGraph`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:75`](../../../../../crates/robby-render/src/view/deps.rs#L75) |
| provenance | extractor · confidence 1.00 |

> Internal call coupling: modules as nested subgraphs, functions as nodes.

```rust
/// Internal call coupling: modules as nested subgraphs, functions as nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingGraph {
    pub root: CouplingModule,
    pub functions: Vec<CouplingFn>,
    pub edges: Vec<WeightedEdge>,
    /// Workspace mode: modules are leaf nodes (no functions).
    pub module_nodes: Vec<CouplingFn>,
}
```

**Fields and variants**

- `root` — `crates/robby-render/src/view/deps.rs:78:5`
- `functions` — `crates/robby-render/src/view/deps.rs:79:5`
- `edges` — `crates/robby-render/src/view/deps.rs:80:5`
- `module_nodes` — `crates/robby-render/src/view/deps.rs:81:5`

