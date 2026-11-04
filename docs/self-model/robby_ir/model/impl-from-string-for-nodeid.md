# `robby_ir::model::impl From<String> for NodeId`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:58`](../../../../crates/robby-ir/src/model.rs#L58) |
| provenance | extractor · confidence 1.00 |

```rust
impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}
```

### `from`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from(1)` |
| location | [`crates/robby-ir/src/model.rs:59`](../../../../crates/robby-ir/src/model.rs#L59) |
| provenance | extractor · confidence 1.00 |

```rust
fn from(value: String) -> Self {
        Self(value)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from as from()
    participant Self as Self
    from-->>Self: Self(value)
```

1 steps · 0 observed · 1 possible only

