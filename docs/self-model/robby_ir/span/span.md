# `robby_ir::span::Span`

[Self-model](../../README.md) · [span](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/span.rs:42`](../../../../crates/robby-ir/src/span.rs#L42) |
| provenance | extractor · confidence 1.00 |

> A half-open byte range within one file.

```rust
/// A half-open byte range within one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Span {
    /// Repository-relative path, always with forward slashes.
    pub file: String,
    pub start: Position,
    pub end: Position,
}
```

**Fields and variants**

- `file` — `crates/robby-ir/src/span.rs:45:5`
- `start` — `crates/robby-ir/src/span.rs:47:5`
- `end` — `crates/robby-ir/src/span.rs:48:5`

