# `robby_render::view::deps::DependencyViews`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/deps.rs:9`](../../../../../crates/robby-render/src/view/deps.rs#L9) |
| provenance | extractor · confidence 1.00 |

> Bundle of the three dependency formalisms for one root.

```rust
/// Bundle of the three dependency formalisms for one root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyViews {
    pub packages: PackageStructure,
    pub coupling: CouplingGraph,
}
```

**Fields and variants**

- `packages` — `crates/robby-render/src/view/deps.rs:12:5`
- `coupling` — `crates/robby-render/src/view/deps.rs:13:5`

