# `robby_extract::typescript::ts_files`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn ts_files(package_dir: &Path) -> Result<Vec<PathBuf>>` |
| location | [`crates/robby-extract/src/typescript.rs:136`](../../../../crates/robby-extract/src/typescript.rs#L136) |
| provenance | extractor · confidence 1.00 |

```rust
fn ts_files(package_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    // Prefer src/, else package root (excluding node_modules/dist).
    let src = package_dir.join("src");
    if src.is_dir() {
        collect_ts(&src, &mut files)?;
    } else {
        collect_ts(package_dir, &mut files)?;
    }
    files.sort();
    Ok(files)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant ts_files as ts_files()
    participant Vec__new as Vec::new
    participant join as join
    participant is_dir as is_dir
    participant collect_ts as collect_ts
    participant sort as sort
    participant Ok as Ok
    ts_files-->>Vec__new: Vec::new()
    ts_files-->>join: package_dir.join('src')
    ts_files-->>is_dir: src.is_dir()
    alt src.is_dir()
    ts_files-->>collect_ts: collect_ts(&src, &mut files)
    ts_files-->>ts_files: ? (propagate)
    else else
    ts_files-->>collect_ts: collect_ts(package_dir, &mut files)
    ts_files-->>ts_files: ? (propagate)
    end
    ts_files-->>sort: files.sort()
    ts_files-->>Ok: Ok(files)
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_extract::typescript::collect_ts`](collect-ts.md)
- calls → [`robby_extract::typescript::collect_ts`](collect-ts.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::typescript`](index.md) — collect_ts

## Referenced by

- calls ← [`robby_extract::typescript::package_at`](package-at.md)

