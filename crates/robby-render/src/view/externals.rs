//! External package imports (npm stubs, tagged packages).

use std::collections::{BTreeMap, BTreeSet};

use robby_ir::{Document, EdgeKind, Node, NodeId, kind};

use super::links::is_module_kind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalsView {
    pub entries: Vec<ExternalEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalEntry {
    pub package: String,
    pub spec: String,
    pub importers: Vec<String>,
}

impl ExternalsView {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let mut by_key: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();

        for node in doc.subtree(root) {
            if node.kind.as_str() != kind::code::IMPORT {
                continue;
            }
            let Some(spec) = import_spec(node) else {
                continue;
            };
            for edge in &node.edges {
                if edge.kind != EdgeKind::Imports {
                    continue;
                }
                let Some(target) = doc.get(&edge.to) else {
                    continue;
                };
                if !is_external_package(target) {
                    continue;
                }
                let package = if target.fqn.starts_with("npm/") {
                    target.fqn.trim_start_matches("npm/").to_owned()
                } else {
                    target.name.clone()
                };
                let importer = owning_container_fqn(doc, node).unwrap_or_else(|| node.fqn.clone());
                by_key
                    .entry((package, spec.clone()))
                    .or_default()
                    .insert(importer);
            }
        }

        let mut entries: Vec<ExternalEntry> = by_key
            .into_iter()
            .map(|((package, spec), importers)| ExternalEntry {
                package,
                spec,
                importers: importers.into_iter().collect(),
            })
            .collect();
        entries.sort_by(|a, b| (&a.package, &a.spec).cmp(&(&b.package, &b.spec)));
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn is_external_package(node: &Node) -> bool {
    if node.kind.as_str() != kind::code::PACKAGE {
        return false;
    }
    node.tags.iter().any(|t| t == "external")
        || node.annotations.get("external").map(|v| v.as_str()) == Some("true")
        || node.fqn.starts_with("npm/")
}

fn import_spec(node: &Node) -> Option<String> {
    let (_, rest) = node.fqn.split_once("import:")?;
    Some(rest.to_owned())
}

fn owning_container_fqn(doc: &Document, node: &Node) -> Option<String> {
    let mut current = node.parent.as_ref().and_then(|id| doc.get(id));
    while let Some(parent) = current {
        if is_module_kind(parent.kind.as_str()) {
            return Some(parent.fqn.clone());
        }
        current = parent.parent.as_ref().and_then(|id| doc.get(id));
    }
    None
}
