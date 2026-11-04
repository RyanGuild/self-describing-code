# `robby_extract::rust::module_prefix`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn module_prefix(fqn: &str) -> &str` |
| location | [`crates/robby-extract/src/rust.rs:947`](../../../../crates/robby-extract/src/rust.rs#L947) |
| provenance | extractor · confidence 1.00 |

```rust
fn module_prefix(fqn: &str) -> &str {
    fqn.rsplit_once("::").map_or(fqn, |(m, _)| m)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant module_prefix as module_prefix()
    participant rsplit_once as rsplit_once
    participant map_or as map_or
    module_prefix-->>rsplit_once: fqn.rsplit_once('::')
    module_prefix-->>map_or: fqn.rsplit_once('::').map_or(fqn, |(m, _)| m)
```

2 steps · 0 observed · 2 possible only

## Referenced by

- calls ← [`robby_extract::rust::pick_call_target`](pick-call-target.md)

