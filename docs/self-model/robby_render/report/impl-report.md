# `robby_render::report::impl Report`

[Self-model](../../README.md) · [report](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/report.rs:42`](../../../../crates/robby-render/src/report.rs#L42) |
| provenance | extractor · confidence 1.00 |

```rust
impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, section: Section) {
        if !section.is_blank() {
            self.sections.push(section);
        }
    }

    pub fn push_body(&mut self, body: impl Into<String>) {
        self.push(Section::untitled(body));
    }

    pub fn push_titled(&mut self, title: impl Into<String>, body: impl Into<String>) {
        self.push(Section::titled(title, body));
    }

    /// Markdown artifact: optional `##` headings, bodies as-is (callers fence diagrams).
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for (index, section) in self.sections.iter().enumerate() {
            if index > 0 && !out.ends_with("\n\n") {
                if out.ends_with('\n') {
                    out.push('\n');
                } else {
                    out.push_str("\n\n");
                }
            }
            if let Some(title) = &section.title {
                let _ = writeln!(out, "## {title}\n");
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }

    /// Terminal artifact: rule-separated titled sections.
    pub fn to_terminal(&self, width: usize) -> String {
        let rule = "─".repeat(width.clamp(24, 78));
        let mut out = String::new();
        for section in &self.sections {
            if let Some(title) = &section.title {
                if !out.is_empty() {
                    out.push('\n');
                }
                let _ = writeln!(out, "{title}\n{rule}");
            } else if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }

    /// Raw concatenation (e.g. a single `.mmd` diagram body).
    pub fn to_plain(&self) -> String {
        let mut out = String::new();
        for section in &self.sections {
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
}
```

## References

- calls → [`robby_render::render::tui`](../render/tui/index.md) — default
- calls → [`robby_render::report`](index.md) — is_blank, titled, untitled
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_agent::context::collect_comment_contexts`](../../robby_agent/context/collect-comment-contexts.md)
- calls ← [`robby_agent::context::line_window`](../../robby_agent/context/line-window.md)
- calls ← [`robby_cli::workspace`](../../robby_cli/workspace.md)
- calls ← [`robby_extract::rust::collect_rs`](../../robby_extract/rust/collect-rs.md)
- calls ← [`robby_extract::rust::discover_crates`](../../robby_extract/rust/discover-crates.md)
- calls ← [`robby_extract::rust::docs_of`](../../robby_extract/rust/docs-of.md)
- calls ← [`robby_extract::rust::impl LineIndex::new`](../../robby_extract/rust/index.md)
- calls ← [`robby_extract::rust::impl ProcBuilder<'_, '_>::push_block`](../../robby_extract/rust/index.md)
- calls ← [`robby_extract::rust::path_text`](../../robby_extract/rust/path-text.md)
- calls ← [`robby_extract::rust::resolve_calls`](../../robby_extract/rust/resolve-calls.md)
- calls ← [`robby_extract::rust::rust_files`](../../robby_extract/rust/rust-files.md)
- calls ← [`robby_extract::rust::sanitize`](../../robby_extract/rust/sanitize.md)
- calls ← [`robby_extract::rust::walk_expr`](../../robby_extract/rust/walk-expr.md)
- calls ← [`robby_extract::typescript::collect_ts`](../../robby_extract/typescript/collect-ts.md)
- calls ← [`robby_extract::typescript::discover_packages`](../../robby_extract/typescript/discover-packages.md)
- calls ← [`robby_extract::typescript::emit_class`](../../robby_extract/typescript/emit-class.md)
- calls ← [`robby_extract::typescript::emit_import`](../../robby_extract/typescript/emit-import.md)
- calls ← [`robby_extract::typescript::emit_interface`](../../robby_extract/typescript/emit-interface.md)
- calls ← [`robby_extract::typescript::resolve_calls`](../../robby_extract/typescript/resolve-calls.md)
- calls ← [`robby_extract::typescript::resolve_relative_import`](../../robby_extract/typescript/resolve-relative-import.md)
- calls ← [`robby_extract::typescript::sanitize`](../../robby_extract/typescript/sanitize.md)
- calls ← [`robby_extract::typescript::walk_expr`](../../robby_extract/typescript/walk-expr.md)
- calls ← [`robby_extract::typescript::walk_stmts`](../../robby_extract/typescript/walk-stmts.md)
- calls ← [`robby_ir::codepath::impl CodePath::from_entry`](../../robby_ir/codepath/index.md)
- calls ← [`robby_ir::codepath::sample_doc`](../../robby_ir/codepath/tests/sample-doc.md)
- calls ← [`robby_ir::contract::impl Absorb for Process::absorb`](../../robby_ir/contract/index.md)
- calls ← [`robby_ir::coverage::all_spans`](../../robby_ir/coverage/all-spans.md)
- calls ← [`robby_ir::coverage::document_coverage`](../../robby_ir/coverage/document-coverage.md)
- calls ← [`robby_ir::coverage::file_coverage`](../../robby_ir/coverage/file-coverage.md)
- calls ← [`robby_ir::coverage::frontier_spans`](../../robby_ir/coverage/frontier-spans.md)
- calls ← [`robby_ir::model::impl Document::declare_file`](../../robby_ir/model/index.md)
- calls ← [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)
- calls ← [`robby_ir::model::impl Document::record_child`](../../robby_ir/model/index.md)
- calls ← [`robby_ir::model::impl Document::walk`](../../robby_ir/model/index.md)
- calls ← [`robby_ir::model::validate`](../../robby_ir/model/validate.md)
- calls ← [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls ← [`robby_ir::process::impl Process::step`](../../robby_ir/process/index.md)
- calls ← [`robby_render::dependency_report`](../dependency-report.md)
- calls ← [`robby_render::markdown::coverage_section`](../markdown/coverage-section.md)
- calls ← [`robby_render::markdown::node_section`](../markdown/node-section.md)
- calls ← [`robby_render::markdown::report`](../markdown/report.md)
- calls ← [`robby_render::pipeline::load_sources`](../pipeline/load-sources.md)
- calls ← [`robby_render::pipeline::slug`](../pipeline/slug.md)
- calls ← [`robby_render::render::markdown::impl MarkdownRenderer::fence`](../render/markdown/index.md)
- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::comments`](../render/markdown/index.md)
- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::links`](../render/markdown/index.md)
- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::meta`](../render/markdown/index.md)
- calls ← [`robby_render::render::mermaid::dsm_markdown_table`](../render/mermaid/dsm-markdown-table.md)
- calls ← [`robby_render::render::mermaid::impl Renderer for MermaidRenderer::coupling_deps`](../render/mermaid/index.md)
- calls ← [`robby_render::render::tui::ascii_coupling`](../render/tui/ascii-coupling.md)
- calls ← [`robby_render::render::tui::clip`](../render/tui/clip.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::comments`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::links`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::meta`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::process`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::sibling_graph`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::source`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::types`](../render/tui/index.md)
- calls ← [`robby_render::render::tui::impl TuiRenderer::render_report`](../render/tui/index.md)
- calls ← [`robby_render::runs::stage_process`](../runs/stage-process.md)
- calls ← [`robby_render::runs::synthetic_extract_run`](../runs/synthetic-extract-run.md)
- calls ← [`robby_render::runs::web_build_run`](../runs/web-build-run.md)
- calls ← [`robby_render::runs::write_runs`](../runs/write-runs.md)
- calls ← [`robby_render::test_markdown::fixture`](../test_markdown/fixture.md)
- calls ← [`robby_render::test_render::class_diagram_marks_traits_as_interfaces`](../test_render/class-diagram-marks-traits-as-interfaces.md)
- calls ← [`robby_render::test_render::dependencies_emits_package_weighted_and_dsm`](../test_render/dependencies-emits-package-weighted-and-dsm.md)
- calls ← [`robby_render::test_render::externals_and_comments_sections_render`](../test_render/externals-and-comments-sections-render.md)
- calls ← [`robby_render::test_render::flowchart_clusters_modules`](../test_render/flowchart-clusters-modules.md)
- calls ← [`robby_render::test_render::process`](../test_render/process.md)
- calls ← [`robby_render::test_tree::every_internal_link_resolves`](../test_tree/every-internal-link-resolves.md)
- calls ← [`robby_render::test_tree::markdown_links`](../test_tree/markdown-links.md)
- calls ← [`robby_render::test_tree::normalize`](../test_tree/normalize.md)
- calls ← [`robby_render::test_tree::page_names_never_collide_with_index`](../test_tree/page-names-never-collide-with-index.md)
- calls ← [`robby_render::test_web::demo_doc`](../test_web/demo-doc.md)
- calls ← [`robby_render::test_web::entrypoints_demo_doc`](../test_web/entrypoints-demo-doc.md)
- calls ← [`robby_render::test_web::frame_detail_includes_execution_mermaid_for_call_path`](../test_web/frame-detail-includes-execution-mermaid-for-call-path.md)
- calls ← [`robby_render::tree::global_index`](../tree/global-index.md)
- calls ← [`robby_render::tree::item_page`](../tree/item-page.md)
- calls ← [`robby_render::tree::item_slug`](../tree/item-slug.md)
- calls ← [`robby_render::tree::item_slugs`](../tree/item-slugs.md)
- calls ← [`robby_render::tree::module_page`](../tree/module-page.md)
- calls ← [`robby_render::tree::module_pages`](../tree/module-pages.md)
- calls ← [`robby_render::tree::nav`](../tree/nav.md)
- calls ← [`robby_render::tree::page_of_node`](../tree/page-of-node.md)
- calls ← [`robby_render::tree::rel_link`](../tree/rel-link.md)
- calls ← [`robby_render::tree::render_links`](../tree/render-links.md)
- calls ← [`robby_render::tree::render_node`](../tree/render-node.md)
- calls ← [`robby_render::tree::resolve_use_path`](../tree/resolve-use-path.md)
- calls ← [`robby_render::tree::tree`](../tree/tree.md)
- calls ← [`robby_render::tui::graph`](../tui/graph.md)
- calls ← [`robby_render::tui::show`](../tui/show.md)
- calls ← [`robby_render::tui::show_collected`](../tui/show-collected.md)
- calls ← [`robby_render::tui::tree`](../tui/tree.md)
- calls ← [`robby_render::util::alias`](../util/alias.md)
- calls ← [`robby_render::util::text`](../util/text.md)
- calls ← [`robby_render::view::comments::apply_highlights`](../view/comments/apply-highlights.md)
- calls ← [`robby_render::view::comments::impl CommentsView::from_root`](../view/comments/index.md)
- calls ← [`robby_render::view::comments::line_window`](../view/comments/line-window.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](../view/deps/index.md)
- calls ← [`robby_render::view::deps::topological_layers`](../view/deps/topological-layers.md)
- calls ← [`robby_render::view::entrypoints::collect_cross_edges`](../view/entrypoints/collect-cross-edges.md)
- calls ← [`robby_render::view::entrypoints::doc_with_mains`](../view/entrypoints/tests/doc-with-mains.md)
- calls ← [`robby_render::view::entrypoints::impl CallTree::from_code_paths`](../view/entrypoints/index.md)
- calls ← [`robby_render::view::execution::sample`](../view/execution/tests/sample.md)
- calls ← [`robby_render::view::hierarchy::build`](../view/hierarchy/build.md)
- calls ← [`robby_render::view::process::impl ProcessView::from_process`](../view/process/index.md)
- calls ← [`robby_render::view::scene::attach_call_portals`](../view/scene/attach-call-portals.md)
- calls ← [`robby_render::view::scene::build_call_frame`](../view/scene/build-call-frame.md)
- calls ← [`robby_render::view::scene::build_frame`](../view/scene/build-frame.md)
- calls ← [`robby_render::view::scene::impl Scene::from_document`](../view/scene/index.md)
- calls ← [`robby_render::view::scene::impl Scene::from_entrypoints`](../view/scene/index.md)
- calls ← [`robby_render::view::scene::layout_grid`](../view/scene/layout-grid.md)
- calls ← [`robby_render::view::scene::portals_for`](../view/scene/portals-for.md)
- calls ← [`robby_render::view::sibling::impl SiblingGraph::from_root`](../view/sibling/index.md)
- calls ← [`robby_render::view::types::class_from_node`](../view/types/class-from-node.md)
- calls ← [`robby_render::view::types::impl TypeModel::from_module`](../view/types/index.md)
- calls ← [`robby_render::web::write_site`](../web/write-site.md)
- calls ← [`robby_render::workspace_dependencies`](../workspace-dependencies.md)

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(0)` |
| location | [`crates/robby-render/src/report.rs:43`](../../../../crates/robby-render/src/report.rs#L43) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new() -> Self {
        Self::default()
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    participant Self__default as Self::default
    new-->>Self__default: Self::default()
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::render::tui::impl Default for ShowOptions::default`](../render/tui/index.md)

