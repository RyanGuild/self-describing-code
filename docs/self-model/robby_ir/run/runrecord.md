# `robby_ir::run::RunRecord`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/run.rs:108`](../../../../crates/robby-ir/src/run.rs#L108) |
| provenance | extractor · confidence 1.00 |

> One invocation: past, live, or queued.
> 
> Persist the same shape as an `agent.run` node payload and/or
> `runs/<run_id>.json`. Optional [`Process`] is the **run** stage timeline
> (extract → check → write), not the subject function body.

```rust
/// One invocation: past, live, or queued.
///
/// Persist the same shape as an `agent.run` node payload and/or
/// `runs/<run_id>.json`. Optional [`Process`] is the **run** stage timeline
/// (extract → check → write), not the subject function body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RunRecord {
    pub run_id: String,
    pub recipe: RecipeId,
    /// Subject node id or stable id (tests).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub status: RunStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout_tail: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stats: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
}
```

**Fields and variants**

- `run_id` — `crates/robby-ir/src/run.rs:115:5`
- `recipe` — `crates/robby-ir/src/run.rs:116:5`
- `subject` — `crates/robby-ir/src/run.rs:117:5`
- `status` — `crates/robby-ir/src/run.rs:120:5`
- `started_at` — `crates/robby-ir/src/run.rs:121:5`
- `finished_at` — `crates/robby-ir/src/run.rs:123:5`
- `exit_code` — `crates/robby-ir/src/run.rs:125:5`
- `stdout_tail` — `crates/robby-ir/src/run.rs:127:5`
- `stats` — `crates/robby-ir/src/run.rs:129:5`
- `products` — `crates/robby-ir/src/run.rs:131:5`
- `process` — `crates/robby-ir/src/run.rs:133:5`

