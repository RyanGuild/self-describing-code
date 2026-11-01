# `robby_ir::coverage::Gap`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/coverage.rs:47`](../../../../crates/robby-ir/src/coverage.rs#L47) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Gap {
    pub span: Span,
    pub reason: GapReason,
    /// First 120 characters of the uncovered text, whitespace-collapsed.
    pub preview: String,
}
```

**Fields and variants**

- `span` — `crates/robby-ir/src/coverage.rs:49:5`
- `reason` — `crates/robby-ir/src/coverage.rs:50:5`
- `preview` — `crates/robby-ir/src/coverage.rs:51:5`

