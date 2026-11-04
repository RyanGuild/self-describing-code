# `robby_agent::model::ChatClient`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-agent/src/model.rs:42`](../../../../crates/robby-agent/src/model.rs#L42) |
| provenance | extractor · confidence 1.00 |

> Pluggable chat backend (OpenRouter or fakes in tests).

```rust
/// Pluggable chat backend (OpenRouter or fakes in tests).
pub trait ChatClient {
    fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String>;
}
```

## Referenced by

- calls ← [`robby_agent::model::impl DecisionModel<'a, C>::decide`](index.md)
- calls ← [`robby_agent::model::impl NoteModel<'a, C>::write_note`](index.md)

### `chat`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-agent/src/model.rs:44`](../../../../crates/robby-agent/src/model.rs#L44) |
| provenance | extractor · confidence 1.00 |

```rust
fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String>;
```

