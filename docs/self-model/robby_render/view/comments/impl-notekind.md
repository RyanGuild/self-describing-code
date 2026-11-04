# `robby_render::view::comments::impl NoteKind`

[Self-model](../../../README.md) · [view::comments](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/view/comments.rs:152`](../../../../../crates/robby-render/src/view/comments.rs#L152) |
| provenance | extractor · confidence 1.00 |

```rust
impl NoteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageExample => "usage_example",
            Self::ImplementationNote => "implementation_note",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::UsageExample => "usage example",
            Self::ImplementationNote => "implementation note",
        }
    }
}
```

## Referenced by

- calls ← [`robby_render::markdown::anchor`](../../markdown/anchor.md)
- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::comments`](../../render/markdown/index.md)
- calls ← [`robby_render::render::mermaid::dsm_markdown_table`](../../render/mermaid/dsm-markdown-table.md)
- calls ← [`robby_render::render::mermaid::emit_coupling_subgraph`](../../render/mermaid/emit-coupling-subgraph.md)
- calls ← [`robby_render::render::mermaid::impl Renderer for MermaidRenderer::coupling_deps`](../../render/mermaid/index.md)
- calls ← [`robby_render::render::tui::ascii_coupling`](../../render/tui/ascii-coupling.md)
- calls ← [`robby_render::render::tui::ascii_package_structure`](../../render/tui/ascii-package-structure.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::comments`](../../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::process`](../../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::sibling_graph`](../../render/tui/index.md)
- calls ← [`robby_render::test_web::entrypoints_scene_roots_mains_and_nests_callees`](../../test_web/entrypoints-scene-roots-mains-and-nests-callees.md)
- calls ← [`robby_render::tree::count_modules`](../../tree/count-modules.md)
- calls ← [`robby_render::tree::is_module`](../../tree/is-module.md)
- calls ← [`robby_render::tree::item_children`](../../tree/item-children.md)
- calls ← [`robby_render::tree::item_slug`](../../tree/item-slug.md)
- calls ← [`robby_render::tree::module_page`](../../tree/module-page.md)
- calls ← [`robby_render::tree::page_of_node`](../../tree/page-of-node.md)
- calls ← [`robby_render::tree::render_node`](../../tree/render-node.md)
- calls ← [`robby_render::util::short_kind`](../../util/short-kind.md)
- calls ← [`robby_render::view::comments::decision_child`](decision-child.md)
- calls ← [`robby_render::view::comments::enrichment_fields`](enrichment-fields.md)
- calls ← [`robby_render::view::comments::impl CommentsView::from_root`](index.md)
- calls ← [`robby_render::view::comments::parse_verdict`](parse-verdict.md)
- calls ← [`robby_render::view::deps::build_package_tree`](../deps/build-package-tree.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::from_root`](../deps/index.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](../deps/index.md)
- calls ← [`robby_render::view::deps::insert_module`](../deps/insert-module.md)
- calls ← [`robby_render::view::deps::package_edges`](../deps/package-edges.md)
- calls ← [`robby_render::view::deps::package_leaf`](../deps/package-leaf.md)
- calls ← [`robby_render::view::entrypoints::collect_cross_edges`](../entrypoints/collect-cross-edges.md)
- calls ← [`robby_render::view::execution::owning_type_label`](../execution/owning-type-label.md)
- calls ← [`robby_render::view::execution::participant_id`](../execution/participant-id.md)
- calls ← [`robby_render::view::externals::impl ExternalsView::from_root`](../externals/index.md)
- calls ← [`robby_render::view::externals::is_external_package`](../externals/is-external-package.md)
- calls ← [`robby_render::view::externals::owning_container_fqn`](../externals/owning-container-fqn.md)
- calls ← [`robby_render::view::hierarchy::build`](../hierarchy/build.md)
- calls ← [`robby_render::view::hierarchy::kind_tag`](../hierarchy/kind-tag.md)
- calls ← [`robby_render::view::scene::attach_call_portals`](../scene/attach-call-portals.md)
- calls ← [`robby_render::view::scene::build_call_frame`](../scene/build-call-frame.md)
- calls ← [`robby_render::view::scene::build_frame`](../scene/build-frame.md)
- calls ← [`robby_render::view::scene::portals_for`](../scene/portals-for.md)
- calls ← [`robby_render::view::select::collect`](../select/collect.md)
- calls ← [`robby_render::view::select::is_binary_module`](../select/is-binary-module.md)
- calls ← [`robby_render::view::select::main_process`](../select/main-process.md)
- calls ← [`robby_render::view::sibling::impl SiblingGraph::from_root`](../sibling/index.md)
- calls ← [`robby_render::view::types::class_from_node`](../types/class-from-node.md)
- calls ← [`robby_render::view::types::impl TypeModel::from_module`](../types/index.md)
- calls ← [`robby_render::view::types::impl_methods`](../types/impl-methods.md)
- calls ← [`robby_render::web::impl FrameDetail::from_node_in_doc`](../../web/index.md)
- calls ← [`robby_render::web::write_site`](../../web/write-site.md)

### `as_str`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn as_str(1)` |
| location | [`crates/robby-render/src/view/comments.rs:153`](../../../../../crates/robby-render/src/view/comments.rs#L153) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageExample => "usage_example",
            Self::ImplementationNote => "implementation_note",
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant as_str as as_str()
```

0 steps · 0 observed · 0 possible only

### `label`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn label(1)` |
| location | [`crates/robby-render/src/view/comments.rs:160`](../../../../../crates/robby-render/src/view/comments.rs#L160) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn label(self) -> &'static str {
        match self {
            Self::UsageExample => "usage example",
            Self::ImplementationNote => "implementation note",
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant label as label()
```

0 steps · 0 observed · 0 possible only

