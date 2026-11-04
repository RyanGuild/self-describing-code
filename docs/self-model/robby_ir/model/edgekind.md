# `robby_ir::model::EdgeKind`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:187`](../../../../crates/robby-ir/src/model.rs#L187) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
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

- `Contains` — `crates/robby-ir/src/model.rs:192:5`
- `Calls` — `crates/robby-ir/src/model.rs:193:5`
- `Uses` — `crates/robby-ir/src/model.rs:194:5`
- `Imports` — `crates/robby-ir/src/model.rs:195:5`
- `Implements` — `crates/robby-ir/src/model.rs:196:5`
- `Extends` — `crates/robby-ir/src/model.rs:197:5`
- `Returns` — `crates/robby-ir/src/model.rs:198:5`
- `Field` — `crates/robby-ir/src/model.rs:199:5`
- `References` — `crates/robby-ir/src/model.rs:200:5`

