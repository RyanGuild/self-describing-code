# `robby_render::view::select`

[Self-model](../../../README.md) · [view](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/view/mod.rs:15`](../../../../../crates/robby-render/src/view/mod.rs#L15) |
| provenance | extractor · confidence 1.00 |
| nodes | 54 |

## Types

```mermaid
classDiagram
    class NodeMeta {
        +fqn
        +kind
        +signature
        +location
        +provenance_source
        +confidence
        +docs
        +node_count
    }
    class SelectOpts {
        +hierarchy
        +hierarchy_depth
        +hierarchy_all
        +sibling
        +source
        +types
        +dependencies
        +externals
        +comments
        +links
        +entry
        +process
        +markdown()
        +tui_all()
    }
    class NodeViews {
        +meta
        +entry
        +types
        +dependencies
        +externals
        +comments
        +process
        +hierarchy
        +links
        +sibling
        +source
    }
    class Default {
        <<interface>>
    }
    SelectOpts ..|> Default : implements
```

## Package diagram

```mermaid
flowchart TB
    n_534805da25f902cb["select"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_534805da25f902cb --> n_356bf2c23da68961
    n_534805da25f902cb --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_534805da25f902cb["select"]
        direction TB
        n_bb9971de7b6aa277["collect"]
        n_52efbca52123222c["default"]
        n_737bc7514c5fccaa["is_binary_module"]
        n_7debed98e7d8376c["main_process"]
        n_d02d0abf1c3600ac["markdown"]
        n_f5a635ba7e165c71["tui_all"]
    end
    n_bb9971de7b6aa277 --> n_52efbca52123222c
    n_bb9971de7b6aa277 --> n_737bc7514c5fccaa
    n_bb9971de7b6aa277 --> n_7debed98e7d8376c
    n_d02d0abf1c3600ac --> n_52efbca52123222c
    n_f5a635ba7e165c71 --> n_52efbca52123222c
```

## Dependency matrix

| | select | robby_ir | robby_render |
|---|---:|---:|---:|
| **select** | · | 3 | 20 |
| **robby_ir** | | · | |
| **robby_render** | | | · |

## Comments

- `crates/robby-render/src/view/select.rs:17:1` (docs, `robby_render::view::select::NodeMeta`): "Identity / provenance shown at the top of every report."
  ```
  /// Identity / provenance shown at the top of every report.
  #[derive(Debug, Clone, PartialEq)]
  pub struct NodeMeta {
      pub fqn: String,
      pub kind: String,
      pub signature: Option<String>,
      pub location: Option<String>,
      pub provenance_source: &'static str,
      pub confidence: f32,
      pub docs: Option<String>,
      pub node_count: usize,
  }
  ```
- `crates/robby-render/src/view/select.rs:30:1` (docs, `robby_render::view::select::SelectOpts`): "Which optional sections to collect."
  ```
  /// Which optional sections to collect.
  #[derive(Debug, Clone)]
  pub struct SelectOpts {
      pub hierarchy: bool,
      pub hierarchy_depth: usize,
      pub hierarchy_all: bool,
      pub sibling: bool,
      pub source: bool,
      pub types: bool,
      pub dependencies: bool,
      pub externals: bool,
      pub comments: bool,
      pub links: bool,
      pub entry: bool,
      pub process: bool,
  }
  ```
- `crates/robby-render/src/view/select.rs:67:5` (docs, `robby_render::view::select::impl SelectOpts::markdown`): "Markdown module/item page defaults."
  ```
  /// Markdown module/item page defaults.
      pub fn markdown() -> Self {
          Self::default()
      }
  ```
- `crates/robby-render/src/view/select.rs:72:5` (docs, `robby_render::view::select::impl SelectOpts::tui_all`): "TUI `show --view all` defaults."
  ```
  /// TUI `show --view all` defaults.
      pub fn tui_all() -> Self {
          Self {
              hierarchy: true,
              ..Self::default()
          }
      }
  ```
- `crates/robby-render/src/view/select.rs:81:1` (docs, `robby_render::view::select::NodeViews`): "All views selected for one node."
  ```
  /// All views selected for one node.
  #[derive(Debug, Clone, PartialEq)]
  pub struct NodeViews {
      pub meta: NodeMeta,
      pub entry: Option<ProcessView>,
      pub types: Option<TypeModel>,
      pub dependencies: Option<DependencyViews>,
      pub externals: Option<ExternalsView>,
      pub comments: Option<CommentsView>,
      pub process: Option<ProcessView>,
      pub hierarchy: Option<HierarchyView>,
      pub links: LinkGraph,
      pub sibling: Option<SiblingGraph>,
      pub source: Option<SourceSlice>,
  }
  ```
- `crates/robby-render/src/view/select.rs:97:1` (docs, `robby_render::view::select::collect`): "Project IR → views for one node."
  ```
  /// Project IR → views for one node.
  pub fn collect(
      doc: &Document,
      node: &Node,
      sources: Option<&BTreeMap<String, String>>,
      opts: &SelectOpts,
  ) -> NodeViews {
      let is_module = is_module_kind(node.kind.as_str());
      let binary = is_module && is_binary_module(doc, node);
  
      let meta = NodeMeta {
          fqn: node.fqn.clone(),
          kind: node.kind.as_str().to_owned(),
          signature: node.signature.clone(),
          location: node.span.as_ref().map(robby_ir::Span::location),
          provenance_source: node.provenance.source.as_str(),
          confidence: node.provenance.confidence,
          docs: node.docs.clone(),
          node_count: doc.subtree(&node.id).len(),
      };
  
      let entry = if opts.entry && binary {
          main_process(doc, node).map(ProcessView::from_process)
      } else {
          None
      };
  
      let types = if opts.types && is_module {
          let model = TypeModel::from_module(doc, &node.id);
          if model.is_empty() { None } else { Some(model) }
      } else {
          None
      };
  
      let dependencies = if opts.dependencies && is_module {
          let views = DependencyViews::from_root(doc, &node.id);
          if views.is_empty() { None } else { Some(views) }
      } else {
          None
      };
  
      let externals = if opts.externals && is_module {
          let view = ExternalsView::from_root(doc, &node.id);
          if view.is_empty() { None } else { Some(view) }
      } else {
          None
      };
  
      let comments = if opts.comments && is_module {
          let view = CommentsView::from_root(doc, &node.id, sources);
          if view.is_empty() { None } else { Some(view) }
      } else {
          None
      };
  
      let process = if opts.process && !is_module {
          node.process.as_ref().map(ProcessView::from_process)
      } else {
          None
      };
  
      let hierarchy = if opts.hierarchy {
          let view =
              HierarchyView::from_node(doc, &node.id, opts.hierarchy_depth, opts.hierarchy_all);
          if view.is_empty() { None } else { Some(view) }
      } else {
          None
      };
  
      let links = if opts.links {
          LinkGraph::for_node(doc, node)
      } else {
          LinkGraph::default()
      };
  
      let sibling = if opts.sibling {
          let graph = SiblingGraph::from_root(doc, &node.id);
          if graph.is_empty() { None } else { Some(graph) }
      } else {
          None
      };
  
      let source = if opts.source {
          sources.and_then(|s| SourceSlice::from_node(node, s))
      } else {
          None
      };
  
      NodeViews {
          meta,
          entry,
          types,
          dependencies,
          externals,
          comments,
          process,
          hierarchy,
          links,
          sibling,
          source,
      }
  }
  ```
## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get, subtree
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::deps`](../deps/index.md) — from_root
- calls → [`robby_render::view::links`](../links/index.md) — for_node, is_empty, is_module_kind
- calls → [`robby_render::view::source`](../source/index.md) — from_node
- calls → [`robby_render::view::types`](../types/index.md) — from_module

## Referenced by

- calls ← [`robby_agent::model::parse_note`](../../../robby_agent/model/parse-note.md)
- calls ← [`robby_agent::openrouter::impl ChatClient for OpenRouterClient::chat`](../../../robby_agent/openrouter/index.md)
- calls ← [`robby_cli::resolve`](../../../robby_cli/resolve.md)
- calls ← [`robby_extract::rust::collect_rs`](../../../robby_extract/rust/collect-rs.md)
- calls ← [`robby_extract::rust::discover_crates`](../../../robby_extract/rust/discover-crates.md)
- calls ← [`robby_extract::rust::label_of`](../../../robby_extract/rust/label-of.md)
- calls ← [`robby_extract::rust::path_text`](../../../robby_extract/rust/path-text.md)
- calls ← [`robby_extract::rust::pick_call_target`](../../../robby_extract/rust/pick-call-target.md)
- calls ← [`robby_extract::rust::shared_module_score`](../../../robby_extract/rust/shared-module-score.md)
- calls ← [`robby_extract::test_extract::the_workspace_is_fully_covered`](../../../robby_extract/test_extract/the-workspace-is-fully-covered.md)
- calls ← [`robby_extract::test_typescript::emits_comment_nodes_and_attaches_docs`](../../../robby_extract/test_typescript/emits-comment-nodes-and-attaches-docs.md)
- calls ← [`robby_extract::test_typescript::typescript_fixture_is_fully_covered`](../../../robby_extract/test_typescript/typescript-fixture-is-fully-covered.md)
- calls ← [`robby_extract::typescript::collect_ts`](../../../robby_extract/typescript/collect-ts.md)
- calls ← [`robby_extract::typescript::discover_packages`](../../../robby_extract/typescript/discover-packages.md)
- calls ← [`robby_ir::codepath::build_code_paths`](../../../robby_ir/codepath/build-code-paths.md)
- calls ← [`robby_ir::codepath::find_entrypoints`](../../../robby_ir/codepath/find-entrypoints.md)
- calls ← [`robby_ir::codepath::impl CodePath::from_entry`](../../../robby_ir/codepath/index.md)
- calls ← [`robby_ir::coverage::file_coverage`](../../../robby_ir/coverage/file-coverage.md)
- calls ← [`robby_ir::coverage::impl CoverageReport::worst_first`](../../../robby_ir/coverage/index.md)
- calls ← [`robby_ir::coverage::preview`](../../../robby_ir/coverage/preview.md)
- calls ← [`robby_ir::test_coverage::report_orders_worst_files_first`](../../../robby_ir/test_coverage/report-orders-worst-files-first.md)
- calls ← [`robby_render::all_sequences`](../../all-sequences.md)
- calls ← [`robby_render::markdown::coverage_section`](../../markdown/coverage-section.md)
- calls ← [`robby_render::markdown::index`](../../markdown/index-n_fadae9.md)
- calls ← [`robby_render::markdown::node_section`](../../markdown/node-section.md)
- calls ← [`robby_render::markdown::report`](../../markdown/report.md)
- calls ← [`robby_render::pipeline::load_sources`](../../pipeline/load-sources.md)
- calls ← [`robby_render::pipeline::watched_files`](../../pipeline/watched-files.md)
- calls ← [`robby_render::render::markdown::trim_comment`](../../render/markdown/trim-comment.md)
- calls ← [`robby_render::render::mermaid::dsm_markdown_table`](../../render/mermaid/dsm-markdown-table.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::process`](../../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::sibling_graph`](../../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl TuiRenderer::show`](../../render/tui/index.md)
- calls ← [`robby_render::test_render::externals_and_comments_sections_render`](../../test_render/externals-and-comments-sections-render.md)
- calls ← [`robby_render::test_tree::every_internal_link_resolves`](../../test_tree/every-internal-link-resolves.md)
- calls ← [`robby_render::test_tree::modules_link_out_and_back`](../../test_tree/modules-link-out-and-back.md)
- calls ← [`robby_render::test_web::entrypoints_scene_roots_mains_and_nests_callees`](../../test_web/entrypoints-scene-roots-mains-and-nests-callees.md)
- calls ← [`robby_render::test_web::sibling_local_rects_do_not_overlap`](../../test_web/sibling-local-rects-do-not-overlap.md)
- calls ← [`robby_render::test_web::sibling_world_rects_do_not_collapse_onto_each_other`](../../test_web/sibling-world-rects-do-not-collapse-onto-each-other.md)
- calls ← [`robby_render::test_web::unresolved_portal_has_no_zoom_target`](../../test_web/unresolved-portal-has-no-zoom-target.md)
- calls ← [`robby_render::tree::item_children`](../../tree/item-children.md)
- calls ← [`robby_render::tree::module_page`](../../tree/module-page.md)
- calls ← [`robby_render::tree::page_paths`](../../tree/page-paths.md)
- calls ← [`robby_render::tree::rel_link`](../../tree/rel-link.md)
- calls ← [`robby_render::tree::render_node`](../../tree/render-node.md)
- calls ← [`robby_render::tree::tree`](../../tree/tree.md)
- calls ← [`robby_render::tui::show_collected`](../../tui/show-collected.md)
- calls ← [`robby_render::util::text`](../../util/text.md)
- calls ← [`robby_render::view::comments::enrichment_fields`](../comments/enrichment-fields.md)
- calls ← [`robby_render::view::deps::build_package_tree`](../deps/build-package-tree.md)
- calls ← [`robby_render::view::deps::finalize_edges`](../deps/finalize-edges.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::from_root`](../deps/index.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](../deps/index.md)
- calls ← [`robby_render::view::deps::impl PackageStructure::from_root`](../deps/index.md)
- calls ← [`robby_render::view::deps::impl PackageStructure::workspace`](../deps/index.md)
- calls ← [`robby_render::view::deps::package_edges`](../deps/package-edges.md)
- calls ← [`robby_render::view::deps::topological_layers`](../deps/topological-layers.md)
- calls ← [`robby_render::view::entrypoints::impl CallTree::from_entries`](../entrypoints/index.md)
- calls ← [`robby_render::view::execution::from_code_path`](../execution/from-code-path.md)
- calls ← [`robby_render::view::externals::impl ExternalsView::from_root`](../externals/index.md)
- calls ← [`robby_render::view::links::inbound`](../links/inbound.md)
- calls ← [`robby_render::view::links::outbound`](../links/outbound.md)
- calls ← [`robby_render::view::links::subtree_ids`](../links/subtree-ids.md)
- calls ← [`robby_render::view::process::impl ProcessView::from_process`](../process/index.md)
- calls ← [`robby_render::view::scene::build_call_frame`](../scene/build-call-frame.md)
- calls ← [`robby_render::view::scene::build_frame`](../scene/build-frame.md)
- calls ← [`robby_render::view::scene::crate_roots`](../scene/crate-roots.md)
- calls ← [`robby_render::view::scene::impl Scene::from_document`](../scene/index.md)
- calls ← [`robby_render::view::scene::portals_for`](../scene/portals-for.md)
- calls ← [`robby_render::view::sibling::impl SiblingGraph::from_root`](../sibling/index.md)
- calls ← [`robby_render::view::types::impl TypeModel::from_module`](../types/index.md)
- calls ← [`robby_render::view::types::impl_methods`](../types/impl-methods.md)

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`NodeMeta`](nodemeta.md) | `code.struct` | — | `crates/robby-render/src/view/select.rs:17:1` |
| [`SelectOpts`](selectopts.md) | `code.struct` | — | `crates/robby-render/src/view/select.rs:30:1` |
| [`impl Default for SelectOpts`](impl-default-for-selectopts.md) | `code.impl` | — | `crates/robby-render/src/view/select.rs:47:1` |
| [`impl SelectOpts`](impl-selectopts.md) | `code.impl` | — | `crates/robby-render/src/view/select.rs:66:1` |
| [`NodeViews`](nodeviews.md) | `code.struct` | — | `crates/robby-render/src/view/select.rs:81:1` |
| [`collect`](collect.md) | `code.function` | 39 | `crates/robby-render/src/view/select.rs:97:1` |
| [`is_binary_module`](is-binary-module.md) | `code.function` | 5 | `crates/robby-render/src/view/select.rs:200:1` |
| [`main_process`](main-process.md) | `code.function` | 7 | `crates/robby-render/src/view/select.rs:207:1` |

## Imports

- `use std::collections::BTreeMap;`
- [`use robby_ir::{Document, Node, kind};`](../../../robby_ir/index.md)
- `use super::comments::CommentsView;`
- `use super::deps::DependencyViews;`
- `use super::externals::ExternalsView;`
- `use super::hierarchy::HierarchyView;`
- `use super::links::{LinkGraph, is_module_kind};`
- `use super::process::ProcessView;`
- `use super::sibling::SiblingGraph;`
- `use super::source::SourceSlice;`
- `use super::types::TypeModel;`

