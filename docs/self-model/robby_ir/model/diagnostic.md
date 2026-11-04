# `robby_ir::model::Diagnostic`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:536`](../../../../crates/robby-ir/src/model.rs#L536) |
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

- `severity` — `crates/robby-ir/src/model.rs:538:5`
- `code` — `crates/robby-ir/src/model.rs:539:5`
- `message` — `crates/robby-ir/src/model.rs:541:5`
- `node` — `crates/robby-ir/src/model.rs:542:5`

