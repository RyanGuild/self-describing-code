# `robby_render::view::scene::WorldRect`

[Self-model](../../../README.md) · [view::scene](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/scene.rs:55`](../../../../../crates/robby-render/src/view/scene.rs#L55) |
| provenance | extractor · confidence 1.00 |

> Absolute placement in workspace space. Floats so nested grids do not
> collapse to 1×1 integer cells and stack on top of each other.

```rust
/// Absolute placement in workspace space. Floats so nested grids do not
/// collapse to 1×1 integer cells and stack on top of each other.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct WorldRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
```

**Fields and variants**

- `x` — `crates/robby-render/src/view/scene.rs:59:5`
- `y` — `crates/robby-render/src/view/scene.rs:60:5`
- `w` — `crates/robby-render/src/view/scene.rs:61:5`
- `h` — `crates/robby-render/src/view/scene.rs:62:5`

