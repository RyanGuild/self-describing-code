# `robby_render::view::process::ProcessView`

[Self-model](../../../README.md) · [view::process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/process.rs:7`](../../../../../crates/robby-render/src/view/process.rs#L7) |
| provenance | extractor · confidence 1.00 |

> Format-agnostic process diagram data.
> 
> This is also the JSON shape under `FrameDetail.process`. There is no
> second DTO.

```rust
/// Format-agnostic process diagram data.
///
/// This is also the JSON shape under `FrameDetail.process`. There is no
/// second DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessView {
    pub participants: Vec<ParticipantView>,
    pub events: Vec<ProcessEvent>,
    pub step_count: usize,
    pub observed_count: usize,
}
```

**Fields and variants**

- `participants` — `crates/robby-render/src/view/process.rs:13:5`
- `events` — `crates/robby-render/src/view/process.rs:14:5`
- `step_count` — `crates/robby-render/src/view/process.rs:15:5`
- `observed_count` — `crates/robby-render/src/view/process.rs:16:5`

