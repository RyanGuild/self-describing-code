# `robby_agent::model::Decision`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/model.rs:47`](../../../../crates/robby-agent/src/model.rs#L47) |
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

- `verdict` — `crates/robby-agent/src/model.rs:49:5`
- `reason` — `crates/robby-agent/src/model.rs:50:5`

