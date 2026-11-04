# `robby_render::view::deps::CouplingModule`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:85`](../../../../../crates/robby-render/src/view/deps.rs#L85) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingModule {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub children: Vec<CouplingModule>,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/deps.rs:87:5`
- `stable_id` — `crates/robby-render/src/view/deps.rs:88:5`
- `name` — `crates/robby-render/src/view/deps.rs:89:5`
- `fqn` — `crates/robby-render/src/view/deps.rs:90:5`
- `children` — `crates/robby-render/src/view/deps.rs:91:5`

