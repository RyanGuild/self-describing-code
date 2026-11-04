# `robby_ir::model::Provenance`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:107`](../../../../crates/robby-ir/src/model.rs#L107) |
| provenance | extractor · confidence 1.00 |

> Where a fact came from and how much to trust it.

```rust
/// Where a fact came from and how much to trust it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Provenance {
    pub source: Source,
    /// Unix milliseconds. `None` means the fact is timeless (static analysis).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<i64>,
    /// `0.0..=1.0`. Static resolution that could not disambiguate a path
    /// records its uncertainty here rather than dropping the edge.
    pub confidence: f32,
}
```

**Fields and variants**

- `source` — `crates/robby-ir/src/model.rs:110:5`
- `observed_at` — `crates/robby-ir/src/model.rs:111:5`
- `confidence` — `crates/robby-ir/src/model.rs:114:5`

