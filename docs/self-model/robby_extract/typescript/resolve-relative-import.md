# `robby_extract::typescript::resolve_relative_import`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn resolve_relative_import(from_module: &str, spec: &str) -> Option<String>` |
| location | [`crates/robby-extract/src/typescript.rs:513`](../../../../crates/robby-extract/src/typescript.rs#L513) |
| provenance | extractor · confidence 1.00 |

```rust
fn resolve_relative_import(from_module: &str, spec: &str) -> Option<String> {
    if !(spec.starts_with("./") || spec.starts_with("../")) {
        return None;
    }
    let pkg = from_module.split('/').next()?;
    let from_dir = {
        let path = from_module.strip_prefix(&format!("{pkg}/")).unwrap_or("");
        Path::new(path).parent().map(|p| p.to_path_buf()).unwrap_or_default()
    };
    let joined = from_dir.join(spec);
    let mut parts = Vec::new();
    for comp in joined.components() {
        match comp {
            std::path::Component::ParentDir => {
                parts.pop();
            }
            std::path::Component::Normal(s) => parts.push(s.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    let rel = parts.join("/");
    let stem = strip_ts_ext(&rel);
    if stem.is_empty() || stem == "index" {
        Some(pkg.to_owned())
    } else {
        Some(format!("{pkg}/{stem}"))
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant resolve_relative_import as resolve_relative_import()
    participant starts_with as starts_with
    participant split as split
    participant next as next
    participant strip_prefix as strip_prefix
    participant unwrap_or as unwrap_or
    participant Path__new as Path::new
    participant parent as parent
    participant to_path_buf as to_path_buf
    participant map as map
    participant unwrap_or_default as unwrap_or_default
    participant join as join
    participant Vec__new as Vec::new
    participant components as components
    participant pop as pop
    participant to_string_lossy as to_string_lossy
    participant into_owned as into_owned
    participant push as push
    participant strip_ts_ext as strip_ts_ext
    participant is_empty as is_empty
    participant to_owned as to_owned
    participant Some as Some
    resolve_relative_import-->>starts_with: spec.starts_with('./')
    resolve_relative_import-->>starts_with: spec.starts_with('../')
    alt !(spec.starts_with('./') || spec.starts_with('../'))
    resolve_relative_import-->>resolve_relative_import: return
    end
    resolve_relative_import-->>split: from_module.split('/')
    resolve_relative_import-->>next: from_module.split('/').next()
    resolve_relative_import-->>resolve_relative_import: ? (propagate)
    resolve_relative_import-->>resolve_relative_import: format!('{pkg}/')
    resolve_relative_import-->>strip_prefix: from_module.strip_prefix(&format!('{pkg}/'))
    resolve_relative_import-->>unwrap_or: from_module.strip_prefix(&format!('{pkg}/')).unwrap_or('')
    resolve_relative_import-->>Path__new: Path::new(path)
    resolve_relative_import-->>parent: Path::new(path).parent()
    resolve_relative_import-->>to_path_buf: p.to_path_buf()
    resolve_relative_import-->>map: Path::new(path).parent().map(|p| p.to_path_buf())
    resolve_relative_import-->>unwrap_or_default: Path::new(path).parent().map(|p| p.to_path_buf()).unwrap_or_default()
    resolve_relative_import-->>join: from_dir.join(spec)
    resolve_relative_import-->>Vec__new: Vec::new()
    resolve_relative_import-->>components: joined.components()
    loop joined.components()
    alt match comp
    resolve_relative_import-->>pop: parts.pop()
    else std::path::Component::Normal(s)
    resolve_relative_import-->>to_string_lossy: s.to_string_lossy()
    resolve_relative_import-->>into_owned: s.to_string_lossy().into_owned()
    resolve_relative_import-->>push: parts.push(s.to_string_lossy().into_owned())
    end
    end
    resolve_relative_import-->>join: parts.join('/')
    resolve_relative_import-->>strip_ts_ext: strip_ts_ext(&rel)
    resolve_relative_import-->>is_empty: stem.is_empty()
    alt stem.is_empty() || stem == 'index'
    resolve_relative_import-->>to_owned: pkg.to_owned()
    resolve_relative_import-->>Some: Some(pkg.to_owned())
    else else
    resolve_relative_import-->>resolve_relative_import: format!('{pkg}/{stem}')
    resolve_relative_import-->>Some: Some(format!('{pkg}/{stem}'))
    end
```

28 steps · 0 observed · 28 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_extract::typescript::strip_ts_ext`](strip-ts-ext.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::typescript`](index.md) — strip_ts_ext
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_extract::typescript::emit_import`](emit-import.md)

