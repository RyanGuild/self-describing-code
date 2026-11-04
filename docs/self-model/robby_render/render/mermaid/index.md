# `robby_render::render::mermaid`

[Self-model](../../../README.md) · [robby_render](../../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/render/mod.rs:5`](../../../../../crates/robby-render/src/render/mod.rs#L5) |
| provenance | extractor · confidence 1.00 |
| nodes | 23 |

## Types

```mermaid
classDiagram
    class MermaidRenderer {
    }
    class Renderer {
        <<interface>>
    }
    MermaidRenderer ..|> Renderer : implements
```

## Package diagram

```mermaid
flowchart TB
    n_1b88225e3fbd26cd["mermaid"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_1b88225e3fbd26cd --> n_356bf2c23da68961
    n_1b88225e3fbd26cd --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_1b88225e3fbd26cd["mermaid"]
        direction TB
        n_12d6cac4f8626411["coupling_deps"]
        n_0ca465745abea557["emit_coupling_subgraph"]
        n_5862da4f52732478["emit_package_subgraph"]
        n_c2ef34cf599b22b2["emit_workspace_coupling"]
        n_09278f41badf9212["package_deps"]
    end
    n_09278f41badf9212 --> n_5862da4f52732478
    n_12d6cac4f8626411 --> n_0ca465745abea557
    n_12d6cac4f8626411 --> n_c2ef34cf599b22b2
```

## Dependency matrix

| | mermaid | robby_ir | robby_render |
|---|---:|---:|---:|
| **mermaid** | · | 10 | 39 |
| **robby_ir** | | · | |
| **robby_render** | | | · |

## Comments

- `crates/robby-render/src/render/mermaid.rs:284:1` (docs, `robby_render::render::mermaid::dsm_markdown_table`): "Markdown DSM table (row depends on column). Shared by Markdown/TUI."
  ```
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
## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
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

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`MermaidRenderer`](mermaidrenderer.md) | `code.struct` | — | `crates/robby-render/src/render/mermaid.rs:14:1` |
| [`impl Renderer for MermaidRenderer`](impl-renderer-for-mermaidrenderer.md) | `code.impl` | — | `crates/robby-render/src/render/mermaid.rs:17:1` |
| [`emit_package_subgraph`](emit-package-subgraph.md) | `code.function` | 8 | `crates/robby-render/src/render/mermaid.rs:216:1` |
| [`emit_workspace_coupling`](emit-workspace-coupling.md) | `code.function` | 8 | `crates/robby-render/src/render/mermaid.rs:235:1` |
| [`emit_coupling_subgraph`](emit-coupling-subgraph.md) | `code.function` | 13 | `crates/robby-render/src/render/mermaid.rs:253:1` |
| [`dsm_markdown_table`](dsm-markdown-table.md) | `code.function` | 56 | `crates/robby-render/src/render/mermaid.rs:284:1` |

## Imports

- `use std::fmt::Write as _;`
- `use crate::util::alias;`
- `use crate::view::{`
- `use super::Renderer;`

