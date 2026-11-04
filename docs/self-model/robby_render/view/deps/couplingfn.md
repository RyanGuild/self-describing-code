# `robby_render::view::deps::CouplingFn`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:94`](../../../../../crates/robby-render/src/view/deps.rs#L94) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingFn {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    /// Owning module (or crate) stable_id.
    pub parent_stable_id: String,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/deps.rs:96:5`
- `stable_id` — `crates/robby-render/src/view/deps.rs:97:5`
- `name` — `crates/robby-render/src/view/deps.rs:98:5`
- `parent_stable_id` — `crates/robby-render/src/view/deps.rs:99:5`

