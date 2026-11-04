//! Entrypoint scene helpers over IR [`robby_ir::CodePath`] values.
//!
//! Discovery and per-entry BFS live in `robby_ir::codepath`. This module merges
//! those paths into a single containment tree (first parent wins) for the Prezi
//! entrypoints org.

use std::collections::{BTreeMap, BTreeSet};

use robby_ir::{CodePath, DEFAULT_MAX_DEPTH, Document, EdgeKind, NodeId, find_entrypoints};

/// Max call-tree depth from an entrypoint (entry = 0). Matches IR default.
pub const MAX_CALL_DEPTH: usize = DEFAULT_MAX_DEPTH as usize;

/// Containment tree over `Calls` / code paths: first parent wins; extras are cross edges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallTree {
    /// Entrypoint node ids (sorted).
    pub entries: Vec<NodeId>,
    /// Parent → ordered children in the tree.
    pub children: BTreeMap<NodeId, Vec<NodeId>>,
    /// Every node placed in the tree (entries + reachable callees).
    pub placed: BTreeSet<NodeId>,
    /// Resolved `Calls` between placed nodes that are not tree edges.
    pub cross_edges: Vec<(NodeId, NodeId)>,
}

impl CallTree {
    pub fn from_document(doc: &Document) -> Self {
        if !doc.code_paths.is_empty() {
            return Self::from_code_paths(doc, &doc.code_paths);
        }
        Self::from_entries(doc, &find_entrypoints(doc), MAX_CALL_DEPTH)
    }

    /// Merge IR code paths into one forest (first parent wins across entries).
    pub fn from_code_paths(doc: &Document, paths: &[CodePath]) -> Self {
        let mut children: BTreeMap<NodeId, Vec<NodeId>> = BTreeMap::new();
        let mut placed: BTreeSet<NodeId> = BTreeSet::new();
        let mut tree_edges: BTreeSet<(NodeId, NodeId)> = BTreeSet::new();
        let mut entries = Vec::new();

        for path in paths {
            if doc.get(&path.entry).is_none() {
                continue;
            }
            if placed.insert(path.entry.clone()) {
                entries.push(path.entry.clone());
                children.entry(path.entry.clone()).or_default();
            }
            for edge in &path.edges {
                if doc.get(&edge.from).is_none() || doc.get(&edge.to).is_none() {
                    continue;
                }
                if !placed.contains(&edge.from) {
                    // Orphan edge — skip until from is placed (paths are BFS-ordered).
                    continue;
                }
                if !placed.insert(edge.to.clone()) {
                    continue;
                }
                children
                    .entry(edge.from.clone())
                    .or_default()
                    .push(edge.to.clone());
                tree_edges.insert((edge.from.clone(), edge.to.clone()));
                children.entry(edge.to.clone()).or_default();
            }
        }

        let cross_edges = collect_cross_edges(doc, &placed, &tree_edges);

        Self {
            entries,
            children,
            placed,
            cross_edges,
        }
    }

    pub fn from_entries(doc: &Document, entries: &[NodeId], max_depth: usize) -> Self {
        let paths: Vec<CodePath> = entries
            .iter()
            .filter_map(|entry| CodePath::from_entry(doc, entry, max_depth as u32))
            .collect();
        Self::from_code_paths(doc, &paths)
    }

    pub fn children_of(&self, id: &NodeId) -> &[NodeId] {
        self.children.get(id).map_or(&[], std::vec::Vec::as_slice)
    }
}

fn collect_cross_edges(
    doc: &Document,
    placed: &BTreeSet<NodeId>,
    tree_edges: &BTreeSet<(NodeId, NodeId)>,
) -> Vec<(NodeId, NodeId)> {
    let mut seen = BTreeSet::new();
    let mut cross = Vec::new();
    for from_id in placed {
        let Some(from) = doc.get(from_id) else {
            continue;
        };
        for edge in &from.edges {
            if edge.kind != EdgeKind::Calls || !edge.resolved {
                continue;
            }
            if !placed.contains(&edge.to) || edge.to == *from_id {
                continue;
            }
            let key = (from_id.clone(), edge.to.clone());
            if tree_edges.contains(&key) || !seen.insert(key.clone()) {
                continue;
            }
            cross.push(key);
        }
    }
    cross.sort_by(|a, b| {
        a.0.as_str()
            .cmp(b.0.as_str())
            .then_with(|| a.1.as_str().cmp(b.1.as_str()))
    });
    cross
}

#[cfg(test)]
mod tests {
    use super::*;
    use robby_ir::{Edge, Node, kind};

    fn doc_with_mains() -> Document {
        let mut doc = Document::new("test");
        let mut main = Node::new(
            NodeId::symbol("rust", "demo::main"),
            kind::code::FUNCTION,
            "demo::main",
        );
        main.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::a"),
            EdgeKind::Calls,
        ));
        doc.insert(main);

        let mut a = Node::new(
            NodeId::symbol("rust", "demo::a"),
            kind::code::FUNCTION,
            "demo::a",
        );
        a.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::b"),
            EdgeKind::Calls,
        ));
        doc.insert(a);

        doc.insert(Node::new(
            NodeId::symbol("rust", "demo::b"),
            kind::code::FUNCTION,
            "demo::b",
        ));

        let mut other = Node::new(
            NodeId::symbol("rust", "demo::other"),
            kind::code::FUNCTION,
            "demo::other",
        );
        other.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::b"),
            EdgeKind::Calls,
        ));
        doc.insert(other);

        doc.rebuild_code_paths();
        doc
    }

    #[test]
    fn prefers_document_code_paths() {
        let doc = doc_with_mains();
        let no_paths: [robby_ir::CodePath; 0] = [];
        assert_ne!(doc.code_paths, no_paths);
        let tree = CallTree::from_document(&doc);
        assert_eq!(tree.entries, vec![NodeId::symbol("rust", "demo::main")]);
        assert_eq!(
            tree.children_of(&NodeId::symbol("rust", "demo::main")),
            &[NodeId::symbol("rust", "demo::a")]
        );
    }

    #[test]
    fn first_parent_wins_shared_callee() {
        let doc = doc_with_mains();
        let entries = vec![
            NodeId::symbol("rust", "demo::main"),
            NodeId::symbol("rust", "demo::other"),
        ];
        let tree = CallTree::from_entries(&doc, &entries, MAX_CALL_DEPTH);

        assert_eq!(
            tree.children_of(&NodeId::symbol("rust", "demo::a")),
            &[NodeId::symbol("rust", "demo::b")]
        );
        assert!(
            tree.cross_edges
                .iter()
                .any(|(f, t)| f == &NodeId::symbol("rust", "demo::other")
                    && t == &NodeId::symbol("rust", "demo::b"))
        );
        assert_eq!(tree.children_of(&NodeId::symbol("rust", "demo::other")), []);
    }
}
