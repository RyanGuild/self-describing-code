# `robby_render::tree::linked_import`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn linked_import(doc: &Document, page: &Path, text: &str) -> String` |
| location | [`crates/robby-render/src/tree.rs:624`](../../../../crates/robby-render/src/tree.rs#L624) |
| provenance | extractor · confidence 1.00 |

```rust
fn linked_import(doc: &Document, page: &Path, text: &str) -> String {
    match resolve_use_path(doc, text).and_then(|node| page_of_node(doc, node)) {
        Some(target) => format!("[`{}`]({})", cell(text), rel_link(page, &target)),
        None => format!("`{}`", cell(text)),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant linked_import as linked_import()
    participant resolve_use_path as resolve_use_path
    participant page_of_node as page_of_node
    participant and_then as and_then
    linked_import-->>resolve_use_path: resolve_use_path(doc, text)
    linked_import-->>page_of_node: page_of_node(doc, node)
    linked_import-->>and_then: resolve_use_path(doc, text).and_then(|node| page_of_node(doc, node))
    alt match resolve_use_path(doc, text).and_then(|node| page_of_node(doc, node))
    linked_import-->>linked_import: format!('[`{}`]({})', cell(text), rel_link(page, &target))
    else None
    linked_import-->>linked_import: format!('`{}`', cell(text))
    end
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_render::tree::resolve_use_path`](resolve-use-path.md)
- calls → [`robby_render::tree::page_of_node`](page-of-node.md)

## References

- calls → [`robby_render::tree`](index.md) — page_of_node, resolve_use_path

