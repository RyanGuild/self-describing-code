# `robby_ir::model::impl Document`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:360`](../../../../crates/robby-ir/src/model.rs#L360) |
| provenance | extractor · confidence 1.00 |

```rust
impl Document {
    pub fn new(generator: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            generator: generator.into(),
            nodes: BTreeMap::new(),
            roots: Vec::new(),
            files: Vec::new(),
            code_paths: Vec::new(),
            pending_children: BTreeMap::new(),
            pending_parents: BTreeMap::new(),
        }
    }

    /// Rebuild [`Self::code_paths`] from current `Calls` edges and `main` entries.
    pub fn rebuild_code_paths(&mut self) {
        self.code_paths = build_code_paths(self, DEFAULT_MAX_DEPTH);
    }

    /// Record a source file as part of the map.
    pub fn declare_file(&mut self, path: impl Into<String>) {
        let path = path.into();
        if !self.files.contains(&path) {
            self.files.push(path);
            self.files.sort();
        }
    }

    pub fn insert(&mut self, node: Node) {
        self.merge(node);
    }

    /// Insert, or merge into an existing node with the same id.
    ///
    /// Merging matters for modules: a `mod foo;` declaration and the file
    /// `foo.rs` both describe `crate::foo`. The declaration carries the span,
    /// the file carries the contents, and they must not overwrite each other.
    ///
    /// Containment has one writer: this method. Set [`Node::parent`] on the
    /// child. A `children` list passed in is normalized into that same index,
    /// including when the child node arrives later.
    pub fn merge(&mut self, mut node: Node) {
        let id = node.id.clone();
        let declared_children = std::mem::take(&mut node.children);
        let incoming_parent = node.parent.clone();

        let adopted_parent = if let Some(existing) = self.nodes.get_mut(&id) {
            if existing.span.is_none() {
                existing.span = node.span;
            }
            let adopted = existing.parent.is_none() && incoming_parent.is_some();
            if adopted {
                existing.parent = incoming_parent;
            }
            for edge in node.edges {
                if !existing.edges.contains(&edge) {
                    existing.edges.push(edge);
                }
            }
            for (key, value) in node.annotations {
                existing.annotations.entry(key).or_insert(value);
            }
            for tag in node.tags {
                if !existing.tags.contains(&tag) {
                    existing.tags.push(tag);
                }
            }
            adopted
        } else {
            if node.parent.is_none()
                && let Some(parent) = self.pending_parents.remove(&id)
            {
                node.parent = Some(parent);
            }
            if node.parent.is_none() {
                self.roots.push(id.clone());
            }
            self.nodes.insert(id.clone(), node);
            if let Some(waiting) = self.pending_children.remove(&id) {
                for child in waiting {
                    self.record_child(&id, &child);
                }
            }
            false
        };

        if let Some(parent) = self.nodes.get(&id).and_then(|n| n.parent.clone()) {
            self.record_child(&parent, &id);
        }
        if adopted_parent {
            self.roots.retain(|root| root != &id);
        }
        for child in declared_children {
            self.record_child(&id, &child);
        }
        // A children list can name a node that is inserted later, with no parent
        // of its own. Point that node back so the two fields cannot diverge.
        self.adopt_parent(&id);
    }

    /// If some node already lists `id` and `id` has no parent, adopt that link.
    fn adopt_parent(&mut self, id: &NodeId) {
        if self.nodes.get(id).is_some_and(|node| node.parent.is_some()) {
            return;
        }
        let listed = self
            .nodes
            .iter()
            .find(|(parent_id, node)| *parent_id != id && node.children.contains(id))
            .map(|(parent_id, _)| parent_id.clone());
        if let Some(parent) = listed {
            self.record_child(&parent, id);
        }
    }

    /// Record `child` under `parent`, and point the child back if it has no parent yet.
    fn record_child(&mut self, parent: &NodeId, child: &NodeId) {
        if parent == child {
            return;
        }
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            if !parent_node.children.contains(child) {
                parent_node.children.push(child.clone());
            }
        } else {
            let pending = self.pending_children.entry(parent.clone()).or_default();
            if !pending.contains(child) {
                pending.push(child.clone());
            }
        }
        if !self.nodes.contains_key(child) {
            self.pending_parents
                .entry(child.clone())
                .or_insert_with(|| parent.clone());
        }
        let needs_parent = self
            .nodes
            .get(child)
            .is_some_and(|node| node.parent.is_none());
        if needs_parent {
            if let Some(node) = self.nodes.get_mut(child) {
                node.parent = Some(parent.clone());
            }
            self.roots.retain(|root| root != child);
        }
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// All nodes under `root`, depth-first, deterministic order.
    pub fn subtree(&self, root: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        self.walk(root, &mut out);
        out
    }

    fn walk<'a>(&'a self, id: &NodeId, out: &mut Vec<&'a Node>) {
        if let Some(node) = self.nodes.get(id) {
            out.push(node);
            for child in &node.children {
                self.walk(child, out);
            }
        }
    }
}
```

