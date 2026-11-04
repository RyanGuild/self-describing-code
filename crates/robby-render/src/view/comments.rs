//! Source comments, attached docs, and cached agent enrichment.

use std::collections::{BTreeMap, BTreeSet};

use robby_ir::{Document, Highlight, Node, NodeId, NoteKind, Verdict, kind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentsView {
    pub entries: Vec<CommentEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentEntry {
    pub location: Option<String>,
    pub kind: CommentKind,
    pub text: String,
    pub owner_fqn: Option<String>,
    pub code: Option<String>,
    pub verdict: Option<Verdict>,
    pub note_kind: Option<NoteKind>,
    pub note: Option<String>,
    /// Byte offsets into `code`.
    pub highlights: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentKind {
    Line,
    Block,
    Docs,
}

impl CommentsView {
    pub fn from_root(
        doc: &Document,
        root: &NodeId,
        sources: Option<&BTreeMap<String, String>>,
    ) -> Self {
        let mut entries = Vec::new();
        let mut covered_docs: BTreeSet<String> = BTreeSet::new();

        for node in doc.subtree(root) {
            if node.kind.as_str() != kind::code::COMMENT {
                continue;
            }
            let text = node.docs.clone().unwrap_or_default();
            if text.is_empty() {
                continue;
            }
            let kind = match node
                .annotations
                .get("comment_kind")
                .map(std::string::String::as_str)
            {
                Some("line") => CommentKind::Line,
                _ => CommentKind::Block,
            };
            covered_docs.insert(text.clone());
            let decision = decision_child(doc, node);
            let verdict = decision.and_then(parse_verdict);
            if verdict == Some(Verdict::Noise) {
                continue;
            }
            let owner = nearest_following_item(doc, node);
            let code = sources.and_then(|s| code_window(s, node, owner));
            let (note_kind, note, highlights) = decision.map_or((None, None, Vec::new()), |d| {
                enrichment_fields(d, code.as_deref())
            });
            entries.push(CommentEntry {
                location: node.span.as_ref().map(robby_ir::Span::location),
                kind,
                text,
                owner_fqn: owner.map(|n| n.fqn.clone()),
                code,
                verdict,
                note_kind,
                note,
                highlights,
            });
        }

        for node in doc.subtree(root) {
            if node.kind.as_str() == kind::code::COMMENT {
                continue;
            }
            let Some(docs) = &node.docs else {
                continue;
            };
            if docs.is_empty() || covered_docs.contains(docs) {
                continue;
            }
            let decision = decision_child(doc, node);
            let verdict = decision.and_then(parse_verdict);
            if verdict == Some(Verdict::Noise) {
                continue;
            }
            let code = sources.and_then(|s| slice_node(s, node));
            let (note_kind, note, highlights) = decision.map_or((None, None, Vec::new()), |d| {
                enrichment_fields(d, code.as_deref())
            });
            entries.push(CommentEntry {
                location: node.span.as_ref().map(robby_ir::Span::location),
                kind: CommentKind::Docs,
                text: docs.clone(),
                owner_fqn: Some(node.fqn.clone()),
                code,
                verdict,
                note_kind,
                note,
                highlights,
            });
        }

        entries.sort_by(|a, b| {
            (
                a.location.as_deref().unwrap_or(""),
                a.owner_fqn.as_deref().unwrap_or(""),
                a.text.as_str(),
            )
                .cmp(&(
                    b.location.as_deref().unwrap_or(""),
                    b.owner_fqn.as_deref().unwrap_or(""),
                    b.text.as_str(),
                ))
        });
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl CommentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Block => "block",
            Self::Docs => "docs",
        }
    }
}

fn decision_child<'a>(doc: &'a Document, node: &Node) -> Option<&'a Node> {
    node.children.iter().find_map(|id| {
        let child = doc.get(id)?;
        (child.kind.as_str() == kind::agent::DECISION).then_some(child)
    })
}

fn parse_verdict(decision: &Node) -> Option<Verdict> {
    Verdict::parse(decision.annotations.get("verdict")?.as_str())
}

fn enrichment_fields(
    decision: &Node,
    _code: Option<&str>,
) -> (Option<NoteKind>, Option<String>, Vec<(u32, u32)>) {
    let note_kind = decision
        .annotations
        .get("note_kind")
        .and_then(|s| NoteKind::parse(s));
    let note = decision.docs.clone();
    let highlights = decision
        .annotations
        .get("highlights")
        .and_then(|raw| serde_json::from_str::<Vec<Highlight>>(raw).ok())
        .unwrap_or_default()
        .into_iter()
        .filter(|h| h.start < h.end)
        .map(|h| (h.start, h.end))
        .collect();
    (note_kind, note, highlights)
}

fn nearest_following_item<'a>(doc: &'a Document, comment: &Node) -> Option<&'a Node> {
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

fn code_window(
    sources: &BTreeMap<String, String>,
    comment: &Node,
    owner: Option<&Node>,
) -> Option<String> {
    if let Some(owner) = owner
        && let Some(slice) = slice_node(sources, owner)
    {
        return Some(slice);
    }
    line_window(sources, comment)
}

fn slice_node(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    source
        .get(span.start.byte as usize..span.end.byte as usize)
        .map(std::borrow::ToOwned::to_owned)
}

fn line_window(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    let start_line = span.start.line.saturating_sub(1) as usize;
    let end_line = span.end.line as usize;
    let from = start_line.saturating_sub(20);
    let line_count = source.lines().count();
    let to = (end_line + 20).min(line_count.saturating_sub(1).max(end_line));
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
    if out.is_empty() { None } else { Some(out) }
}

/// Apply «…» markers around highlight ranges in `code` (byte offsets).
pub fn apply_highlights(code: &str, highlights: &[(u32, u32)]) -> String {
    if highlights.is_empty() {
        return code.to_owned();
    }
    let mut ranges = highlights.to_vec();
    ranges.sort_by_key(|(s, _)| *s);
    let bytes = code.as_bytes();
    let mut out = String::new();
    let mut cursor = 0usize;
    for &(start, end) in &ranges {
        let start = start as usize;
        let end = (end as usize).min(bytes.len());
        if start < cursor || start >= end || start > bytes.len() {
            continue;
        }
        out.push_str(&code[cursor..start]);
        out.push('«');
        out.push_str(&code[start..end]);
        out.push('»');
        cursor = end;
    }
    out.push_str(&code[cursor..]);
    out
}
