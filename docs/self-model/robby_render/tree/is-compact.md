# `robby_render::tree::is_compact`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_compact(kind_name: &str) -> bool` |
| location | [`crates/robby-render/src/tree.rs:704`](../../../../crates/robby-render/src/tree.rs#L704) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_compact(kind_name: &str) -> bool {
    matches!(
        kind_name,
        kind::code::FIELD | kind::code::VARIANT | kind::code::IMPORT | kind::code::COMMENT
    )
}
```

```mermaid
sequenceDiagram
    autonumber
    participant is_compact as is_compact()
    is_compact-->>is_compact: matches!( kind_name, kind::code::FIELD | kind::code::VARIANT | kind::...
```

1 steps · 0 observed · 1 possible only

## Referenced by

- calls ← [`robby_render::tree::item_children`](item-children.md)
- calls ← [`robby_render::tree::page_of_node`](page-of-node.md)
- calls ← [`robby_render::tree::render_node`](render-node.md)

