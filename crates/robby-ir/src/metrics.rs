//! IR improvement metric computers.
//!
//! Catalog ids live in Project `docs/ir-improvement-metrics.md`. Metrics **read**
//! the graph (and coverage report) and stamp measured values onto
//! [`crate::run::RunRecord::stats`]. They do not invent a parallel scorecard
//! schema.

use std::collections::BTreeMap;

use thiserror::Error;

use crate::coverage::CoverageReport;
use crate::model::{Document, Edge, NodeId};
use crate::run::RunRecord;

/// Well-known catalog metric ids (open strings; more ids land as computers grow).
pub mod ids {
    /// `CoverageReport::is_complete()` as `1` / `0`.
    ///
    /// **`doc` here means Document / workspace source extract**, not Markdown.
    /// Generated `docs/self-model/` pages are report *outputs*; closing this
    /// metric means fixing Rust/TS extract coverage gaps (prefer `crates/`,
    /// `apps/`), never hand-editing generated MD / `.mmd` / `.robby/`.
    pub const DOC_COVERAGE_COMPLETE: &str = "doc.coverage_complete";
    /// Human-readable fix-surface label stamped into metric details / prompts.
    pub const DOC_COVERAGE_FIX_SURFACE: &str = "source_extract";
    /// Resolved edges / all edges (document-wide or per-node).
    pub const EDGE_RESOLVE_RATE: &str = "edge.resolve_rate";
}

/// Optimization direction from the catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    HigherBetter,
    LowerBetter,
    Context,
}

impl Direction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HigherBetter => "higher_better",
            Self::LowerBetter => "lower_better",
            Self::Context => "context",
        }
    }
}

/// One measured catalog metric, ready for run `stats` / observations.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricMeasurement {
    pub id: String,
    /// Numeric score (bool metrics use `1.0` / `0.0`).
    pub value: f64,
    pub direction: Direction,
    /// Supporting counts / labels written beside the primary score.
    pub details: BTreeMap<String, String>,
}

impl MetricMeasurement {
    /// String form of [`Self::value`] for `stats` / decision `score` annotations.
    ///
    /// Bool-like 0/1 values stay integer; rates keep a short decimal.
    pub fn score_string(&self) -> String {
        if (self.value - self.value.round()).abs() < f64::EPSILON
            && (0.0..=1.0).contains(&self.value)
            && (self.value.abs() < f64::EPSILON || (self.value - 1.0).abs() < f64::EPSILON)
        {
            format!("{}", self.value as i32)
        } else {
            // Trim noisy float tails while staying parseable.
            let s = format!("{:.6}", self.value);
            s.trim_end_matches('0').trim_end_matches('.').to_owned()
        }
    }

    /// Insert this measurement into a run `stats` map.
    ///
    /// Primary key is the catalog id → score. Also records
    /// `{id}.direction` and any [`Self::details`] entries.
    pub fn write_into(&self, stats: &mut BTreeMap<String, String>) {
        stats.insert(self.id.clone(), self.score_string());
        stats.insert(
            format!("{}.direction", self.id),
            self.direction.as_str().to_owned(),
        );
        for (key, value) in &self.details {
            stats.insert(key.clone(), value.clone());
        }
    }
}

/// Unknown catalog id — computers stay stringly/open like kinds.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("unknown metric id: {0}")]
pub struct UnknownMetric(pub String);

/// `doc.coverage_complete`: `1` if the coverage gate passes, else `0`.
///
/// Stamps `doc.coverage_fix_surface=source_extract` and gap source paths so
/// improve/integrate do not treat generated Markdown as the edit surface.
pub fn doc_coverage_complete(report: &CoverageReport) -> MetricMeasurement {
    let complete = report.is_complete();
    let unknown_gaps = report.failures().count();
    let mut details = BTreeMap::new();
    details.insert(
        "doc.coverage_files".to_owned(),
        report.files.len().to_string(),
    );
    details.insert("doc.unknown_gap_count".to_owned(), unknown_gaps.to_string());
    details.insert(
        "doc.coverage_fix_surface".to_owned(),
        ids::DOC_COVERAGE_FIX_SURFACE.to_owned(),
    );
    details.insert(
        "doc.coverage_not".to_owned(),
        "generated_markdown".to_owned(),
    );
    let gap_files: Vec<String> = {
        let mut files: Vec<String> = report.failures().map(|(f, _)| f.file.clone()).collect();
        files.sort();
        files.dedup();
        files
    };
    if !gap_files.is_empty() {
        details.insert("doc.coverage_gap_files".to_owned(), gap_files.join(","));
    }
    MetricMeasurement {
        id: ids::DOC_COVERAGE_COMPLETE.to_owned(),
        value: if complete { 1.0 } else { 0.0 },
        direction: Direction::HigherBetter,
        details,
    }
}