### `push`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn push(2)` |
| location | [`crates/robby-render/src/report.rs:47`](../../../../crates/robby-render/src/report.rs#L47) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn push(&mut self, section: Section) {
        if !section.is_blank() {
            self.sections.push(section);
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant push as push()
    participant is_blank as is_blank
    push-->>is_blank: section.is_blank()
    alt !section.is_blank()
    push-->>push: self.sections.push(section)
    end
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::report::impl Section::is_blank`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

### `push_body`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn push_body(2)` |
| location | [`crates/robby-render/src/report.rs:53`](../../../../crates/robby-render/src/report.rs#L53) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn push_body(&mut self, body: impl Into<String>) {
        self.push(Section::untitled(body));
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant push_body as push_body()
    participant Section__untitled as Section::untitled
    participant push as push
    push_body-->>Section__untitled: Section::untitled(body)
    push_body-->>push: self.push(Section::untitled(body))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::report::impl Section::untitled`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

### `push_titled`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn push_titled(3)` |
| location | [`crates/robby-render/src/report.rs:57`](../../../../crates/robby-render/src/report.rs#L57) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn push_titled(&mut self, title: impl Into<String>, body: impl Into<String>) {
        self.push(Section::titled(title, body));
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant push_titled as push_titled()
    participant Section__titled as Section::titled
    participant push as push
    push_titled-->>Section__titled: Section::titled(title, body)
    push_titled-->>push: self.push(Section::titled(title, body))
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_render::report::impl Section::titled`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

### `to_markdown`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn to_markdown(1)` |
| location | [`crates/robby-render/src/report.rs:61`](../../../../crates/robby-render/src/report.rs#L61) |
| provenance | extractor · confidence 1.00 |

> Markdown artifact: optional `##` headings, bodies as-is (callers fence diagrams).

```rust
/// Markdown artifact: optional `##` headings, bodies as-is (callers fence diagrams).
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for (index, section) in self.sections.iter().enumerate() {
            if index > 0 && !out.ends_with("\n\n") {
                if out.ends_with('\n') {
                    out.push('\n');
                } else {
                    out.push_str("\n\n");
                }
            }
            if let Some(title) = &section.title {
                let _ = writeln!(out, "## {title}\n");
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant to_markdown as to_markdown()
    participant String__new as String::new
    participant iter as iter
    participant enumerate as enumerate
    participant ends_with as ends_with
    participant push as push
    participant push_str as push_str
    to_markdown-->>String__new: String::new()
    to_markdown-->>iter: self.sections.iter()
    to_markdown-->>enumerate: self.sections.iter().enumerate()
    loop self.sections.iter().enumerate()
    to_markdown-->>ends_with: out.ends_with('\n\n')
    alt index > 0 && !out.ends_with('\n\n')
    to_markdown-->>ends_with: out.ends_with('\n')
    alt out.ends_with('\n')
    to_markdown-->>push: out.push('\n')
    else else
    to_markdown-->>push_str: out.push_str('\n\n')
    end
    end
    alt let Some(title) = &section.title
    to_markdown-->>to_markdown: writeln!(out, ' {title}\n')
    end
    to_markdown-->>push_str: out.push_str(&section.body)
    to_markdown-->>ends_with: section.body.ends_with('\n')
    alt !section.body.ends_with('\n')
    to_markdown-->>push: out.push('\n')
    end
    end
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

### `to_terminal`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn to_terminal(2)` |
| location | [`crates/robby-render/src/report.rs:83`](../../../../crates/robby-render/src/report.rs#L83) |
| provenance | extractor · confidence 1.00 |

> Terminal artifact: rule-separated titled sections.

```rust
/// Terminal artifact: rule-separated titled sections.
    pub fn to_terminal(&self, width: usize) -> String {
        let rule = "─".repeat(width.clamp(24, 78));
        let mut out = String::new();
        for section in &self.sections {
            if let Some(title) = &section.title {
                if !out.is_empty() {
                    out.push('\n');
                }
                let _ = writeln!(out, "{title}\n{rule}");
            } else if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant to_terminal as to_terminal()
    participant clamp as clamp
    participant repeat as repeat
    participant String__new as String::new
    participant is_empty as is_empty
    participant push as push
    participant ends_with as ends_with
    participant push_str as push_str
    to_terminal-->>clamp: width.clamp(24, 78)
    to_terminal-->>repeat: '─'.repeat(width.clamp(24, 78))
    to_terminal-->>String__new: String::new()
    loop &self.sections
    alt let Some(title) = &section.title
    to_terminal-->>is_empty: out.is_empty()
    alt !out.is_empty()
    to_terminal-->>push: out.push('\n')
    end
    to_terminal-->>to_terminal: writeln!(out, '{title}\n{rule}')
    else else
    to_terminal-->>is_empty: out.is_empty()
    to_terminal-->>ends_with: out.ends_with('\n')
    alt !out.is_empty() && !out.ends_with('\n')
    to_terminal-->>push: out.push('\n')
    end
    end
    to_terminal-->>push_str: out.push_str(&section.body)
    to_terminal-->>ends_with: section.body.ends_with('\n')
    alt !section.body.ends_with('\n')
    to_terminal-->>push: out.push('\n')
    end
    end
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

### `to_plain`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn to_plain(1)` |
| location | [`crates/robby-render/src/report.rs:104`](../../../../crates/robby-render/src/report.rs#L104) |
| provenance | extractor · confidence 1.00 |

> Raw concatenation (e.g. a single `.mmd` diagram body).

```rust
/// Raw concatenation (e.g. a single `.mmd` diagram body).
    pub fn to_plain(&self) -> String {
        let mut out = String::new();
        for section in &self.sections {
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant to_plain as to_plain()
    participant String__new as String::new
    participant push_str as push_str
    participant ends_with as ends_with
    participant push as push
    to_plain-->>String__new: String::new()
    loop &self.sections
    to_plain-->>push_str: out.push_str(&section.body)
    to_plain-->>ends_with: section.body.ends_with('\n')
    alt !section.body.ends_with('\n')
    to_plain-->>push: out.push('\n')
    end
    end
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](index.md)
- calls → [`robby_render::report::impl Report::push`](index.md)

