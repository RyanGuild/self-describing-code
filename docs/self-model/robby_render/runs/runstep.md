# `robby_render::runs::RunStep`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/runs.rs:65`](../../../../crates/robby-render/src/runs.rs#L65) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunStep {
    pub from: String,
    pub to: String,
    pub label: String,
    #[serde(default)]
    pub observed: bool,
}
```

**Fields and variants**

- `from` — `crates/robby-render/src/runs.rs:67:5`
- `to` — `crates/robby-render/src/runs.rs:68:5`
- `label` — `crates/robby-render/src/runs.rs:69:5`
- `observed` — `crates/robby-render/src/runs.rs:70:5`

