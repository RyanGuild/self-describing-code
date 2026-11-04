# `robby_render::view::process::ProcessView`

[Self-model](../../../README.md) · [view::process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/process.rs:7`](../../../../../crates/robby-render/src/view/process.rs#L7) |
| provenance | extractor · confidence 1.00 |

> Format-agnostic process diagram data.

```rust
/// Format-agnostic process diagram data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessView {
    pub participants: Vec<ParticipantView>,
    pub events: Vec<ProcessEvent>,
    pub step_count: usize,
    pub observed_count: usize,
}
```

**Fields and variants**

- `participants` — `crates/robby-render/src/view/process.rs:10:5`
- `events` — `crates/robby-render/src/view/process.rs:11:5`
- `step_count` — `crates/robby-render/src/view/process.rs:12:5`
- `observed_count` — `crates/robby-render/src/view/process.rs:13:5`

