//! Sibling inventory graph (legacy flowchart / old TUI graph).

use robby_ir::{Document, EdgeKind, Node, NodeId};

use crate::util::{edge_label, is_structural, short_kind, stable_id, text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingGraph {
    pub root_id: String,
    pub root_label: String,
    pub nodes: Vec<SiblingNode>,
    pub edges: Vec<SiblingEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingNode {
    pub id: NodeId,
    pub stable_id: String,
    pub label: String,
    pub name: String,
    pub kind_tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiblingEdge {
    pub from: String,
    pub to: String,
    pub label: &'static str,
    pub resolved: bool,
    pub kind: EdgeKind,
}

impl SiblingGraph {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self {
                root_id: String::new(),
                root_label: String::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
            };
        };

        let visible: Vec<&Node> = root_node
            .children
            .iter()
            .filter_map(|child| doc.get(child))
            .filter(|node| is_structural(&node.kind))
            .collect();

        let nodes: Vec<SiblingNode> = visible
            .iter()
            .map(|node| {
                let sig = node.signature.as_deref().unwrap_or(&node.name);
                SiblingNode {
                    id: node.id.clone(),
                    stable_id: stable_id(node.id.as_str()),
                    label: text(&format!("{}  [{}]", sig, short_kind(&node.kind))),
                    name: node.name.clone(),
                    kind_tag: short_kind(&node.kind).to_owned(),
                }
            })
            .collect();

        let ids: Vec<&NodeId> = visible.iter().map(|n| &n.id).collect();
        let mut edges: Vec<(String, String, EdgeKind, bool)> = Vec::new();
        for node in &visible {
            for edge in &node.edges {
                if !ids.contains(&&edge.to) {
                    continue;
                }
                edges.push((
                    stable_id(node.id.as_str()),
                    stable_id(edge.to.as_str()),
                    edge.kind,
                    edge.resolved,
                ));
            }
        }
        edges.sort();
        edges.dedup();

        let edges = edges
            .into_iter()
            .map(|(from, to, kind, resolved)| SiblingEdge {
                from,
                to,
                label: edge_label(kind),
                resolved,
                kind,
            })
            .collect();

        let sig = root_node.signature.as_deref().unwrap_or(&root_node.name);
        Self {
            root_id: stable_id(root.as_str()),
            root_label: text(&format!("{}  [{}]", sig, short_kind(&root_node.kind))),
            nodes,
            edges,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
