# `robby_render::view::deps::WeightedEdge`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:67`](../../../../../crates/robby-render/src/view/deps.rs#L67) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightedEdge {
    pub from: String,
    pub to: String,
    pub weight: usize,
    pub in_cycle: bool,
}
```

**Fields and variants**

- `from` — `crates/robby-render/src/view/deps.rs:69:5`
- `to` — `crates/robby-render/src/view/deps.rs:70:5`
- `weight` — `crates/robby-render/src/view/deps.rs:71:5`
- `in_cycle` — `crates/robby-render/src/view/deps.rs:72:5`

