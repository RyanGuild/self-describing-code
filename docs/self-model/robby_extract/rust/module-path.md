# `robby_extract::rust::module_path`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn module_path(krate: &Krate, file: &Path) -> String` |
| location | [`crates/robby-extract/src/rust.rs:103`](../../../../crates/robby-extract/src/rust.rs#L103) |
| provenance | extractor · confidence 1.00 |

```rust
fn module_path(krate: &Krate, file: &Path) -> String {
    let rel = file
        .strip_prefix(&krate.dir)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");

    if rel == "src/lib.rs" || rel == "src/main.rs" {
        return krate.name.clone();
    }
    if rel == "build.rs" {
        return format!("{}::build", krate.name);
    }
    if let Some(stem) = rel
        .strip_prefix("examples/")
        .and_then(|s| s.strip_suffix(".rs"))
    {
        return format!("{}::example_{}", krate.name, stem.replace('-', "_"));
    }
    if let Some(stem) = rel
        .strip_prefix("tests/")
        .and_then(|s| s.strip_suffix(".rs"))
    {
        return format!("{}::test_{}", krate.name, stem.replace('-', "_"));
    }
    if let Some(rest) = rel.strip_prefix("src/") {
        let mut p = rest.trim_end_matches(".rs").to_string();
        if p.ends_with("/mod") {
            p.truncate(p.len() - 4);
        }
        return format!("{}::{}", krate.name, p.replace('/', "::"));
    }
    format!(
        "{}::{}",
        krate.name,
        rel.trim_end_matches(".rs").replace('/', "::")
    )
}
```

```mermaid
sequenceDiagram
    autonumber
    participant module_path as module_path()
    participant strip_prefix as strip_prefix
    participant unwrap_or as unwrap_or
    participant to_string_lossy as to_string_lossy
    participant replace as replace
    participant clone as clone
    participant strip_suffix as strip_suffix
    participant and_then as and_then
    participant trim_end_matches as trim_end_matches
    participant to_string as to_string
    participant ends_with as ends_with
    participant len as len
    participant truncate as truncate
    module_path-->>strip_prefix: file .strip_prefix(&krate.dir)
    module_path-->>unwrap_or: file .strip_prefix(&krate.dir) .unwrap_or(file)
    module_path-->>to_string_lossy: file .strip_prefix(&krate.dir) .unwrap_or(file) .to_string_lossy()
    module_path-->>replace: file .strip_prefix(&krate.dir) .unwrap_or(file) .to_string_lossy() .r...
    alt rel == 'src/lib.rs' || rel == 'src/main.rs'
    module_path-->>clone: krate.name.clone()
    module_path-->>module_path: return
    end
    alt rel == 'build.rs'
    module_path-->>module_path: format!('{}::build', krate.name)
    module_path-->>module_path: return
    end
    module_path-->>strip_prefix: rel .strip_prefix('examples/')
    module_path-->>strip_suffix: s.strip_suffix('.rs')
    module_path-->>and_then: rel .strip_prefix('examples/') .and_then(|s| s.strip_suffix('.rs'))
    alt let Some(stem) = rel .strip_prefix('examples/') .and_then(|s| s.strip...
    module_path-->>module_path: format!('{}::example_{}', krate.name, stem.replace('-', '_'))
    module_path-->>module_path: return
    end
    module_path-->>strip_prefix: rel .strip_prefix('tests/')
    module_path-->>strip_suffix: s.strip_suffix('.rs')
    module_path-->>and_then: rel .strip_prefix('tests/') .and_then(|s| s.strip_suffix('.rs'))
    alt let Some(stem) = rel .strip_prefix('tests/') .and_then(|s| s.strip_su...
    module_path-->>module_path: format!('{}::test_{}', krate.name, stem.replace('-', '_'))
    module_path-->>module_path: return
    end
    module_path-->>strip_prefix: rel.strip_prefix('src/')
    alt let Some(rest) = rel.strip_prefix('src/')
    module_path-->>trim_end_matches: rest.trim_end_matches('.rs')
    module_path-->>to_string: rest.trim_end_matches('.rs').to_string()
    module_path-->>ends_with: p.ends_with('/mod')
    alt p.ends_with('/mod')
    module_path-->>len: p.len()
    module_path-->>truncate: p.truncate(p.len() - 4)
    end
    module_path-->>module_path: format!('{}::{}', krate.name, p.replace('/', '::'))
    module_path-->>module_path: return
    end
    module_path-->>module_path: format!( '{}::{}', krate.name, rel.trim_end_matches('.rs').replace('/...
```

27 steps · 0 observed · 27 possible only

## Referenced by

- calls ← [`robby_extract::rust::extract_file`](extract-file.md)

