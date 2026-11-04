# `robby_render::tree::item_page`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_page(     doc: &Document,     node: &Node,     page: &Path,     sources: Option<&BTreeMap<String, String>>,     out_rel: &str, ) -> String` |
| location | [`crates/robby-render/src/tree.rs:369`](../../../../crates/robby-render/src/tree.rs#L369) |
| provenance | extractor · confidence 1.00 |

> An item page: the source, its process, its edges.

```rust
/// An item page: the source, its process, its edges.
fn item_page(
    doc: &Document,
    node: &Node,
    page: &Path,
    sources: Option<&BTreeMap<String, String>>,
    out_rel: &str,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# `{}`\n", node.fqn);
    out.push_str(&nav(doc, node, page, true));
    render_node(&mut out, doc, node, 2, page, sources, out_rel, false);
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant item_page as item_page()
    participant String__new as String::new
    participant nav as nav
    participant push_str as push_str
    participant render_node as render_node
    item_page-->>String__new: String::new()
    item_page-->>item_page: writeln!(out, ' `{}`\n', node.fqn)
    item_page-->>nav: nav(doc, node, page, true)
    item_page-->>push_str: out.push_str(&nav(doc, node, page, true))
    item_page-->>render_node: render_node(&mut out, doc, node, 2, page, sources, out_rel, false)
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::tree::nav`](nav.md)
- calls → [`robby_render::tree::render_node`](render-node.md)

## References

- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::tree`](index.md) — nav, render_node

## Referenced by

- calls ← [`robby_render::tree::module_pages`](module-pages.md)

