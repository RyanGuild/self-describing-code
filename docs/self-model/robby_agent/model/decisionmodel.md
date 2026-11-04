# `robby_agent::model::DecisionModel`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:114`](../../../../crates/robby-agent/src/model.rs#L114) |
| provenance | extractor · confidence 1.00 |

```rust
pub struct DecisionModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}
```

**Fields and variants**

- `client` — `crates/robby-agent/src/model.rs:115:5`
- `model` — `crates/robby-agent/src/model.rs:116:5`

