# `robby_render::view::links::LinkGraph`

[Self-model](../../../README.md) · [view::links](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/links.rs:9`](../../../../../crates/robby-render/src/view/links.rs#L9) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkGraph {
    pub outbound: Vec<OutboundRef>,
    pub inbound: Vec<InboundRef>,
}
```

**Fields and variants**

- `outbound` — `crates/robby-render/src/view/links.rs:11:5`
- `inbound` — `crates/robby-render/src/view/links.rs:12:5`

