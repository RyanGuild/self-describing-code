# `robby_render::tree::item_children`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_children<'a>(doc: &'a Document, node: &Node) -> Vec<&'a Node>` |
| location | [`crates/robby-render/src/tree.rs:104`](../../../../crates/robby-render/src/tree.rs#L104) |
| provenance | extractor · confidence 1.00 |

> Direct, non-module, non-trivial children of a module, in source order.

```rust
/// Direct, non-module, non-trivial children of a module, in source order.
fn item_children<'a>(doc: &'a Document, node: &Node) -> Vec<&'a Node> {
    node.children
        .iter()
        .filter_map(|child| doc.get(child))
        .filter(|child| !is_module(child) && !is_compact(child.kind.as_str()))
        .collect()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant item_children as item_children()
    participant iter as iter
    participant get as get
    participant filter_map as filter_map
    participant is_module as is_module
    participant as_str as as_str
    participant is_compact as is_compact
    participant filter as filter
    participant collect as collect
    item_children-->>iter: node.children .iter()
    item_children-->>get: doc.get(child)
    item_children-->>filter_map: node.children .iter() .filter_map(|child| doc.get(child))
    item_children-->>is_module: is_module(child)
    item_children-->>as_str: child.kind.as_str()
    item_children-->>is_compact: is_compact(child.kind.as_str())
    item_children-->>filter: node.children .iter() .filter_map(|child| doc.get(child)) .filter(|ch...
    item_children-->>collect: node.children .iter() .filter_map(|child| doc.get(child)) .filter(|ch...
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::is_module`](is-module.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::tree::is_compact`](is-compact.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)

## References

- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — get
- calls → [`robby_render::tree`](index.md) — is_compact, is_module
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_render::tree::module_pages`](module-pages.md)
- calls ← [`robby_render::tree::page_of_node`](page-of-node.md)

