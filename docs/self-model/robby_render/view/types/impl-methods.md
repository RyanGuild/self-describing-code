# `robby_render::view::types::impl_methods`

[Self-model](../../../README.md) · [view::types](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn impl_methods(doc: &Document, impl_node: &Node) -> Vec<String>` |
| location | [`crates/robby-render/src/view/types.rs:218`](../../../../../crates/robby-render/src/view/types.rs#L218) |
| provenance | extractor · confidence 1.00 |

```rust
fn impl_methods(doc: &Document, impl_node: &Node) -> Vec<String> {
    let mut methods: Vec<String> = impl_node
        .children
        .iter()
        .filter_map(|id| doc.get(id))
        .filter(|node| node.kind.as_str() == kind::code::METHOD)
        .map(|node| text(&node.name))
        .collect();
    methods.sort();
    methods
}
```

```mermaid
sequenceDiagram
    autonumber
    participant impl_methods as impl_methods()
    participant iter as iter
    participant get as get
    participant filter_map as filter_map
    participant as_str as as_str
    participant filter as filter
    participant text as text
    participant map as map
    participant collect as collect
    participant sort as sort
    impl_methods-->>iter: impl_node .children .iter()
    impl_methods-->>get: doc.get(id)
    impl_methods-->>filter_map: impl_node .children .iter() .filter_map(|id| doc.get(id))
    impl_methods-->>as_str: node.kind.as_str()
    impl_methods-->>filter: impl_node .children .iter() .filter_map(|id| doc.get(id)) .filter(|no...
    impl_methods-->>text: text(&node.name)
    impl_methods-->>map: impl_node .children .iter() .filter_map(|id| doc.get(id)) .filter(|no...
    impl_methods-->>collect: impl_node .children .iter() .filter_map(|id| doc.get(id)) .filter(|no...
    impl_methods-->>sort: methods.sort()
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::util::text`](../../util/text.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)

## References

- calls → [`robby_ir::run`](../../../robby_ir/run/index.md) — get
- calls → [`robby_render::util`](../../util/index.md) — text
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::types::impl TypeModel::from_module`](index.md)

