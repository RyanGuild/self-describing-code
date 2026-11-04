# `robby_render::web::EventDetail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-render/src/web.rs:63`](../../../../crates/robby-render/src/web.rs#L63) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EventDetail {
    End,
    Else {
        label: String,
    },
    Begin {
        keyword: String,
        label: String,
    },
    Interaction {
        index: usize,
        from: String,
        to: String,
        label: String,
        observed: bool,
        kind: String,
    },
}
```

**Fields and variants**

- `End` — `crates/robby-render/src/web.rs:66:5`
- `Else` — `crates/robby-render/src/web.rs:67:5`
- `Begin` — `crates/robby-render/src/web.rs:70:5`
- `Interaction` — `crates/robby-render/src/web.rs:74:5`

