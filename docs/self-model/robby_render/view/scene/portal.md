# `robby_render::view::scene::Portal`

[Self-model](../../../README.md) · [view::scene](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/scene.rs:106`](../../../../../crates/robby-render/src/view/scene.rs#L106) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Portal {
    pub label: String,
    pub kind_label: String,
    /// Stable id of the destination frame when the edge resolves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub target_name: String,
}
```

**Fields and variants**

- `label` — `crates/robby-render/src/view/scene.rs:108:5`
- `kind_label` — `crates/robby-render/src/view/scene.rs:109:5`
- `target` — `crates/robby-render/src/view/scene.rs:110:5`
- `target_name` — `crates/robby-render/src/view/scene.rs:113:5`

