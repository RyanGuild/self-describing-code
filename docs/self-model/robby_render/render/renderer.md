# `robby_render::render::Renderer`

[Self-model](../../README.md) · [render](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-render/src/render/mod.rs:18`](../../../../crates/robby-render/src/render/mod.rs#L18) |
| provenance | extractor · confidence 1.00 |

> Transform one view into text.
> 
> Callers that need a full artifact assemble many of these into a [`Report`].

```rust
/// Transform one view into text.
///
/// Callers that need a full artifact assemble many of these into a [`Report`].
pub trait Renderer {
    fn meta(&self, meta: &NodeMeta) -> String;
    fn process(&self, view: &ProcessView) -> String;
    fn types(&self, view: &TypeModel) -> String;
    /// Nested source-structure package digraph + externals.
    fn package_deps(&self, view: &PackageStructure) -> String;
    /// Internal call coupling (modules as subgraphs, functions as nodes).
    fn coupling_deps(&self, view: &CouplingGraph) -> String;
    /// Design / dependency structure matrix over packages.
    fn dependency_matrix(&self, view: &PackageStructure) -> String;
    fn hierarchy(&self, view: &HierarchyView) -> String;
    fn externals(&self, view: &ExternalsView) -> String;
    fn comments(&self, view: &CommentsView) -> String;
    fn links(&self, view: &LinkGraph) -> String;
    fn sibling_graph(&self, view: &SiblingGraph) -> String;
    fn source(&self, view: &SourceSlice) -> String;

    /// Dependency formalisms as report sections (skips empty bodies).
    fn dependency_sections(&self, views: &DependencyViews) -> Vec<Section> {
        let mut sections = Vec::new();
        if !views.packages.is_empty() {
            let body = self.package_deps(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Package diagram", body));
            }
        }
        if !views.coupling.is_empty() {
            let body = self.coupling_deps(&views.coupling);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Module coupling", body));
            }
        }
        if !views.packages.is_empty() {
            let body = self.dependency_matrix(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Dependency matrix", body));
            }
        }
        sections
    }

    /// Build a report from the views selected for one node.
    fn report(&self, views: &NodeViews) -> Report {
        let mut report = Report::new();
        let meta = self.meta(&views.meta);
        if !meta.trim().is_empty() {
            report.push_body(meta);
        }
        if let Some(entry) = &views.entry {
            report.push(Section::titled("Entry", self.process(entry)));
        }
        if let Some(types) = &views.types {
            report.push(Section::titled("Types", self.types(types)));
        }
        if let Some(deps) = &views.dependencies {
            for section in self.dependency_sections(deps) {
                report.push(section);
            }
        }
        if let Some(externals) = &views.externals {
            let body = self.externals(externals);
            if !body.trim().is_empty() {
                report.push(Section::titled("Externals", body));
            }
        }
        if let Some(comments) = &views.comments {
            let body = self.comments(comments);
            if !body.trim().is_empty() {
                report.push(Section::titled("Comments", body));
            }
        }
        if let Some(process) = &views.process {
            report.push(Section::titled("Process", self.process(process)));
        }
        if let Some(hierarchy) = &views.hierarchy {
            report.push(Section::titled("Tree", self.hierarchy(hierarchy)));
        }
        if !views.links.is_empty() {
            report.push(Section::titled("Links", self.links(&views.links)));
        }
        if let Some(sibling) = &views.sibling {
            report.push(Section::titled("Graph", self.sibling_graph(sibling)));
        }
        if let Some(source) = &views.source {
            report.push(Section::titled("Source", self.source(source)));
        }
        report
    }
}
```

## Referenced by

- calls ← [`robby_render::class_diagram`](../class-diagram.md)
- calls ← [`robby_render::coupling`](../coupling.md)
- calls ← [`robby_render::dependencies`](../dependencies.md)
- calls ← [`robby_render::dependency_report`](../dependency-report.md)
- calls ← [`robby_render::flowchart`](../flowchart.md)
- calls ← [`robby_render::markdown::report`](../markdown/report.md)
- calls ← [`robby_render::render::tui::impl TuiRenderer::render_report`](tui/index.md)
- calls ← [`robby_render::test_render::externals_and_comments_sections_render`](../test_render/externals-and-comments-sections-render.md)
- calls ← [`robby_render::tree::module_page`](../tree/module-page.md)
- calls ← [`robby_render::tui::graph`](../tui/graph.md)
- calls ← [`robby_render::tui::tree`](../tui/tree.md)
- calls ← [`robby_render::workspace_dependencies`](../workspace-dependencies.md)

### `meta`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:22`](../../../../crates/robby-render/src/render/mod.rs#L22) |
| provenance | extractor · confidence 1.00 |

```rust
fn meta(&self, meta: &NodeMeta) -> String;
```

### `process`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:23`](../../../../crates/robby-render/src/render/mod.rs#L23) |
| provenance | extractor · confidence 1.00 |

```rust
fn process(&self, view: &ProcessView) -> String;
```

### `types`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:24`](../../../../crates/robby-render/src/render/mod.rs#L24) |
| provenance | extractor · confidence 1.00 |

```rust
fn types(&self, view: &TypeModel) -> String;
```

### `package_deps`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:25`](../../../../crates/robby-render/src/render/mod.rs#L25) |
| provenance | extractor · confidence 1.00 |

```rust
/// Nested source-structure package digraph + externals.
    fn package_deps(&self, view: &PackageStructure) -> String;
```

### `coupling_deps`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:27`](../../../../crates/robby-render/src/render/mod.rs#L27) |
| provenance | extractor · confidence 1.00 |

```rust
/// Internal call coupling (modules as subgraphs, functions as nodes).
    fn coupling_deps(&self, view: &CouplingGraph) -> String;
```

### `dependency_matrix`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:29`](../../../../crates/robby-render/src/render/mod.rs#L29) |
| provenance | extractor · confidence 1.00 |

```rust
/// Design / dependency structure matrix over packages.
    fn dependency_matrix(&self, view: &PackageStructure) -> String;
```

### `hierarchy`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:31`](../../../../crates/robby-render/src/render/mod.rs#L31) |
| provenance | extractor · confidence 1.00 |

```rust
fn hierarchy(&self, view: &HierarchyView) -> String;
```

### `externals`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:32`](../../../../crates/robby-render/src/render/mod.rs#L32) |
| provenance | extractor · confidence 1.00 |

```rust
fn externals(&self, view: &ExternalsView) -> String;
```

### `comments`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:33`](../../../../crates/robby-render/src/render/mod.rs#L33) |
| provenance | extractor · confidence 1.00 |

```rust
fn comments(&self, view: &CommentsView) -> String;
```

### `links`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:34`](../../../../crates/robby-render/src/render/mod.rs#L34) |
| provenance | extractor · confidence 1.00 |

```rust
fn links(&self, view: &LinkGraph) -> String;
```

### `sibling_graph`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:35`](../../../../crates/robby-render/src/render/mod.rs#L35) |
| provenance | extractor · confidence 1.00 |

```rust
fn sibling_graph(&self, view: &SiblingGraph) -> String;
```

### `source`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:36`](../../../../crates/robby-render/src/render/mod.rs#L36) |
| provenance | extractor · confidence 1.00 |

```rust
fn source(&self, view: &SourceSlice) -> String;
```

### `dependency_sections`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:38`](../../../../crates/robby-render/src/render/mod.rs#L38) |
| provenance | extractor · confidence 1.00 |

```rust
/// Dependency formalisms as report sections (skips empty bodies).
    fn dependency_sections(&self, views: &DependencyViews) -> Vec<Section> {
        let mut sections = Vec::new();
        if !views.packages.is_empty() {
            let body = self.package_deps(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Package diagram", body));
            }
        }
        if !views.coupling.is_empty() {
            let body = self.coupling_deps(&views.coupling);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Module coupling", body));
            }
        }
        if !views.packages.is_empty() {
            let body = self.dependency_matrix(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Dependency matrix", body));
            }
        }
        sections
    }
```

### `report`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-render/src/render/mod.rs:62`](../../../../crates/robby-render/src/render/mod.rs#L62) |
| provenance | extractor · confidence 1.00 |

```rust
/// Build a report from the views selected for one node.
    fn report(&self, views: &NodeViews) -> Report {
        let mut report = Report::new();
        let meta = self.meta(&views.meta);
        if !meta.trim().is_empty() {
            report.push_body(meta);
        }
        if let Some(entry) = &views.entry {
            report.push(Section::titled("Entry", self.process(entry)));
        }
        if let Some(types) = &views.types {
            report.push(Section::titled("Types", self.types(types)));
        }
        if let Some(deps) = &views.dependencies {
            for section in self.dependency_sections(deps) {
                report.push(section);
            }
        }
        if let Some(externals) = &views.externals {
            let body = self.externals(externals);
            if !body.trim().is_empty() {
                report.push(Section::titled("Externals", body));
            }
        }
        if let Some(comments) = &views.comments {
            let body = self.comments(comments);
            if !body.trim().is_empty() {
                report.push(Section::titled("Comments", body));
            }
        }
        if let Some(process) = &views.process {
            report.push(Section::titled("Process", self.process(process)));
        }
        if let Some(hierarchy) = &views.hierarchy {
            report.push(Section::titled("Tree", self.hierarchy(hierarchy)));
        }
        if !views.links.is_empty() {
            report.push(Section::titled("Links", self.links(&views.links)));
        }
        if let Some(sibling) = &views.sibling {
            report.push(Section::titled("Graph", self.sibling_graph(sibling)));
        }
        if let Some(source) = &views.source {
            report.push(Section::titled("Source", self.source(source)));
        }
        report
    }
```

