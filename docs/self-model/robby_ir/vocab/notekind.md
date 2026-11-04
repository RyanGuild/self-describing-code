# `robby_ir::vocab::NoteKind`

[Self-model](../../README.md) · [vocab](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/vocab.rs:34`](../../../../crates/robby-ir/src/vocab.rs#L34) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    UsageExample,
    ImplementationNote,
}
```

**Fields and variants**

- `UsageExample` — `crates/robby-ir/src/vocab.rs:37:5`
- `ImplementationNote` — `crates/robby-ir/src/vocab.rs:38:5`

