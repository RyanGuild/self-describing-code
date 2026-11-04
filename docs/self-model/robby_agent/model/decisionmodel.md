# `robby_agent::model::DecisionModel`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:62`](../../../../crates/robby-agent/src/model.rs#L62) |
| provenance | extractor · confidence 1.00 |

```rust
pub struct DecisionModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}
```

**Fields and variants**

- `client` — `crates/robby-agent/src/model.rs:63:5`
- `model` — `crates/robby-agent/src/model.rs:64:5`

