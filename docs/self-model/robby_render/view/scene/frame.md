# `robby_render::view::scene::Frame`

[Self-model](../../../README.md) · [view::scene](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/scene.rs:79`](../../../../../crates/robby-render/src/view/scene.rs#L79) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Frame {
    pub id: String,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub kind_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    pub children: Vec<String>,
    /// Absolute placement in the workspace world.
    pub world: WorldRect,
    /// Placement inside the parent's local canvas (absent on the root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_in_parent: Option<Rect>,
    pub edges: Vec<SceneEdge>,
    pub portals: Vec<Portal>,
}
```

**Fields and variants**

- `id` — `crates/robby-render/src/view/scene.rs:81:5`
- `stable_id` — `crates/robby-render/src/view/scene.rs:82:5`
- `name` — `crates/robby-render/src/view/scene.rs:83:5`
- `fqn` — `crates/robby-render/src/view/scene.rs:84:5`
- `kind_tag` — `crates/robby-render/src/view/scene.rs:85:5`
- `parent` — `crates/robby-render/src/view/scene.rs:86:5`
- `children` — `crates/robby-render/src/view/scene.rs:88:5`
- `world` — `crates/robby-render/src/view/scene.rs:89:5`
- `local_in_parent` — `crates/robby-render/src/view/scene.rs:91:5`
- `edges` — `crates/robby-render/src/view/scene.rs:94:5`
- `portals` — `crates/robby-render/src/view/scene.rs:95:5`

