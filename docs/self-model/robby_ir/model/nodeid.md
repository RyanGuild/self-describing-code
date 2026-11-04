# `robby_ir::model::NodeId`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/model.rs:16`](../../../../crates/robby-ir/src/model.rs#L16) |
| provenance | extractor · confidence 1.00 |

> A stable, content-addressed node identifier.
> 
> Symbols use `lang://fqn`, e.g. `rust://robby_ir::model::Node`. Statements
> use `parent#ordinal`. Ordinal-based ids are stable within a run but shift
> when earlier siblings change; that is documented and accepted, because
> byte-offset ids would shift on every edit above them instead.

```rust
/// A stable, content-addressed node identifier.
///
/// Symbols use `lang://fqn`, e.g. `rust://robby_ir::model::Node`. Statements
/// use `parent#ordinal`. Ordinal-based ids are stable within a run but shift
/// when earlier siblings change; that is documented and accepted, because
/// byte-offset ids would shift on every edit above them instead.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct NodeId(String);
```

