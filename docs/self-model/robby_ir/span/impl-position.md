# `robby_ir::span::impl Position`

[Self-model](../../README.md) · [span](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/span.rs:23`](../../../../crates/robby-ir/src/span.rs#L23) |
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
        let line_start = prefix.rfind('\n').map_or(0, |i| i + 1);
        let column = prefix[line_start..].chars().count() as u32 + 1;
        Self { line, column, byte }
    }
}
```

## Referenced by

- calls ← [`robby_ir::coverage::make_gap`](../coverage/make-gap.md)
- calls ← [`robby_ir::span::impl Span::new`](index.md)

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(3)` |
| location | [`crates/robby-ir/src/span.rs:24`](../../../../crates/robby-ir/src/span.rs#L24) |
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
```

0 steps · 0 observed · 0 possible only

### `at`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn at(2)` |
| location | [`crates/robby-ir/src/span.rs:28`](../../../../crates/robby-ir/src/span.rs#L28) |
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
        let line_start = prefix.rfind('\n').map_or(0, |i| i + 1);
        let column = prefix[line_start..].chars().count() as u32 + 1;
        Self { line, column, byte }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant at as at()
    participant len as len
    participant min as min
    participant matches as matches
    participant count as count
    participant rfind as rfind
    participant map_or as map_or
    participant chars as chars
    at-->>len: source.len()
    at-->>min: byte.min(source.len() as u32)
    at-->>matches: prefix.matches('\n')
    at-->>count: prefix.matches('\n').count()
    at-->>rfind: prefix.rfind('\n')
    at-->>map_or: prefix.rfind('\n').map_or(0, |i| i + 1)
    at-->>chars: prefix[line_start..].chars()
    at-->>count: prefix[line_start..].chars().count()
```

8 steps · 0 observed · 8 possible only

