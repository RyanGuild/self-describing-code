# `robby_render::view::links::InboundRef`

[Self-model](../../../README.md) · [view::links](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/links.rs:24`](../../../../../crates/robby-render/src/view/links.rs#L24) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundRef {
    pub kinds: Vec<&'static str>,
    pub caller_id: NodeId,
    pub caller_fqn: String,
}
```

**Fields and variants**

- `kinds` — `crates/robby-render/src/view/links.rs:26:5`
- `caller_id` — `crates/robby-render/src/view/links.rs:27:5`
- `caller_fqn` — `crates/robby-render/src/view/links.rs:28:5`

