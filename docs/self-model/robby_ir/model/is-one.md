# `robby_ir::model::is_one`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn is_one(v: &f32) -> bool` |
| location | [`crates/robby-ir/src/model.rs:152`](../../../../crates/robby-ir/src/model.rs#L152) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_one(v: &f32) -> bool {
    (*v - 1.0).abs() < f32::EPSILON
}
```

```mermaid
sequenceDiagram
    autonumber
    participant is_one as is_one()
    participant abs as abs
    is_one-->>abs: (*v - 1.0).abs()
```

1 steps · 0 observed · 1 possible only

