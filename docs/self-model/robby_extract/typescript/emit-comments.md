# `robby_extract::typescript::emit_comments`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn emit_comments(doc: &mut Document, comments: &[Comment], parent: &NodeId, ctx: &Ctx<'_>)` |
| location | [`crates/robby-extract/src/typescript.rs:394`](../../../../crates/robby-extract/src/typescript.rs#L394) |
| provenance | extractor · confidence 1.00 |

```rust
fn emit_comments(doc: &mut Document, comments: &[Comment], parent: &NodeId, ctx: &Ctx<'_>) {
    for (index, comment) in comments.iter().enumerate() {
        let content = comment.content_span();
        let raw = ctx
            .source
            .get(content.start as usize..content.end as usize)
            .unwrap_or("")
            .trim();
        if raw.is_empty() {
            continue;
        }
        let fqn = format!("{}/comment@{}", ctx.module, comment.span.start);
        let mut node = item_node(
            kind::code::COMMENT,
            &fqn,
            oxc_span(comment.span, ctx),
            parent,
        );
        node.name = format!("comment_{index}");
        node.docs = Some(raw.to_owned());
        let kind_tag = if comment.is_line() { "line" } else { "block" };
        node.annotations
            .insert("comment_kind".into(), kind_tag.into());
        doc.merge(node);
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant emit_comments as emit_comments()
    participant iter as iter
    participant enumerate as enumerate
    participant content_span as content_span
    participant get as get
    participant unwrap_or as unwrap_or
    participant trim as trim
    participant is_empty as is_empty
    participant oxc_span as oxc_span
    participant item_node as item_node
    participant to_owned as to_owned
    participant Some as Some
    participant is_line as is_line
    participant into as into
    participant insert as insert
    participant merge as merge
    emit_comments-->>iter: comments.iter()
    emit_comments-->>enumerate: comments.iter().enumerate()
    loop comments.iter().enumerate()
    emit_comments-->>content_span: comment.content_span()
    emit_comments-->>get: ctx .source .get(content.start as usize..content.end as usize)
    emit_comments-->>unwrap_or: ctx .source .get(content.start as usize..content.end as usize) .unwra...
    emit_comments-->>trim: ctx .source .get(content.start as usize..content.end as usize) .unwra...
    emit_comments-->>is_empty: raw.is_empty()
    emit_comments-->>emit_comments: format!('{}/comment@{}', ctx.module, comment.span.start)
    emit_comments-->>oxc_span: oxc_span(comment.span, ctx)
    emit_comments-->>item_node: item_node( kind::code::COMMENT, &fqn, oxc_span(comment.span, ctx), pa...
    emit_comments-->>emit_comments: format!('comment_{index}')
    emit_comments-->>to_owned: raw.to_owned()
    emit_comments-->>Some: Some(raw.to_owned())
    emit_comments-->>is_line: comment.is_line()
    emit_comments-->>into: 'comment_kind'.into()
    emit_comments-->>into: kind_tag.into()
    emit_comments-->>insert: node.annotations .insert('comment_kind'.into(), kind_tag.into())
    emit_comments-->>merge: doc.merge(node)
    end
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_extract::typescript::oxc_span`](oxc-span.md)
- calls → [`robby_extract::typescript::item_node`](item-node.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)

## References

- calls → [`robby_extract::typescript`](index.md) — item_node, oxc_span
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get, insert, merge
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty

## Referenced by

- calls ← [`robby_extract::typescript::extract_file`](extract-file.md)

