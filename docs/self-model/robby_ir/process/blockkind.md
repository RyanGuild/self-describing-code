# `robby_ir::process::BlockKind`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/process.rs:69`](../../../../crates/robby-ir/src/process.rs#L69) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BlockKind {
    Alt,
    Opt,
    Loop,
    Par,
    Break,
}
```

**Fields and variants**

- `Alt` — `crates/robby-ir/src/process.rs:72:5`
- `Opt` — `crates/robby-ir/src/process.rs:73:5`
- `Loop` — `crates/robby-ir/src/process.rs:74:5`
- `Par` — `crates/robby-ir/src/process.rs:75:5`
- `Break` — `crates/robby-ir/src/process.rs:76:5`