/// `edge.resolve_rate` over every edge in the document.
///
/// Vacuous documents (no edges) score `1.0`.
pub fn edge_resolve_rate(doc: &Document) -> MetricMeasurement {
    resolve_rate_from(doc.nodes.values().flat_map(|n| n.edges.iter()))
}

/// `edge.resolve_rate` for one node's outgoing edges.
///
/// Returns `None` when the node is missing from the document.
pub fn edge_resolve_rate_at(doc: &Document, node: &NodeId) -> Option<MetricMeasurement> {
    doc.get(node).map(|n| resolve_rate_from(n.edges.iter()))
}

/// `edge.resolve_rate` for an arbitrary edge slice (tests / callers).
pub fn edge_resolve_rate_of<'a, I>(edges: I) -> MetricMeasurement
where
    I: IntoIterator<Item = &'a Edge>,
{
    resolve_rate_from(edges)
}

fn resolve_rate_from<'a, I>(edges: I) -> MetricMeasurement
where
    I: IntoIterator<Item = &'a Edge>,
{
    let mut total = 0u64;
    let mut resolved = 0u64;
    for edge in edges {
        total += 1;
        if edge.resolved {
            resolved += 1;
        }
    }
    let value = if total == 0 {
        1.0
    } else {
        resolved as f64 / total as f64
    };
    let mut details = BTreeMap::new();
    details.insert("edge.resolved".to_owned(), resolved.to_string());
    details.insert("edge.total".to_owned(), total.to_string());
    MetricMeasurement {
        id: ids::EDGE_RESOLVE_RATE.to_owned(),
        value,
        direction: Direction::HigherBetter,
        details,
    }
}

/// Dispatch a catalog id to its computer.
///
/// `coverage` is required for `doc.coverage_complete` and ignored for
/// edge-only metrics.
pub fn compute(
    id: &str,
    doc: &Document,
    coverage: &CoverageReport,
) -> Result<MetricMeasurement, UnknownMetric> {
    match id {
        ids::DOC_COVERAGE_COMPLETE => Ok(doc_coverage_complete(coverage)),
        ids::EDGE_RESOLVE_RATE => Ok(edge_resolve_rate(doc)),
        other => Err(UnknownMetric(other.to_owned())),
    }
}

/// First-slice pair: coverage completeness + document edge resolve rate.
pub fn first_slice(doc: &Document, coverage: &CoverageReport) -> Vec<MetricMeasurement> {
    vec![doc_coverage_complete(coverage), edge_resolve_rate(doc)]
}

/// Write measurements into [`RunRecord::stats`].
pub fn write_into_stats(record: &mut RunRecord, measurements: &[MetricMeasurement]) {
    for measurement in measurements {
        measurement.write_into(&mut record.stats);
    }
}

