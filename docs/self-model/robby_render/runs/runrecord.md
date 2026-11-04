# `robby_render::runs::RunRecord`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/runs.rs:31`](../../../../crates/robby-render/src/runs.rs#L31) |
| provenance | extractor · confidence 1.00 |

> One invocation — IR `agent.run` payload and/or `runs/<id>.json`.

```rust
/// One invocation — IR `agent.run` payload and/or `runs/<id>.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: String,
    pub recipe: String,
    pub subject: String,
    pub status: String,
    pub started_at: u64,
    pub finished_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout_tail: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stats: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<RunProcess>,
}
```

**Fields and variants**

- `run_id` — `crates/robby-render/src/runs.rs:34:5`
- `recipe` — `crates/robby-render/src/runs.rs:35:5`
- `subject` — `crates/robby-render/src/runs.rs:36:5`
- `status` — `crates/robby-render/src/runs.rs:37:5`
- `started_at` — `crates/robby-render/src/runs.rs:38:5`
- `finished_at` — `crates/robby-render/src/runs.rs:39:5`
- `exit_code` — `crates/robby-render/src/runs.rs:40:5`
- `stdout_tail` — `crates/robby-render/src/runs.rs:42:5`
- `stats` — `crates/robby-render/src/runs.rs:44:5`
- `products` — `crates/robby-render/src/runs.rs:46:5`
- `process` — `crates/robby-render/src/runs.rs:48:5`

