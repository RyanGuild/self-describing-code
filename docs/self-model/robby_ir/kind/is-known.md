# `robby_ir::kind::is_known`

[Self-model](../../README.md) · [kind](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_known(kind: &Kind) -> bool` |
| location | [`crates/robby-ir/src/kind.rs:110`](../../../../crates/robby-ir/src/kind.rs#L110) |
| provenance | extractor · confidence 1.00 |

> True when `kind` is understood by the current version of the tool.
> 
> Unknown kinds are never an error — they are rendered generically and
> reported as a soft gap, so an older viewer can still display a newer graph.

```rust
/// True when `kind` is understood by the current version of the tool.
///
/// Unknown kinds are never an error — they are rendered generically and
/// reported as a soft gap, so an older viewer can still display a newer graph.
pub fn is_known(kind: &Kind) -> bool {
    use crate::contract::KindNamespace;
    kind.is_known_namespace()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant is_known as is_known()
    participant is_known_namespace as is_known_namespace
    is_known-->>is_known: use crate::contract::KindNamespace
    is_known-->>is_known_namespace: kind.is_known_namespace()
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_ir::contract::KindNamespace::is_known_namespace`](../contract/index.md)

## References

- calls → [`robby_ir::contract`](../contract/index.md) — is_known_namespace

## Referenced by

- calls ← [`robby_ir::model::validate`](../model/validate.md)

