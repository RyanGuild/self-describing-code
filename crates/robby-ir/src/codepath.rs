//! Entrypoint-rooted call reachability.
//!
//! A [`CodePath`] is the stored form of "drill by code path": the BFS tree of
//! resolved [`crate::EdgeKind::Calls`] edges from an entrypoint. Diagrams and
//! the Prezi entrypoints org are views over these paths, never the other way
//! around.

use std::collections::{BTreeSet, VecDeque};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::kind;
use crate::model::{Document, EdgeKind, NodeId};

/// Default BFS depth from an entrypoint (entry = depth 0).
pub const DEFAULT_MAX_DEPTH: u32 = 6;

/// One hop in an entrypoint call tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CodePathEdge {
    pub from: NodeId,
    pub to: NodeId,
    /// Depth of [`Self::to`] from the path entry (entry itself is 0).
    pub depth: u32,
    #[serde(default = "default_confidence", skip_serializing_if = "is_one")]
    pub confidence: f32,
}

fn default_confidence() -> f32 {
    1.0
}

fn is_one(v: &f32) -> bool {
    (*v - 1.0).abs() < f32::EPSILON
}

/// Reachability tree rooted at one entrypoint function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CodePath {
    /// Entrypoint node (typically a `main`).
    pub entry: NodeId,
    /// BFS-ordered tree edges within this path. Shared callees may appear in
    /// multiple paths; consumers that need a single tree apply first-parent wins.
    pub edges: Vec<CodePathEdge>,
    pub max_depth: u32,
}

impl CodePath {
    /// Build one path from `entry` over resolved `Calls` edges.
    pub fn from_entry(doc: &Document, entry: &NodeId, max_depth: u32) -> Option<Self> {
        if doc.get(entry).is_none() {
            return None;
        }

        let mut edges = Vec::new();
        let mut placed = BTreeSet::new();
        placed.insert(entry.clone());

        let mut queue: VecDeque<(NodeId, u32)> = VecDeque::new();
        queue.push_back((entry.clone(), 0));

        while let Some((from_id, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }
            let Some(from) = doc.get(&from_id) else {
                continue;
            };

            let mut callees: Vec<(NodeId, f32)> = from
                .edges
                .iter()
                .filter(|e| e.kind == EdgeKind::Calls && e.resolved)
                .filter(|e| doc.get(&e.to).is_some())
                .map(|e| (e.to.clone(), e.confidence))
                .collect();
            callees.sort_by(|(a, _), (b, _)| {
                let fa = doc.get(a).map(|n| n.fqn.as_str()).unwrap_or("");
                let fb = doc.get(b).map(|n| n.fqn.as_str()).unwrap_or("");
                fa.cmp(fb).then_with(|| a.as_str().cmp(b.as_str()))
            });
            callees.dedup_by(|(a, _), (b, _)| a == b);

            for (to, confidence) in callees {
                if to == from_id || !placed.insert(to.clone()) {
                    continue;
                }
                let child_depth = depth + 1;
                edges.push(CodePathEdge {
                    from: from_id.clone(),
                    to: to.clone(),
                    depth: child_depth,
                    confidence,
                });
                queue.push_back((to, child_depth));
            }
        }

        Some(Self {
            entry: entry.clone(),
            edges,
            max_depth,
        })
    }
}

/// Functions named `main` (MVP entrypoint set), sorted by FQN.
pub fn find_entrypoints(doc: &Document) -> Vec<NodeId> {
    let mut entries: Vec<&crate::Node> = doc
        .nodes
        .values()
        .filter(|node| {
            node.name == "main" && node.kind.as_str() == kind::code::FUNCTION
        })
        .collect();
    entries.sort_by(|a, b| a.fqn.cmp(&b.fqn).then_with(|| a.id.as_str().cmp(b.id.as_str())));
    entries.into_iter().map(|n| n.id.clone()).collect()
}

/// Build a [`CodePath`] for every discovered entrypoint.
pub fn build_code_paths(doc: &Document, max_depth: u32) -> Vec<CodePath> {
    find_entrypoints(doc)
        .iter()
        .filter_map(|entry| CodePath::from_entry(doc, entry, max_depth))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Edge, Node};

    fn sample_doc() -> Document {
        let mut doc = Document::new("test");
        let mut main = Node::new(
            NodeId::symbol("rust", "demo::main"),
            kind::code::FUNCTION,
            "demo::main",
        );
        main.edges
            .push(Edge::new(NodeId::symbol("rust", "demo::a"), EdgeKind::Calls));
        doc.insert(main);

        let mut a = Node::new(
            NodeId::symbol("rust", "demo::a"),
            kind::code::FUNCTION,
            "demo::a",
        );
        a.edges
            .push(Edge::new(NodeId::symbol("rust", "demo::b"), EdgeKind::Calls));
        doc.insert(a);

        doc.insert(Node::new(
            NodeId::symbol("rust", "demo::b"),
            kind::code::FUNCTION,
            "demo::b",
        ));
        doc
    }

    #[test]
    fn builds_path_from_main() {
        let doc = sample_doc();
        let paths = build_code_paths(&doc, DEFAULT_MAX_DEPTH);
        assert_eq!(paths.len(), 1);
        let path = &paths[0];
        assert_eq!(path.entry, NodeId::symbol("rust", "demo::main"));
        assert_eq!(path.edges.len(), 2);
        assert_eq!(path.edges[0].to, NodeId::symbol("rust", "demo::a"));
        assert_eq!(path.edges[0].depth, 1);
        assert_eq!(path.edges[1].to, NodeId::symbol("rust", "demo::b"));
        assert_eq!(path.edges[1].depth, 2);
    }

    #[test]
    fn respects_max_depth() {
        let doc = sample_doc();
        let path = CodePath::from_entry(&doc, &NodeId::symbol("rust", "demo::main"), 1)
            .expect("path");
        assert_eq!(path.edges.len(), 1);
        assert_eq!(path.edges[0].to, NodeId::symbol("rust", "demo::a"));
    }
}
