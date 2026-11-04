# `robby_render::tree::page_of_node`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn page_of_node(doc: &Document, node: &Node) -> Option<PathBuf>` |
| location | [`crates/robby-render/src/tree.rs:723`](../../../../crates/robby-render/src/tree.rs#L723) |
| provenance | extractor · confidence 1.00 |

> The most specific page for a node: its item page if it has one.

```rust
/// The most specific page for a node: its item page if it has one.
fn page_of_node(doc: &Document, node: &Node) -> Option<PathBuf> {
    let crate_dir = crate_of(node);

    if !is_module(node)
        && !is_compact(node.kind.as_str())
        && let Some(parent) = node.parent.as_ref().and_then(|id| doc.get(id))
        && is_module(parent)
        && let Some(index) = module_index_path(parent, &crate_dir)
    {
        let module_dir = index.parent().unwrap_or(Path::new("."));
        let items = item_children(doc, parent);
        let slugs = item_slugs(&items);
        if let Some(slug) = slugs.get(&node.id) {
            return Some(module_dir.join(format!("{slug}.md")));
        }
    }

    let mut current = Some(node.id.clone());
    while let Some(id) = current {
        let candidate = doc.get(&id)?;
        if is_module(candidate) {
            return module_index_path(candidate, &crate_dir);
        }
        // `while let` moves `current`, so this has to be a fresh assignment.
        #[allow(clippy::assigning_clones)]
        {
            current = candidate.parent.clone();
        }
    }
    None
}
```

```mermaid
sequenceDiagram
    autonumber
    participant page_of_node as page_of_node()
    participant crate_of as crate_of
    participant is_module as is_module
    participant as_str as as_str
    participant is_compact as is_compact
    participant as_ref as as_ref
    participant get as get
    participant and_then as and_then
    participant module_index_path as module_index_path
    participant parent as parent
    participant Path__new as Path::new
    participant unwrap_or as unwrap_or
    participant item_children as item_children
    participant item_slugs as item_slugs
    participant join as join
    participant Some as Some
    participant clone as clone
    page_of_node-->>crate_of: crate_of(node)
    page_of_node-->>is_module: is_module(node)
    page_of_node-->>as_str: node.kind.as_str()
    page_of_node-->>is_compact: is_compact(node.kind.as_str())
    page_of_node-->>as_ref: node.parent.as_ref()
    page_of_node-->>get: doc.get(id)
    page_of_node-->>and_then: node.parent.as_ref().and_then(|id| doc.get(id))
    page_of_node-->>is_module: is_module(parent)
    page_of_node-->>module_index_path: module_index_path(parent, &crate_dir)
    alt !is_module(node) && !is_compact(node.kind.as_str()) && let Some(paren...
    page_of_node-->>parent: index.parent()
    page_of_node-->>Path__new: Path::new('.')
    page_of_node-->>unwrap_or: index.parent().unwrap_or(Path::new('.'))
    page_of_node-->>item_children: item_children(doc, parent)
    page_of_node-->>item_slugs: item_slugs(&items)
    page_of_node-->>get: slugs.get(&node.id)
    alt let Some(slug) = slugs.get(&node.id)
    page_of_node-->>page_of_node: format!('{slug}.md')
    page_of_node-->>join: module_dir.join(format!('{slug}.md'))
    page_of_node-->>Some: Some(module_dir.join(format!('{slug}.md')))
    page_of_node-->>page_of_node: return
    end
    end
    page_of_node-->>clone: node.id.clone()
    page_of_node-->>Some: Some(node.id.clone())
    loop let Some(id) = current
    page_of_node-->>get: doc.get(&id)
    page_of_node-->>page_of_node: ? (propagate)
    page_of_node-->>is_module: is_module(candidate)
    alt is_module(candidate)
    page_of_node-->>module_index_path: module_index_path(candidate, &crate_dir)
    page_of_node-->>page_of_node: return
    end
    page_of_node-->>clone: candidate.parent.clone()
    end
```

27 steps · 0 observed · 27 possible only

**Edges**

- calls → [`robby_render::tree::crate_of`](crate-of.md)
- calls → [`robby_render::tree::is_module`](is-module.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::tree::is_compact`](is-compact.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::is_module`](is-module.md)
- calls → [`robby_render::tree::module_index_path`](module-index-path.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::tree::item_children`](item-children.md)
- calls → [`robby_render::tree::item_slugs`](item-slugs.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::is_module`](is-module.md)
- calls → [`robby_render::tree::module_index_path`](module-index-path.md)

## References

- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — get
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::tree`](index.md) — 6 items
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str

## Referenced by

- calls ← [`robby_render::tree::linked_import`](linked-import.md)
- calls ← [`robby_render::tree::render_links`](render-links.md)
- calls ← [`robby_render::tree::render_node`](render-node.md)

