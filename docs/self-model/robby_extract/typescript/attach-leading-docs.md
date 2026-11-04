# `robby_extract::typescript::attach_leading_docs`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn attach_leading_docs(     doc: &mut Document,     comments: &[Comment],     module_id: &NodeId,     ctx: &Ctx<'_>, )` |
| location | [`crates/robby-extract/src/typescript.rs:421`](../../../../crates/robby-extract/src/typescript.rs#L421) |
| provenance | extractor · confidence 1.00 |

> Attach leading comments (JSDoc-style) onto the following item's `docs`.
> 
> Oxc's `attached_to` is the start of the next *token* (often `export`), while
> our item spans begin at the declaration keyword (`interface` / `class`).
> Prefer an exact span match, else the nearest following item.

```rust
/// Attach leading comments (JSDoc-style) onto the following item's `docs`.
///
/// Oxc's `attached_to` is the start of the next *token* (often `export`), while
/// our item spans begin at the declaration keyword (`interface` / `class`).
/// Prefer an exact span match, else the nearest following item.
fn attach_leading_docs(
    doc: &mut Document,
    comments: &[Comment],
    module_id: &NodeId,
    ctx: &Ctx<'_>,
) {
    let children: Vec<NodeId> = doc
        .get(module_id)
        .map(|m| m.children.clone())
        .unwrap_or_default();

    for comment in comments {
        if comment.position != CommentPosition::Leading {
            continue;
        }
        let content = comment.content_span();
        let raw = ctx
            .source
            .get(content.start as usize..content.end as usize)
            .unwrap_or("")
            .trim();
        if raw.is_empty() {
            continue;
        }

        let anchor = if comment.attached_to != 0 {
            comment.attached_to
        } else {
            comment.span.end
        };

        let mut best: Option<(u32, NodeId)> = None;
        for child_id in &children {
            let Some(child) = doc.get(child_id) else {
                continue;
            };
            if matches!(
                child.kind.as_str(),
                kind::code::COMMENT | kind::code::IMPORT
            ) {
                continue;
            }
            let Some(span) = &child.span else {
                continue;
            };
            if span.start.byte < anchor {
                continue;
            }
            if !gap_is_attachable(ctx.source, comment.span.end, span.start.byte) {
                continue;
            }
            match &best {
                Some((byte, _)) if span.start.byte >= *byte => {}
                _ => best = Some((span.start.byte, child_id.clone())),
            }
        }

        if let Some((_, id)) = best
            && let Some(child) = doc.nodes.get_mut(&id)
            && child.docs.is_none()
        {
            child.docs = Some(raw.to_owned());
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant attach_leading_docs as attach_leading_docs()
    participant get as get
    participant clone as clone
    participant map as map
    participant unwrap_or_default as unwrap_or_default
    participant content_span as content_span
    participant unwrap_or as unwrap_or
    participant trim as trim
    participant is_empty as is_empty
    participant gap_is_attachable as gap_is_attachable
    participant Some as Some
    participant get_mut as get_mut
    participant is_none as is_none
    participant to_owned as to_owned
    attach_leading_docs-->>get: doc .get(module_id)
    attach_leading_docs-->>clone: m.children.clone()
    attach_leading_docs-->>map: doc .get(module_id) .map(|m| m.children.clone())
    attach_leading_docs-->>unwrap_or_default: doc .get(module_id) .map(|m| m.children.clone()) .unwrap_or_default()
    loop comments
    attach_leading_docs-->>content_span: comment.content_span()
    attach_leading_docs-->>get: ctx .source .get(content.start as usize..content.end as usize)
    attach_leading_docs-->>unwrap_or: ctx .source .get(content.start as usize..content.end as usize) .unwra...
    attach_leading_docs-->>trim: ctx .source .get(content.start as usize..content.end as usize) .unwra...
    attach_leading_docs-->>is_empty: raw.is_empty()
    loop &children
    attach_leading_docs-->>get: doc.get(child_id)
    attach_leading_docs-->>attach_leading_docs: matches!( child.kind.as_str(), kind::code::COMMENT | kind::code::IMPO...
    attach_leading_docs-->>gap_is_attachable: gap_is_attachable(ctx.source, comment.span.end, span.start.byte)
    alt match &best
    attach_leading_docs-->>clone: child_id.clone()
    attach_leading_docs-->>Some: Some((span.start.byte, child_id.clone()))
    end
    end
    attach_leading_docs-->>get_mut: doc.nodes.get_mut(&id)
    attach_leading_docs-->>is_none: child.docs.is_none()
    alt let Some((_, id)) = best && let Some(child) = doc.nodes.get_mut(&id) ...
    attach_leading_docs-->>to_owned: raw.to_owned()
    attach_leading_docs-->>Some: Some(raw.to_owned())
    end
    end
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_extract::typescript::gap_is_attachable`](gap-is-attachable.md)

## References

- calls → [`robby_extract::typescript`](index.md) — gap_is_attachable
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty

## Referenced by

- calls ← [`robby_extract::typescript::extract_file`](extract-file.md)

