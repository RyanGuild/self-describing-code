# `robby_render::view::comments::code_window`

[Self-model](../../../README.md) · [view::comments](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn code_window(     sources: &BTreeMap<String, String>,     comment: &Node,     owner: Option<&Node>, ) -> Option<String>` |
| location | [`crates/robby-render/src/view/comments.rs:242`](../../../../../crates/robby-render/src/view/comments.rs#L242) |
| provenance | extractor · confidence 1.00 |

```rust
fn code_window(
    sources: &BTreeMap<String, String>,
    comment: &Node,
    owner: Option<&Node>,
) -> Option<String> {
    if let Some(owner) = owner {
        if let Some(slice) = slice_node(sources, owner) {
            return Some(slice);
        }
    }
    line_window(sources, comment)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant code_window as code_window()
    participant slice_node as slice_node
    participant Some as Some
    participant line_window as line_window
    alt let Some(owner) = owner
    code_window-->>slice_node: slice_node(sources, owner)
    alt let Some(slice) = slice_node(sources, owner)
    code_window-->>Some: Some(slice)
    code_window-->>code_window: return
    end
    end
    code_window-->>line_window: line_window(sources, comment)
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::view::comments::slice_node`](slice-node.md)
- calls → [`robby_render::view::comments::line_window`](line-window.md)

## References

- calls → [`robby_render::view::comments`](index.md) — line_window, slice_node

## Referenced by

- calls ← [`robby_render::view::comments::impl CommentsView::from_root`](index.md)

