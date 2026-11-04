# `robby_ir::model::Edge`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:135`](../../../../crates/robby-ir/src/model.rs#L135) |
| provenance | extractor · confidence 1.00 |

> A directed relationship between nodes.

```rust
/// A directed relationship between nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Edge {
    pub to: NodeId,
    pub kind: EdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "is_one")]
    pub confidence: f32,
    /// `false` when the target could not be resolved to a node in this graph.
    ///
    /// Unresolved edges are kept, never dropped. Losing edges silently is how
    /// a map stops being trustworthy.
    #[serde(default = "default_true")]
    pub resolved: bool,
}
```

**Fields and variants**

- `to` — `crates/robby-ir/src/model.rs:138:5`
- `kind` — `crates/robby-ir/src/model.rs:139:5`
- `span` — `crates/robby-ir/src/model.rs:140:5`
- `confidence` — `crates/robby-ir/src/model.rs:142:5`
- `resolved` — `crates/robby-ir/src/model.rs:144:5`

