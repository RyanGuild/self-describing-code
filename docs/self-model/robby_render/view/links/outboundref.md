# `robby_render::view::links::OutboundRef`

[Self-model](../../../README.md) · [view::links](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/links.rs:15`](../../../../../crates/robby-render/src/view/links.rs#L15) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundRef {
    pub kind: EdgeKind,
    pub kind_label: &'static str,
    pub module_id: NodeId,
    pub module_fqn: String,
    pub items: Vec<String>,
}
```

**Fields and variants**

- `kind` — `crates/robby-render/src/view/links.rs:17:5`
- `kind_label` — `crates/robby-render/src/view/links.rs:18:5`
- `module_id` — `crates/robby-render/src/view/links.rs:19:5`
- `module_fqn` — `crates/robby-render/src/view/links.rs:20:5`
- `items` — `crates/robby-render/src/view/links.rs:21:5`

