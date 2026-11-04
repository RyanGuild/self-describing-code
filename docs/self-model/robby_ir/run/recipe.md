# `robby_ir::run::Recipe`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/run.rs:98`](../../../../crates/robby-ir/src/run.rs#L98) |
| provenance | extractor · confidence 1.00 |

> Catalog entry for an invokable procedure.

```rust
/// Catalog entry for an invokable procedure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Recipe {
    pub id: RecipeId,
    pub label: String,
    /// Policy / allowlist key for local invoke (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowlist: Option<String>,
}
```

**Fields and variants**

- `id` — `crates/robby-ir/src/run.rs:101:5`
- `label` — `crates/robby-ir/src/run.rs:102:5`
- `allowlist` — `crates/robby-ir/src/run.rs:103:5`

