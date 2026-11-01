# `robby_ir::process::impl Default for Process`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/process.rs:148`](../../../../crates/robby-ir/src/process.rs#L148) |
| provenance | extractor · confidence 1.00 |

```rust
impl Default for Process {
    fn default() -> Self {
        Self::new()
    }
}
```

### `default`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn default(0)` |
| location | [`crates/robby-ir/src/process.rs:149`](../../../../crates/robby-ir/src/process.rs#L149) |
| provenance | extractor · confidence 1.00 |

```rust
fn default() -> Self {
        Self::new()
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant default as default()
    participant Self__new as Self::new
    default-->>Self__new: Self::new()
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

