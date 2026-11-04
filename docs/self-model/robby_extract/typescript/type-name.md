# `robby_extract::typescript::type_name`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn type_name(name: &TSTypeName<'_>) -> Option<String>` |
| location | [`crates/robby-extract/src/typescript.rs:721`](../../../../crates/robby-extract/src/typescript.rs#L721) |
| provenance | extractor · confidence 1.00 |

```rust
fn type_name(name: &TSTypeName<'_>) -> Option<String> {
    match name {
        TSTypeName::IdentifierReference(id) => Some(id.name.as_str().to_owned()),
        TSTypeName::QualifiedName(q) => {
            let left = type_name(&q.left)?;
            Some(format!("{left}.{}", q.right.name.as_str()))
        }
        TSTypeName::ThisExpression(_) => Some("this".into()),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant type_name as type_name()
    participant as_str as as_str
    participant to_owned as to_owned
    participant Some as Some
    participant into as into
    alt match name
    type_name-->>as_str: id.name.as_str()
    type_name-->>to_owned: id.name.as_str().to_owned()
    type_name-->>Some: Some(id.name.as_str().to_owned())
    else TSTypeName::QualifiedName(q)
    type_name-->>type_name: type_name(&q.left)
    type_name-->>type_name: ? (propagate)
    type_name-->>type_name: format!('{left}.{}', q.right.name.as_str())
    type_name-->>Some: Some(format!('{left}.{}', q.right.name.as_str()))
    else TSTypeName::ThisExpression(_)
    type_name-->>into: 'this'.into()
    type_name-->>Some: Some('this'.into())
    end
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::typescript::type_name`](type-name.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str

## Referenced by

- calls ← [`robby_extract::typescript::emit_class`](emit-class.md)
- calls ← [`robby_extract::typescript::emit_interface`](emit-interface.md)

