# `robby_ir::model::Node`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:185`](../../../../crates/robby-ir/src/model.rs#L185) |
| provenance | extractor · confidence 1.00 |

> A single described thing: a crate, a function, a statement, an observation.

```rust
/// A single described thing: a crate, a function, a statement, an observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Node {
    pub id: NodeId,
    pub kind: Kind,
    pub name: String,
    /// Fully-qualified, language-native path where one exists.
    pub fqn: String,
    pub lang: Lang,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    pub provenance: Provenance,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<NodeId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<Edge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
```

**Fields and variants**

- `id` — `crates/robby-ir/src/model.rs:188:5`
- `kind` — `crates/robby-ir/src/model.rs:189:5`
- `name` — `crates/robby-ir/src/model.rs:190:5`
- `fqn` — `crates/robby-ir/src/model.rs:191:5`
- `lang` — `crates/robby-ir/src/model.rs:193:5`
- `span` — `crates/robby-ir/src/model.rs:194:5`
- `signature` — `crates/robby-ir/src/model.rs:196:5`
- `docs` — `crates/robby-ir/src/model.rs:198:5`
- `provenance` — `crates/robby-ir/src/model.rs:200:5`
- `parent` — `crates/robby-ir/src/model.rs:201:5`
- `children` — `crates/robby-ir/src/model.rs:203:5`
- `edges` — `crates/robby-ir/src/model.rs:205:5`
- `process` — `crates/robby-ir/src/model.rs:207:5`
- `annotations` — `crates/robby-ir/src/model.rs:209:5`
- `tags` — `crates/robby-ir/src/model.rs:211:5`

