# `robby_ir::model::Diagnostic`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:379`](../../../../crates/robby-ir/src/model.rs#L379) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Diagnostic {
    pub severity: Severity,
    /// Short machine-readable code, e.g. `edge/unresolved`.
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeId>,
}
```

**Fields and variants**

- `severity` — `crates/robby-ir/src/model.rs:381:5`
- `code` — `crates/robby-ir/src/model.rs:382:5`
- `message` — `crates/robby-ir/src/model.rs:384:5`
- `node` — `crates/robby-ir/src/model.rs:385:5`

