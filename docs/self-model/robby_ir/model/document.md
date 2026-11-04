# `robby_ir::model::Document`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:328`](../../../../crates/robby-ir/src/model.rs#L328) |
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
    /// Entrypoint-rooted call trees derived from resolved `Calls` edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_paths: Vec<CodePath>,
    /// Children whose parent node is not in the document yet.
    ///
    /// Not part of the schema. Flushed when the parent is inserted so a child
    /// can be recorded before its parent without a second writer.
    #[serde(skip)]
    #[schemars(skip)]
    pending_children: BTreeMap<NodeId, Vec<NodeId>>,
    /// Parent declared for a child node that has not been inserted yet.
    #[serde(skip)]
    #[schemars(skip)]
    pending_parents: BTreeMap<NodeId, NodeId>,
}
```

**Fields and variants**

- `schema_version` — `crates/robby-ir/src/model.rs:331:5`
- `generator` — `crates/robby-ir/src/model.rs:332:5`
- `nodes` — `crates/robby-ir/src/model.rs:334:5`
- `roots` — `crates/robby-ir/src/model.rs:335:5`
- `files` — `crates/robby-ir/src/model.rs:337:5`
- `code_paths` — `crates/robby-ir/src/model.rs:344:5`
- `pending_children` — `crates/robby-ir/src/model.rs:347:5`
- `pending_parents` — `crates/robby-ir/src/model.rs:354:5`

