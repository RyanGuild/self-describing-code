# `robby_ir::model::impl From<String> for NodeId`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:55`](../../../../crates/robby-ir/src/model.rs#L55) |
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
| location | [`crates/robby-ir/src/model.rs:56`](../../../../crates/robby-ir/src/model.rs#L56) |
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

