# `robby_render::web::step_kind_name`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn step_kind_name(kind: StepKind) -> &'static str` |
| location | [`crates/robby-render/src/web.rs:204`](../../../../crates/robby-render/src/web.rs#L204) |
| provenance | extractor · confidence 1.00 |

```rust
fn step_kind_name(kind: StepKind) -> &'static str {
    match kind {
        StepKind::Call => "call",
        StepKind::Return => "return",
        StepKind::Signal => "signal",
        StepKind::Decision => "decision",
        StepKind::Note => "note",
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant step_kind_name as step_kind_name()
```

0 steps · 0 observed · 0 possible only

## Referenced by

- calls ← [`robby_render::web::event_detail`](event-detail.md)

