# `robby_render::render::mermaid::dsm_markdown_table`

[Self-model](../../../README.md) · [render::mermaid](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn dsm_markdown_table(view: &PackageStructure) -> String` |
| location | [`crates/robby-render/src/render/mermaid.rs:284`](../../../../../crates/robby-render/src/render/mermaid.rs#L284) |
| provenance | extractor · confidence 1.00 |

> Markdown DSM table (row depends on column). Shared by Markdown/TUI.

```rust
/// Markdown DSM table (row depends on column). Shared by Markdown/TUI.
pub fn dsm_markdown_table(view: &PackageStructure) -> String {
    if view.modules.is_empty() {
        return String::new();
    }

    let order: Vec<&crate::view::DepModule> = {
        // Prefer layer order for headers, then any leftovers by name.
        let mut seen = std::collections::BTreeSet::new();
        let mut ordered = Vec::new();
        for layer in &view.layers {
            for sid in layer {
                if let Some(m) = view.module_by_stable(sid) {
                    if seen.insert(sid.clone()) {
                        ordered.push(m);
                    }
                }
            }
        }
        for m in &view.modules {
            if seen.insert(m.stable_id.clone()) {
                ordered.push(m);
            }
        }
        ordered
    };

    let index: std::collections::BTreeMap<&str, usize> = order
        .iter()
        .enumerate()
        .map(|(i, m)| (m.stable_id.as_str(), i))
        .collect();

    let n = order.len();
    let mut matrix = vec![vec![0usize; n]; n];
    let mut cyclic = vec![vec![false; n]; n];
    for edge in &view.edges {
        let Some(&i) = index.get(edge.from.as_str()) else {
            continue;
        };
        let Some(&j) = index.get(edge.to.as_str()) else {
            continue;
        };
        matrix[i][j] = edge.weight;
        cyclic[i][j] = edge.in_cycle;
    }

    let mut out = String::new();
    out.push_str("| |");
    for m in &order {
        let _ = write!(out, " {} |", m.name);
    }
    out.push('\n');
    out.push_str("|---|");
    for _ in &order {
        out.push_str("---:|");
    }
    out.push('\n');

    for (i, row) in order.iter().enumerate() {
        let _ = write!(out, "| **{}** |", row.name);
        for (j, _) in order.iter().enumerate() {
            if i == j {
                out.push_str(" · |");
            } else if matrix[i][j] == 0 {
                out.push_str(" |");
            } else if cyclic[i][j] {
                let _ = write!(out, " {}⇄ |", matrix[i][j]);
            } else {
                let _ = write!(out, " {} |", matrix[i][j]);
            }
        }
        out.push('\n');
    }

    if !view.cycles.is_empty() {
        out.push_str("\nCycles: ");
        let names: Vec<String> = view
            .cycles
            .iter()
            .map(|scc| {
                scc.iter()
                    .filter_map(|sid| view.module_by_stable(sid).map(|m| m.name.as_str()))
                    .collect::<Vec<_>>()
                    .join(" ↔ ")
            })
            .collect();
        out.push_str(&names.join("; "));
        out.push('\n');
    }

    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant dsm_markdown_table as dsm_markdown_table()
    participant is_empty as is_empty
    participant String__new as String::new
    participant std__collections__BTreeSet__new as std::collections::BTreeSet::new
    participant Vec__new as Vec::new
    participant module_by_stable as module_by_stable
    participant clone as clone
    participant insert as insert
    participant push as push
    participant iter as iter
    participant enumerate as enumerate
    participant as_str as as_str
    participant map as map
    participant collect as collect
    participant len as len
    participant get as get
    participant push_str as push_str
    participant filter_map as filter_map
    participant join as join
    dsm_markdown_table-->>is_empty: view.modules.is_empty()
    alt view.modules.is_empty()
    dsm_markdown_table-->>String__new: String::new()
    dsm_markdown_table-->>dsm_markdown_table: return
    end
    dsm_markdown_table-->>std__collections__BTreeSet__new: std::collections::BTreeSet::new()
    dsm_markdown_table-->>Vec__new: Vec::new()
    loop &view.layers
    loop layer
    dsm_markdown_table-->>module_by_stable: view.module_by_stable(sid)
    alt let Some(m) = view.module_by_stable(sid)
    dsm_markdown_table-->>clone: sid.clone()
    dsm_markdown_table-->>insert: seen.insert(sid.clone())
    alt seen.insert(sid.clone())
    dsm_markdown_table-->>push: ordered.push(m)
    end
    end
    end
    end
    loop &view.modules
    dsm_markdown_table-->>clone: m.stable_id.clone()
    dsm_markdown_table-->>insert: seen.insert(m.stable_id.clone())
    alt seen.insert(m.stable_id.clone())
    dsm_markdown_table-->>push: ordered.push(m)
    end
    end
    dsm_markdown_table-->>iter: order .iter()
    dsm_markdown_table-->>enumerate: order .iter() .enumerate()
    dsm_markdown_table-->>as_str: m.stable_id.as_str()
    dsm_markdown_table-->>map: order .iter() .enumerate() .map(|(i, m)| (m.stable_id.as_str(), i))
    dsm_markdown_table-->>collect: order .iter() .enumerate() .map(|(i, m)| (m.stable_id.as_str(), i)) ....
    dsm_markdown_table-->>len: order.len()
    dsm_markdown_table-->>dsm_markdown_table: vec![vec![0usize n] n]
    dsm_markdown_table-->>dsm_markdown_table: vec![vec![false n] n]
    loop &view.edges
    dsm_markdown_table-->>as_str: edge.from.as_str()
    dsm_markdown_table-->>get: index.get(edge.from.as_str())
    dsm_markdown_table-->>as_str: edge.to.as_str()
    dsm_markdown_table-->>get: index.get(edge.to.as_str())
    end
    dsm_markdown_table-->>String__new: String::new()
    dsm_markdown_table-->>push_str: out.push_str('| |')
    loop &order
    dsm_markdown_table-->>dsm_markdown_table: write!(out, ' {} |', m.name)
    end
    dsm_markdown_table-->>push: out.push('\n')
    dsm_markdown_table-->>push_str: out.push_str('|---|')
    loop &order
    dsm_markdown_table-->>push_str: out.push_str('---:|')
    end
    dsm_markdown_table-->>push: out.push('\n')
    dsm_markdown_table-->>iter: order.iter()
    dsm_markdown_table-->>enumerate: order.iter().enumerate()
    loop order.iter().enumerate()
    dsm_markdown_table-->>dsm_markdown_table: write!(out, '| **{}** |', row.name)
    dsm_markdown_table-->>iter: order.iter()
    dsm_markdown_table-->>enumerate: order.iter().enumerate()
    loop order.iter().enumerate()
    alt i == j
    dsm_markdown_table-->>push_str: out.push_str(' · |')
    else else
    alt matrix[i][j] == 0
    dsm_markdown_table-->>push_str: out.push_str(' |')
    else else
    alt cyclic[i][j]
    dsm_markdown_table-->>dsm_markdown_table: write!(out, ' {}⇄ |', matrix[i][j])
    else else
    dsm_markdown_table-->>dsm_markdown_table: write!(out, ' {} |', matrix[i][j])
    end
    end
    end
    end
    dsm_markdown_table-->>push: out.push('\n')
    end
    dsm_markdown_table-->>is_empty: view.cycles.is_empty()
    alt !view.cycles.is_empty()
    dsm_markdown_table-->>push_str: out.push_str('\nCycles: ')
    dsm_markdown_table-->>iter: view .cycles .iter()
    dsm_markdown_table-->>iter: scc.iter()
    dsm_markdown_table-->>module_by_stable: view.module_by_stable(sid)
    dsm_markdown_table-->>as_str: m.name.as_str()
    dsm_markdown_table-->>map: view.module_by_stable(sid).map(|m| m.name.as_str())
    dsm_markdown_table-->>filter_map: scc.iter() .filter_map(|sid| view.module_by_stable(sid).map(|m| m.nam...
    dsm_markdown_table-->>collect: scc.iter() .filter_map(|sid| view.module_by_stable(sid).map(|m| m.nam...
    dsm_markdown_table-->>join: scc.iter() .filter_map(|sid| view.module_by_stable(sid).map(|m| m.nam...
    dsm_markdown_table-->>map: view .cycles .iter() .map(|scc| { scc.iter() .filter_map(|sid| view.m...
    dsm_markdown_table-->>collect: view .cycles .iter() .map(|scc| { scc.iter() .filter_map(|sid| view.m...
    dsm_markdown_table-->>join: names.join(' ')
    dsm_markdown_table-->>push_str: out.push_str(&names.join(' '))
    dsm_markdown_table-->>push: out.push('\n')
    end
```

56 steps · 0 observed · 56 possible only

**Edges**

- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::render::tui::impl TuiRenderer::new`](../tui/index.md)
- calls → [`robby_render::render::tui::impl TuiRenderer::new`](../tui/index.md)
- calls → [`robby_render::render::tui::impl TuiRenderer::new`](../tui/index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::module_by_stable`](../../view/deps/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::view::select::collect`](../../view/select/collect.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../../view/comments/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../../view/comments/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::render::tui::impl TuiRenderer::new`](../tui/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../../view/links/index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::module_by_stable`](../../view/deps/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../../view/comments/index.md)
- calls → [`robby_render::view::select::collect`](../../view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../../view/select/collect.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get, insert
- calls → [`robby_render::render::tui`](../tui/index.md) — new
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::view::comments`](../../view/comments/index.md) — as_str
- calls → [`robby_render::view::deps`](../../view/deps/index.md) — module_by_stable
- calls → [`robby_render::view::links`](../../view/links/index.md) — is_empty
- calls → [`robby_render::view::select`](../../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::dependency_matrix`](../markdown/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::dependency_matrix`](../tui/index.md)

