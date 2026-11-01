# `robby_ir::process::StepKind`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/process.rs:32`](../../../../crates/robby-ir/src/process.rs#L32) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StepKind {
    Call,
    Return,
    Signal,
    Decision,
    Note,
}
```

**Fields and variants**

- `Call` — `crates/robby-ir/src/process.rs:35:5`
- `Return` — `crates/robby-ir/src/process.rs:36:5`
- `Signal` — `crates/robby-ir/src/process.rs:37:5`
- `Decision` — `crates/robby-ir/src/process.rs:38:5`
- `Note` — `crates/robby-ir/src/process.rs:39:5`

