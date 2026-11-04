# `robby_ir::process::Process`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/process.rs:111`](../../../../crates/robby-ir/src/process.rs#L111) |
| provenance | extractor · confidence 1.00 |

> The process a node performs, or the process a trace recorded.

```rust
/// The process a node performs, or the process a trace recorded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Process {
    pub participants: Vec<Participant>,
    pub steps: Vec<Step>,
}
```

**Fields and variants**

- `participants` — `crates/robby-ir/src/process.rs:114:5`
- `steps` — `crates/robby-ir/src/process.rs:115:5`

