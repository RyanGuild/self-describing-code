# `robby_render::runs::RunProcess`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/runs.rs:52`](../../../../crates/robby-render/src/runs.rs#L52) |
| provenance | extractor · confidence 1.00 |

> Coarse stage timeline for the invocation itself (not the subject body).

```rust
/// Coarse stage timeline for the invocation itself (not the subject body).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunProcess {
    pub participants: Vec<RunParticipant>,
    pub steps: Vec<RunStep>,
}
```

**Fields and variants**

- `participants` — `crates/robby-render/src/runs.rs:55:5`
- `steps` — `crates/robby-render/src/runs.rs:56:5`

