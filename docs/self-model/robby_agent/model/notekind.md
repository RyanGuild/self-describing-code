# `robby_agent::model::NoteKind`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-agent/src/model.rs:76`](../../../../crates/robby-agent/src/model.rs#L76) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    UsageExample,
    ImplementationNote,
}
```

**Fields and variants**

- `UsageExample` — `crates/robby-agent/src/model.rs:79:5`
- `ImplementationNote` — `crates/robby-agent/src/model.rs:80:5`

