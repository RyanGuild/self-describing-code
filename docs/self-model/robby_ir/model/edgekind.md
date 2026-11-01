# `robby_ir::model::EdgeKind`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:171`](../../../../crates/robby-ir/src/model.rs#L171) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Contains,
    Calls,
    Uses,
    Imports,
    Implements,
    Extends,
    Returns,
    Field,
    References,
}
```

**Fields and variants**

- `Contains` — `crates/robby-ir/src/model.rs:174:5`
- `Calls` — `crates/robby-ir/src/model.rs:175:5`
- `Uses` — `crates/robby-ir/src/model.rs:176:5`
- `Imports` — `crates/robby-ir/src/model.rs:177:5`
- `Implements` — `crates/robby-ir/src/model.rs:178:5`
- `Extends` — `crates/robby-ir/src/model.rs:179:5`
- `Returns` — `crates/robby-ir/src/model.rs:180:5`
- `Field` — `crates/robby-ir/src/model.rs:181:5`
- `References` — `crates/robby-ir/src/model.rs:182:5`

