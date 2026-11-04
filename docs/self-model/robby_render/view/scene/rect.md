# `robby_render::view::scene::Rect`

[Self-model](../../../README.md) · [view::scene](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/scene.rs:40`](../../../../../crates/robby-render/src/view/scene.rs#L40) |
| provenance | extractor · confidence 1.00 |

> Integer rect on a frame's local 1600×1000 canvas.

```rust
/// Integer rect on a frame's local 1600×1000 canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}
```

**Fields and variants**

- `x` — `crates/robby-render/src/view/scene.rs:43:5`
- `y` — `crates/robby-render/src/view/scene.rs:44:5`
- `w` — `crates/robby-render/src/view/scene.rs:45:5`
- `h` — `crates/robby-render/src/view/scene.rs:46:5`

