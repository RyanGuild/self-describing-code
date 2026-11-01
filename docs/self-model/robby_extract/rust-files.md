# `robby_extract::rust_files`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn rust_files(crate_dir: &Path) -> Result<Vec<PathBuf>>` |
| location | [`crates/robby-extract/src/lib.rs:86`](../../../crates/robby-extract/src/lib.rs#L86) |
| provenance | extractor · confidence 1.00 |

```rust
fn rust_files(crate_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_rs(&crate_dir.join("src"), &mut files)?;
    let build = crate_dir.join("build.rs");
    if build.is_file() {
        files.push(build);
    }
    collect_rs(&crate_dir.join("examples"), &mut files)?;
    collect_rs(&crate_dir.join("tests"), &mut files)?;
    files.sort();
    Ok(files)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant rust_files as rust_files()
    participant Vec__new as Vec::new
    participant collect_rs as collect_rs
    participant join as join
    participant push as push
    participant sort as sort
    participant Ok as Ok
    rust_files-->>Vec__new: Vec::new()
    rust_files-->>collect_rs: collect_rs(&crate_dir.join('src'), &mut files)
    rust_files-->>rust_files: ? (propagate)
    rust_files-->>join: crate_dir.join('build.rs')
    alt build.is_file()
    rust_files-->>push: files.push(build)
    end
    rust_files-->>collect_rs: collect_rs(&crate_dir.join('examples'), &mut files)
    rust_files-->>rust_files: ? (propagate)
    rust_files-->>collect_rs: collect_rs(&crate_dir.join('tests'), &mut files)
    rust_files-->>rust_files: ? (propagate)
    rust_files-->>sort: files.sort()
    rust_files-->>Ok: Ok(files)
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_extract::collect_rs`](collect-rs.md)
- calls → [`robby_extract::collect_rs`](collect-rs.md)
- calls → [`robby_extract::collect_rs`](collect-rs.md)

