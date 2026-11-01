# `robby_ir::kind::impl From<String> for Kind`

[Self-model](../../README.md) · [kind](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/kind.rs:49`](../../../../crates/robby-ir/src/kind.rs#L49) |
| provenance | extractor · confidence 1.00 |

```rust
impl From<String> for Kind {
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
| location | [`crates/robby-ir/src/kind.rs:50`](../../../../crates/robby-ir/src/kind.rs#L50) |
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