/// Compute the first-slice metrics and stamp them onto `record.stats`.
pub fn stamp_first_slice(record: &mut RunRecord, doc: &Document, coverage: &CoverageReport) {
    write_into_stats(record, &first_slice(doc, coverage));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coverage::file_coverage;
    use crate::kind;
    use crate::model::{EdgeKind, Node};
    use crate::run::{RunStatus, recipes};
    use crate::span::Span;

    fn empty_coverage() -> CoverageReport {
        CoverageReport { files: vec![] }
    }

    #[test]
    fn coverage_complete_is_one_when_gate_passes() {
        let source = "fn add() { 1 }";
        let file = file_coverage(
            "src/a.rs",
            source,
            &[Span::new("src/a.rs", source, 0, source.len() as u32)],
        );
        assert!(file.is_complete());
        let report = CoverageReport { files: vec![file] };
        let m = doc_coverage_complete(&report);
        assert_eq!(m.id, ids::DOC_COVERAGE_COMPLETE);
        assert_eq!(m.value, 1.0);
        assert_eq!(m.score_string(), "1");
        assert_eq!(m.direction, Direction::HigherBetter);
    }

    #[test]
    fn coverage_complete_is_zero_when_unknown_gap() {
        let source = "fn a() {}\nlet x = 1;\n";
        let end = source.find('\n').unwrap() as u32 + 1;
        let file = file_coverage("src/a.rs", source, &[Span::new("src/a.rs", source, 0, end)]);
        assert!(!file.is_complete());
        let report = CoverageReport { files: vec![file] };
        let m = doc_coverage_complete(&report);
        assert_eq!(m.value, 0.0);
        assert_eq!(m.score_string(), "0");
        assert_eq!(
            m.details.get("doc.unknown_gap_count").map(String::as_str),
            Some("1")
        );
        assert_eq!(
            m.details
                .get("doc.coverage_fix_surface")
                .map(String::as_str),
            Some(ids::DOC_COVERAGE_FIX_SURFACE)
        );
        assert_eq!(
            m.details.get("doc.coverage_not").map(String::as_str),
            Some("generated_markdown")
        );
        assert_eq!(
            m.details.get("doc.coverage_gap_files").map(String::as_str),
            Some("src/a.rs")
        );
    }

    #[test]
    fn edge_resolve_rate_counts_resolved() {
        let mut doc = Document::new("test");
        let mut n = Node::new("n:a", kind::code::FUNCTION, "a");
        n.edges.push(Edge::new("n:b", EdgeKind::Calls));
        let mut unresolved = Edge::new("missing", EdgeKind::Calls);
        unresolved.resolved = false;
        n.edges.push(unresolved);
        doc.insert(n);

        let m = edge_resolve_rate(&doc);
        assert_eq!(m.id, ids::EDGE_RESOLVE_RATE);
        assert!((m.value - 0.5).abs() < 1e-9);
        assert_eq!(m.score_string(), "0.5");
        assert_eq!(
            m.details.get("edge.resolved").map(String::as_str),
            Some("1")
        );
        assert_eq!(m.details.get("edge.total").map(String::as_str), Some("2"));
    }

    #[test]
    fn edge_resolve_rate_vacuous_is_one() {
        let doc = Document::new("test");
        let m = edge_resolve_rate(&doc);
        assert_eq!(m.value, 1.0);
        assert_eq!(m.details.get("edge.total").map(String::as_str), Some("0"));
    }

    #[test]
    fn compute_dispatches_known_ids() {
        let doc = Document::new("test");
        let coverage = empty_coverage();
        assert_eq!(
            compute(ids::DOC_COVERAGE_COMPLETE, &doc, &coverage)
                .unwrap()
                .id,
            ids::DOC_COVERAGE_COMPLETE
        );
        assert_eq!(
            compute(ids::EDGE_RESOLVE_RATE, &doc, &coverage).unwrap().id,
            ids::EDGE_RESOLVE_RATE
        );
        assert!(compute("nope.metric", &doc, &coverage).is_err());
    }

    #[test]
    fn stamp_first_slice_writes_run_stats() {
        let mut doc = Document::new("test");
        let mut n = Node::new("n:a", kind::code::FUNCTION, "a");
        n.edges.push(Edge::new("n:b", EdgeKind::Calls));
        doc.insert(n);

        let coverage = empty_coverage();
        let mut record = RunRecord::new("01METRIC", recipes::METRIC_TEST, RunStatus::Pass)
            .with_subject(ids::DOC_COVERAGE_COMPLETE);
        stamp_first_slice(&mut record, &doc, &coverage);

        assert_eq!(
            record
                .stats
                .get(ids::DOC_COVERAGE_COMPLETE)
                .map(String::as_str),
            Some("1")
        );
        assert_eq!(
            record.stats.get(ids::EDGE_RESOLVE_RATE).map(String::as_str),
            Some("1")
        );
        assert_eq!(
            record
                .stats
                .get(&format!("{}.direction", ids::EDGE_RESOLVE_RATE))
                .map(String::as_str),
            Some("higher_better")
        );
        assert_eq!(
            record.stats.get("edge.total").map(String::as_str),
            Some("1")
        );
    }

    #[test]
    fn edge_resolve_rate_at_missing_node_is_none() {
        let doc = Document::new("test");
        assert!(edge_resolve_rate_at(&doc, &NodeId::from("nope")).is_none());
    }
}
