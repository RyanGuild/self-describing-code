# `robby_render::view::scene::Scene`

[Self-model](../../../README.md) · [view::scene](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/scene.rs:26`](../../../../../crates/robby-render/src/view/scene.rs#L26) |
| provenance | extractor · confidence 1.00 |

> Whole zoomable map: one frame per structural node, plus a synthetic workspace.

```rust
/// Whole zoomable map: one frame per structural node, plus a synthetic workspace.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Scene {
    pub root: String,
    pub canvas: CanvasSize,
    pub frames: BTreeMap<String, Frame>,
}
```

**Fields and variants**

- `root` — `crates/robby-render/src/view/scene.rs:29:5`
- `canvas` — `crates/robby-render/src/view/scene.rs:30:5`
- `frames` — `crates/robby-render/src/view/scene.rs:31:5`

