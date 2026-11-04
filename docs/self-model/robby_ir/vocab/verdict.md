# `robby_ir::vocab::Verdict`

[Self-model](../../README.md) · [vocab](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/vocab.rs:10`](../../../../crates/robby-ir/src/vocab.rs#L10) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Relevant,
    Noise,
}
```

**Fields and variants**

- `Relevant` — `crates/robby-ir/src/vocab.rs:13:5`
- `Noise` — `crates/robby-ir/src/vocab.rs:14:5`

