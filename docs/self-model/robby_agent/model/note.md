# `robby_agent::model::Note`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:106`](../../../../crates/robby-agent/src/model.rs#L106) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub kind: NoteKind,
    pub body: String,
    #[serde(default)]
    pub highlights: Vec<Highlight>,
}
```

**Fields and variants**

- `kind` — `crates/robby-agent/src/model.rs:108:5`
- `body` — `crates/robby-agent/src/model.rs:109:5`
- `highlights` — `crates/robby-agent/src/model.rs:110:5`

