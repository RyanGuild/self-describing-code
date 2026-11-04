# `robby_render::web::ProcessDetail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/web.rs:49`](../../../../crates/robby-render/src/web.rs#L49) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessDetail {
    pub participants: Vec<ParticipantDetail>,
    pub events: Vec<EventDetail>,
    pub step_count: usize,
    pub observed_count: usize,
}
```

**Fields and variants**

- `participants` — `crates/robby-render/src/web.rs:51:5`
- `events` — `crates/robby-render/src/web.rs:52:5`
- `step_count` — `crates/robby-render/src/web.rs:53:5`
- `observed_count` — `crates/robby-render/src/web.rs:54:5`

