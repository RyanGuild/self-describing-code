//! Select which views apply to a node.

use std::collections::BTreeMap;

use robby_ir::{Document, Node, kind};

use super::comments::CommentsView;
use super::deps::DependencyViews;
use super::externals::ExternalsView;
use super::hierarchy::HierarchyView;
use super::links::{LinkGraph, is_module_kind};
use super::process::ProcessView;
use super::sibling::SiblingGraph;
use super::source::SourceSlice;
use super::types::TypeModel;

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

impl Default for SelectOpts {
    fn default() -> Self {
        Self {
            hierarchy: false,
            hierarchy_depth: 4,
            hierarchy_all: false,
            sibling: false,
            source: true,
            types: true,
            dependencies: true,
            externals: true,
            comments: true,
            links: true,
            entry: true,
            process: true,
        }
    }
}

impl SelectOpts {
    /// Markdown module/item page defaults.
    pub fn markdown() -> Self {
        Self::default()
    }

    /// TUI `show --view all` defaults.
    pub fn tui_all() -> Self {
        Self {
            hierarchy: true,
            ..Self::default()
        }
    }
}

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

pub fn is_binary_module(doc: &Document, node: &Node) -> bool {
    node.children
        .iter()
        .filter_map(|id| doc.get(id))
        .any(|child| child.name == "main" && child.kind.as_str() == kind::code::FUNCTION)
}

pub fn main_process<'a>(doc: &'a Document, node: &Node) -> Option<&'a robby_ir::Process> {
    node.children
        .iter()
        .filter_map(|id| doc.get(id))
        .find(|child| child.name == "main" && child.kind.as_str() == kind::code::FUNCTION)
        .and_then(|main| main.process.as_ref())
}
