//! Depth-limited ownership tree.

use robby_ir::{Document, Node, NodeId, kind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyView {
    pub root: HierarchyNode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyNode {
    pub id: NodeId,
    pub name: String,
    pub kind_tag: String,
    pub children: Vec<HierarchyNode>,
}

impl HierarchyView {
    pub fn from_node(doc: &Document, root: &NodeId, depth: usize, include_compact: bool) -> Self {
        let Some(node) = doc.get(root) else {
            return Self {
                root: HierarchyNode {
                    id: root.clone(),
                    name: String::new(),
                    kind_tag: String::new(),
                    children: Vec::new(),
                },
            };
        };
        Self {
            root: build(doc, node, depth, 0, include_compact),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }
}

fn build(
    doc: &Document,
    node: &Node,
    max_depth: usize,
    depth: usize,
    include_compact: bool,
) -> HierarchyNode {
    let mut children = Vec::new();
    if depth < max_depth {
        for child_id in &node.children {
            let Some(child) = doc.get(child_id) else {
                continue;
            };
            if !include_compact && is_compact(child.kind.as_str()) {
                continue;
            }
            children.push(build(doc, child, max_depth, depth + 1, include_compact));
        }
    } else {
        let remaining = node
            .children
            .iter()
            .filter_map(|id| doc.get(id))
            .filter(|child| include_compact || !is_compact(child.kind.as_str()))
            .count();
        if remaining > 0 {
            children.push(HierarchyNode {
                id: NodeId::from(format!("…{} more", remaining)),
                name: format!("…{remaining} more"),
                kind_tag: String::new(),
                children: Vec::new(),
            });
        }
    }

    HierarchyNode {
        id: node.id.clone(),
        name: node.name.clone(),
        kind_tag: kind_tag(&node.kind).to_owned(),
        children,
    }
}

fn is_compact(kind_name: &str) -> bool {
    matches!(
        kind_name,
        kind::code::FIELD | kind::code::VARIANT | kind::code::IMPORT
    )
}

fn kind_tag(kind: &robby_ir::Kind) -> &'static str {
    match kind.as_str() {
        kind::code::CRATE => "crate",
        kind::code::PACKAGE => "package",
        kind::code::MODULE => "mod",
        kind::code::STRUCT => "struct",
        kind::code::CLASS => "class",
        kind::code::ENUM => "enum",
        kind::code::TRAIT => "trait",
        kind::code::INTERFACE => "interface",
        kind::code::IMPL => "impl",
        kind::code::FUNCTION | kind::code::METHOD => "fn",
        kind::code::CONST => "const",
        kind::code::FIELD => "field",
        kind::code::VARIANT => "variant",
        kind::code::IMPORT => "use",
        _ => "node",
    }
}
