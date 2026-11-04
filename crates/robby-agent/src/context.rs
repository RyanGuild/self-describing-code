//! Assemble comment + surrounding code windows from IR + sources.

use std::collections::BTreeMap;

use robby_ir::{Document, Node, NodeId, kind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentContext {
    pub parent_id: NodeId,
    pub comment_text: String,
    pub code_window: String,
    pub owner_fqn: Option<String>,
    pub file: String,
    pub location: Option<String>,
}

/// Collect enrichable comment/docs contexts under the whole document.
pub fn collect_comment_contexts(
    doc: &Document,
    sources: &BTreeMap<String, String>,
) -> Vec<CommentContext> {
    let mut out = Vec::new();

    for node in doc.nodes.values() {
        if node.kind.as_str() == kind::code::COMMENT {
            let text = node.docs.clone().unwrap_or_default();
            if text.is_empty() {
                continue;
            }
            if has_decision_child(doc, node) {
                continue;
            }
            out.push(build_for_comment(doc, sources, node, text));
            continue;
        }

        // Docs-only: item docs with no covering comment node already enriched.
        let Some(docs) = &node.docs else {
            continue;
        };
        if docs.is_empty() || has_decision_child(doc, node) {
            continue;
        }
        if node.kind.as_str() == kind::code::COMMENT {
            continue;
        }
        // Skip if a comment node already carries the same text under the same parent.
        if parent_has_comment_with_text(doc, node, docs) {
            continue;
        }
        out.push(build_for_docs_owner(doc, sources, node, docs.clone()));
    }

    out.sort_by(|a, b| {
        (
            a.file.as_str(),
            a.location.as_deref().unwrap_or(""),
            a.parent_id.as_str(),
        )
            .cmp(&(
                b.file.as_str(),
                b.location.as_deref().unwrap_or(""),
                b.parent_id.as_str(),
            ))
    });
    out
}

pub fn has_decision_child(doc: &Document, node: &Node) -> bool {
    node.children.iter().any(|id| {
        doc.get(id)
            .is_some_and(|c| c.kind.as_str() == kind::agent::DECISION)
    })
}

fn parent_has_comment_with_text(doc: &Document, node: &Node, docs: &str) -> bool {
    let Some(parent) = node.parent.as_ref().and_then(|id| doc.get(id)) else {
        return false;
    };
    parent.children.iter().any(|id| {
        doc.get(id).is_some_and(|c| {
            c.kind.as_str() == kind::code::COMMENT && c.docs.as_deref() == Some(docs)
        })
    })
}

fn build_for_comment(
    doc: &Document,
    sources: &BTreeMap<String, String>,
    comment: &Node,
    text: String,
) -> CommentContext {
    let (file, location) = file_location(comment);
    let owner = nearest_following_item(doc, comment);
    let code_window = code_window_for(doc, sources, comment, owner);
    CommentContext {
        parent_id: comment.id.clone(),
        comment_text: text,
        code_window,
        owner_fqn: owner.map(|n| n.fqn.clone()),
        file,
        location,
    }
}

fn build_for_docs_owner(
    doc: &Document,
    sources: &BTreeMap<String, String>,
    owner: &Node,
    text: String,
) -> CommentContext {
    let (file, location) = file_location(owner);
    let code_window = slice_node(sources, owner).unwrap_or_else(|| {
        line_window(sources, owner).unwrap_or_default()
    });
    let _ = doc;
    CommentContext {
        parent_id: owner.id.clone(),
        comment_text: text,
        code_window,
        owner_fqn: Some(owner.fqn.clone()),
        file,
        location,
    }
}

fn file_location(node: &Node) -> (String, Option<String>) {
    match &node.span {
        Some(span) => (span.file.clone(), Some(span.location())),
        None => (String::new(), None),
    }
}

fn code_window_for(
    _doc: &Document,
    sources: &BTreeMap<String, String>,
    comment: &Node,
    owner: Option<&Node>,
) -> String {
    if let Some(owner) = owner {
        if let Some(slice) = slice_node(sources, owner) {
            return slice;
        }
    }
    line_window(sources, comment).unwrap_or_default()
}

/// Nearest following sibling item (not comment/import), by span start.
pub fn nearest_following_item<'a>(doc: &'a Document, comment: &Node) -> Option<&'a Node> {
    let parent = comment.parent.as_ref().and_then(|id| doc.get(id))?;
    let comment_start = comment.span.as_ref()?.start.byte;
    let mut best: Option<(&Node, u32)> = None;
    for child_id in &parent.children {
        let Some(child) = doc.get(child_id) else {
            continue;
        };
        if matches!(
            child.kind.as_str(),
            kind::code::COMMENT | kind::code::IMPORT
        ) {
            continue;
        }
        let Some(span) = &child.span else {
            continue;
        };
        if span.start.byte < comment_start {
            continue;
        }
        match best {
            Some((_, byte)) if span.start.byte >= byte => {}
            _ => best = Some((child, span.start.byte)),
        }
    }
    best.map(|(n, _)| n)
}

fn slice_node(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    source
        .get(span.start.byte as usize..span.end.byte as usize)
        .map(|s| s.to_owned())
}

/// ±20 lines around the node's span.
fn line_window(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    let start_line = span.start.line.saturating_sub(1) as usize;
    let end_line = span.end.line as usize;
    let from = start_line.saturating_sub(20);
    let to = (end_line + 20).min(source.lines().count().saturating_sub(1).max(end_line));
    let mut out = String::new();
    for (i, line) in source.lines().enumerate() {
        if i < from {
            continue;
        }
        if i > to {
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
