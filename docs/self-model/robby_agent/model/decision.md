# `robby_agent::model::Decision`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:69`](../../../../crates/robby-agent/src/model.rs#L69) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub verdict: Verdict,
    #[serde(default)]
    pub reason: String,
}
```

**Fields and variants**

- `verdict` — `crates/robby-agent/src/model.rs:71:5`
- `reason` — `crates/robby-agent/src/model.rs:72:5`

