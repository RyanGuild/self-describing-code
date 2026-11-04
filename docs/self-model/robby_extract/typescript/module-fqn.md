# `robby_extract::typescript::module_fqn`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn module_fqn(package: &Package, file: &Path) -> String` |
| location | [`crates/robby-extract/src/typescript.rs:174`](../../../../crates/robby-extract/src/typescript.rs#L174) |
| provenance | extractor · confidence 1.00 |

```rust
fn module_fqn(package: &Package, file: &Path) -> String {
    let rel = file
        .strip_prefix(&package.dir)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");
    let stem = strip_ts_ext(&rel);
    if stem == "src/index" || stem == "index" {
        return package.name.clone();
    }
    format!("{}/{}", package.name, stem)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant module_fqn as module_fqn()
    participant strip_prefix as strip_prefix
    participant unwrap_or as unwrap_or
    participant to_string_lossy as to_string_lossy
    participant replace as replace
    participant strip_ts_ext as strip_ts_ext
    participant clone as clone
    module_fqn-->>strip_prefix: file .strip_prefix(&package.dir)
    module_fqn-->>unwrap_or: file .strip_prefix(&package.dir) .unwrap_or(file)
    module_fqn-->>to_string_lossy: file .strip_prefix(&package.dir) .unwrap_or(file) .to_string_lossy()
    module_fqn-->>replace: file .strip_prefix(&package.dir) .unwrap_or(file) .to_string_lossy() ...
    module_fqn-->>strip_ts_ext: strip_ts_ext(&rel)
    alt stem == 'src/index' || stem == 'index'
    module_fqn-->>clone: package.name.clone()
    module_fqn-->>module_fqn: return
    end
    module_fqn-->>module_fqn: format!('{}/{}', package.name, stem)
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_extract::typescript::strip_ts_ext`](strip-ts-ext.md)

## References

- calls → [`robby_extract::typescript`](index.md) — strip_ts_ext

## Referenced by

- calls ← [`robby_extract::typescript::extract_file`](extract-file.md)

