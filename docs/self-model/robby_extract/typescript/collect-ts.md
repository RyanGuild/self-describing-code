# `robby_extract::typescript::collect_ts`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn collect_ts(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()>` |
| location | [`crates/robby-extract/src/typescript.rs:149`](../../../../crates/robby-extract/src/typescript.rs#L149) |
| provenance | extractor · confidence 1.00 |

```rust
fn collect_ts(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if name == "node_modules" || name == "dist" {
        return Ok(());
    }
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_ts(&path, out)?;
        } else if is_ts_file(&path) {
            out.push(path);
        }
    }
    Ok(())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant collect_ts as collect_ts()
    participant is_dir as is_dir
    participant Ok as Ok
    participant file_name as file_name
    participant to_str as to_str
    participant and_then as and_then
    participant unwrap_or as unwrap_or
    participant fs__read_dir as fs::read_dir
    participant ok as ok
    participant path as path
    participant map as map
    participant filter_map as filter_map
    participant collect as collect
    participant sort as sort
    participant is_ts_file as is_ts_file
    participant push as push
    collect_ts-->>is_dir: dir.is_dir()
    alt !dir.is_dir()
    collect_ts-->>Ok: Ok(())
    collect_ts-->>collect_ts: return
    end
    collect_ts-->>file_name: dir.file_name()
    collect_ts-->>to_str: n.to_str()
    collect_ts-->>and_then: dir.file_name().and_then(|n| n.to_str())
    collect_ts-->>unwrap_or: dir.file_name().and_then(|n| n.to_str()).unwrap_or('')
    alt name == 'node_modules' || name == 'dist'
    collect_ts-->>Ok: Ok(())
    collect_ts-->>collect_ts: return
    end
    collect_ts-->>fs__read_dir: fs::read_dir(dir)
    collect_ts-->>collect_ts: ? (propagate)
    collect_ts-->>ok: e.ok()
    collect_ts-->>path: e.path()
    collect_ts-->>map: e.ok().map(|e| e.path())
    collect_ts-->>filter_map: fs::read_dir(dir)? .filter_map(|e| e.ok().map(|e| e.path()))
    collect_ts-->>collect: fs::read_dir(dir)? .filter_map(|e| e.ok().map(|e| e.path())) .collect()
    collect_ts-->>sort: entries.sort()
    loop entries
    collect_ts-->>is_dir: path.is_dir()
    alt path.is_dir()
    collect_ts-->>collect_ts: collect_ts(&path, out)
    collect_ts-->>collect_ts: ? (propagate)
    else else
    collect_ts-->>is_ts_file: is_ts_file(&path)
    alt is_ts_file(&path)
    collect_ts-->>push: out.push(path)
    end
    end
    end
    collect_ts-->>Ok: Ok(())
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_extract::typescript::collect_ts`](collect-ts.md)
- calls → [`robby_extract::typescript::is_ts_file`](is-ts-file.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_extract::typescript`](index.md) — is_ts_file
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_extract::typescript::ts_files`](ts-files.md)

