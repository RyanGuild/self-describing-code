# `robby_render::view::deps::DepModule`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:57`](../../../../../crates/robby-render/src/view/deps.rs#L57) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepModule {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub label: String,
    pub item_count: usize,
    pub external: bool,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/deps.rs:59:5`
- `stable_id` — `crates/robby-render/src/view/deps.rs:60:5`
- `name` — `crates/robby-render/src/view/deps.rs:61:5`
- `label` — `crates/robby-render/src/view/deps.rs:62:5`
- `item_count` — `crates/robby-render/src/view/deps.rs:63:5`
- `external` — `crates/robby-render/src/view/deps.rs:64:5`

