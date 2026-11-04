# `robby_render::tree::is_crate`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_crate(node: &Node) -> bool` |
| location | [`crates/robby-render/src/tree.rs:682`](../../../../crates/robby-render/src/tree.rs#L682) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_crate(node: &Node) -> bool {
    matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant is_crate as is_crate()
    is_crate-->>is_crate: matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE)
```

1 steps · 0 observed · 1 possible only

## Referenced by

- calls ← [`robby_render::tree::nav`](nav.md)
- calls ← [`robby_render::tree::tree`](tree.md)

