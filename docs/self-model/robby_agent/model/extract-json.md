# `robby_agent::model::extract_json`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn extract_json(raw: &str) -> Result<serde_json::Value>` |
| location | [`crates/robby-agent/src/model.rs:185`](../../../../crates/robby-agent/src/model.rs#L185) |
| provenance | extractor · confidence 1.00 |

```rust
fn extract_json(raw: &str) -> Result<serde_json::Value> {
    let trimmed = raw.trim();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Ok(v);
    }
    let start = trimmed
        .find('{')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    let end = trimmed
        .rfind('}')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    serde_json::from_str(&trimmed[start..=end])
        .with_context(|| format!("invalid JSON in response: {raw}"))
}
```

```mermaid
sequenceDiagram
    autonumber
    participant extract_json as extract_json()
    participant trim as trim
    participant serde_json__from_str_serde_json__Value_ as serde_json::from_str<serde_json::Value>
    participant Ok as Ok
    participant find as find
    participant with_context as with_context
    participant rfind as rfind
    participant serde_json__from_str as serde_json::from_str
    extract_json-->>trim: raw.trim()
    extract_json-->>serde_json__from_str_serde_json__Value_: serde_json::from_str::<serde_json::Value>(trimmed)
    alt let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed)
    extract_json-->>Ok: Ok(v)
    extract_json-->>extract_json: return
    end
    extract_json-->>find: trimmed .find('{')
    extract_json-->>extract_json: format!('no JSON object in response: {raw}')
    extract_json-->>with_context: trimmed .find('{') .with_context(|| format!('no JSON object in respon...
    extract_json-->>extract_json: ? (propagate)
    extract_json-->>rfind: trimmed .rfind('}')
    extract_json-->>extract_json: format!('no JSON object in response: {raw}')
    extract_json-->>with_context: trimmed .rfind('}') .with_context(|| format!('no JSON object in respo...
    extract_json-->>extract_json: ? (propagate)
    extract_json-->>serde_json__from_str: serde_json::from_str(&trimmed[start..=end])
    extract_json-->>extract_json: format!('invalid JSON in response: {raw}')
    extract_json-->>with_context: serde_json::from_str(&trimmed[start..=end]) .with_context(|| format!(...
```

15 steps · 0 observed · 15 possible only

## Referenced by

- calls ← [`robby_agent::model::parse_decision`](parse-decision.md)
- calls ← [`robby_agent::model::parse_note`](parse-note.md)

