# `robby_render::view::hierarchy::is_compact`

[Self-model](../../../README.md) · [view::hierarchy](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_compact(kind_name: &str) -> bool` |
| location | [`crates/robby-render/src/view/hierarchy.rs:83`](../../../../../crates/robby-render/src/view/hierarchy.rs#L83) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_compact(kind_name: &str) -> bool {
    matches!(
        kind_name,
        kind::code::FIELD | kind::code::VARIANT | kind::code::IMPORT
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

- calls ← [`robby_render::view::hierarchy::build`](build.md)

