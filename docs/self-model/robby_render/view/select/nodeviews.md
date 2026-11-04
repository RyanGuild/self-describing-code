# `robby_render::view::select::NodeViews`

[Self-model](../../../README.md) · [view::select](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/select.rs:81`](../../../../../crates/robby-render/src/view/select.rs#L81) |
| provenance | extractor · confidence 1.00 |

> All views selected for one node.

```rust
/// All views selected for one node.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeViews {
    pub meta: NodeMeta,
    pub entry: Option<ProcessView>,
    pub types: Option<TypeModel>,
    pub dependencies: Option<DependencyViews>,
    pub externals: Option<ExternalsView>,
    pub comments: Option<CommentsView>,
    pub process: Option<ProcessView>,
    pub hierarchy: Option<HierarchyView>,
    pub links: LinkGraph,
    pub sibling: Option<SiblingGraph>,
    pub source: Option<SourceSlice>,
}
```

**Fields and variants**

- `meta` — `crates/robby-render/src/view/select.rs:84:5`
- `entry` — `crates/robby-render/src/view/select.rs:85:5`
- `types` — `crates/robby-render/src/view/select.rs:86:5`
- `dependencies` — `crates/robby-render/src/view/select.rs:87:5`
- `externals` — `crates/robby-render/src/view/select.rs:88:5`
- `comments` — `crates/robby-render/src/view/select.rs:89:5`
- `process` — `crates/robby-render/src/view/select.rs:90:5`
- `hierarchy` — `crates/robby-render/src/view/select.rs:91:5`
- `links` — `crates/robby-render/src/view/select.rs:92:5`
- `sibling` — `crates/robby-render/src/view/select.rs:93:5`
- `source` — `crates/robby-render/src/view/select.rs:94:5`

