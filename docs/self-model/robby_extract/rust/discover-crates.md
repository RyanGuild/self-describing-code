# `robby_extract::rust::discover_crates`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn discover_crates(root: &Path) -> Result<Vec<Krate>>` |
| location | [`crates/robby-extract/src/rust.rs:47`](../../../../crates/robby-extract/src/rust.rs#L47) |
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
    participant is_dir as is_dir
    participant Ok as Ok
    participant fs__read_dir as fs::read_dir
    participant ok as ok
    participant path as path
    participant map as map
    participant filter_map as filter_map
    participant is_file as is_file
    participant filter as filter
    participant collect as collect
    participant sort as sort
    participant file_name as file_name
    participant to_str as to_str
    participant and_then as and_then
    participant unwrap_or as unwrap_or
    participant replace as replace
    participant rust_files as rust_files
    participant push as push
    discover_crates-->>join: root.join('crates')
    discover_crates-->>Vec__new: Vec::new()
    discover_crates-->>is_dir: crates_dir.is_dir()
    alt !crates_dir.is_dir()
    discover_crates-->>Ok: Ok(out)
    discover_crates-->>discover_crates: return
    end
    discover_crates-->>fs__read_dir: fs::read_dir(&crates_dir)
    discover_crates-->>discover_crates: ? (propagate)
    discover_crates-->>ok: e.ok()
    discover_crates-->>path: e.path()
    discover_crates-->>map: e.ok().map(|e| e.path())
    discover_crates-->>filter_map: fs::read_dir(&crates_dir)? .filter_map(|e| e.ok().map(|e| e.path()))
    discover_crates-->>join: p.join('Cargo.toml')
    discover_crates-->>is_file: p.join('Cargo.toml').is_file()
    discover_crates-->>filter: fs::read_dir(&crates_dir)? .filter_map(|e| e.ok().map(|e| e.path())) ...
    discover_crates-->>collect: fs::read_dir(&crates_dir)? .filter_map(|e| e.ok().map(|e| e.path())) ...
    discover_crates-->>sort: entries.sort()
    loop entries
    discover_crates-->>file_name: dir .file_name()
    discover_crates-->>to_str: n.to_str()
    discover_crates-->>and_then: dir .file_name() .and_then(|n| n.to_str())
    discover_crates-->>unwrap_or: dir .file_name() .and_then(|n| n.to_str()) .unwrap_or('unknown')
    discover_crates-->>replace: dir .file_name() .and_then(|n| n.to_str()) .unwrap_or('unknown') .rep...
    discover_crates-->>rust_files: rust_files(&dir)
    discover_crates-->>discover_crates: ? (propagate)
    discover_crates-->>push: out.push(Krate { name, dir, files })
    end
    discover_crates-->>Ok: Ok(out)
```

25 steps · 0 observed · 25 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_extract::rust::rust_files`](rust-files.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_extract::rust`](index.md) — new, rust_files
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_extract::rust::extract`](extract.md)
- calls ← [`robby_extract::rust::source_file_count`](source-file-count.md)

