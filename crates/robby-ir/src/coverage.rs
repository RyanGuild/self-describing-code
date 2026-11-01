//! The coverage invariant.
//!
//! Robby's trust argument rests entirely on this module: **every byte of source
//! is either owned by a node or classified with a reason.** Nothing is dropped
//! silently.
//!
//! The partition is computed in byte space. Node spans are unioned (a child span
//! nested inside a parent adds no new coverage), then the complement is scanned
//! and classified as whitespace, comment, attribute, or trivia. Anything that
//! does not classify is an [`GapReason::Unknown`] gap and **fails the gate**.
//!
//! This is intentionally strict. Macro-generated code that `syn` cannot see
//! shows up as an unknown gap, which is the correct behaviour: it surfaces a
//! real blind spot instead of hiding it.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::model::Document;
use crate::span::{Position, Span};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GapReason {
    /// Nothing but whitespace.
    Whitespace,
    /// Nothing but comments. Rendered, but not counted as a failure: comments
    /// are the raw material for the `why` question.
    Comment,
    /// Nothing but attributes, e.g. `#[derive(Debug)]`.
    Attribute,
    /// A mixture of the above.
    Trivia,
    /// Something is here that no node claims. This fails the gate.
    Unknown { at: Position },
}

impl GapReason {
    /// Only `Unknown` is a failure.
    pub fn is_failure(&self) -> bool {
        matches!(self, GapReason::Unknown { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Gap {
    pub span: Span,
    pub reason: GapReason,
    /// First 120 characters of the uncovered text, whitespace-collapsed.
    pub preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FileCoverage {
    pub file: String,
    pub total_bytes: u32,
    /// Bytes claimed by at least one node.
    pub covered_bytes: u32,
    pub gaps: Vec<Gap>,
}

impl FileCoverage {
    pub fn is_complete(&self) -> bool {
        !self.gaps.iter().any(|g| g.reason.is_failure())
    }

    /// Bytes in gaps that fail the gate.
    pub fn uncovered_bytes(&self) -> u32 {
        self.gaps
            .iter()
            .filter(|g| g.reason.is_failure())
            .map(|g| g.span.end.byte.saturating_sub(g.span.start.byte))
            .sum()
    }

    /// Fraction of the file owned by nodes, in `0.0..=1.0`.
    ///
    /// This counts comments and blank lines against the file, which is honest
    /// but reads badly. Prefer [`FileCoverage::code_ratio`] for a headline.
    pub fn ratio(&self) -> f32 {
        if self.total_bytes == 0 {
            1.0
        } else {
            self.covered_bytes as f32 / self.total_bytes as f32
        }
    }

    /// Fraction of *code* bytes owned by nodes, excluding classified trivia.
    ///
    /// This is the number that should be 100%: every byte that is not
    /// whitespace, comment, or attribute must belong to something.
    pub fn code_ratio(&self) -> f32 {
        let code = self.covered_bytes + self.uncovered_bytes();
        if code == 0 {
            1.0
        } else {
            self.covered_bytes as f32 / code as f32
        }
    }

    /// Bytes classified as trivia rather than claimed by a node.
    pub fn trivia_bytes(&self) -> u32 {
        self.gaps
            .iter()
            .filter(|g| !g.reason.is_failure())
            .map(|g| g.span.end.byte.saturating_sub(g.span.start.byte))
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CoverageReport {
    pub files: Vec<FileCoverage>,
}

impl CoverageReport {
    pub fn is_complete(&self) -> bool {
        self.files.iter().all(FileCoverage::is_complete)
    }

    pub fn failures(&self) -> impl Iterator<Item = (&FileCoverage, &Gap)> {
        self.files.iter().flat_map(|f| {
            f.gaps
                .iter()
                .filter(|g| g.reason.is_failure())
                .map(move |g| (f, g))
        })
    }

    /// Files sorted worst-first. This is the "where do I look?" ordering.
    pub fn worst_first(&self) -> Vec<&FileCoverage> {
        let mut files: Vec<&FileCoverage> = self.files.iter().collect();
        files.sort_by(|a, b| {
            a.code_ratio()
                .partial_cmp(&b.code_ratio())
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.file.cmp(&b.file))
        });
        files
    }
}

/// Compute coverage for one file given the spans that claim it.
pub fn file_coverage(file: &str, source: &str, spans: &[Span]) -> FileCoverage {
    let len = source.len() as u32;
    let bytes = source.as_bytes();

    let mut ranges: Vec<(u32, u32)> = spans
        .iter()
        .filter(|s| s.file == file)
        .map(|s| {
            let (a, b) = s.byte_range();
            (a.min(len), b.min(len))
        })
        .filter(|(a, b)| a < b)
        .collect();
    ranges.sort_unstable();

    let mut merged: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for (a, b) in ranges {
        match merged.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => merged.push((a, b)),
        }
    }

    let covered_bytes: u32 = merged.iter().map(|(a, b)| b - a).sum();

    let mut gaps = Vec::new();
    let mut cursor = 0u32;
    for (a, b) in &merged {
        if *a > cursor {
            gaps.push(make_gap(file, source, bytes, cursor, *a));
        }
        cursor = (*b).max(cursor);
    }
    if cursor < len {
        gaps.push(make_gap(file, source, bytes, cursor, len));
    }

    FileCoverage {
        file: file.to_owned(),
        total_bytes: len,
        covered_bytes,
        gaps,
    }
}

/// Compute coverage for every source file referenced by the document.
///
/// Files present in `sources` but never referenced still appear in the report,
/// fully uncovered. A file that exists and is invisible to the map is exactly
/// the failure mode this tool exists to prevent.
pub fn document_coverage(doc: &Document, sources: &BTreeMap<String, String>) -> CoverageReport {
    let spans_by_file = frontier_spans(doc);

    let mut files = Vec::new();
    for (path, source) in sources {
        let spans = spans_by_file.get(path.as_str()).cloned().unwrap_or_default();
        files.push(file_coverage(path, source, &spans));
    }
    files.sort_by(|a, b| a.file.cmp(&b.file));

    CoverageReport { files }
}

/// The spans that actually claim source, per file.
///
/// A node is a **frontier** node when no descendant carries a span. Only
/// frontier spans count towards coverage: a parent's span is structural, and
/// including it would make coverage trivially 100% for any file with a
/// top-level item. The finest claim is the claim.
pub fn frontier_spans(doc: &Document) -> BTreeMap<String, Vec<Span>> {
    let mut out: BTreeMap<String, Vec<Span>> = BTreeMap::new();
    for (id, node) in &doc.nodes {
        let Some(span) = &node.span else { continue };
        if has_span_descendant(doc, id) {
            continue;
        }
        out.entry(span.file.clone()).or_default().push(span.clone());
    }
    out
}

fn has_span_descendant(doc: &Document, id: &crate::model::NodeId) -> bool {
    let Some(node) = doc.get(id) else {
        return false;
    };
    node.children.iter().any(|child| {
        doc.get(child).is_some_and(|c| c.span.is_some()) || has_span_descendant(doc, child)
    })
}

fn make_gap(file: &str, source: &str, bytes: &[u8], start: u32, end: u32) -> Gap {
    let reason = match scan_trivia(bytes, start, end) {
        Ok(flags) => flags.classify(),
        Err(at) => GapReason::Unknown {
            at: Position::at(source, at),
        },
    };
    Gap {
        span: Span::new(file, source, start, end),
        reason,
        preview: preview(source, start, end),
    }
}

fn preview(source: &str, start: u32, end: u32) -> String {
    let slice = &source[start as usize..end as usize];
    let collapsed: String = slice.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() > 120 {
        let truncated: String = collapsed.chars().take(117).collect();
        format!("{truncated}...")
    } else {
        collapsed
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TriviaFlags {
    whitespace: bool,
    comment: bool,
    attribute: bool,
}

impl TriviaFlags {
    fn classify(self) -> GapReason {
        match (self.attribute, self.comment) {
            (true, false) => GapReason::Attribute,
            (false, true) => GapReason::Comment,
            (false, false) => GapReason::Whitespace,
            (true, true) => GapReason::Trivia,
        }
    }
}

/// Walk a byte range, accepting only whitespace, comments and attributes.
///
/// Returns the byte offset of the first token that is none of those, which is
/// what makes the gap report actionable: it points at the exact byte that is
/// missing from the map.
fn scan_trivia(bytes: &[u8], start: u32, end: u32) -> Result<TriviaFlags, u32> {
    let mut flags = TriviaFlags::default();
    let mut i = start;
    while i < end {
        let c = bytes[i as usize];
        if c.is_ascii_whitespace() {
            flags.whitespace = true;
            i += 1;
        } else if c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'/' {
            flags.comment = true;
            while i < end && bytes[i as usize] != b'\n' {
                i += 1;
            }
        } else if c == b'/' && i + 1 < end && bytes[(i + 1) as usize] == b'*' {
            flags.comment = true;
            let mut depth = 1u32;
            i += 2;
            while i < end && depth > 0 {
                if i + 1 < end && bytes[i as usize] == b'/' && bytes[(i + 1) as usize] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < end && bytes[i as usize] == b'*' && bytes[(i + 1) as usize] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else if c == b'#' {
            match skip_attribute(bytes, i, end) {
                Some(next) => {
                    flags.attribute = true;
                    i = next;
                }
                None => return Err(i),
            }
        } else {
            return Err(i);
        }
    }
    Ok(flags)
}

/// Skip `#[...]` or `#![...]`, honouring nested brackets and string literals.
fn skip_attribute(bytes: &[u8], start: u32, end: u32) -> Option<u32> {
    let mut i = start + 1;
    if i < end && bytes[i as usize] == b'!' {
        i += 1;
    }
    if i >= end || bytes[i as usize] != b'[' {
        return None;
    }
    let mut depth = 0i32;
    let mut in_string = false;
    while i < end {
        let c = bytes[i as usize];
        if in_string {
            if c == b'\\' {
                i += 2;
                continue;
            }
            if c == b'"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        match c {
            b'"' => {
                in_string = true;
                i += 1;
            }
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' => {
                depth -= 1;
                i += 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => i += 1,
        }
    }
    None
}
