# `robby_render::view::select::NodeMeta`

[Self-model](../../../README.md) · [view::select](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/select.rs:17`](../../../../../crates/robby-render/src/view/select.rs#L17) |
| provenance | extractor · confidence 1.00 |

> Identity / provenance shown at the top of every report.

```rust
/// Identity / provenance shown at the top of every report.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeMeta {
    pub fqn: String,
    pub kind: String,
    pub signature: Option<String>,
    pub location: Option<String>,
    pub provenance_source: &'static str,
    pub confidence: f32,
    pub docs: Option<String>,
    pub node_count: usize,
}
```

**Fields and variants**

- `fqn` — `crates/robby-render/src/view/select.rs:20:5`
- `kind` — `crates/robby-render/src/view/select.rs:21:5`
- `signature` — `crates/robby-render/src/view/select.rs:22:5`
- `location` — `crates/robby-render/src/view/select.rs:23:5`
- `provenance_source` — `crates/robby-render/src/view/select.rs:24:5`
- `confidence` — `crates/robby-render/src/view/select.rs:25:5`
- `docs` — `crates/robby-render/src/view/select.rs:26:5`
- `node_count` — `crates/robby-render/src/view/select.rs:27:5`

