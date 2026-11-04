# `robby_ir::model::impl From<&str> for NodeId`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:50`](../../../../crates/robby-ir/src/model.rs#L50) |
| provenance | extractor · confidence 1.00 |

```rust
impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}
```

### `from`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from(1)` |
| location | [`crates/robby-ir/src/model.rs:51`](../../../../crates/robby-ir/src/model.rs#L51) |
| provenance | extractor · confidence 1.00 |

```rust
fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from as from()
    participant to_owned as to_owned
    participant Self as Self
    from-->>to_owned: value.to_owned()
    from-->>Self: Self(value.to_owned())
```

2 steps · 0 observed · 2 possible only

