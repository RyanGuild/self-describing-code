# `robby_extract::discover_crates`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn discover_crates(root: &Path) -> Result<Vec<Krate>>` |
| location | [`crates/robby-extract/src/lib.rs:61`](../../../crates/robby-extract/src/lib.rs#L61) |
| provenance | extractor · confidence 1.00 |

```rust
fn discover_crates(root: &Path) -> Result<Vec<Krate>> {
    let crates_dir = root.join("crates");
    let mut out = Vec::new();
    if !crates_dir.is_dir() {
        return Ok(out);
    }

    let mut entries: Vec<PathBuf> = fs::read_dir(&crates_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("Cargo.toml").is_file())
        .collect();
    entries.sort();

    for dir in entries {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .replace('-', "_");
        let files = rust_files(&dir)?;
        out.push(Krate { name, dir, files });
    }
    Ok(out)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant discover_crates as discover_crates()
    participant join as join
    participant Vec__new as Vec::new
    participant Ok as Ok
    participant collect as collect
    participant sort as sort
    participant replace as replace
    participant rust_files as rust_files
    participant push as push
    discover_crates-->>join: root.join('crates')
    discover_crates-->>Vec__new: Vec::new()
    alt !crates_dir.is_dir()
    discover_crates-->>Ok: Ok(out)
    discover_crates-->>discover_crates: return
    end
    discover_crates-->>collect: fs::read_dir(&crates_dir)? .filter_map(|e| e.ok().map(|e| e.path())) ...
    discover_crates-->>sort: entries.sort()
    loop entries
    discover_crates-->>replace: dir .file_name() .and_then(|n| n.to_str()) .unwrap_or('unknown') .rep...
    discover_crates-->>rust_files: rust_files(&dir)
    discover_crates-->>discover_crates: ? (propagate)
    discover_crates-->>push: out.push(Krate { name, dir, files })
    end
    discover_crates-->>Ok: Ok(out)
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_extract::rust_files`](rust-files.md)

