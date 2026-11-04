# `robby_ir::kind::impl std::fmt::Display for Kind`

[Self-model](../../README.md) · [kind](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/kind.rs:39`](../../../../crates/robby-ir/src/kind.rs#L39) |
| provenance | extractor · confidence 1.00 |

```rust
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
```

### `fmt`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn fmt(2)` |
| location | [`crates/robby-ir/src/kind.rs:40`](../../../../crates/robby-ir/src/kind.rs#L40) |
| provenance | extractor · confidence 1.00 |

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant fmt as fmt()
    participant write_str as write_str
    fmt-->>write_str: f.write_str(&self.0)
```

1 steps · 0 observed · 1 possible only

