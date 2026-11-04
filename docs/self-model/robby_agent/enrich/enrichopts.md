# `robby_agent::enrich::EnrichOpts`

[Self-model](../../README.md) · [enrich](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/enrich.rs:13`](../../../../crates/robby-agent/src/enrich.rs#L13) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone)]
pub struct EnrichOpts {
    pub decision_model: String,
    pub note_model: String,
    /// Cap how many comments to process this run (`None` = all).
    pub limit: Option<usize>,
}
```

**Fields and variants**

- `decision_model` — `crates/robby-agent/src/enrich.rs:15:5`
- `note_model` — `crates/robby-agent/src/enrich.rs:16:5`
- `limit` — `crates/robby-agent/src/enrich.rs:17:5`

