# `robby_agent::model::NoteModel`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:142`](../../../../crates/robby-agent/src/model.rs#L142) |
| provenance | extractor · confidence 1.00 |

```rust
pub struct NoteModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}
```

**Fields and variants**

- `client` — `crates/robby-agent/src/model.rs:143:5`
- `model` — `crates/robby-agent/src/model.rs:144:5`

