# `robby_extract::typescript::property_key_name`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn property_key_name(key: &PropertyKey<'_>) -> Option<String>` |
| location | [`crates/robby-extract/src/typescript.rs:701`](../../../../crates/robby-extract/src/typescript.rs#L701) |
| provenance | extractor · confidence 1.00 |

```rust
fn property_key_name(key: &PropertyKey<'_>) -> Option<String> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str().to_owned()),
        PropertyKey::Identifier(id) => Some(id.name.as_str().to_owned()),
        PropertyKey::StringLiteral(s) => Some(js_str(&s.value).to_owned()),
        _ => None,
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant property_key_name as property_key_name()
    participant as_str as as_str
    participant to_owned as to_owned
    participant Some as Some
    participant js_str as js_str
    alt match key
    property_key_name-->>as_str: id.name.as_str()
    property_key_name-->>to_owned: id.name.as_str().to_owned()
    property_key_name-->>Some: Some(id.name.as_str().to_owned())
    else PropertyKey::Identifier(id)
    property_key_name-->>as_str: id.name.as_str()
    property_key_name-->>to_owned: id.name.as_str().to_owned()
    property_key_name-->>Some: Some(id.name.as_str().to_owned())
    else PropertyKey::StringLiteral(s)
    property_key_name-->>js_str: js_str(&s.value)
    property_key_name-->>to_owned: js_str(&s.value).to_owned()
    property_key_name-->>Some: Some(js_str(&s.value).to_owned())
    end
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::typescript::js_str`](js-str.md)

## References

- calls → [`robby_extract::typescript`](index.md) — js_str
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str

## Referenced by

- calls ← [`robby_extract::typescript::emit_class`](emit-class.md)
- calls ← [`robby_extract::typescript::emit_method`](emit-method.md)

