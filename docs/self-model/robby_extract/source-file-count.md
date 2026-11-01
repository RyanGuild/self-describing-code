# `robby_extract::source_file_count`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn source_file_count(root: &Path) -> Result<usize>` |
| location | [`crates/robby-extract/src/lib.rs:834`](../../../crates/robby-extract/src/lib.rs#L834) |
| provenance | extractor · confidence 1.00 |

> Count of `.rs` files the extractor considers part of the workspace.

```rust
/// Count of `.rs` files the extractor considers part of the workspace.
pub fn source_file_count(root: &Path) -> Result<usize> {
    Ok(discover_crates(root)?.iter().map(|k| k.files.len()).sum())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant source_file_count as source_file_count()
    participant Ok as Ok
    source_file_count-->>Ok: Ok(discover_crates(root)?.iter().map(|k| k.files.len()).sum())
```

1 steps · 0 observed · 1 possible only

