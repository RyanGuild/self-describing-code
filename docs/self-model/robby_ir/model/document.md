# `robby_ir::model::Document`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:264`](../../../../crates/robby-ir/src/model.rs#L264) |
| provenance | extractor · confidence 1.00 |

> The whole graph for one workspace snapshot.

```rust
/// The whole graph for one workspace snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Document {
    pub schema_version: u32,
    /// Tool name and version that produced this document.
    pub generator: String,
    pub nodes: BTreeMap<NodeId, Node>,
    /// Node ids with no parent.
    pub roots: Vec<NodeId>,
    /// Every source file this document claims to describe.
    ///
    /// This is what makes "no file is invisible" checkable: the coverage gate
    /// reads this list, so a file that was never parsed is a failure rather
    /// than an absence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
}
```

**Fields and variants**

- `schema_version` — `crates/robby-ir/src/model.rs:267:5`
- `generator` — `crates/robby-ir/src/model.rs:268:5`
- `nodes` — `crates/robby-ir/src/model.rs:270:5`
- `roots` — `crates/robby-ir/src/model.rs:271:5`
- `files` — `crates/robby-ir/src/model.rs:273:5`

