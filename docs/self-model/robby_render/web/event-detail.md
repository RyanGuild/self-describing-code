# `robby_render::web::event_detail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn event_detail(event: ProcessEvent) -> EventDetail` |
| location | [`crates/robby-render/src/web.rs:178`](../../../../crates/robby-render/src/web.rs#L178) |
| provenance | extractor · confidence 1.00 |

```rust
fn event_detail(event: ProcessEvent) -> EventDetail {
    match event {
        ProcessEvent::End => EventDetail::End,
        ProcessEvent::Else { label } => EventDetail::Else { label },
        ProcessEvent::Begin { keyword, label } => EventDetail::Begin {
            keyword: keyword.to_owned(),
            label,
        },
        ProcessEvent::Interaction {
            index,
            from,
            to,
            label,
            observed,
            kind,
        } => EventDetail::Interaction {
            index,
            from,
            to,
            label,
            observed,
            kind: step_kind_name(kind).to_owned(),
        },
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant event_detail as event_detail()
    participant to_owned as to_owned
    participant step_kind_name as step_kind_name
    alt match event
    event_detail-->>to_owned: keyword.to_owned()
    else ProcessEvent::Interaction { index, from, to, label, observed, kind, }
    event_detail-->>step_kind_name: step_kind_name(kind)
    event_detail-->>to_owned: step_kind_name(kind).to_owned()
    end
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_render::web::step_kind_name`](step-kind-name.md)

## References

- calls → [`robby_render::web`](index.md) — step_kind_name

