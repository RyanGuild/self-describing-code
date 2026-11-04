# `robby_ir::run::RecipeId`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/run.rs:13`](../../../../crates/robby-ir/src/run.rs#L13) |
| provenance | extractor · confidence 1.00 |

> Stable recipe identifier, e.g. `robby.extract` or `cargo.test`.

```rust
/// Stable recipe identifier, e.g. `robby.extract` or `cargo.test`.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct RecipeId(String);
```

