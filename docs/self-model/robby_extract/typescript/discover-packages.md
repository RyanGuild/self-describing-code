# `robby_extract::typescript::discover_packages`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn discover_packages(root: &Path) -> Result<Vec<Package>>` |
| location | [`crates/robby-extract/src/typescript.rs:63`](../../../../crates/robby-extract/src/typescript.rs#L63) |
| provenance | extractor · confidence 1.00 |

```rust
fn discover_packages(root: &Path) -> Result<Vec<Package>> {
    let mut out = Vec::new();

    // Fixture / single-package roots: package.json at the extract root.
    if root.join("package.json").is_file() {
        if let Some(pkg) = package_at(root, root)? {
            out.push(pkg);
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        return Ok(out);
    }

    // Workspace layout: only `packages/*` and `apps/*` (skip crates/, fixtures, etc.).
    for dir_name in ["packages", "apps"] {
        let base = root.join(dir_name);
        if !base.is_dir() {
            continue;
        }
        let mut kids: Vec<PathBuf> = fs::read_dir(&base)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_dir())
            .collect();
        kids.sort();
        for kid in kids {
            let n = kid.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if n == "node_modules" || n == "dist" {
                continue;
            }
            if let Some(pkg) = package_at(root, &kid)? {
                out.push(pkg);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant discover_packages as discover_packages()
    participant Vec__new as Vec::new
    participant join as join
    participant is_file as is_file
    participant package_at as package_at
    participant push as push
    participant cmp as cmp
    participant sort_by as sort_by
    participant Ok as Ok
    participant is_dir as is_dir
    participant fs__read_dir as fs::read_dir
    participant ok as ok
    participant path as path
    participant map as map
    participant filter_map as filter_map
    participant filter as filter
    participant collect as collect
    participant sort as sort
    participant file_name as file_name
    participant to_str as to_str
    participant and_then as and_then
    participant unwrap_or as unwrap_or
    discover_packages-->>Vec__new: Vec::new()
    discover_packages-->>join: root.join('package.json')
    discover_packages-->>is_file: root.join('package.json').is_file()
    alt root.join('package.json').is_file()
    discover_packages-->>package_at: package_at(root, root)
    discover_packages-->>discover_packages: ? (propagate)
    alt let Some(pkg) = package_at(root, root)?
    discover_packages-->>push: out.push(pkg)
    end
    discover_packages-->>cmp: a.name.cmp(&b.name)
    discover_packages-->>sort_by: out.sort_by(|a, b| a.name.cmp(&b.name))
    discover_packages-->>Ok: Ok(out)
    discover_packages-->>discover_packages: return
    end
    loop ['packages', 'apps']
    discover_packages-->>join: root.join(dir_name)
    discover_packages-->>is_dir: base.is_dir()
    discover_packages-->>fs__read_dir: fs::read_dir(&base)
    discover_packages-->>discover_packages: ? (propagate)
    discover_packages-->>ok: e.ok()
    discover_packages-->>path: e.path()
    discover_packages-->>map: e.ok().map(|e| e.path())
    discover_packages-->>filter_map: fs::read_dir(&base)? .filter_map(|e| e.ok().map(|e| e.path()))
    discover_packages-->>is_dir: p.is_dir()
    discover_packages-->>filter: fs::read_dir(&base)? .filter_map(|e| e.ok().map(|e| e.path())) .filte...
    discover_packages-->>collect: fs::read_dir(&base)? .filter_map(|e| e.ok().map(|e| e.path())) .filte...
    discover_packages-->>sort: kids.sort()
    loop kids
    discover_packages-->>file_name: kid.file_name()
    discover_packages-->>to_str: n.to_str()
    discover_packages-->>and_then: kid.file_name().and_then(|n| n.to_str())
    discover_packages-->>unwrap_or: kid.file_name().and_then(|n| n.to_str()).unwrap_or('')
    discover_packages-->>package_at: package_at(root, &kid)
    discover_packages-->>discover_packages: ? (propagate)
    alt let Some(pkg) = package_at(root, &kid)?
    discover_packages-->>push: out.push(pkg)
    end
    end
    end
    discover_packages-->>cmp: a.name.cmp(&b.name)
    discover_packages-->>sort_by: out.sort_by(|a, b| a.name.cmp(&b.name))
    discover_packages-->>Ok: Ok(out)
```

32 steps · 0 observed · 32 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_extract::typescript::package_at`](package-at.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_extract::typescript::package_at`](package-at.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::typescript`](index.md) — package_at
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_extract::typescript::extract`](extract.md)
- calls ← [`robby_extract::typescript::source_file_count`](source-file-count.md)

