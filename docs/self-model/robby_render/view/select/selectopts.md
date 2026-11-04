# `robby_render::view::select::SelectOpts`

[Self-model](../../../README.md) · [view::select](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/select.rs:30`](../../../../../crates/robby-render/src/view/select.rs#L30) |
| provenance | extractor · confidence 1.00 |

> Which optional sections to collect.

```rust
/// Which optional sections to collect.
#[derive(Debug, Clone)]
pub struct SelectOpts {
    pub hierarchy: bool,
    pub hierarchy_depth: usize,
    pub hierarchy_all: bool,
    pub sibling: bool,
    pub source: bool,
    pub types: bool,
    pub dependencies: bool,
    pub externals: bool,
    pub comments: bool,
    pub links: bool,
    pub entry: bool,
    pub process: bool,
}
```

**Fields and variants**

- `hierarchy` — `crates/robby-render/src/view/select.rs:33:5`
- `hierarchy_depth` — `crates/robby-render/src/view/select.rs:34:5`
- `hierarchy_all` — `crates/robby-render/src/view/select.rs:35:5`
- `sibling` — `crates/robby-render/src/view/select.rs:36:5`
- `source` — `crates/robby-render/src/view/select.rs:37:5`
- `types` — `crates/robby-render/src/view/select.rs:38:5`
- `dependencies` — `crates/robby-render/src/view/select.rs:39:5`
- `externals` — `crates/robby-render/src/view/select.rs:40:5`
- `comments` — `crates/robby-render/src/view/select.rs:41:5`
- `links` — `crates/robby-render/src/view/select.rs:42:5`
- `entry` — `crates/robby-render/src/view/select.rs:43:5`
- `process` — `crates/robby-render/src/view/select.rs:44:5`

