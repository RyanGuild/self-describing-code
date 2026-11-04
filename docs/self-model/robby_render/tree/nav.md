# `robby_render::tree::nav`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn nav(doc: &Document, node: &Node, page: &Path, item: bool) -> String` |
| location | [`crates/robby-render/src/tree.rs:384`](../../../../crates/robby-render/src/tree.rs#L384) |
| provenance | extractor · confidence 1.00 |

```rust
fn nav(doc: &Document, node: &Node, page: &Path, item: bool) -> String {
    let mut out = String::new();
    let _ = write!(
        out,
        "[Self-model]({})",
        rel_link(page, Path::new("README.md"))
    );

    let crate_dir = crate_of(node);

    let parent_target = if item {
        node.parent
            .as_ref()
            .and_then(|id| doc.get(id))
            .and_then(|parent| module_index_path(parent, &crate_dir))
    } else {
        node.parent
            .as_ref()
            .and_then(|id| doc.get(id))
            .and_then(|parent| {
                if is_crate(parent) {
                    Some(crate_index_path(&crate_dir))
                } else {
                    module_index_path(parent, &crate_dir)
                }
            })
    };

    if let Some(target) = parent_target {
        let label = if item {
            node.parent
                .as_ref()
                .and_then(|id| doc.get(id))
                .map(|parent| short_fqn(parent, &crate_dir))
                .unwrap_or_default()
        } else {
            node.parent.as_ref().and_then(|id| doc.get(id)).map_or_else(
                || "crate".to_owned(),
                |parent| short_fqn(parent, &crate_dir),
            )
        };
        let _ = write!(out, " · [{}]({})", label, rel_link(page, &target));
    }

    out.push_str("\n\n");
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant nav as nav()
    participant String__new as String::new
    participant crate_of as crate_of
    participant as_ref as as_ref
    participant get as get
    participant and_then as and_then
    participant module_index_path as module_index_path
    participant is_crate as is_crate
    participant crate_index_path as crate_index_path
    participant Some as Some
    participant short_fqn as short_fqn
    participant map as map
    participant unwrap_or_default as unwrap_or_default
    participant to_owned as to_owned
    participant map_or_else as map_or_else
    participant push_str as push_str
    nav-->>String__new: String::new()
    nav-->>nav: write!( out, '[Self-model]({})', rel_link(page, Path::new('README.md'...
    nav-->>crate_of: crate_of(node)
    alt item
    nav-->>as_ref: node.parent .as_ref()
    nav-->>get: doc.get(id)
    nav-->>and_then: node.parent .as_ref() .and_then(|id| doc.get(id))
    nav-->>module_index_path: module_index_path(parent, &crate_dir)
    nav-->>and_then: node.parent .as_ref() .and_then(|id| doc.get(id)) .and_then(|parent| ...
    else else
    nav-->>as_ref: node.parent .as_ref()
    nav-->>get: doc.get(id)
    nav-->>and_then: node.parent .as_ref() .and_then(|id| doc.get(id))
    nav-->>is_crate: is_crate(parent)
    alt is_crate(parent)
    nav-->>crate_index_path: crate_index_path(&crate_dir)
    nav-->>Some: Some(crate_index_path(&crate_dir))
    else else
    nav-->>module_index_path: module_index_path(parent, &crate_dir)
    end
    nav-->>and_then: node.parent .as_ref() .and_then(|id| doc.get(id)) .and_then(|parent| ...
    end
    alt let Some(target) = parent_target
    alt item
    nav-->>as_ref: node.parent .as_ref()
    nav-->>get: doc.get(id)
    nav-->>and_then: node.parent .as_ref() .and_then(|id| doc.get(id))
    nav-->>short_fqn: short_fqn(parent, &crate_dir)
    nav-->>map: node.parent .as_ref() .and_then(|id| doc.get(id)) .map(|parent| short...
    nav-->>unwrap_or_default: node.parent .as_ref() .and_then(|id| doc.get(id)) .map(|parent| short...
    else else
    nav-->>as_ref: node.parent.as_ref()
    nav-->>get: doc.get(id)
    nav-->>and_then: node.parent.as_ref().and_then(|id| doc.get(id))
    nav-->>to_owned: 'crate'.to_owned()
    nav-->>short_fqn: short_fqn(parent, &crate_dir)
    nav-->>map_or_else: node.parent.as_ref().and_then(|id| doc.get(id)).map_or_else( || 'crat...
    end
    nav-->>nav: write!(out, ' · [{}]({})', label, rel_link(page, &target))
    end
    nav-->>push_str: out.push_str('\n\n')
```

30 steps · 0 observed · 30 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::tree::crate_of`](crate-of.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::module_index_path`](module-index-path.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::is_crate`](is-crate.md)
- calls → [`robby_render::tree::crate_index_path`](crate-index-path.md)
- calls → [`robby_render::tree::module_index_path`](module-index-path.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::short_fqn`](short-fqn.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../robby_ir/run/index.md)
- calls → [`robby_render::tree::short_fqn`](short-fqn.md)

## References

- calls → [`robby_ir::run`](../../robby_ir/run/index.md) — get
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::tree`](index.md) — 5 items

## Referenced by

- calls ← [`robby_render::tree::item_page`](item-page.md)
- calls ← [`robby_render::tree::module_page`](module-page.md)