## References

- calls → [`robby_ir::codepath`](../codepath/index.md) — build_code_paths
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::scene`](../../robby_render/view/scene/index.md) — contains

## Referenced by

- calls ← [`robby_agent::context::has_decision_child`](../../robby_agent/context/has-decision-child.md)
- calls ← [`robby_agent::context::line_window`](../../robby_agent/context/line-window.md)
- calls ← [`robby_agent::context::nearest_following_item`](../../robby_agent/context/nearest-following-item.md)
- calls ← [`robby_agent::context::parent_has_comment_with_text`](../../robby_agent/context/parent-has-comment-with-text.md)
- calls ← [`robby_agent::context::slice_node`](../../robby_agent/context/slice-node.md)
- calls ← [`robby_agent::enrich::insert_decision`](../../robby_agent/enrich/insert-decision.md)
- calls ← [`robby_agent::enrich::mini_doc`](../../robby_agent/enrich/tests/mini-doc.md)
- calls ← [`robby_agent::model::parse_decision`](../../robby_agent/model/parse-decision.md)
- calls ← [`robby_agent::model::parse_note`](../../robby_agent/model/parse-note.md)
- calls ← [`robby_extract::merge_documents`](../../robby_extract/merge-documents.md)
- calls ← [`robby_extract::rust::extract`](../../robby_extract/rust/extract.md)
- calls ← [`robby_extract::rust::extract_file`](../../robby_extract/rust/extract-file.md)
- calls ← [`robby_extract::rust::resolve_calls`](../../robby_extract/rust/resolve-calls.md)
- calls ← [`robby_extract::rust::sanitize`](../../robby_extract/rust/sanitize.md)
- calls ← [`robby_extract::rust::slice`](../../robby_extract/rust/slice.md)
- calls ← [`robby_extract::rust::walk_items`](../../robby_extract/rust/walk-items.md)
- calls ← [`robby_extract::test_extract::the_workspace_is_fully_covered`](../../robby_extract/test_extract/the-workspace-is-fully-covered.md)
- calls ← [`robby_extract::test_typescript::typescript_fixture_is_fully_covered`](../../robby_extract/test_typescript/typescript-fixture-is-fully-covered.md)
- calls ← [`robby_extract::typescript::attach_leading_docs`](../../robby_extract/typescript/attach-leading-docs.md)
- calls ← [`robby_extract::typescript::emit_class`](../../robby_extract/typescript/emit-class.md)
- calls ← [`robby_extract::typescript::emit_comments`](../../robby_extract/typescript/emit-comments.md)
- calls ← [`robby_extract::typescript::emit_function`](../../robby_extract/typescript/emit-function.md)
- calls ← [`robby_extract::typescript::emit_import`](../../robby_extract/typescript/emit-import.md)
- calls ← [`robby_extract::typescript::emit_interface`](../../robby_extract/typescript/emit-interface.md)
- calls ← [`robby_extract::typescript::emit_method`](../../robby_extract/typescript/emit-method.md)
- calls ← [`robby_extract::typescript::emit_variable`](../../robby_extract/typescript/emit-variable.md)
- calls ← [`robby_extract::typescript::extract`](../../robby_extract/typescript/extract.md)
- calls ← [`robby_extract::typescript::extract_file`](../../robby_extract/typescript/extract-file.md)
- calls ← [`robby_extract::typescript::gap_is_attachable`](../../robby_extract/typescript/gap-is-attachable.md)
- calls ← [`robby_extract::typescript::read_package_name`](../../robby_extract/typescript/read-package-name.md)
- calls ← [`robby_extract::typescript::resolve_calls`](../../robby_extract/typescript/resolve-calls.md)
- calls ← [`robby_extract::typescript::sanitize`](../../robby_extract/typescript/sanitize.md)
- calls ← [`robby_extract::typescript::walk_declaration`](../../robby_extract/typescript/walk-declaration.md)
- calls ← [`robby_extract::typescript::walk_statements`](../../robby_extract/typescript/walk-statements.md)
- calls ← [`robby_ir::codepath::impl CodePath::from_entry`](../codepath/index.md)
- calls ← [`robby_ir::codepath::sample_doc`](../codepath/tests/sample-doc.md)
- calls ← [`robby_ir::coverage::document_coverage`](../coverage/document-coverage.md)
- calls ← [`robby_ir::coverage::has_span_descendant`](../coverage/has-span-descendant.md)
- calls ← [`robby_ir::model::child_inserted_before_its_parent_is_linked`](tests/child-inserted-before-its-parent-is-linked.md)
- calls ← [`robby_ir::model::declared_children_set_the_child_parent`](tests/declared-children-set-the-child-parent.md)
- calls ← [`robby_ir::model::parent_pointer_builds_the_child_index`](tests/parent-pointer-builds-the-child-index.md)
- calls ← [`robby_ir::model::validate`](validate.md)
- calls ← [`robby_ir::test_coverage::document_coverage_is_union_at_any_depth`](../test_coverage/document-coverage-is-union-at-any-depth.md)
- calls ← [`robby_ir::test_coverage::frontier_spans_exclude_parents_that_have_span_children`](../test_coverage/frontier-spans-exclude-parents-that-have-span-children.md)
- calls ← [`robby_ir::test_coverage::report_orders_worst_files_first`](../test_coverage/report-orders-worst-files-first.md)
- calls ← [`robby_ir::test_coverage::unreferenced_files_appear_fully_uncovered`](../test_coverage/unreferenced-files-appear-fully-uncovered.md)
- calls ← [`robby_render::markdown::heading_depth`](../../robby_render/markdown/heading-depth.md)
- calls ← [`robby_render::markdown::node_section`](../../robby_render/markdown/node-section.md)
- calls ← [`robby_render::markdown::report`](../../robby_render/markdown/report.md)
- calls ← [`robby_render::pipeline::from_document`](../../robby_render/pipeline/from-document.md)
- calls ← [`robby_render::pipeline::load_sources`](../../robby_render/pipeline/load-sources.md)
- calls ← [`robby_render::render::mermaid::dsm_markdown_table`](../../robby_render/render/mermaid/dsm-markdown-table.md)
- calls ← [`robby_render::render::mermaid::emit_coupling_subgraph`](../../robby_render/render/mermaid/emit-coupling-subgraph.md)
- calls ← [`robby_render::render::tui::ascii_coupling`](../../robby_render/render/tui/ascii-coupling.md)
- calls ← [`robby_render::render::tui::ascii_package_structure`](../../robby_render/render/tui/ascii-package-structure.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::process`](../../robby_render/render/tui/index.md)
- calls ← [`robby_render::render::tui::impl Renderer for TuiRenderer::sibling_graph`](../../robby_render/render/tui/index.md)
- calls ← [`robby_render::test_markdown::fixture`](../../robby_render/test_markdown/fixture.md)
- calls ← [`robby_render::test_render::class_diagram_marks_traits_as_interfaces`](../../robby_render/test_render/class-diagram-marks-traits-as-interfaces.md)
- calls ← [`robby_render::test_render::dependencies_emits_package_weighted_and_dsm`](../../robby_render/test_render/dependencies-emits-package-weighted-and-dsm.md)
- calls ← [`robby_render::test_render::externals_and_comments_sections_render`](../../robby_render/test_render/externals-and-comments-sections-render.md)
- calls ← [`robby_render::test_render::flowchart_clusters_modules`](../../robby_render/test_render/flowchart-clusters-modules.md)
- calls ← [`robby_render::test_tree::modules_link_out_and_back`](../../robby_render/test_tree/modules-link-out-and-back.md)
- calls ← [`robby_render::test_web::demo_doc`](../../robby_render/test_web/demo-doc.md)
- calls ← [`robby_render::test_web::entrypoints_demo_doc`](../../robby_render/test_web/entrypoints-demo-doc.md)
- calls ← [`robby_render::test_web::entrypoints_scene_roots_mains_and_nests_callees`](../../robby_render/test_web/entrypoints-scene-roots-mains-and-nests-callees.md)
- calls ← [`robby_render::test_web::frame_detail_includes_execution_mermaid_for_call_path`](../../robby_render/test_web/frame-detail-includes-execution-mermaid-for-call-path.md)
- calls ← [`robby_render::test_web::leaf_payload_includes_source_and_process`](../../robby_render/test_web/leaf-payload-includes-source-and-process.md)
- calls ← [`robby_render::tree::count_modules`](../../robby_render/tree/count-modules.md)
- calls ← [`robby_render::tree::global_index`](../../robby_render/tree/global-index.md)
- calls ← [`robby_render::tree::item_children`](../../robby_render/tree/item-children.md)
- calls ← [`robby_render::tree::item_slug`](../../robby_render/tree/item-slug.md)
- calls ← [`robby_render::tree::item_slugs`](../../robby_render/tree/item-slugs.md)
- calls ← [`robby_render::tree::module_page`](../../robby_render/tree/module-page.md)
- calls ← [`robby_render::tree::module_pages`](../../robby_render/tree/module-pages.md)
- calls ← [`robby_render::tree::module_tree_line`](../../robby_render/tree/module-tree-line.md)
- calls ← [`robby_render::tree::nav`](../../robby_render/tree/nav.md)
- calls ← [`robby_render::tree::page_of_node`](../../robby_render/tree/page-of-node.md)
- calls ← [`robby_render::tree::render_links`](../../robby_render/tree/render-links.md)
- calls ← [`robby_render::tree::render_node`](../../robby_render/tree/render-node.md)
- calls ← [`robby_render::tree::resolve_use_path`](../../robby_render/tree/resolve-use-path.md)
- calls ← [`robby_render::tree::slice`](../../robby_render/tree/slice.md)
- calls ← [`robby_render::util::alias`](../../robby_render/util/alias.md)
- calls ← [`robby_render::view::comments::decision_child`](../../robby_render/view/comments/decision-child.md)
- calls ← [`robby_render::view::comments::enrichment_fields`](../../robby_render/view/comments/enrichment-fields.md)
- calls ← [`robby_render::view::comments::impl CommentsView::from_root`](../../robby_render/view/comments/index.md)
- calls ← [`robby_render::view::comments::line_window`](../../robby_render/view/comments/line-window.md)
- calls ← [`robby_render::view::comments::nearest_following_item`](../../robby_render/view/comments/nearest-following-item.md)
- calls ← [`robby_render::view::comments::parse_verdict`](../../robby_render/view/comments/parse-verdict.md)
- calls ← [`robby_render::view::comments::slice_node`](../../robby_render/view/comments/slice-node.md)
- calls ← [`robby_render::view::deps::best_scope_prefix`](../../robby_render/view/deps/best-scope-prefix.md)
- calls ← [`robby_render::view::deps::container_of`](../../robby_render/view/deps/container-of.md)
- calls ← [`robby_render::view::deps::crate_of`](../../robby_render/view/deps/crate-of.md)
- calls ← [`robby_render::view::deps::finalize_edges`](../../robby_render/view/deps/finalize-edges.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::from_root`](../../robby_render/view/deps/index.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](../../robby_render/view/deps/index.md)
- calls ← [`robby_render::view::deps::impl PackageStructure::from_root`](../../robby_render/view/deps/index.md)
- calls ← [`robby_render::view::deps::insert_module`](../../robby_render/view/deps/insert-module.md)
- calls ← [`robby_render::view::deps::lift_to_scope`](../../robby_render/view/deps/lift-to-scope.md)
- calls ← [`robby_render::view::deps::owning_in_scope`](../../robby_render/view/deps/owning-in-scope.md)
- calls ← [`robby_render::view::deps::package_edges`](../../robby_render/view/deps/package-edges.md)
- calls ← [`robby_render::view::deps::topological_layers`](../../robby_render/view/deps/topological-layers.md)
- calls ← [`robby_render::view::entrypoints::collect_cross_edges`](../../robby_render/view/entrypoints/collect-cross-edges.md)
- calls ← [`robby_render::view::entrypoints::doc_with_mains`](../../robby_render/view/entrypoints/tests/doc-with-mains.md)
- calls ← [`robby_render::view::entrypoints::impl CallTree::children_of`](../../robby_render/view/entrypoints/index.md)
- calls ← [`robby_render::view::entrypoints::impl CallTree::from_code_paths`](../../robby_render/view/entrypoints/index.md)
- calls ← [`robby_render::view::execution::from_code_path`](../../robby_render/view/execution/from-code-path.md)
- calls ← [`robby_render::view::execution::owning_type_label`](../../robby_render/view/execution/owning-type-label.md)
- calls ← [`robby_render::view::execution::sample`](../../robby_render/view/execution/tests/sample.md)
- calls ← [`robby_render::view::externals::impl ExternalsView::from_root`](../../robby_render/view/externals/index.md)
- calls ← [`robby_render::view::externals::is_external_package`](../../robby_render/view/externals/is-external-package.md)
- calls ← [`robby_render::view::externals::owning_container_fqn`](../../robby_render/view/externals/owning-container-fqn.md)
- calls ← [`robby_render::view::hierarchy::build`](../../robby_render/view/hierarchy/build.md)
- calls ← [`robby_render::view::hierarchy::impl HierarchyView::from_node`](../../robby_render/view/hierarchy/index.md)
- calls ← [`robby_render::view::links::inbound`](../../robby_render/view/links/inbound.md)
- calls ← [`robby_render::view::links::outbound`](../../robby_render/view/links/outbound.md)
- calls ← [`robby_render::view::links::owning_module`](../../robby_render/view/links/owning-module.md)
- calls ← [`robby_render::view::links::subtree_ids`](../../robby_render/view/links/subtree-ids.md)
- calls ← [`robby_render::view::scene::attach_call_portals`](../../robby_render/view/scene/attach-call-portals.md)
- calls ← [`robby_render::view::scene::build_call_frame`](../../robby_render/view/scene/build-call-frame.md)
- calls ← [`robby_render::view::scene::build_frame`](../../robby_render/view/scene/build-frame.md)
- calls ← [`robby_render::view::scene::crate_roots`](../../robby_render/view/scene/crate-roots.md)
- calls ← [`robby_render::view::scene::impl Scene::from_document`](../../robby_render/view/scene/index.md)
- calls ← [`robby_render::view::scene::impl Scene::from_entrypoints`](../../robby_render/view/scene/index.md)
- calls ← [`robby_render::view::scene::portals_for`](../../robby_render/view/scene/portals-for.md)
- calls ← [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls ← [`robby_render::view::select::is_binary_module`](../../robby_render/view/select/is-binary-module.md)
- calls ← [`robby_render::view::select::main_process`](../../robby_render/view/select/main-process.md)
- calls ← [`robby_render::view::sibling::impl SiblingGraph::from_root`](../../robby_render/view/sibling/index.md)
- calls ← [`robby_render::view::source::impl SourceSlice::from_node`](../../robby_render/view/source/index.md)
- calls ← [`robby_render::view::types::class_from_node`](../../robby_render/view/types/class-from-node.md)
- calls ← [`robby_render::view::types::impl TypeModel::from_module`](../../robby_render/view/types/index.md)
- calls ← [`robby_render::view::types::impl_methods`](../../robby_render/view/types/impl-methods.md)
- calls ← [`robby_render::web::write_site`](../../robby_render/web/write-site.md)

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(1)` |
| location | [`crates/robby-ir/src/model.rs:361`](../../../../crates/robby-ir/src/model.rs#L361) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new(generator: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            generator: generator.into(),
            nodes: BTreeMap::new(),
            roots: Vec::new(),
            files: Vec::new(),
            code_paths: Vec::new(),
            pending_children: BTreeMap::new(),
            pending_parents: BTreeMap::new(),
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    participant into as into
    participant BTreeMap__new as BTreeMap::new
    participant Vec__new as Vec::new
    new-->>into: generator.into()
    new-->>BTreeMap__new: BTreeMap::new()
    new-->>Vec__new: Vec::new()
    new-->>Vec__new: Vec::new()
    new-->>Vec__new: Vec::new()
    new-->>BTreeMap__new: BTreeMap::new()
    new-->>BTreeMap__new: BTreeMap::new()
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::new`](index.md)

### `rebuild_code_paths`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn rebuild_code_paths(1)` |
| location | [`crates/robby-ir/src/model.rs:374`](../../../../crates/robby-ir/src/model.rs#L374) |
| provenance | extractor · confidence 1.00 |

> Rebuild [`Self::code_paths`] from current `Calls` edges and `main` entries.

```rust
/// Rebuild [`Self::code_paths`] from current `Calls` edges and `main` entries.
    pub fn rebuild_code_paths(&mut self) {
        self.code_paths = build_code_paths(self, DEFAULT_MAX_DEPTH);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant rebuild_code_paths as rebuild_code_paths()
    participant build_code_paths as build_code_paths
    rebuild_code_paths-->>build_code_paths: build_code_paths(self, DEFAULT_MAX_DEPTH)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_ir::codepath::build_code_paths`](../codepath/build-code-paths.md)

### `declare_file`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn declare_file(2)` |
| location | [`crates/robby-ir/src/model.rs:379`](../../../../crates/robby-ir/src/model.rs#L379) |
| provenance | extractor · confidence 1.00 |

> Record a source file as part of the map.

```rust
/// Record a source file as part of the map.
    pub fn declare_file(&mut self, path: impl Into<String>) {
        let path = path.into();
        if !self.files.contains(&path) {
            self.files.push(path);
            self.files.sort();
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant declare_file as declare_file()
    participant into as into
    participant contains as contains
    participant push as push
    participant sort as sort
    declare_file-->>into: path.into()
    declare_file-->>contains: self.files.contains(&path)
    alt !self.files.contains(&path)
    declare_file-->>push: self.files.push(path)
    declare_file-->>sort: self.files.sort()
    end
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

### `insert`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn insert(2)` |
| location | [`crates/robby-ir/src/model.rs:388`](../../../../crates/robby-ir/src/model.rs#L388) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn insert(&mut self, node: Node) {
        self.merge(node);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant insert as insert()
    participant merge as merge
    insert-->>merge: self.merge(node)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::merge`](index.md)

### `merge`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn merge(2)` |
| location | [`crates/robby-ir/src/model.rs:392`](../../../../crates/robby-ir/src/model.rs#L392) |
| provenance | extractor · confidence 1.00 |

> Insert, or merge into an existing node with the same id.
> 
> Merging matters for modules: a `mod foo;` declaration and the file
> `foo.rs` both describe `crate::foo`. The declaration carries the span,
> the file carries the contents, and they must not overwrite each other.
> 
> Containment has one writer: this method. Set [`Node::parent`] on the
> child. A `children` list passed in is normalized into that same index,
> including when the child node arrives later.

```rust
/// Insert, or merge into an existing node with the same id.
    ///
    /// Merging matters for modules: a `mod foo;` declaration and the file
    /// `foo.rs` both describe `crate::foo`. The declaration carries the span,
    /// the file carries the contents, and they must not overwrite each other.
    ///
    /// Containment has one writer: this method. Set [`Node::parent`] on the
    /// child. A `children` list passed in is normalized into that same index,
    /// including when the child node arrives later.
    pub fn merge(&mut self, mut node: Node) {
        let id = node.id.clone();
        let declared_children = std::mem::take(&mut node.children);
        let incoming_parent = node.parent.clone();

        let adopted_parent = if let Some(existing) = self.nodes.get_mut(&id) {
            if existing.span.is_none() {
                existing.span = node.span;
            }
            let adopted = existing.parent.is_none() && incoming_parent.is_some();
            if adopted {
                existing.parent = incoming_parent;
            }
            for edge in node.edges {
                if !existing.edges.contains(&edge) {
                    existing.edges.push(edge);
                }
            }
            for (key, value) in node.annotations {
                existing.annotations.entry(key).or_insert(value);
            }
            for tag in node.tags {
                if !existing.tags.contains(&tag) {
                    existing.tags.push(tag);
                }
            }
            adopted
        } else {
            if node.parent.is_none()
                && let Some(parent) = self.pending_parents.remove(&id)
            {
                node.parent = Some(parent);
            }
            if node.parent.is_none() {
                self.roots.push(id.clone());
            }
            self.nodes.insert(id.clone(), node);
            if let Some(waiting) = self.pending_children.remove(&id) {
                for child in waiting {
                    self.record_child(&id, &child);
                }
            }
            false
        };

        if let Some(parent) = self.nodes.get(&id).and_then(|n| n.parent.clone()) {
            self.record_child(&parent, &id);
        }
        if adopted_parent {
            self.roots.retain(|root| root != &id);
        }
        for child in declared_children {
            self.record_child(&id, &child);
        }
        // A children list can name a node that is inserted later, with no parent
        // of its own. Point that node back so the two fields cannot diverge.
        self.adopt_parent(&id);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant merge as merge()
    participant clone as clone
    participant std__mem__take as std::mem::take
    participant get_mut as get_mut
    participant is_none as is_none
    participant is_some as is_some
    participant contains as contains
    participant push as push
    participant entry as entry
    participant or_insert as or_insert
    participant remove as remove
    participant Some as Some
    participant insert as insert
    participant record_child as record_child
    participant get as get
    participant and_then as and_then
    participant retain as retain
    participant adopt_parent as adopt_parent
    merge-->>clone: node.id.clone()
    merge-->>std__mem__take: std::mem::take(&mut node.children)
    merge-->>clone: node.parent.clone()
    merge-->>get_mut: self.nodes.get_mut(&id)
    alt let Some(existing) = self.nodes.get_mut(&id)
    merge-->>is_none: existing.span.is_none()
    merge-->>is_none: existing.parent.is_none()
    merge-->>is_some: incoming_parent.is_some()
    loop node.edges
    merge-->>contains: existing.edges.contains(&edge)
    alt !existing.edges.contains(&edge)
    merge-->>push: existing.edges.push(edge)
    end
    end
    loop node.annotations
    merge-->>entry: existing.annotations.entry(key)
    merge-->>or_insert: existing.annotations.entry(key).or_insert(value)
    end
    loop node.tags
    merge-->>contains: existing.tags.contains(&tag)
    alt !existing.tags.contains(&tag)
    merge-->>push: existing.tags.push(tag)
    end
    end
    else else
    merge-->>is_none: node.parent.is_none()
    merge-->>remove: self.pending_parents.remove(&id)
    alt node.parent.is_none() && let Some(parent) = self.pending_parents.remo...
    merge-->>Some: Some(parent)
    end
    merge-->>is_none: node.parent.is_none()
    alt node.parent.is_none()
    merge-->>clone: id.clone()
    merge-->>push: self.roots.push(id.clone())
    end
    merge-->>clone: id.clone()
    merge-->>insert: self.nodes.insert(id.clone(), node)
    merge-->>remove: self.pending_children.remove(&id)
    alt let Some(waiting) = self.pending_children.remove(&id)
    loop waiting
    merge-->>record_child: self.record_child(&id, &child)
    end
    end
    end
    merge-->>get: self.nodes.get(&id)
    merge-->>clone: n.parent.clone()
    merge-->>and_then: self.nodes.get(&id).and_then(|n| n.parent.clone())
    alt let Some(parent) = self.nodes.get(&id).and_then(|n| n.parent.clone())
    merge-->>record_child: self.record_child(&parent, &id)
    end
    alt adopted_parent
    merge-->>retain: self.roots.retain(|root| root != &id)
    end
    loop declared_children
    merge-->>record_child: self.record_child(&id, &child)
    end
    merge-->>adopt_parent: self.adopt_parent(&id)
```

30 steps · 0 observed · 30 possible only

**Edges**

- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](index.md)
- calls → [`robby_ir::model::impl Document::record_child`](index.md)
- calls → [`robby_ir::model::impl Document::get`](index.md)
- calls → [`robby_ir::model::impl Document::record_child`](index.md)
- calls → [`robby_ir::model::impl Document::record_child`](index.md)
- calls → [`robby_ir::model::impl Document::adopt_parent`](index.md)

### `adopt_parent`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn adopt_parent(2)` |
| location | [`crates/robby-ir/src/model.rs:460`](../../../../crates/robby-ir/src/model.rs#L460) |
| provenance | extractor · confidence 1.00 |

> If some node already lists `id` and `id` has no parent, adopt that link.

```rust
/// If some node already lists `id` and `id` has no parent, adopt that link.
    fn adopt_parent(&mut self, id: &NodeId) {
        if self.nodes.get(id).is_some_and(|node| node.parent.is_some()) {
            return;
        }
        let listed = self
            .nodes
            .iter()
            .find(|(parent_id, node)| *parent_id != id && node.children.contains(id))
            .map(|(parent_id, _)| parent_id.clone());
        if let Some(parent) = listed {
            self.record_child(&parent, id);
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant adopt_parent as adopt_parent()
    participant get as get
    participant is_some as is_some
    participant is_some_and as is_some_and
    participant iter as iter
    participant contains as contains
    participant find as find
    participant clone as clone
    participant map as map
    participant record_child as record_child
    adopt_parent-->>get: self.nodes.get(id)
    adopt_parent-->>is_some: node.parent.is_some()
    adopt_parent-->>is_some_and: self.nodes.get(id).is_some_and(|node| node.parent.is_some())
    alt self.nodes.get(id).is_some_and(|node| node.parent.is_some())
    adopt_parent-->>adopt_parent: return
    end
    adopt_parent-->>iter: self .nodes .iter()
    adopt_parent-->>contains: node.children.contains(id)
    adopt_parent-->>find: self .nodes .iter() .find(|(parent_id, node)| *parent_id != id && nod...
    adopt_parent-->>clone: parent_id.clone()
    adopt_parent-->>map: self .nodes .iter() .find(|(parent_id, node)| *parent_id != id && nod...
    alt let Some(parent) = listed
    adopt_parent-->>record_child: self.record_child(&parent, id)
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_ir::model::impl Document::record_child`](index.md)

### `record_child`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn record_child(3)` |
| location | [`crates/robby-ir/src/model.rs:475`](../../../../crates/robby-ir/src/model.rs#L475) |
| provenance | extractor · confidence 1.00 |

> Record `child` under `parent`, and point the child back if it has no parent yet.

```rust
/// Record `child` under `parent`, and point the child back if it has no parent yet.
    fn record_child(&mut self, parent: &NodeId, child: &NodeId) {
        if parent == child {
            return;
        }
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            if !parent_node.children.contains(child) {
                parent_node.children.push(child.clone());
            }
        } else {
            let pending = self.pending_children.entry(parent.clone()).or_default();
            if !pending.contains(child) {
                pending.push(child.clone());
            }
        }
        if !self.nodes.contains_key(child) {
            self.pending_parents
                .entry(child.clone())
                .or_insert_with(|| parent.clone());
        }
        let needs_parent = self
            .nodes
            .get(child)
            .is_some_and(|node| node.parent.is_none());
        if needs_parent {
            if let Some(node) = self.nodes.get_mut(child) {
                node.parent = Some(parent.clone());
            }
            self.roots.retain(|root| root != child);
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant record_child as record_child()
    participant get_mut as get_mut
    participant contains as contains
    participant clone as clone
    participant push as push
    participant entry as entry
    participant or_default as or_default
    participant contains_key as contains_key
    participant or_insert_with as or_insert_with
    participant get as get
    participant is_none as is_none
    participant is_some_and as is_some_and
    participant Some as Some
    participant retain as retain
    alt parent == child
    record_child-->>record_child: return
    end
    record_child-->>get_mut: self.nodes.get_mut(parent)
    alt let Some(parent_node) = self.nodes.get_mut(parent)
    record_child-->>contains: parent_node.children.contains(child)
    alt !parent_node.children.contains(child)
    record_child-->>clone: child.clone()
    record_child-->>push: parent_node.children.push(child.clone())
    end
    else else
    record_child-->>clone: parent.clone()
    record_child-->>entry: self.pending_children.entry(parent.clone())
    record_child-->>or_default: self.pending_children.entry(parent.clone()).or_default()
    record_child-->>contains: pending.contains(child)
    alt !pending.contains(child)
    record_child-->>clone: child.clone()
    record_child-->>push: pending.push(child.clone())
    end
    end
    record_child-->>contains_key: self.nodes.contains_key(child)
    alt !self.nodes.contains_key(child)
    record_child-->>clone: child.clone()
    record_child-->>entry: self.pending_parents .entry(child.clone())
    record_child-->>clone: parent.clone()
    record_child-->>or_insert_with: self.pending_parents .entry(child.clone()) .or_insert_with(|| parent....
    end
    record_child-->>get: self .nodes .get(child)
    record_child-->>is_none: node.parent.is_none()
    record_child-->>is_some_and: self .nodes .get(child) .is_some_and(|node| node.parent.is_none())
    alt needs_parent
    record_child-->>get_mut: self.nodes.get_mut(child)
    alt let Some(node) = self.nodes.get_mut(child)
    record_child-->>clone: parent.clone()
    record_child-->>Some: Some(parent.clone())
    end
    record_child-->>retain: self.roots.retain(|root| root != child)
    end
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::get`](index.md)

### `get`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn get(2)` |
| location | [`crates/robby-ir/src/model.rs:507`](../../../../crates/robby-ir/src/model.rs#L507) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn get(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant get as get()
    get-->>get: self.nodes.get(id)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](index.md)

### `subtree`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn subtree(2)` |
| location | [`crates/robby-ir/src/model.rs:511`](../../../../crates/robby-ir/src/model.rs#L511) |
| provenance | extractor · confidence 1.00 |

> All nodes under `root`, depth-first, deterministic order.

```rust
/// All nodes under `root`, depth-first, deterministic order.
    pub fn subtree(&self, root: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        self.walk(root, &mut out);
        out
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant subtree as subtree()
    participant Vec__new as Vec::new
    participant walk as walk
    subtree-->>Vec__new: Vec::new()
    subtree-->>walk: self.walk(root, &mut out)
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::new`](index.md)
- calls → [`robby_ir::model::impl Document::walk`](index.md)

### `walk`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn walk(3)` |
| location | [`crates/robby-ir/src/model.rs:518`](../../../../crates/robby-ir/src/model.rs#L518) |
| provenance | extractor · confidence 1.00 |

```rust
fn walk<'a>(&'a self, id: &NodeId, out: &mut Vec<&'a Node>) {
        if let Some(node) = self.nodes.get(id) {
            out.push(node);
            for child in &node.children {
                self.walk(child, out);
            }
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant walk as walk()
    participant get as get
    participant push as push
    walk-->>get: self.nodes.get(id)
    alt let Some(node) = self.nodes.get(id)
    walk-->>push: out.push(node)
    loop &node.children
    walk-->>walk: self.walk(child, out)
    end
    end
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::walk`](index.md)

