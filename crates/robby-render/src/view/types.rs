//! UML-style type model for a single module.

use std::collections::{BTreeMap, BTreeSet};

use robby_ir::{Document, EdgeKind, Node, NodeId, kind};

use crate::util::{class_id, is_type, strip_generics, text};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeModel {
    pub classes: Vec<ClassView>,
    pub relations: Vec<TypeRelation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassView {
    /// Sanitized identifier safe for Mermaid class names.
    pub id: String,
    pub name: String,
    pub stereotype: Option<ClassStereotype>,
    pub members: Vec<ClassMember>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassStereotype {
    Interface,
    Enumeration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassMember {
    Field(String),
    Method(String),
    Variant(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeRelation {
    pub from_id: String,
    pub to_id: String,
    pub kind: RelationKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelationKind {
    Implements,
    Extends,
}

impl TypeModel {
    pub fn from_module(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self {
                classes: Vec::new(),
                relations: Vec::new(),
            };
        };

        let children: Vec<&Node> = root_node
            .children
            .iter()
            .filter_map(|id| doc.get(id))
            .collect();

        let types: Vec<&Node> = children
            .iter()
            .copied()
            .filter(|node| is_type(node))
            .collect();
        let impls: Vec<&Node> = children
            .iter()
            .copied()
            .filter(|node| node.kind.as_str() == kind::code::IMPL)
            .collect();

        let type_by_name: BTreeMap<String, &Node> = types
            .iter()
            .map(|node| (strip_generics(&node.name), *node))
            .collect();

        let mut classes: Vec<ClassView> = Vec::new();
        let mut emitted: BTreeSet<String> = BTreeSet::new();
        let mut relations: Vec<TypeRelation> = Vec::new();

        for node in &types {
            let id = class_id(&node.name);
            if !emitted.insert(id.clone()) {
                continue;
            }
            classes.push(class_from_node(doc, node, id));
        }

        for impl_node in &impls {
            let Some((trait_name, self_name)) = parse_impl_target(impl_node) else {
                continue;
            };
            let self_key = strip_generics(&self_name);
            let Some(target) = type_by_name.get(&self_key).copied() else {
                continue;
            };
            let target_id = class_id(&target.name);

            match trait_name {
                None => {
                    if let Some(class) = classes.iter_mut().find(|c| c.id == target_id) {
                        for method in impl_methods(doc, impl_node) {
                            class.members.push(ClassMember::Method(method));
                        }
                    }
                }
                Some(trait_name) => {
                    let trait_id = class_id(&trait_name);
                    if emitted.insert(trait_id.clone()) {
                        classes.push(ClassView {
                            id: trait_id.clone(),
                            name: strip_generics(&trait_name),
                            stereotype: Some(ClassStereotype::Interface),
                            members: Vec::new(),
                        });
                    }
                    relations.push(TypeRelation {
                        from_id: target_id,
                        to_id: trait_id,
                        kind: RelationKind::Implements,
                    });
                }
            }
        }

        // Prefer explicit Implements / Extends edges (TypeScript classes, etc.).
        for node in &types {
            let from_id = class_id(&node.name);
            for edge in &node.edges {
                let kind = match edge.kind {
                    EdgeKind::Implements => RelationKind::Implements,
                    EdgeKind::Extends => RelationKind::Extends,
                    _ => continue,
                };
                let Some(target) = doc.get(&edge.to) else {
                    continue;
                };
                let to_id = class_id(&target.name);
                if emitted.insert(to_id.clone()) {
                    classes.push(class_from_node(doc, target, to_id.clone()));
                }
                relations.push(TypeRelation {
                    from_id: from_id.clone(),
                    to_id,
                    kind,
                });
            }
        }

        relations
            .sort_by(|a, b| (&a.from_id, &a.to_id, a.kind).cmp(&(&b.from_id, &b.to_id, b.kind)));
        relations.dedup();

        Self { classes, relations }
    }

    pub fn is_empty(&self) -> bool {
        self.classes.is_empty() && self.relations.is_empty()
    }
}

fn class_from_node(doc: &Document, node: &Node, id: String) -> ClassView {
    let stereotype = match node.kind.as_str() {
        kind::code::TRAIT | kind::code::INTERFACE => Some(ClassStereotype::Interface),
        kind::code::ENUM => Some(ClassStereotype::Enumeration),
        _ => None,
    };

    let mut members = Vec::new();
    match node.kind.as_str() {
        kind::code::STRUCT | kind::code::CLASS | kind::code::UNION => {
            for child_id in &node.children {
                let Some(child) = doc.get(child_id) else {
                    continue;
                };
                match child.kind.as_str() {
                    kind::code::FIELD => members.push(ClassMember::Field(text(&child.name))),
                    kind::code::METHOD => members.push(ClassMember::Method(text(&child.name))),
                    _ => {}
                }
            }
        }
        kind::code::ENUM => {
            for child_id in &node.children {
                let Some(variant) = doc.get(child_id) else {
                    continue;
                };
                if variant.kind.as_str() == kind::code::VARIANT {
                    members.push(ClassMember::Variant(text(&variant.name)));
                }
            }
        }
        kind::code::TRAIT | kind::code::INTERFACE => {
            for child_id in &node.children {
                let Some(method) = doc.get(child_id) else {
                    continue;
                };
                if method.kind.as_str() == kind::code::METHOD {
                    members.push(ClassMember::Method(text(&method.name)));
                }
            }
        }
        _ => {}
    }

    ClassView {
        id,
        name: node.name.clone(),
        stereotype,
        members,
    }
}

fn impl_methods(doc: &Document, impl_node: &Node) -> Vec<String> {
    let mut methods: Vec<String> = impl_node
        .children
        .iter()
        .filter_map(|id| doc.get(id))
        .filter(|node| node.kind.as_str() == kind::code::METHOD)
        .map(|node| text(&node.name))
        .collect();
    methods.sort();
    methods
}

fn parse_impl_target(node: &Node) -> Option<(Option<String>, String)> {
    let rest = node
        .fqn
        .rsplit_once("::impl ")
        .map(|(_, rest)| rest)
        .or_else(|| node.fqn.strip_prefix("impl "))
        .or_else(|| node.name.strip_prefix("impl "))?;

    if let Some((trait_part, self_part)) = rest.split_once(" for ") {
        Some((
            Some(trait_part.trim().to_owned()),
            self_part.trim().to_owned(),
        ))
    } else {
        Some((None, rest.trim().to_owned()))
    }
}
