# `robby_agent::context::build_for_docs_owner`

[Self-model](../../README.md) · [context](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn build_for_docs_owner(     doc: &Document,     sources: &BTreeMap<String, String>,     owner: &Node,     text: String, ) -> CommentContext` |
| location | [`crates/robby-agent/src/context.rs:106`](../../../../crates/robby-agent/src/context.rs#L106) |
| provenance | extractor · confidence 1.00 |

```rust
fn build_for_docs_owner(
    doc: &Document,
    sources: &BTreeMap<String, String>,
    owner: &Node,
    text: String,
) -> CommentContext {
    let (file, location) = file_location(owner);
    let code_window = slice_node(sources, owner).unwrap_or_else(|| {
        line_window(sources, owner).unwrap_or_default()
    });
    let _ = doc;
    CommentContext {
        parent_id: owner.id.clone(),
        comment_text: text,
        code_window,
        owner_fqn: Some(owner.fqn.clone()),
        file,
        location,
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant build_for_docs_owner as build_for_docs_owner()
    participant file_location as file_location
    participant slice_node as slice_node
    participant line_window as line_window
    participant unwrap_or_default as unwrap_or_default
    participant unwrap_or_else as unwrap_or_else
    participant clone as clone
    participant Some as Some
    build_for_docs_owner-->>file_location: file_location(owner)
    build_for_docs_owner-->>slice_node: slice_node(sources, owner)
    build_for_docs_owner-->>line_window: line_window(sources, owner)
    build_for_docs_owner-->>unwrap_or_default: line_window(sources, owner).unwrap_or_default()
    build_for_docs_owner-->>unwrap_or_else: slice_node(sources, owner).unwrap_or_else(|| { line_window(sources, o...
    build_for_docs_owner-->>clone: owner.id.clone()
    build_for_docs_owner-->>clone: owner.fqn.clone()
    build_for_docs_owner-->>Some: Some(owner.fqn.clone())
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_agent::context::file_location`](file-location.md)
- calls → [`robby_agent::context::slice_node`](slice-node.md)
- calls → [`robby_agent::context::line_window`](line-window.md)

## References

- calls → [`robby_agent::context`](index.md) — file_location, line_window, slice_node

## Referenced by

- calls ← [`robby_agent::context::collect_comment_contexts`](collect-comment-contexts.md)

