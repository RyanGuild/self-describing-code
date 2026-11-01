# `robby_render::tree::crate_index_path`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn crate_index_path(crate_dir: &str) -> PathBuf` |
| location | [`crates/robby-render/src/tree.rs:535`](../../../../crates/robby-render/src/tree.rs#L535) |
| provenance | extractor · confidence 1.00 |

```rust
fn crate_index_path(crate_dir: &str) -> PathBuf {
    PathBuf::from(format!("{crate_dir}/index.md"))
}
```

```mermaid
sequenceDiagram
    autonumber
    participant crate_index_path as crate_index_path()
    participant PathBuf__from as PathBuf::from
    crate_index_path-->>PathBuf__from: PathBuf::from(format!('{crate_dir}/index.md'))
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)

