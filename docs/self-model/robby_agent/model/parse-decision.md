# `robby_agent::model::parse_decision`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn parse_decision(raw: &str) -> Result<Decision>` |
| location | [`crates/robby-agent/src/model.rs:186`](../../../../crates/robby-agent/src/model.rs#L186) |
| provenance | extractor · confidence 1.00 |

```rust
fn parse_decision(raw: &str) -> Result<Decision> {
    let value = extract_json(raw)?;
    let verdict = value
        .get("verdict")
        .and_then(|v| v.as_str())
        .and_then(Verdict::parse)
        .with_context(|| format!("missing/invalid verdict in {raw}"))?;
    let reason = value
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    Ok(Decision { verdict, reason })
}
```

```mermaid
sequenceDiagram
    autonumber
    participant parse_decision as parse_decision()
    participant extract_json as extract_json
    participant get as get
    participant as_str as as_str
    participant and_then as and_then
    participant with_context as with_context
    participant unwrap_or as unwrap_or
    participant to_owned as to_owned
    participant Ok as Ok
    parse_decision-->>extract_json: extract_json(raw)
    parse_decision-->>parse_decision: ? (propagate)
    parse_decision-->>get: value .get('verdict')
    parse_decision-->>as_str: v.as_str()
    parse_decision-->>and_then: value .get('verdict') .and_then(|v| v.as_str())
    parse_decision-->>and_then: value .get('verdict') .and_then(|v| v.as_str()) .and_then(Verdict::pa...
    parse_decision-->>parse_decision: format!('missing/invalid verdict in {raw}')
    parse_decision-->>with_context: value .get('verdict') .and_then(|v| v.as_str()) .and_then(Verdict::pa...
    parse_decision-->>parse_decision: ? (propagate)
    parse_decision-->>get: value .get('reason')
    parse_decision-->>as_str: v.as_str()
    parse_decision-->>and_then: value .get('reason') .and_then(|v| v.as_str())
    parse_decision-->>unwrap_or: value .get('reason') .and_then(|v| v.as_str()) .unwrap_or('')
    parse_decision-->>to_owned: value .get('reason') .and_then(|v| v.as_str()) .unwrap_or('') .to_own...
    parse_decision-->>Ok: Ok(Decision { verdict, reason })
```

15 steps · 0 observed · 15 possible only

**Edges**

- calls → [`robby_agent::model::extract_json`](extract-json.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](index.md)

## References

- calls → [`robby_agent::model`](index.md) — as_str, extract_json
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get

## Referenced by

- calls ← [`robby_agent::model::impl DecisionModel<'a, C>::decide`](index.md)
- calls ← [`robby_agent::model::parses_decision_and_note_json`](tests/parses-decision-and-note-json.md)

