//! Terminal show API — re-exports the TUI renderer.
//!
//! Prefer [`crate::TuiRenderer`] and [`crate::view::collect`] directly for new code.

pub use crate::render::tui::{ShowOptions, TuiRenderer};

use std::collections::BTreeMap;

use robby_ir::{Document, Node, NodeId, Process};

use crate::render::Renderer;
use crate::view::{HierarchyView, ProcessView, SiblingGraph, collect};

/// Render one node as a complete terminal report artifact.
pub fn show(
    doc: &Document,
    node: &Node,
    sources: Option<&BTreeMap<String, String>>,
    opts: &ShowOptions,
) -> String {
    TuiRenderer::new(opts.width).show(doc, node, sources, opts)
}

/// Box-drawing hierarchy.
pub fn tree(doc: &Document, root: &NodeId, opts: &ShowOptions) -> String {
    let view = HierarchyView::from_node(doc, root, opts.depth, opts.all);
    TuiRenderer::new(opts.width).hierarchy(&view)
}

/// Legacy sibling ASCII DAG.
pub fn graph(doc: &Document, root: &NodeId, opts: &ShowOptions) -> String {
    let view = SiblingGraph::from_root(doc, root);
    TuiRenderer::new(opts.width).sibling_graph(&view)
}

/// Process timeline.
pub fn timeline(process: &Process) -> String {
    TuiRenderer::default().process(&ProcessView::from_process(process))
}

/// Collect + render convenience.
pub fn show_collected(
    doc: &Document,
    node: &Node,
    sources: Option<&BTreeMap<String, String>>,
    opts: &ShowOptions,
) -> String {
    let views = collect(doc, node, sources, &opts.select_opts());
    TuiRenderer::new(opts.width).render_report(&views)
}
