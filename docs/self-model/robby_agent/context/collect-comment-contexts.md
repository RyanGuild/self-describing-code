# `robby_agent::context::collect_comment_contexts`

[Self-model](../../README.md) · [context](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn collect_comment_contexts(     doc: &Document,     sources: &BTreeMap<String, String>, ) -> Vec<CommentContext>` |
| location | [`crates/robby-agent/src/context.rs:17`](../../../../crates/robby-agent/src/context.rs#L17) |
| provenance | extractor · confidence 1.00 |

> Collect enrichable comment/docs contexts under the whole document.

```rust
/// Collect enrichable comment/docs contexts under the whole document.
pub fn collect_comment_contexts(
    doc: &Document,
    sources: &BTreeMap<String, String>,
) -> Vec<CommentContext> {
    let mut out = Vec::new();

    for node in doc.nodes.values() {
        if node.kind.as_str() == kind::code::COMMENT {
            let text = node.docs.clone().unwrap_or_default();
            if text.is_empty() {
                continue;
            }
            if has_decision_child(doc, node) {
                continue;
            }
            out.push(build_for_comment(doc, sources, node, text));
            continue;
        }

        // Docs-only: item docs with no covering comment node already enriched.
        let Some(docs) = &node.docs else {
            continue;
        };
        if docs.is_empty() || has_decision_child(doc, node) {
            continue;
        }
        if node.kind.as_str() == kind::code::COMMENT {
            continue;
        }
        // Skip if a comment node already carries the same text under the same parent.
        if parent_has_comment_with_text(doc, node, docs) {
            continue;
        }
        out.push(build_for_docs_owner(doc, sources, node, docs.clone()));
    }

    out.sort_by(|a, b| {
        (
            a.file.as_str(),
            a.location.as_deref().unwrap_or(""),
            a.parent_id.as_str(),
        )
            .cmp(&(
                b.file.as_str(),
                b.location.as_deref().unwrap_or(""),
                b.parent_id.as_str(),
            ))
    });
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant collect_comment_contexts as collect_comment_contexts()
    participant Vec__new as Vec::new
    participant values as values
    participant as_str as as_str
    participant clone as clone
    participant unwrap_or_default as unwrap_or_default
    participant is_empty as is_empty
    participant has_decision_child as has_decision_child
    participant build_for_comment as build_for_comment
    participant push as push
    participant parent_has_comment_with_text as parent_has_comment_with_text
    participant build_for_docs_owner as build_for_docs_owner
    participant as_deref as as_deref
    participant unwrap_or as unwrap_or
    participant cmp as cmp
    participant sort_by as sort_by
    collect_comment_contexts-->>Vec__new: Vec::new()
    collect_comment_contexts-->>values: doc.nodes.values()
    loop doc.nodes.values()
    collect_comment_contexts-->>as_str: node.kind.as_str()
    alt node.kind.as_str() == kind::code::COMMENT
    collect_comment_contexts-->>clone: node.docs.clone()
    collect_comment_contexts-->>unwrap_or_default: node.docs.clone().unwrap_or_default()
    collect_comment_contexts-->>is_empty: text.is_empty()
    collect_comment_contexts-->>has_decision_child: has_decision_child(doc, node)
    collect_comment_contexts-->>build_for_comment: build_for_comment(doc, sources, node, text)
    collect_comment_contexts-->>push: out.push(build_for_comment(doc, sources, node, text))
    end
    collect_comment_contexts-->>is_empty: docs.is_empty()
    collect_comment_contexts-->>has_decision_child: has_decision_child(doc, node)
    collect_comment_contexts-->>as_str: node.kind.as_str()
    collect_comment_contexts-->>parent_has_comment_with_text: parent_has_comment_with_text(doc, node, docs)
    collect_comment_contexts-->>clone: docs.clone()
    collect_comment_contexts-->>build_for_docs_owner: build_for_docs_owner(doc, sources, node, docs.clone())
    collect_comment_contexts-->>push: out.push(build_for_docs_owner(doc, sources, node, docs.clone()))
    end
    collect_comment_contexts-->>as_str: a.file.as_str()
    collect_comment_contexts-->>as_deref: a.location.as_deref()
    collect_comment_contexts-->>unwrap_or: a.location.as_deref().unwrap_or('')
    collect_comment_contexts-->>as_str: a.parent_id.as_str()
    collect_comment_contexts-->>as_str: b.file.as_str()
    collect_comment_contexts-->>as_deref: b.location.as_deref()
    collect_comment_contexts-->>unwrap_or: b.location.as_deref().unwrap_or('')
    collect_comment_contexts-->>as_str: b.parent_id.as_str()
    collect_comment_contexts-->>cmp: ( a.file.as_str(), a.location.as_deref().unwrap_or(''), a.parent_id.a...
    collect_comment_contexts-->>sort_by: out.sort_by(|a, b| { ( a.file.as_str(), a.location.as_deref().unwrap_...
```

26 steps · 0 observed · 26 possible only

**Edges**

- calls → [`robby_agent::enrich::impl FakeClient::new`](../enrich/tests/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_agent::context::has_decision_child`](has-decision-child.md)
- calls → [`robby_agent::context::build_for_comment`](build-for-comment.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)
- calls → [`robby_agent::context::has_decision_child`](has-decision-child.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)
- calls → [`robby_agent::context::parent_has_comment_with_text`](parent-has-comment-with-text.md)
- calls → [`robby_agent::context::build_for_docs_owner`](build-for-docs-owner.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)
- calls → [`robby_agent::model::impl Verdict::as_str`](../model/index.md)

## References

- calls → [`robby_agent::context`](index.md) — build_for_comment, build_for_docs_owner, has_decision_child, parent_has_comment_with_text
- calls → [`robby_agent::enrich::tests`](../enrich/tests/index.md) — new
- calls → [`robby_agent::model`](../model/index.md) — as_str
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_agent::enrich::enrich_comments`](../enrich/enrich-comments.md)

