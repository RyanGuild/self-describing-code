# `robby_ir::model::Source`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:61`](../../../../crates/robby-ir/src/model.rs#L61) |
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

- `Extractor` — `crates/robby-ir/src/model.rs:65:5`
- `Macro` — `crates/robby-ir/src/model.rs:67:5`
- `Trace` — `crates/robby-ir/src/model.rs:69:5`
- `Agent` — `crates/robby-ir/src/model.rs:71:5`
- `Hand` — `crates/robby-ir/src/model.rs:73:5`

