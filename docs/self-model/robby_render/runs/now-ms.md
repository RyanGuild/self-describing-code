# `robby_render::runs::now_ms`

[Self-model](../../README.md) · [runs](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn now_ms() -> u64` |
| location | [`crates/robby-render/src/runs.rs:113`](../../../../crates/robby-render/src/runs.rs#L113) |
| provenance | extractor · confidence 1.00 |

```rust
fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant now_ms as now_ms()
    participant std__time__SystemTime__now as std::time::SystemTime::now
    participant duration_since as duration_since
    participant as_millis as as_millis
    participant map as map
    participant unwrap_or as unwrap_or
    now_ms-->>std__time__SystemTime__now: std::time::SystemTime::now()
    now_ms-->>duration_since: std::time::SystemTime::now() .duration_since(std::time::UNIX_EPOCH)
    now_ms-->>as_millis: d.as_millis()
    now_ms-->>map: std::time::SystemTime::now() .duration_since(std::time::UNIX_EPOCH) ....
    now_ms-->>unwrap_or: std::time::SystemTime::now() .duration_since(std::time::UNIX_EPOCH) ....
```

5 steps · 0 observed · 5 possible only

## Referenced by

- calls ← [`robby_render::runs::write_site_runs`](write-site-runs.md)

