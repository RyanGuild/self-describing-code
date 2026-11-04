# `robby_extract::typescript::read_package_name`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn read_package_name(path: &Path) -> Result<Option<String>>` |
| location | [`crates/robby-extract/src/typescript.rs:119`](../../../../crates/robby-extract/src/typescript.rs#L119) |
| provenance | extractor · confidence 1.00 |

```rust
fn read_package_name(path: &Path) -> Result<Option<String>> {
    let text = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(value
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.trim_start_matches('@').replace('/', "__")))
}
```

```mermaid
sequenceDiagram
    autonumber
    participant read_package_name as read_package_name()
    participant fs__read_to_string as fs::read_to_string
    participant with_context as with_context
    participant serde_json__from_str as serde_json::from_str
    participant get as get
    participant as_str as as_str
    participant and_then as and_then
    participant trim_start_matches as trim_start_matches
    participant replace as replace
    participant map as map
    participant Ok as Ok
    read_package_name-->>fs__read_to_string: fs::read_to_string(path)
    read_package_name-->>read_package_name: format!('reading {}', path.display())
    read_package_name-->>with_context: fs::read_to_string(path).with_context(|| format!('reading {}', path.d...
    read_package_name-->>read_package_name: ? (propagate)
    read_package_name-->>serde_json__from_str: serde_json::from_str(&text)
    read_package_name-->>read_package_name: format!('parsing {}', path.display())
    read_package_name-->>with_context: serde_json::from_str(&text).with_context(|| format!('parsing {}', pat...
    read_package_name-->>read_package_name: ? (propagate)
    read_package_name-->>get: value .get('name')
    read_package_name-->>as_str: v.as_str()
    read_package_name-->>and_then: value .get('name') .and_then(|v| v.as_str())
    read_package_name-->>trim_start_matches: s.trim_start_matches('@')
    read_package_name-->>replace: s.trim_start_matches('@').replace('/', '__')
    read_package_name-->>map: value .get('name') .and_then(|v| v.as_str()) .map(|s| s.trim_start_ma...
    read_package_name-->>Ok: Ok(value .get('name') .and_then(|v| v.as_str()) .map(|s| s.trim_start...
```

15 steps · 0 observed · 15 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get

## Referenced by

- calls ← [`robby_extract::typescript::package_at`](package-at.md)

