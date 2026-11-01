# `robby_ir::coverage::FileCoverage`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/coverage.rs:55`](../../../../crates/robby-ir/src/coverage.rs#L55) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FileCoverage {
    pub file: String,
    pub total_bytes: u32,
    /// Bytes claimed by at least one node.
    pub covered_bytes: u32,
    pub gaps: Vec<Gap>,
}
```

**Fields and variants**

- `file` — `crates/robby-ir/src/coverage.rs:57:5`
- `total_bytes` — `crates/robby-ir/src/coverage.rs:58:5`
- `covered_bytes` — `crates/robby-ir/src/coverage.rs:59:5`
- `gaps` — `crates/robby-ir/src/coverage.rs:61:5`

