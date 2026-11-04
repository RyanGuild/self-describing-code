# `robby_ir::codepath::find_entrypoints`

[Self-model](../../README.md) · [codepath](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn find_entrypoints(doc: &Document) -> Vec<NodeId>` |
| location | [`crates/robby-ir/src/codepath.rs:108`](../../../../crates/robby-ir/src/codepath.rs#L108) |
| provenance | extractor · confidence 1.00 |

> Functions named `main` (MVP entrypoint set), sorted by FQN.

```rust
/// Functions named `main` (MVP entrypoint set), sorted by FQN.
pub fn find_entrypoints(doc: &Document) -> Vec<NodeId> {
    let mut entries: Vec<&crate::Node> = doc
        .nodes
        .values()
        .filter(|node| {
            node.name == "main" && node.kind.as_str() == kind::code::FUNCTION
        })
        .collect();
    entries.sort_by(|a, b| a.fqn.cmp(&b.fqn).then_with(|| a.id.as_str().cmp(b.id.as_str())));
    entries.into_iter().map(|n| n.id.clone()).collect()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant find_entrypoints as find_entrypoints()
    participant values as values
    participant as_str as as_str
    participant filter as filter
    participant collect as collect
    participant cmp as cmp
    participant then_with as then_with
    participant sort_by as sort_by
    participant into_iter as into_iter
    participant clone as clone
    participant map as map
    find_entrypoints-->>values: doc .nodes .values()
    find_entrypoints-->>as_str: node.kind.as_str()
    find_entrypoints-->>filter: doc .nodes .values() .filter(|node| { node.name == 'main' && node.kin...
    find_entrypoints-->>collect: doc .nodes .values() .filter(|node| { node.name == 'main' && node.kin...
    find_entrypoints-->>cmp: a.fqn.cmp(&b.fqn)
    find_entrypoints-->>as_str: a.id.as_str()
    find_entrypoints-->>as_str: b.id.as_str()
    find_entrypoints-->>cmp: a.id.as_str().cmp(b.id.as_str())
    find_entrypoints-->>then_with: a.fqn.cmp(&b.fqn).then_with(|| a.id.as_str().cmp(b.id.as_str()))
    find_entrypoints-->>sort_by: entries.sort_by(|a, b| a.fqn.cmp(&b.fqn).then_with(|| a.id.as_str().c...
    find_entrypoints-->>into_iter: entries.into_iter()
    find_entrypoints-->>clone: n.id.clone()
    find_entrypoints-->>map: entries.into_iter().map(|n| n.id.clone())
    find_entrypoints-->>collect: entries.into_iter().map(|n| n.id.clone()).collect()
```

14 steps · 0 observed · 14 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../kind/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../kind/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_ir::kind`](../kind/index.md) — as_str
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_ir::codepath::build_code_paths`](build-code-paths.md)
- calls ← [`robby_render::view::entrypoints::impl CallTree::from_document`](../../robby_render/view/entrypoints/index.md)

