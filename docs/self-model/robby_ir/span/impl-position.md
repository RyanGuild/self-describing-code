# `robby_ir::span::impl Position`

[Self-model](../../README.md) · [span](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/span.rs:21`](../../../../crates/robby-ir/src/span.rs#L21) |
| provenance | extractor · confidence 1.00 |

```rust
impl Position {
    pub fn new(line: u32, column: u32, byte: u32) -> Self {
        Self { line, column, byte }
    }

    /// Resolve a byte offset to a line/column position.
    ///
    /// Out-of-range offsets clamp to the end of the source rather than panicking:
    /// a slightly wrong span must never take down the whole render.
    pub fn at(source: &str, byte: u32) -> Self {
        let byte = byte.min(source.len() as u32);
        let prefix = &source[..byte as usize];
        let line = prefix.matches('\n').count() as u32 + 1;
        let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = prefix[line_start..].chars().count() as u32 + 1;
        Self { line, column, byte }
    }
}
```

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(3)` |
| location | [`crates/robby-ir/src/span.rs:22`](../../../../crates/robby-ir/src/span.rs#L22) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new(line: u32, column: u32, byte: u32) -> Self {
        Self { line, column, byte }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    new-->>new: Self { line, column, byte }
```

1 steps · 0 observed · 1 possible only

### `at`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn at(2)` |
| location | [`crates/robby-ir/src/span.rs:26`](../../../../crates/robby-ir/src/span.rs#L26) |
| provenance | extractor · confidence 1.00 |

> Resolve a byte offset to a line/column position.
> 
> Out-of-range offsets clamp to the end of the source rather than panicking:
> a slightly wrong span must never take down the whole render.

```rust
/// Resolve a byte offset to a line/column position.
    ///
    /// Out-of-range offsets clamp to the end of the source rather than panicking:
    /// a slightly wrong span must never take down the whole render.
    pub fn at(source: &str, byte: u32) -> Self {
        let byte = byte.min(source.len() as u32);
        let prefix = &source[..byte as usize];
        let line = prefix.matches('\n').count() as u32 + 1;
        let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = prefix[line_start..].chars().count() as u32 + 1;
        Self { line, column, byte }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant at as at()
    participant min as min
    participant unwrap_or as unwrap_or
    at-->>min: byte.min(source.len() as u32)
    at-->>at: &source[..byte as usize]
    at-->>at: prefix.matches('\n').count() as u32 + 1
    at-->>unwrap_or: prefix.rfind('\n').map(|i| i + 1).unwrap_or(0)
    at-->>at: prefix[line_start..].chars().count() as u32 + 1
    at-->>at: Self { line, column, byte }
```

6 steps · 0 observed · 6 possible only

