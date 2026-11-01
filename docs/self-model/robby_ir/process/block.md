# `robby_ir::process::Block`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/process.rs:42`](../../../../crates/robby-ir/src/process.rs#L42) |
| provenance | extractor · confidence 1.00 |

> A control-flow frame enclosing a step.
> 
> Frames form a stack per step. The renderer diffs consecutive stacks and
> emits `alt` / `else` / `opt` / `loop` / `end` accordingly, which keeps the
> IR flat and diff-friendly while still expressing nesting.

```rust
/// A control-flow frame enclosing a step.
///
/// Frames form a stack per step. The renderer diffs consecutive stacks and
/// emits `alt` / `else` / `opt` / `loop` / `end` accordingly, which keeps the
/// IR flat and diff-friendly while still expressing nesting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Block {
    pub kind: BlockKind,
    pub label: String,
    /// For `alt`: which branch this step belongs to. Rendered as `else`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}
```

**Fields and variants**

- `kind` — `crates/robby-ir/src/process.rs:49:5`
- `label` — `crates/robby-ir/src/process.rs:50:5`
- `branch` — `crates/robby-ir/src/process.rs:51:5`

