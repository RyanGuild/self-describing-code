# `robby_agent::model::Verdict`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-agent/src/model.rs:45`](../../../../crates/robby-agent/src/model.rs#L45) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Relevant,
    Noise,
}
```

**Fields and variants**

- `Relevant` — `crates/robby-agent/src/model.rs:48:5`
- `Noise` — `crates/robby-agent/src/model.rs:49:5`

