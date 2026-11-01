# `robby_ir::coverage::CoverageReport`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/coverage.rs:113`](../../../../crates/robby-ir/src/coverage.rs#L113) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CoverageReport {
    pub files: Vec<FileCoverage>,
}
```

**Fields and variants**

- `files` — `crates/robby-ir/src/coverage.rs:115:5`

