# `robby_render::tree::is_module_kind`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_module_kind(kind_name: &str) -> bool` |
| location | [`crates/robby-render/src/tree.rs:690`](../../../../crates/robby-render/src/tree.rs#L690) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_module_kind(kind_name: &str) -> bool {
    matches!(
        kind_name,
        kind::code::MODULE | kind::code::CRATE | kind::code::PACKAGE
    )
}
```

```mermaid
sequenceDiagram
    autonumber
    participant is_module_kind as is_module_kind()
    is_module_kind-->>is_module_kind: matches!( kind_name, kind::code::MODULE | kind::code::CRATE | kind::c...
```

1 steps · 0 observed · 1 possible only

## Referenced by

- calls ← [`robby_render::tree::is_module`](is-module.md)

