# `robby_extract::typescript::npm_bare_name`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn npm_bare_name(spec: &str) -> String` |
| location | [`crates/robby-extract/src/typescript.rs:388`](../../../../crates/robby-extract/src/typescript.rs#L388) |
| provenance | extractor · confidence 1.00 |

> `react/jsx-runtime` → `react`; `@scope/pkg/sub` → `@scope/pkg`.

```rust
/// `react/jsx-runtime` → `react`; `@scope/pkg/sub` → `@scope/pkg`.
fn npm_bare_name(spec: &str) -> String {
    if let Some(rest) = spec.strip_prefix('@') {
        let mut parts = rest.splitn(3, '/');
        let scope = parts.next().unwrap_or(rest);
        match parts.next() {
            Some(pkg) if !pkg.is_empty() => format!("@{scope}/{pkg}"),
            _ => format!("@{scope}"),
        }
    } else {
        spec.split('/').next().unwrap_or(spec).to_owned()
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant npm_bare_name as npm_bare_name()
    participant strip_prefix as strip_prefix
    participant splitn as splitn
    participant next as next
    participant unwrap_or as unwrap_or
    participant split as split
    participant to_owned as to_owned
    npm_bare_name-->>strip_prefix: spec.strip_prefix('@')
    alt let Some(rest) = spec.strip_prefix('@')
    npm_bare_name-->>splitn: rest.splitn(3, '/')
    npm_bare_name-->>next: parts.next()
    npm_bare_name-->>unwrap_or: parts.next().unwrap_or(rest)
    npm_bare_name-->>next: parts.next()
    alt match parts.next()
    npm_bare_name-->>npm_bare_name: format!('@{scope}/{pkg}')
    else _
    npm_bare_name-->>npm_bare_name: format!('@{scope}')
    end
    else else
    npm_bare_name-->>split: spec.split('/')
    npm_bare_name-->>next: spec.split('/').next()
    npm_bare_name-->>unwrap_or: spec.split('/').next().unwrap_or(spec)
    npm_bare_name-->>to_owned: spec.split('/').next().unwrap_or(spec).to_owned()
    end
```

11 steps · 0 observed · 11 possible only

