# `robby_ir::process::Step`

[Self-model](../../README.md) · [process](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/process.rs:92`](../../../../crates/robby-ir/src/process.rs#L92) |
| provenance | extractor · confidence 1.00 |

> A single ordered interaction.

```rust
/// A single ordered interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Step {
    pub from: String,
    pub to: String,
    pub label: String,
    pub kind: StepKind,
    /// `true` when a runtime trace observed this step.
    ///
    /// `false` means "possible per static analysis". The renderer draws
    /// observed steps solid and possible steps dashed, which is how the
    /// static/runtime merge is expressed visually.
    pub observed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,
}
```

**Fields and variants**

- `from` — `crates/robby-ir/src/process.rs:95:5`
- `to` — `crates/robby-ir/src/process.rs:96:5`
- `label` — `crates/robby-ir/src/process.rs:97:5`
- `kind` — `crates/robby-ir/src/process.rs:98:5`
- `observed` — `crates/robby-ir/src/process.rs:99:5`
- `span` — `crates/robby-ir/src/process.rs:105:5`
- `blocks` — `crates/robby-ir/src/process.rs:107:5`

