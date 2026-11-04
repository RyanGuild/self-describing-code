# `robby_render::runs::RunIndexEntry`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/runs.rs:74`](../../../../crates/robby-render/src/runs.rs#L74) |
| provenance | extractor · confidence 1.00 |

> Compact row for `runs/index.json` (viewer list).

```rust
/// Compact row for `runs/index.json` (viewer list).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunIndexEntry {
    pub run_id: String,
    pub recipe: String,
    pub subject: String,
    pub status: String,
    pub started_at: u64,
    pub finished_at: u64,
}
```

**Fields and variants**

- `run_id` — `crates/robby-render/src/runs.rs:77:5`
- `recipe` — `crates/robby-render/src/runs.rs:78:5`
- `subject` — `crates/robby-render/src/runs.rs:79:5`
- `status` — `crates/robby-render/src/runs.rs:80:5`
- `started_at` — `crates/robby-render/src/runs.rs:81:5`
- `finished_at` — `crates/robby-render/src/runs.rs:82:5`

