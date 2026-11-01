# `robby_ir::process::Process`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/process.rs:85`](../../../../crates/robby-ir/src/process.rs#L85) |
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

- `participants` — `crates/robby-ir/src/process.rs:88:5`
- `steps` — `crates/robby-ir/src/process.rs:89:5`

