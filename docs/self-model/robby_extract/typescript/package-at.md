# `robby_extract::typescript::package_at`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn package_at(extract_root: &Path, dir: &Path) -> Result<Option<Package>>` |
| location | [`crates/robby-extract/src/typescript.rs:100`](../../../../crates/robby-extract/src/typescript.rs#L100) |
| provenance | extractor · confidence 1.00 |

```rust
fn package_at(extract_root: &Path, dir: &Path) -> Result<Option<Package>> {
    let pkg_json = dir.join("package.json");
    if !pkg_json.is_file() {
        return Ok(None);
    }
    let fallback = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("package");
    let name = read_package_name(&pkg_json)?.unwrap_or_else(|| sanitize_pkg_name(fallback));
    let files = ts_files(dir)?;
    if files.is_empty() {
        return Ok(None);
    }
    Ok(Some(Package {
        name,
        dir: dir.to_path_buf(),
        root: extract_root.to_path_buf(),
        files,
    }))
}
```

```mermaid
sequenceDiagram
    autonumber
    participant package_at as package_at()
    participant join as join
    participant is_file as is_file
    participant Ok as Ok
    participant file_name as file_name
    participant to_str as to_str
    participant and_then as and_then
    participant unwrap_or as unwrap_or
    participant read_package_name as read_package_name
    participant sanitize_pkg_name as sanitize_pkg_name
    participant unwrap_or_else as unwrap_or_else
    participant ts_files as ts_files
    participant is_empty as is_empty
    participant to_path_buf as to_path_buf
    participant Some as Some
    package_at-->>join: dir.join('package.json')
    package_at-->>is_file: pkg_json.is_file()
    alt !pkg_json.is_file()
    package_at-->>Ok: Ok(None)
    package_at-->>package_at: return
    end
    package_at-->>file_name: dir .file_name()
    package_at-->>to_str: n.to_str()
    package_at-->>and_then: dir .file_name() .and_then(|n| n.to_str())
    package_at-->>unwrap_or: dir .file_name() .and_then(|n| n.to_str()) .unwrap_or('package')
    package_at-->>read_package_name: read_package_name(&pkg_json)
    package_at-->>package_at: ? (propagate)
    package_at-->>sanitize_pkg_name: sanitize_pkg_name(fallback)
    package_at-->>unwrap_or_else: read_package_name(&pkg_json)?.unwrap_or_else(|| sanitize_pkg_name(fal...
    package_at-->>ts_files: ts_files(dir)
    package_at-->>package_at: ? (propagate)
    package_at-->>is_empty: files.is_empty()
    alt files.is_empty()
    package_at-->>Ok: Ok(None)
    package_at-->>package_at: return
    end
    package_at-->>to_path_buf: dir.to_path_buf()
    package_at-->>to_path_buf: extract_root.to_path_buf()
    package_at-->>Some: Some(Package { name, dir: dir.to_path_buf(), root: extract_root.to_pa...
    package_at-->>Ok: Ok(Some(Package { name, dir: dir.to_path_buf(), root: extract_root.to...
```

21 steps · 0 observed · 21 possible only

**Edges**

- calls → [`robby_extract::typescript::read_package_name`](read-package-name.md)
- calls → [`robby_extract::typescript::sanitize_pkg_name`](sanitize-pkg-name.md)
- calls → [`robby_extract::typescript::ts_files`](ts-files.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)

## References

- calls → [`robby_extract::typescript`](index.md) — read_package_name, sanitize_pkg_name, ts_files
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty

## Referenced by

- calls ← [`robby_extract::typescript::discover_packages`](discover-packages.md)

