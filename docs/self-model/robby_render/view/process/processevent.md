# `robby_render::view::process::ProcessEvent`

[Self-model](../../../README.md) · [view::process](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-render/src/view/process.rs:22`](../../../../../crates/robby-render/src/view/process.rs#L22) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessEvent {
    End,
    Else {
        label: String,
    },
    Begin {
        keyword: &'static str,
        label: String,
    },
    Interaction {
        index: usize,
        from: String,
        to: String,
        label: String,
        observed: bool,
        kind: StepKind,
    },
}
```

**Fields and variants**

- `End` — `crates/robby-render/src/view/process.rs:24:5`
- `Else` — `crates/robby-render/src/view/process.rs:25:5`
- `Begin` — `crates/robby-render/src/view/process.rs:28:5`
- `Interaction` — `crates/robby-render/src/view/process.rs:32:5`

