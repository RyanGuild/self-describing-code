# `robby_render::view::deps::package_leaf`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn package_leaf(node: &Node) -> PackageNode` |
| location | [`crates/robby-render/src/view/deps.rs:636`](../../../../../crates/robby-render/src/view/deps.rs#L636) |
| provenance | extractor · confidence 1.00 |

```rust
fn package_leaf(node: &Node) -> PackageNode {
    PackageNode {
        id: node.id.clone(),
        stable_id: stable_id(node.id.as_str()),
        name: node.name.clone(),
        fqn: node.fqn.clone(),
        children: Vec::new(),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant package_leaf as package_leaf()
    participant clone as clone
    participant as_str as as_str
    participant stable_id as stable_id
    participant Vec__new as Vec::new
    package_leaf-->>clone: node.id.clone()
    package_leaf-->>as_str: node.id.as_str()
    package_leaf-->>stable_id: stable_id(node.id.as_str())
    package_leaf-->>clone: node.name.clone()
    package_leaf-->>clone: node.fqn.clone()
    package_leaf-->>Vec__new: Vec::new()
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)

## References

- calls → [`robby_render`](../../index.md) — stable_id
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::scene`](../scene/index.md) — new

## Referenced by

- calls ← [`robby_render::view::deps::build_package_tree`](build-package-tree.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](index.md)
- calls ← [`robby_render::view::deps::impl PackageStructure::workspace`](index.md)

