//! Cross-module references and backlinks.

use std::collections::{BTreeMap, BTreeSet};

use robby_ir::{Document, EdgeKind, Node, NodeId, kind};

use crate::util::{edge_label, is_container};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkGraph {
    pub outbound: Vec<OutboundRef>,
    pub inbound: Vec<InboundRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundRef {
    pub kind: EdgeKind,
    pub kind_label: &'static str,
    pub module_id: NodeId,
    pub module_fqn: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundRef {
    pub kinds: Vec<&'static str>,
    pub caller_id: NodeId,
    pub caller_fqn: String,
}

impl LinkGraph {
    pub fn for_node(doc: &Document, root: &Node) -> Self {
        Self {
            outbound: outbound(doc, root),
            inbound: inbound(doc, root),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.outbound.is_empty() && self.inbound.is_empty()
    }
}

fn outbound(doc: &Document, root: &Node) -> Vec<OutboundRef> {
    let scope = subtree_ids(doc, root);
    let mut by_module: BTreeMap<NodeId, (EdgeKind, BTreeSet<String>)> = BTreeMap::new();

    for node in doc.subtree(&root.id) {
        for edge in &node.edges {
            let Some(target) = doc.get(&edge.to) else {
                continue;
            };
            if scope.contains(&target.id) {
                continue;
            }
            let Some(target_mod) = owning_module(doc, target) else {
                continue;
            };
            if scope.contains(&target_mod.id) {
                continue;
            }
            let entry = by_module
                .entry(target_mod.id.clone())
                .or_insert((edge.kind, BTreeSet::new()));
            entry.1.insert(target.name.clone());
        }
    }

    by_module
        .into_iter()
        .filter_map(|(module_id, (kind, items))| {
            let module = doc.get(&module_id)?;
            Some(OutboundRef {
                kind,
                kind_label: edge_label(kind),
                module_id,
                module_fqn: module.fqn.clone(),
                items: items.into_iter().collect(),
            })
        })
        .collect()
}

fn inbound(doc: &Document, root: &Node) -> Vec<InboundRef> {
    let scope = subtree_ids(doc, root);
    let mut callers: BTreeMap<NodeId, BTreeSet<&'static str>> = BTreeMap::new();

    for node in doc.nodes.values() {
        if scope.contains(&node.id) {
            continue;
        }
        for edge in &node.edges {
            if !scope.contains(&edge.to) {
                continue;
            }
            callers
                .entry(node.id.clone())
                .or_default()
                .insert(edge_label(edge.kind));
        }
    }

    callers
        .into_iter()
        .filter_map(|(caller_id, kinds)| {
            let caller = doc.get(&caller_id)?;
            Some(InboundRef {
                kinds: kinds.into_iter().collect(),
                caller_id,
                caller_fqn: caller.fqn.clone(),
            })
        })
        .collect()
}

fn owning_module<'a>(doc: &'a Document, node: &'a Node) -> Option<&'a Node> {
    if is_container(node) {
        return Some(node);
    }
    let mut current = node.parent.as_ref().and_then(|id| doc.get(id));
    while let Some(parent) = current {
        if is_container(parent) {
            return Some(parent);
        }
        current = parent.parent.as_ref().and_then(|id| doc.get(id));
    }
    None
}

fn subtree_ids(doc: &Document, root: &Node) -> BTreeSet<NodeId> {
    doc.subtree(&root.id)
        .into_iter()
        .map(|node| node.id.clone())
        .collect()
}

pub(super) fn is_module_kind(kind_name: &str) -> bool {
    matches!(
        kind_name,
        kind::code::MODULE | kind::code::CRATE | kind::code::PACKAGE
    )
}
