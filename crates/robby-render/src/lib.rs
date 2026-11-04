//! Deterministic views and renderers over Robby IR.
//!
//! - **Views** ([`view`]) — format-agnostic graph/process/type projections.
//! - **Renderers** ([`render`]) — turn one view into Mermaid, Markdown, or TUI text.
//! - **Reports** ([`report`]) — many rendered views assembled into a static artifact.
//!
//! Running any path twice on the same document must produce byte-identical
//! output: generated diagrams and pages are committed and diffed in CI.

pub mod improve_loop;
pub mod ingest;
pub mod markdown;
pub mod metric_test;
pub mod pipeline;
pub mod render;
pub mod report;
pub mod runs;
pub mod tree;
pub mod tui;
pub mod util;
pub mod view;
pub mod web;

pub use improve_loop::{
    ImproveLoopResult, link_metric_to_improve, spawn_improve_integrate, write_improve_run,
    write_integrate_run,
};
pub use ingest::{CursorAgentStub, CursorEvent, ingest_cursor_agents, ingest_cursor_json};
pub use metric_test::{
    MetricTestRequest, allow_and_spawn_metric_test, build_metric_test_run, write_metric_test_run,
};
pub use render::{MarkdownRenderer, MermaidRenderer, Renderer, ShowOptions, TuiRenderer};
pub use report::{Report, Section};
pub use robby_ir::RunRecord;
pub use runs::{
    LiveRunSourcesFile, RunIndexEntry, RunsIndex, ScoredDecisionProduct, SiteRunsStore,
    ensure_runs_layout, mint_run_id, resolve_live_run_sources, review_metrics, run_file_stem,
    synthetic_extract_run, web_build_run, write_live_run_sources, write_runs, write_site_runs,
};
pub use view::{
    CouplingGraph, DependencyViews, LinkGraph, NodeViews, PackageStructure, ProcessView, Scene,
    SelectOpts, SiblingGraph, TypeModel, collect, is_binary_module, main_process,
};
pub use web::{FrameDetail, write_site};

use robby_ir::{Document, NodeId, Process, Step};

use view::ProcessView as ProcView;

/// Render a process as a Mermaid `sequenceDiagram`.
pub fn sequence(process: &Process) -> String {
    MermaidRenderer.process(&ProcView::from_process(process))
}

/// Legacy sibling inventory flowchart.
pub fn flowchart(doc: &Document, root: &NodeId) -> String {
    MermaidRenderer.sibling_graph(&SiblingGraph::from_root(doc, root))
}

/// UML class diagram for a module's types.
pub fn class_diagram(doc: &Document, root: &NodeId) -> String {
    let model = TypeModel::from_module(doc, root);
    if model.is_empty() {
        return String::from("classDiagram\n");
    }
    MermaidRenderer.types(&model)
}

/// Nested package-structure digraph (Mermaid) for a module/crate root.
pub fn dependencies(doc: &Document, root: &NodeId) -> String {
    let graph = PackageStructure::from_root(doc, root);
    if graph.is_empty() {
        return String::from("flowchart TB\n");
    }
    MermaidRenderer.package_deps(&graph)
}

/// Internal call-coupling digraph (Mermaid) for a module/crate root.
pub fn coupling(doc: &Document, root: &NodeId) -> String {
    let graph = CouplingGraph::from_root(doc, root);
    if graph.is_empty() {
        return String::from("flowchart TB\n");
    }
    MermaidRenderer.coupling_deps(&graph)
}

/// All three dependency formalisms for a root, as a Markdown report fragment.
pub fn dependency_report(doc: &Document, root: &NodeId) -> String {
    let views = DependencyViews::from_root(doc, root);
    if views.is_empty() {
        return String::new();
    }
    let md = MarkdownRenderer::new();
    let mut report = Report::new();
    for section in md.dependency_sections(&views) {
        report.push(section);
    }
    report.to_markdown()
}

/// Workspace-wide dependency formalisms (crates).
pub fn workspace_dependencies(doc: &Document) -> String {
    let views = DependencyViews::workspace(doc);
    if views.is_empty() {
        return String::new();
    }
    let md = MarkdownRenderer::new();
    let mut report = Report::new();
    for section in md.dependency_sections(&views) {
        report.push(section);
    }
    report.to_markdown()
}

/// Stable Mermaid/layout id.
pub fn stable_id(input: &str) -> String {
    util::stable_id(input)
}

/// Every process in the document as Mermaid, keyed by node id.
pub fn all_sequences(doc: &Document) -> Vec<(NodeId, String)> {
    let mut out: Vec<(NodeId, String)> = doc
        .nodes
        .values()
        .filter_map(|node| {
            node.process
                .as_ref()
                .map(|p| (node.id.clone(), sequence(p)))
        })
        .collect();
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Compact one-liner for a step.
pub fn step_summary(step: &Step) -> String {
    format!("{} {} -> {}", step.kind.as_str(), step.from, step.to)
}
