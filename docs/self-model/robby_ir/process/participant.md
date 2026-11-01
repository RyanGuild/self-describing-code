# `robby_ir::process::Participant`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/process.rs:18`](../../../../crates/robby-ir/src/process.rs#L18) |
| provenance | extractor · confidence 1.00 |

> One lifeline in a sequence diagram.
> 
> Today a participant is a module, scope, or type. The field set is
> deliberately actor-neutral so that `agent.*` participants can be added
> later without touching the renderer.

```rust
/// One lifeline in a sequence diagram.
///
/// Today a participant is a module, scope, or type. The field set is
/// deliberately actor-neutral so that `agent.*` participants can be added
/// later without touching the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Participant {
    /// Local identifier, referenced by [`Step::from`] and [`Step::to`].
    pub id: String,
    /// Human-facing label, e.g. `io::load` or `planner`.
    pub label: String,
    pub kind: Kind,
}
```

**Fields and variants**

- `id` — `crates/robby-ir/src/process.rs:25:5`
- `label` — `crates/robby-ir/src/process.rs:27:5`
- `kind` — `crates/robby-ir/src/process.rs:29:5`

