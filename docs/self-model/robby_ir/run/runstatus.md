# `robby_ir::run::RunStatus`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/run.rs:48`](../../../../crates/robby-ir/src/run.rs#L48) |
| provenance | extractor · confidence 1.00 |

> Shared status for tests, extract, enrich, and future trace runs.

```rust
/// Shared status for tests, extract, enrich, and future trace runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Pass,
    Fail,
    Error,
    Cancelled,
}
```

**Fields and variants**

- `Queued` — `crates/robby-ir/src/run.rs:52:5`
- `Running` — `crates/robby-ir/src/run.rs:53:5`
- `Pass` — `crates/robby-ir/src/run.rs:54:5`
- `Fail` — `crates/robby-ir/src/run.rs:55:5`
- `Error` — `crates/robby-ir/src/run.rs:56:5`
- `Cancelled` — `crates/robby-ir/src/run.rs:57:5`

