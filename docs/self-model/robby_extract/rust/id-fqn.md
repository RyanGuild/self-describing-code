# `robby_extract::rust::id_fqn`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn id_fqn(id: &NodeId) -> &str` |
| location | [`crates/robby-extract/src/rust.rs:935`](../../../../crates/robby-extract/src/rust.rs#L935) |
| provenance | extractor · confidence 1.00 |

```rust
fn id_fqn(id: &NodeId) -> &str {
    id.as_str()
        .split_once("://")
        .map(|(_, fqn)| fqn)
        .unwrap_or(id.as_str())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant id_fqn as id_fqn()
    participant as_str as as_str
    participant split_once as split_once
    participant map as map
    participant unwrap_or as unwrap_or
    id_fqn-->>as_str: id.as_str()
    id_fqn-->>split_once: id.as_str() .split_once('://')
    id_fqn-->>map: id.as_str() .split_once('://') .map(|(_, fqn)| fqn)
    id_fqn-->>as_str: id.as_str()
    id_fqn-->>unwrap_or: id.as_str() .split_once('://') .map(|(_, fqn)| fqn) .unwrap_or(id.as_...
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str

## Referenced by

- calls ← [`robby_extract::rust::pick_call_target`](pick-call-target.md)

