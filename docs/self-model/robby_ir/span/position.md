# `robby_ir::span::Position`

[Self-model](../../README.md) · [span](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/span.rs:10`](../../../../crates/robby-ir/src/span.rs#L10) |
| provenance | extractor · confidence 1.00 |

> A single point in a source file.

```rust
/// A single point in a source file.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column, counted in characters.
    pub column: u32,
    /// 0-based byte offset into the file.
    pub byte: u32,
}
```

**Fields and variants**

- `line` — `crates/robby-ir/src/span.rs:15:5`
- `column` — `crates/robby-ir/src/span.rs:17:5`
- `byte` — `crates/robby-ir/src/span.rs:19:5`

