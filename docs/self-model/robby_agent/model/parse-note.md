# `robby_agent::model::parse_note`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn parse_note(raw: &str) -> Result<Note>` |
| location | [`crates/robby-agent/src/model.rs:149`](../../../../crates/robby-agent/src/model.rs#L149) |
| provenance | extractor · confidence 1.00 |

```rust
fn parse_note(raw: &str) -> Result<Note> {
    let value = extract_json(raw)?;
    let kind = value
        .get("kind")
        .and_then(|v| v.as_str())
        .and_then(NoteKind::parse)
        .with_context(|| format!("missing/invalid note kind in {raw}"))?;
    let body = value
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_owned();
    if body.is_empty() {
        bail!("empty note body in {raw}");
    }
    let highlights = value
        .get("highlights")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|h| {
                    let start = h.get("start")?.as_u64()? as u32;
                    let end = h.get("end")?.as_u64()? as u32;
                    Some(Highlight { start, end })
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Note {
        kind,
        body,
        highlights,
    })
}
```

```mermaid
sequenceDiagram
    autonumber
    participant parse_note as parse_note()
    participant extract_json as extract_json
    participant get as get
    participant as_str as as_str
    participant and_then as and_then
    participant with_context as with_context
    participant unwrap_or as unwrap_or
    participant trim as trim
    participant to_owned as to_owned
    participant is_empty as is_empty
    participant as_array as as_array
    participant iter as iter
    participant as_u64 as as_u64
    participant Some as Some
    participant filter_map as filter_map
    participant collect as collect
    participant map as map
    participant unwrap_or_default as unwrap_or_default
    participant Ok as Ok
    parse_note-->>extract_json: extract_json(raw)
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>get: value .get('kind')
    parse_note-->>as_str: v.as_str()
    parse_note-->>and_then: value .get('kind') .and_then(|v| v.as_str())
    parse_note-->>and_then: value .get('kind') .and_then(|v| v.as_str()) .and_then(NoteKind::parse)
    parse_note-->>parse_note: format!('missing/invalid note kind in {raw}')
    parse_note-->>with_context: value .get('kind') .and_then(|v| v.as_str()) .and_then(NoteKind::pars...
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>get: value .get('body')
    parse_note-->>as_str: v.as_str()
    parse_note-->>and_then: value .get('body') .and_then(|v| v.as_str())
    parse_note-->>unwrap_or: value .get('body') .and_then(|v| v.as_str()) .unwrap_or('')
    parse_note-->>trim: value .get('body') .and_then(|v| v.as_str()) .unwrap_or('') .trim()
    parse_note-->>to_owned: value .get('body') .and_then(|v| v.as_str()) .unwrap_or('') .trim() ....
    parse_note-->>is_empty: body.is_empty()
    alt body.is_empty()
    parse_note-->>parse_note: bail!('empty note body in {raw}')
    end
    parse_note-->>get: value .get('highlights')
    parse_note-->>as_array: v.as_array()
    parse_note-->>and_then: value .get('highlights') .and_then(|v| v.as_array())
    parse_note-->>iter: arr.iter()
    parse_note-->>get: h.get('start')
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>as_u64: h.get('start')?.as_u64()
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>get: h.get('end')
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>as_u64: h.get('end')?.as_u64()
    parse_note-->>parse_note: ? (propagate)
    parse_note-->>Some: Some(Highlight { start, end })
    parse_note-->>filter_map: arr.iter() .filter_map(|h| { let start = h.get('start')?.as_u64()? as...
    parse_note-->>collect: arr.iter() .filter_map(|h| { let start = h.get('start')?.as_u64()? as...
    parse_note-->>map: value .get('highlights') .and_then(|v| v.as_array()) .map(|arr| { arr...
    parse_note-->>unwrap_or_default: value .get('highlights') .and_then(|v| v.as_array()) .map(|arr| { arr...
    parse_note-->>Ok: Ok(Note { kind, body, highlights, })
```

35 steps · 0 observed · 35 possible only

**Edges**

- calls → [`robby_agent::model::extract_json`](extract-json.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_agent::model`](index.md) — extract_json
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — get
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_agent::model::impl NoteModel<'a, C>::write_note`](index.md)
- calls ← [`robby_agent::model::parses_decision_and_note_json`](tests/parses-decision-and-note-json.md)

