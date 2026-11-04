# `robby_agent::enrich::EnrichStats`

[Self-model](../../README.md) · [enrich](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-agent/src/enrich.rs:33`](../../../../crates/robby-agent/src/enrich.rs#L33) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnrichStats {
    pub considered: usize,
    pub decided: usize,
    pub relevant: usize,
    pub noise: usize,
    pub noted: usize,
    pub skipped_existing: usize,
}
```

**Fields and variants**

- `considered` — `crates/robby-agent/src/enrich.rs:35:5`
- `decided` — `crates/robby-agent/src/enrich.rs:36:5`
- `relevant` — `crates/robby-agent/src/enrich.rs:37:5`
- `noise` — `crates/robby-agent/src/enrich.rs:38:5`
- `noted` — `crates/robby-agent/src/enrich.rs:39:5`
- `skipped_existing` — `crates/robby-agent/src/enrich.rs:40:5`

