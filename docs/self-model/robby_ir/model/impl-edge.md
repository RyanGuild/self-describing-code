# `robby_ir::model::impl Edge`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:159`](../../../../crates/robby-ir/src/model.rs#L159) |
| provenance | extractor · confidence 1.00 |

```rust
impl Edge {
    pub fn new(to: impl Into<NodeId>, kind: EdgeKind) -> Self {
        Self {
            to: to.into(),
            kind,
            span: None,
            confidence: 1.0,
            resolved: true,
        }
    }
}
```

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(2)` |
| location | [`crates/robby-ir/src/model.rs:160`](../../../../crates/robby-ir/src/model.rs#L160) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new(to: impl Into<NodeId>, kind: EdgeKind) -> Self {
        Self {
            to: to.into(),
            kind,
            span: None,
            confidence: 1.0,
            resolved: true,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    new-->>new: Self { to: to.into(), kind, span: None, confidence: 1.0, resolved: tr...
```

1 steps · 0 observed · 1 possible only

