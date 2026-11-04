# `robby_agent::enrich::insert_decision`

[Self-model](../../README.md) · [enrich](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn insert_decision(     doc: &mut Document,     ctx: &CommentContext,     decision: &crate::model::Decision,     note: Option<&crate::model::Note>,     decision_model: &str,     note_model: Option<&str>, )` |
| location | [`crates/robby-agent/src/enrich.rs:93`](../../../../crates/robby-agent/src/enrich.rs#L93) |
| provenance | extractor · confidence 1.00 |

```rust
fn insert_decision(
    doc: &mut Document,
    ctx: &CommentContext,
    decision: &crate::model::Decision,
    note: Option<&crate::model::Note>,
    decision_model: &str,
    note_model: Option<&str>,
) {
    let Some(parent) = doc.get(&ctx.parent_id) else {
        return;
    };
    let fqn = format!("{}/decision", parent.fqn);
    let lang = match parent.id.as_str().split("://").next() {
        Some("typescript") => "typescript",
        Some("rust") => "rust",
        other => other.unwrap_or("agent"),
    };
    let parent_lang = parent.lang;
    let parent_id = parent.id.clone();

    let mut node = Node::with_lang(
        NodeId::symbol(lang, &fqn),
        kind::agent::DECISION,
        fqn,
        match parent_lang {
            Lang::TypeScript => Lang::TypeScript,
            Lang::Rust => Lang::Rust,
            Lang::Unknown => Lang::Unknown,
        },
    );
    node.parent = Some(parent_id);
    node.provenance = Provenance::new(Source::Agent);
    node.annotations
        .insert("verdict".into(), decision.verdict.as_str().into());
    if !decision.reason.is_empty() {
        node.annotations
            .insert("reason".into(), decision.reason.clone());
    }
    node.annotations
        .insert("decision_model".into(), decision_model.into());

    if let Some(note) = note {
        node.docs = Some(note.body.clone());
        node.annotations
            .insert("note_kind".into(), note.kind.as_str().into());
        if let Some(model) = note_model {
            node.annotations.insert("note_model".into(), model.into());
        }
        if let Ok(json) = serde_json::to_string(&note.highlights) {
            node.annotations.insert("highlights".into(), json);
        }
    }

    doc.merge(node);
}
```

```mermaid
sequenceDiagram
    autonumber
    participant insert_decision as insert_decision()
    participant get as get
    participant as_str as as_str
    participant split as split
    participant next as next
    participant unwrap_or as unwrap_or
    participant clone as clone
    participant NodeId__symbol as NodeId::symbol
    participant Node__with_lang as Node::with_lang
    participant Some as Some
    participant Provenance__new as Provenance::new
    participant into as into
    participant insert as insert
    participant is_empty as is_empty
    participant serde_json__to_string as serde_json::to_string
    participant merge as merge
    insert_decision-->>get: doc.get(&ctx.parent_id)
    insert_decision-->>insert_decision: format!('{}/decision', parent.fqn)
    insert_decision-->>as_str: parent.id.as_str()
    insert_decision-->>split: parent.id.as_str().split('://')
    insert_decision-->>next: parent.id.as_str().split('://').next()
    alt match parent.id.as_str().split('://').next()
    insert_decision-->>unwrap_or: other.unwrap_or('agent')
    end
    insert_decision-->>clone: parent.id.clone()
    insert_decision-->>NodeId__symbol: NodeId::symbol(lang, &fqn)
    insert_decision-->>Node__with_lang: Node::with_lang( NodeId::symbol(lang, &fqn), kind::agent::DECISION, f...
    insert_decision-->>Some: Some(parent_id)
    insert_decision-->>Provenance__new: Provenance::new(Source::Agent)
    insert_decision-->>into: 'verdict'.into()
    insert_decision-->>as_str: decision.verdict.as_str()
    insert_decision-->>into: decision.verdict.as_str().into()
    insert_decision-->>insert: node.annotations .insert('verdict'.into(), decision.verdict.as_str()....
    insert_decision-->>is_empty: decision.reason.is_empty()
    alt !decision.reason.is_empty()
    insert_decision-->>into: 'reason'.into()
    insert_decision-->>clone: decision.reason.clone()
    insert_decision-->>insert: node.annotations .insert('reason'.into(), decision.reason.clone())
    end
    insert_decision-->>into: 'decision_model'.into()
    insert_decision-->>into: decision_model.into()
    insert_decision-->>insert: node.annotations .insert('decision_model'.into(), decision_model.into())
    alt let Some(note) = note
    insert_decision-->>clone: note.body.clone()
    insert_decision-->>Some: Some(note.body.clone())
    insert_decision-->>into: 'note_kind'.into()
    insert_decision-->>as_str: note.kind.as_str()
    insert_decision-->>into: note.kind.as_str().into()
    insert_decision-->>insert: node.annotations .insert('note_kind'.into(), note.kind.as_str().into())
    alt let Some(model) = note_model
    insert_decision-->>into: 'note_model'.into()
    insert_decision-->>into: model.into()
    insert_decision-->>insert: node.annotations.insert('note_model'.into(), model.into())
    end
    insert_decision-->>serde_json__to_string: serde_json::to_string(&note.highlights)
    alt let Ok(json) = serde_json::to_string(&note.highlights)
    insert_decision-->>into: 'highlights'.into()
    insert_decision-->>insert: node.annotations.insert('highlights'.into(), json)
    end
    end
    insert_decision-->>merge: doc.merge(node)
```

35 steps · 0 observed · 35 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Node::with_lang`](../../robby_ir/model/index.md)
- calls → [`robby_agent::enrich::impl FakeClient::new`](tests/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)

## References

- calls → [`robby_agent::enrich::tests`](tests/index.md) — new
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — 5 items
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty

## Referenced by

- calls ← [`robby_agent::enrich::enrich_comments`](enrich-comments.md)

