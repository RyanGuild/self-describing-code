# `robby_ir::run::impl From<String> for RecipeId`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/run.rs:42`](../../../../crates/robby-ir/src/run.rs#L42) |
| provenance | extractor · confidence 1.00 |

```rust
impl From<String> for RecipeId {
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
| location | [`crates/robby-ir/src/run.rs:43`](../../../../crates/robby-ir/src/run.rs#L43) |
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

