# `robby_render::render::tui::ascii_package_structure`

[Self-model](../../../README.md) · [render::tui](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn ascii_package_structure(view: &PackageStructure) -> String` |
| location | [`crates/robby-render/src/render/tui.rs:450`](../../../../../crates/robby-render/src/render/tui.rs#L450) |
| provenance | extractor · confidence 1.00 |

```rust
fn ascii_package_structure(view: &PackageStructure) -> String {
    fn walk(node: &PackageNode, prefix: &str, is_last: bool, out: &mut String) {
        let branch = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        let _ = writeln!(out, "{prefix}{branch}{}", node.name);
        let child_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };
        let count = node.children.len();
        for (index, child) in node.children.iter().enumerate() {
            walk(child, &child_prefix, index + 1 == count, out);
        }
    }
    if view.is_empty() {
        return String::from("(no package structure)\n");
    }
    let mut out = String::new();
    walk(&view.root, "", true, &mut out);
    if !view.externals.is_empty() {
        out.push_str("externals\n");
        for ext in &view.externals {
            let _ = writeln!(out, "  · {}", ext.name);
        }
    }
    if !view.edges.is_empty() {
        out.push_str("depends\n");
        for edge in &view.edges {
            let mark = if edge.in_cycle { " ⇄" } else { "" };
            let from = view
                .module_by_stable(&edge.from)
                .map_or(edge.from.as_str(), |m| m.name.as_str());
            let to = view
                .module_by_stable(&edge.to)
                .map_or(edge.to.as_str(), |m| m.name.as_str());
            let _ = writeln!(out, "  {from} → {to}{mark}");
        }
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant ascii_package_structure as ascii_package_structure()
    participant is_empty as is_empty
    participant String__from as String::from
    participant String__new as String::new
    participant walk as walk
    participant push_str as push_str
    participant module_by_stable as module_by_stable
    participant as_str as as_str
    participant map_or as map_or
    ascii_package_structure-->>ascii_package_structure: fn walk(node: &PackageNode, prefix: &str, is_last: bool, out: &mut St...
    ascii_package_structure-->>is_empty: view.is_empty()
    alt view.is_empty()
    ascii_package_structure-->>String__from: String::from('(no package structure)\n')
    ascii_package_structure-->>ascii_package_structure: return
    end
    ascii_package_structure-->>String__new: String::new()
    ascii_package_structure-->>walk: walk(&view.root, '', true, &mut out)
    ascii_package_structure-->>is_empty: view.externals.is_empty()
    alt !view.externals.is_empty()
    ascii_package_structure-->>push_str: out.push_str('externals\n')
    loop &view.externals
    ascii_package_structure-->>ascii_package_structure: writeln!(out, ' · {}', ext.name)
    end
    end
    ascii_package_structure-->>is_empty: view.edges.is_empty()
    alt !view.edges.is_empty()
    ascii_package_structure-->>push_str: out.push_str('depends\n')
    loop &view.edges
    ascii_package_structure-->>module_by_stable: view .module_by_stable(&edge.from)
    ascii_package_structure-->>as_str: edge.from.as_str()
    ascii_package_structure-->>as_str: m.name.as_str()
    ascii_package_structure-->>map_or: view .module_by_stable(&edge.from) .map_or(edge.from.as_str(), |m| m....
    ascii_package_structure-->>module_by_stable: view .module_by_stable(&edge.to)
    ascii_package_structure-->>as_str: edge.to.as_str()
    ascii_package_structure-->>as_str: m.name.as_str()
    ascii_package_structure-->>map_or: view .module_by_stable(&edge.to) .map_or(edge.to.as_str(), |m| m.name...
    ascii_package_structure-->>ascii_package_structure: writeln!(out, ' {from} → {to}{mark}')
    end
    end
```

20 steps · 0 observed · 20 possible only

**Edges**

- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../../robby_ir/kind/index.md)
- calls → [`robby_render::render::tui::impl TuiRenderer::new`](index.md)
- calls → [`robby_ir::model::impl Document::walk`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::module_by_stable`](../../view/deps/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::module_by_stable`](../../view/deps/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../../view/comments/index.md)

## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — walk
- calls → [`robby_render::render::tui`](index.md) — new
- calls → [`robby_render::view::comments`](../../view/comments/index.md) — as_str
- calls → [`robby_render::view::deps`](../../view/deps/index.md) — module_by_stable
- calls → [`robby_render::view::links`](../../view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::package_deps`](index.md)

