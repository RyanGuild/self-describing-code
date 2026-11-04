# `robby_ir::contract::kind_namespace_helpers`

[Self-model](../../../README.md) · [contract::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn kind_namespace_helpers()` |
| location | [`crates/robby-ir/src/contract.rs:131`](../../../../../crates/robby-ir/src/contract.rs#L131) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn kind_namespace_helpers() {
        let code = Kind::new(kind::code::FUNCTION);
        assert!(code.is_code());
        assert!(code.is_known_namespace());
        assert!(!code.is_agent());

        let weird = Kind::new("plugin.widget");
        assert!(!weird.is_known_namespace());
        assert_eq!(weird.namespace(), "plugin");
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant kind_namespace_helpers as kind_namespace_helpers()
    participant Kind__new as Kind::new
    kind_namespace_helpers-->>Kind__new: Kind::new(kind::code::FUNCTION)
    kind_namespace_helpers-->>kind_namespace_helpers: assert!(code.is_code())
    kind_namespace_helpers-->>kind_namespace_helpers: assert!(code.is_known_namespace())
    kind_namespace_helpers-->>kind_namespace_helpers: assert!(!code.is_agent())
    kind_namespace_helpers-->>Kind__new: Kind::new('plugin.widget')
    kind_namespace_helpers-->>kind_namespace_helpers: assert!(!weird.is_known_namespace())
    kind_namespace_helpers-->>kind_namespace_helpers: assert_eq!(weird.namespace(), 'plugin')
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)

## References

- calls → [`robby_ir::kind`](../../kind/index.md) — new

