# `robby_ir::codepath::CodePathEdge`

[Self-model](../../README.md) · [codepath](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/codepath.rs:19`](../../../../crates/robby-ir/src/codepath.rs#L19) |
| provenance | extractor · confidence 1.00 |

> One hop in an entrypoint call tree.

```rust
/// One hop in an entrypoint call tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CodePathEdge {
    pub from: NodeId,
    pub to: NodeId,
    /// Depth of [`Self::to`] from the path entry (entry itself is 0).
    pub depth: u32,
    #[serde(default = "default_confidence", skip_serializing_if = "is_one")]
    pub confidence: f32,
}
```

**Fields and variants**

- `from` — `crates/robby-ir/src/codepath.rs:22:5`
- `to` — `crates/robby-ir/src/codepath.rs:23:5`
- `depth` — `crates/robby-ir/src/codepath.rs:24:5`
- `confidence` — `crates/robby-ir/src/codepath.rs:26:5`

