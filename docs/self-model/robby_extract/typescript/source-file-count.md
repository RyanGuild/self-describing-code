# `robby_extract::typescript::source_file_count`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn source_file_count(root: &Path) -> Result<usize>` |
| location | [`crates/robby-extract/src/typescript.rs:48`](../../../../crates/robby-extract/src/typescript.rs#L48) |
| provenance | extractor · confidence 1.00 |

```rust
pub(crate) fn source_file_count(root: &Path) -> Result<usize> {
    Ok(discover_packages(root)?
        .iter()
        .map(|p| p.files.len())
        .sum())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant source_file_count as source_file_count()
    participant discover_packages as discover_packages
    participant iter as iter
    participant len as len
    participant map as map
    participant sum as sum
    participant Ok as Ok
    source_file_count-->>discover_packages: discover_packages(root)
    source_file_count-->>source_file_count: ? (propagate)
    source_file_count-->>iter: discover_packages(root)? .iter()
    source_file_count-->>len: p.files.len()
    source_file_count-->>map: discover_packages(root)? .iter() .map(|p| p.files.len())
    source_file_count-->>sum: discover_packages(root)? .iter() .map(|p| p.files.len()) .sum()
    source_file_count-->>Ok: Ok(discover_packages(root)? .iter() .map(|p| p.files.len()) .sum())
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_extract::typescript::discover_packages`](discover-packages.md)

## References

- calls → [`robby_extract::typescript`](index.md) — discover_packages

