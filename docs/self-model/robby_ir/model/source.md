# `robby_ir::model::Source`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:64`](../../../../crates/robby-ir/src/model.rs#L64) |
| provenance | extractor · confidence 1.00 |

> Which producer contributed a fact.

```rust
/// Which producer contributed a fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Parsed from source without annotations.
    Extractor,
    /// Contributed by an explicit macro annotation.
    Macro,
    /// Recorded while the code ran.
    Trace,
    /// Emitted by an agent describing its own process.
    Agent,
    /// Authored by hand (fixtures, bootstrap data).
    Hand,
}
```

**Fields and variants**

- `Extractor` — `crates/robby-ir/src/model.rs:68:5`
- `Macro` — `crates/robby-ir/src/model.rs:70:5`
- `Trace` — `crates/robby-ir/src/model.rs:72:5`
- `Agent` — `crates/robby-ir/src/model.rs:74:5`
- `Hand` — `crates/robby-ir/src/model.rs:76:5`

