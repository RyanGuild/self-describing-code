# `robby_ir::model::Severity`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:528`](../../../../crates/robby-ir/src/model.rs#L528) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}
```

**Fields and variants**

- `Error` — `crates/robby-ir/src/model.rs:531:5`
- `Warning` — `crates/robby-ir/src/model.rs:532:5`
- `Info` — `crates/robby-ir/src/model.rs:533:5`

