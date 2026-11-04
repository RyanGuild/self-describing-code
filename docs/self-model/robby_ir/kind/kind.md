# `robby_ir::kind::Kind`

[Self-model](../../README.md) · [kind](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/kind.rs:17`](../../../../crates/robby-ir/src/kind.rs#L17) |
| provenance | extractor · confidence 1.00 |

> A namespaced, open-ended kind tag.

```rust
/// A namespaced, open-ended kind tag.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct Kind(String);
```

