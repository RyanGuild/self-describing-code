# `robby_render::view::entrypoints::CallTree`

[Self-model](../../../README.md) · [view::entrypoints](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/entrypoints.rs:14`](../../../../../crates/robby-render/src/view/entrypoints.rs#L14) |
| provenance | extractor · confidence 1.00 |

> Containment tree over `Calls` / code paths: first parent wins; extras are cross edges.

```rust
/// Containment tree over `Calls` / code paths: first parent wins; extras are cross edges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallTree {
    /// Entrypoint node ids (sorted).
    pub entries: Vec<NodeId>,
    /// Parent → ordered children in the tree.
    pub children: BTreeMap<NodeId, Vec<NodeId>>,
    /// Every node placed in the tree (entries + reachable callees).
    pub placed: BTreeSet<NodeId>,
    /// Resolved `Calls` between placed nodes that are not tree edges.
    pub cross_edges: Vec<(NodeId, NodeId)>,
}
```

**Fields and variants**

- `entries` — `crates/robby-render/src/view/entrypoints.rs:17:5`
- `children` — `crates/robby-render/src/view/entrypoints.rs:19:5`
- `placed` — `crates/robby-render/src/view/entrypoints.rs:21:5`
- `cross_edges` — `crates/robby-render/src/view/entrypoints.rs:23:5`

