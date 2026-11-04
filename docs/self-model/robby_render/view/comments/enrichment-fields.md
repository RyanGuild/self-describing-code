# `robby_render::view::comments::enrichment_fields`

[Self-model](../../../README.md) · [view::comments](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn enrichment_fields(     decision: &Node,     _code: Option<&str>, ) -> (Option<NoteKind>, Option<String>, Vec<(u32, u32)>)` |
| location | [`crates/robby-render/src/view/comments.rs:183`](../../../../../crates/robby-render/src/view/comments.rs#L183) |
| provenance | extractor · confidence 1.00 |

```rust
fn enrichment_fields(
    decision: &Node,
    _code: Option<&str>,
) -> (Option<NoteKind>, Option<String>, Vec<(u32, u32)>) {
    let note_kind = decision
        .annotations
        .get("note_kind")
        .and_then(|s| match s.as_str() {
            "usage_example" => Some(NoteKind::UsageExample),
            "implementation_note" => Some(NoteKind::ImplementationNote),
            _ => None,
        });
    let note = decision.docs.clone();
    let highlights = decision
        .annotations
        .get("highlights")
        .and_then(|raw| serde_json::from_str::<Vec<HighlightJson>>(raw).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|h| h.start < h.end)
        .map(|h| (h.start, h.end))
        .collect();
    (note_kind, note, highlights)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant enrichment_fields as enrichment_fields()
    participant get as get
    participant as_str as as_str
    participant Some as Some
    participant and_then as and_then
    participant clone as clone
    participant serde_json__from_str_Vec_HighlightJson__ as serde_json::from_str<Vec<HighlightJson>>
    participant ok as ok
    participant unwrap_or_default as unwrap_or_default
    participant into_iter as into_iter
    participant filter as filter
    participant map as map
    participant collect as collect
    enrichment_fields-->>get: decision .annotations .get('note_kind')
    enrichment_fields-->>as_str: s.as_str()
    alt match s.as_str()
    enrichment_fields-->>Some: Some(NoteKind::UsageExample)
    else 'implementation_note'
    enrichment_fields-->>Some: Some(NoteKind::ImplementationNote)
    end
    enrichment_fields-->>and_then: decision .annotations .get('note_kind') .and_then(|s| match s.as_str(...
    enrichment_fields-->>clone: decision.docs.clone()
    enrichment_fields-->>get: decision .annotations .get('highlights')
    enrichment_fields-->>serde_json__from_str_Vec_HighlightJson__: serde_json::from_str::<Vec<HighlightJson>>(raw)
    enrichment_fields-->>ok: serde_json::from_str::<Vec<HighlightJson>>(raw).ok()
    enrichment_fields-->>and_then: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
    enrichment_fields-->>unwrap_or_default: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
    enrichment_fields-->>into_iter: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
    enrichment_fields-->>filter: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
    enrichment_fields-->>map: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
    enrichment_fields-->>collect: decision .annotations .get('highlights') .and_then(|raw| serde_json::...
```

15 steps · 0 observed · 15 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get
- calls → [`robby_render::view::comments`](index.md) — as_str
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::comments::impl CommentsView::from_root`](index.md)

