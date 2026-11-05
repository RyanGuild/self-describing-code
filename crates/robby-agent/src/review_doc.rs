//! Markdown-bound IR metric review (`robby.review_doc`).
//!
//! First-slice producer: read a Project markdown plan/report, bind related IR
//! subjects, score a small metric set against the live Document, emit
//! `agent.decision` products with metric annotations, and return a
//! [`RunRecord`]. No LLM required — empirical gate is `robby.metric_test`.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use robby_ir::run::{RecipeId, RunRecord, RunStatus, recipes};
use robby_ir::{
    Document, Kind, Lang, Node, NodeId, Process, Provenance, Source, Step, StepKind, Verdict,
    coverage::document_coverage, kind,
};

/// Catalog metric: resolved edges / all edges.
pub const METRIC_EDGE_RESOLVE_RATE: &str = "edge.resolve_rate";
/// Catalog metric: `CoverageReport::is_complete` as 1/0.
pub const METRIC_DOC_COVERAGE_COMPLETE: &str = "doc.coverage_complete";

const DEFAULT_MAX_SUBJECTS: usize = 5;

#[derive(Debug, Clone)]
pub struct ReviewDocOpts {
    /// Repository-relative (or absolute) markdown path; becomes run `subject`.
    pub doc_path: String,
    /// Override minted run id.
    pub run_id: Option<String>,
    /// Metrics to claim. Empty → first-slice defaults.
    pub metrics: Vec<String>,
    /// Cap on IR subjects bound beside the doc path.
    pub max_subjects: usize,
}

impl Default for ReviewDocOpts {
    fn default() -> Self {
        Self {
            doc_path: String::new(),
            run_id: None,
            metrics: Vec::new(),
            max_subjects: DEFAULT_MAX_SUBJECTS,
        }
    }
}

impl ReviewDocOpts {
    fn metrics_or_default(&self) -> Vec<String> {
        if self.metrics.is_empty() {
            vec![
                METRIC_EDGE_RESOLVE_RATE.to_owned(),
                METRIC_DOC_COVERAGE_COMPLETE.to_owned(),
            ]
        } else {
            self.metrics.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReviewFinding {
    pub decision_id: NodeId,
    pub metric: String,
    pub score: String,
    pub subject: String,
    pub verdict: Verdict,
    pub test_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReviewDocStats {
    pub findings: usize,
    pub relevant: usize,
    pub metrics_claimed: usize,
    /// Always 0 until a chained `robby.metric_test` runs.
    pub metrics_tested: usize,
    pub subjects: usize,
}

#[derive(Debug, Clone)]
pub struct ReviewDocResult {
    pub record: RunRecord,
    pub findings: Vec<ReviewFinding>,
    pub subjects: Vec<String>,
    pub stats: ReviewDocStats,
}

/// Run `robby.review_doc` against `doc`, mutating it with decision products.
pub fn review_doc(
    doc: &mut Document,
    markdown: &str,
    sources: &BTreeMap<String, String>,
    opts: &ReviewDocOpts,
) -> Result<ReviewDocResult> {
    let started_at = now_ms();
    let run_id = opts
        .run_id
        .clone()
        .unwrap_or_else(|| mint_run_id("review-doc"));
    let subject = format!("path:{}", normalize_doc_path(&opts.doc_path));
    let metrics = opts.metrics_or_default();

    let subjects = bind_subjects(doc, markdown, &opts.doc_path, opts.max_subjects);
    let scores = score_metrics(doc, sources, &metrics);

    let doc_parent = ensure_doc_subject(doc, &opts.doc_path, &run_id);

    let mut findings = Vec::new();
    for claim in &scores {
        let parent = pick_parent(doc, &doc_parent, &subjects, claim);
        let finding = insert_metric_decision(doc, &parent, claim, &run_id, &subject)?;
        findings.push(finding);
    }

    let relevant = findings
        .iter()
        .filter(|f| f.verdict == Verdict::Relevant)
        .count();
    let stats = ReviewDocStats {
        findings: findings.len(),
        relevant,
        metrics_claimed: scores.len(),
        metrics_tested: 0,
        subjects: subjects.len(),
    };

    let finished_at = now_ms();
    let mut record = RunRecord::new(
        run_id.clone(),
        RecipeId::from(recipes::REVIEW_DOC),
        RunStatus::Pass,
    )
    .with_subject(subject)
    .with_process(review_doc_process());
    record.started_at = Some(started_at);
    record.finished_at = Some(finished_at);
    record.exit_code = Some(0);
    record.stdout_tail = Some(format!(
        "robby.review_doc · {} finding(s) · {} relevant · {} subject(s)",
        stats.findings, stats.relevant, stats.subjects
    ));
    record
        .stats
        .insert("findings".into(), stats.findings.to_string());
    record
        .stats
        .insert("relevant".into(), stats.relevant.to_string());
    record
        .stats
        .insert("metrics_claimed".into(), stats.metrics_claimed.to_string());
    record
        .stats
        .insert("metrics_tested".into(), stats.metrics_tested.to_string());
    record
        .stats
        .insert("subjects".into(), stats.subjects.to_string());
    record
        .stats
        .insert("doc_path".into(), normalize_doc_path(&opts.doc_path));

    for finding in &findings {
        record
            .products
            .push(format!("ir:{}", finding.decision_id.as_str()));
    }
    let artifact = review_artifact_product(&opts.doc_path);
    record.products.push(artifact);

    Ok(ReviewDocResult {
        record,
        findings,
        subjects,
        stats,
    })
}

/// Path product for the JSON review artifact (CLI writes the file).
pub fn review_artifact_product(doc_path: &str) -> String {
    format!("path:{}", review_artifact_relpath(doc_path))
}

/// Repository-relative path under `.robby/reviews/` for the review JSON.
pub fn review_artifact_relpath(doc_path: &str) -> String {
    let stem = Path::new(doc_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("review");
    format!(".robby/reviews/{stem}.json")
}

#[derive(Debug, Clone)]
struct MetricClaim {
    metric: String,
    score: f64,
    direction: &'static str,
    test_id: String,
    action: &'static str,
    reason: String,
    /// Prefer this IR subject node id when present.
    preferred_subject: Option<NodeId>,
}

fn score_metrics(
    doc: &Document,
    sources: &BTreeMap<String, String>,
    metrics: &[String],
) -> Vec<MetricClaim> {
    let mut out = Vec::new();
    for metric in metrics {
        match metric.as_str() {
            METRIC_EDGE_RESOLVE_RATE => {
                let score = edge_resolve_rate(doc);
                out.push(MetricClaim {
                    metric: metric.clone(),
                    score,
                    direction: "higher_better",
                    test_id: "metric_test.edge.resolve_rate".into(),
                    action: "improve_extract",
                    reason: format!(
                        "edge.resolve_rate={score:.4} over document edges (unresolved kept)"
                    ),
                    preferred_subject: None,
                });
            }
            METRIC_DOC_COVERAGE_COMPLETE => {
                let complete = doc_coverage_complete(doc, sources);
                let preferred = preferred_coverage_source_subject(doc, sources);
                out.push(MetricClaim {
                    metric: metric.clone(),
                    score: complete,
                    direction: "higher_better",
                    test_id: "metric_test.doc.coverage_complete".into(),
                    // Coverage is source-extract completeness. Markdown under
                    // docs/self-model is generated OUTPUT — never revise_doc.
                    action: "improve_extract",
                    reason: if complete >= 1.0 {
                        "doc.coverage_complete=1 (source extract CoverageReport::is_complete; not markdown)"
                            .into()
                    } else {
                        "doc.coverage_complete=0 (unknown gaps in Rust/TS source extract — fix crates/apps extractors/spans, not docs/self-model)"
                            .into()
                    },
                    preferred_subject: preferred,
                });
            }
            other => {
                out.push(MetricClaim {
                    metric: other.to_owned(),
                    score: 0.0,
                    direction: "higher_better",
                    test_id: format!("metric_test.{other}"),
                    action: "revise_doc",
                    reason: format!("metric `{other}` has no computer in review_doc first slice"),
                    preferred_subject: None,
                });
            }
        }
    }
    out
}

fn edge_resolve_rate(doc: &Document) -> f64 {
    let mut total = 0u64;
    let mut resolved = 0u64;
    for node in doc.nodes.values() {
        for edge in &node.edges {
            total += 1;
            if edge.resolved {
                resolved += 1;
            }
        }
    }
    if total == 0 {
        1.0
    } else {
        resolved as f64 / total as f64
    }
}

fn doc_coverage_complete(doc: &Document, sources: &BTreeMap<String, String>) -> f64 {
    if sources.is_empty() && doc.files.is_empty() {
        return 0.0;
    }
    let report = document_coverage(doc, sources);
    if report.is_complete() { 1.0 } else { 0.0 }
}

fn bind_subjects(
    doc: &Document,
    markdown: &str,
    doc_path: &str,
    max_subjects: usize,
) -> Vec<String> {
    let mut subjects: Vec<String> = Vec::new();
    let push = |subjects: &mut Vec<String>, value: String| {
        if !subjects.iter().any(|s| s == &value) {
            subjects.push(value);
        }
    };

    push(
        &mut subjects,
        format!("path:{}", normalize_doc_path(doc_path)),
    );

    for root in &doc.roots {
        push(&mut subjects, root.as_str().to_owned());
        if subjects.len() >= max_subjects {
            return subjects;
        }
    }

    // Prefer callables / nodes that already carry Process.
    let mut with_process: Vec<&Node> = doc
        .nodes
        .values()
        .filter(|n| n.process.as_ref().is_some_and(|p| !p.steps.is_empty()))
        .collect();
    with_process.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    for node in with_process {
        push(&mut subjects, node.id.as_str().to_owned());
        if subjects.len() >= max_subjects {
            return subjects;
        }
    }

    // Unresolved Calls — improvement surface for edge.resolve_rate.
    let mut unresolved: Vec<&Node> = doc
        .nodes
        .values()
        .filter(|n| {
            n.edges
                .iter()
                .any(|e| e.kind == robby_ir::EdgeKind::Calls && !e.resolved)
        })
        .collect();
    unresolved.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    for node in unresolved {
        push(&mut subjects, node.id.as_str().to_owned());
        if subjects.len() >= max_subjects {
            return subjects;
        }
    }

    // Markdown mentions of recipe / metric ids (anchors for later slices).
    for token in extract_backtick_tokens(markdown) {
        if token.starts_with("robby.") || token.contains('.') && looks_like_metric(&token) {
            push(&mut subjects, format!("mention:{token}"));
            if subjects.len() >= max_subjects {
                return subjects;
            }
        }
    }

    subjects
}

fn looks_like_metric(token: &str) -> bool {
    matches!(
        token.split('.').next(),
        Some(
            "doc"
                | "edge"
                | "fn"
                | "file"
                | "module"
                | "type"
                | "comment"
                | "process"
                | "path"
                | "agent"
                | "ui"
                | "prov"
                | "runtime"
                | "leaf"
                | "import"
        )
    )
}

fn extract_backtick_tokens(markdown: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = markdown;
    while let Some(start) = rest.find('`') {
        rest = &rest[start + 1..];
        let Some(end) = rest.find('`') else {
            break;
        };
        let token = rest[..end].trim();
        if !token.is_empty() && !token.contains('\n') && token.len() < 120 {
            out.push(token.to_owned());
        }
        rest = &rest[end + 1..];
    }
    out
}

fn ensure_doc_subject(doc: &mut Document, doc_path: &str, run_id: &str) -> NodeId {
    let pipeline_id = NodeId::symbol("agent", recipes::REVIEW_DOC);
    if doc.get(&pipeline_id).is_none() {
        let mut pipeline = Node::with_lang(
            pipeline_id.clone(),
            kind::agent::PIPELINE,
            recipes::REVIEW_DOC,
            Lang::Unknown,
        );
        pipeline.provenance = Provenance::new(Source::Agent);
        pipeline
            .annotations
            .insert("recipe".into(), recipes::REVIEW_DOC.into());
        doc.insert(pipeline);
    }

    let fqn = format!(
        "{}/doc/{}",
        recipes::REVIEW_DOC,
        normalize_doc_path(doc_path).replace('/', ".")
    );
    let id = NodeId::symbol("agent", &fqn);
    if doc.get(&id).is_none() {
        let mut node = Node::with_lang(id.clone(), kind::agent::OBSERVATION, fqn, Lang::Unknown);
        node.parent = Some(pipeline_id);
        node.provenance = Provenance::new(Source::Agent);
        node.annotations
            .insert("doc_path".into(), normalize_doc_path(doc_path));
        node.annotations.insert("run_id".into(), run_id.to_owned());
        doc.insert(node);
    }
    id
}

fn pick_parent(
    doc: &Document,
    doc_parent: &NodeId,
    subjects: &[String],
    claim: &MetricClaim,
) -> NodeId {
    if let Some(preferred) = &claim.preferred_subject
        && doc.get(preferred).is_some()
    {
        return preferred.clone();
    }
    // Prefer a real IR subject (skip path:/mention: binders) for extract metrics.
    // Markdown path binders are review inputs, not the coverage fix surface.
    if claim.metric == METRIC_EDGE_RESOLVE_RATE || claim.metric == METRIC_DOC_COVERAGE_COMPLETE {
        for subject in subjects {
            if subject.starts_with("path:") || subject.starts_with("mention:") {
                continue;
            }
            let id = NodeId::from(subject.as_str());
            if doc.get(&id).is_some() {
                return id;
            }
        }
    }
    doc_parent.clone()
}

/// Prefer a source node that owns a coverage-gap file (Rust/TS), never a MD path.
fn preferred_coverage_source_subject(
    doc: &Document,
    sources: &BTreeMap<String, String>,
) -> Option<NodeId> {
    if sources.is_empty() && doc.files.is_empty() {
        return None;
    }
    let report = document_coverage(doc, sources);
    let mut gap_files: Vec<&str> = report.failures().map(|(f, _)| f.file.as_str()).collect();
    gap_files.sort_unstable();
    gap_files.dedup();
    for file in gap_files {
        let mut candidates: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|n| n.span.as_ref().is_some_and(|s| s.file == file))
            .collect();
        candidates.sort_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
        if let Some(node) = candidates.first() {
            return Some(node.id.clone());
        }
    }
    None
}

fn insert_metric_decision(
    doc: &mut Document,
    parent_id: &NodeId,
    claim: &MetricClaim,
    run_id: &str,
    doc_subject: &str,
) -> Result<ReviewFinding> {
    let Some(parent) = doc.get(parent_id) else {
        anyhow::bail!("missing parent {parent_id}");
    };
    let fqn = format!("{}/decision/{}", parent.fqn, claim.metric.replace('.', "_"));
    let id = NodeId::symbol("agent", &fqn);
    let parent_lang = parent.lang;

    let score_str = format_score(claim.score);
    let measured =
        claim.metric == METRIC_DOC_COVERAGE_COMPLETE || claim.metric == METRIC_EDGE_RESOLVE_RATE;
    let verdict = if measured && claim.score >= 1.0 {
        Verdict::Noise
    } else {
        Verdict::Relevant
    };

    let mut node = Node::with_lang(
        id.clone(),
        kind::agent::DECISION,
        fqn,
        match parent_lang {
            Lang::TypeScript => Lang::TypeScript,
            Lang::Rust => Lang::Rust,
            Lang::Unknown => Lang::Unknown,
        },
    );
    node.parent = Some(parent_id.clone());
    node.provenance = Provenance::new(Source::Agent);
    node.docs = Some(claim.reason.clone());
    node.annotations
        .insert("verdict".into(), verdict.as_str().into());
    node.annotations
        .insert("reason".into(), claim.reason.clone());
    node.annotations
        .insert("metric".into(), claim.metric.clone());
    node.annotations.insert("score".into(), score_str.clone());
    node.annotations
        .insert("direction".into(), claim.direction.into());
    node.annotations
        .insert("test_id".into(), claim.test_id.clone());
    node.annotations
        .insert("subject".into(), parent_id.as_str().into());
    node.annotations
        .insert("action".into(), claim.action.into());
    node.annotations.insert("run_id".into(), run_id.to_owned());
    node.annotations
        .insert("doc_subject".into(), doc_subject.to_owned());
    node.annotations
        .insert("recipe".into(), recipes::REVIEW_DOC.into());

    doc.merge(node);

    Ok(ReviewFinding {
        decision_id: id,
        metric: claim.metric.clone(),
        score: score_str,
        subject: parent_id.as_str().to_owned(),
        verdict,
        test_id: claim.test_id.clone(),
    })
}

fn review_doc_process() -> Process {
    let mut process = Process::new();
    process.participant("reviewer", "review_doc", Kind::from(kind::agent::PIPELINE));
    process.participant("ir", "Document", Kind::from(kind::code::WORKSPACE));
    let stages = [
        "read_doc",
        "bind_subjects",
        "score_metrics",
        "propose_tests",
        "emit_decisions",
    ];
    for label in stages {
        process.step(Step {
            from: "reviewer".to_owned(),
            to: "ir".to_owned(),
            label: label.to_owned(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
    }
    process
}

fn format_score(score: f64) -> String {
    if (score - score.round()).abs() < f64::EPSILON {
        format!("{}", score as i64)
    } else {
        format!("{score:.6}")
    }
}

fn normalize_doc_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

/// Monotonic suffix so burst mints in the same millisecond never collide.
static RUN_ID_SEQ: AtomicU64 = AtomicU64::new(0);

fn mint_run_id(prefix: &str) -> String {
    let seq = RUN_ID_SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{}-{seq}", now_ms())
}

#[cfg(test)]
mod tests {
    use super::*;
    use robby_ir::{Edge, EdgeKind, Position, Span};

    fn fixture_doc() -> (Document, BTreeMap<String, String>) {
        let mut doc = Document::new("test");
        let mut root = Node::with_lang(
            NodeId::symbol("rust", "demo"),
            kind::code::CRATE,
            "demo",
            Lang::Rust,
        );
        root.span = Some(Span {
            file: "demo.rs".into(),
            start: Position::new(1, 1, 0),
            end: Position::new(3, 2, 40),
        });
        doc.insert(root);
        doc.roots.push(NodeId::symbol("rust", "demo"));
        doc.declare_file("demo.rs");

        let mut fn_node = Node::with_lang(
            NodeId::symbol("rust", "demo::draw"),
            kind::code::FUNCTION,
            "demo::draw",
            Lang::Rust,
        );
        fn_node.parent = Some(NodeId::symbol("rust", "demo"));
        fn_node.span = Some(Span {
            file: "demo.rs".into(),
            start: Position::new(1, 1, 0),
            end: Position::new(3, 2, 40),
        });
        let mut unresolved = Edge::new(NodeId::symbol("rust", "demo::missing"), EdgeKind::Calls);
        unresolved.resolved = false;
        fn_node.edges.push(unresolved);
        let mut process = Process::new();
        process.participant("a", "a", Kind::from(kind::code::FUNCTION));
        process.step(Step {
            from: "a".into(),
            to: "a".into(),
            label: "body".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: vec![],
        });
        fn_node.process = Some(process);
        doc.insert(fn_node);

        let mut sources = BTreeMap::new();
        sources.insert("demo.rs".into(), "fn draw() {\n    missing();\n}\n".into());
        (doc, sources)
    }

    #[test]
    fn review_doc_emits_decisions_and_run_record() {
        let (mut doc, sources) = fixture_doc();
        let markdown = "# Plan\n\nScore `edge.resolve_rate` and `robby.extract`.\n";
        let opts = ReviewDocOpts {
            doc_path: "docs/immersive-web-view-features.md".into(),
            run_id: Some("review-doc-test".into()),
            metrics: vec![
                METRIC_EDGE_RESOLVE_RATE.into(),
                METRIC_DOC_COVERAGE_COMPLETE.into(),
            ],
            max_subjects: 5,
        };
        let result = review_doc(&mut doc, markdown, &sources, &opts).unwrap();

        assert_eq!(result.record.recipe.as_str(), recipes::REVIEW_DOC);
        assert_eq!(result.record.status, RunStatus::Pass);
        assert_eq!(
            result.record.subject.as_deref(),
            Some("path:docs/immersive-web-view-features.md")
        );
        assert_eq!(result.findings.len(), 2);
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.metric == METRIC_EDGE_RESOLVE_RATE)
        );
        assert!(
            result
                .findings
                .iter()
                .any(|f| f.metric == METRIC_DOC_COVERAGE_COMPLETE)
        );

        let edge = result
            .findings
            .iter()
            .find(|f| f.metric == METRIC_EDGE_RESOLVE_RATE)
            .unwrap();
        assert_eq!(edge.verdict, Verdict::Relevant);
        assert!(edge.score.parse::<f64>().unwrap() < 1.0);

        let decisions: Vec<_> = doc
            .nodes
            .values()
            .filter(|n| n.kind.as_str() == kind::agent::DECISION)
            .collect();
        assert_eq!(decisions.len(), 2);
        for decision in decisions {
            assert_eq!(
                decision.annotations.get("run_id").map(String::as_str),
                Some("review-doc-test")
            );
            assert!(decision.annotations.contains_key("metric"));
            assert!(decision.annotations.contains_key("test_id"));
            assert!(decision.annotations.contains_key("score"));
        }

        let process = result.record.process.as_ref().expect("process");
        assert!(
            process
                .steps
                .iter()
                .any(|s| s.label == "bind_subjects" && s.observed)
        );
        assert!(
            process
                .steps
                .iter()
                .any(|s| s.label == "emit_decisions" && s.observed)
        );
        assert!(
            result
                .record
                .products
                .iter()
                .any(|p| p.starts_with("ir:agent://"))
        );
        assert!(
            result
                .record
                .products
                .iter()
                .any(|p| { p == "path:.robby/reviews/immersive-web-view-features.json" })
        );
        assert!(result.subjects.iter().any(|s| s == "rust://demo::draw"));
    }

    #[test]
    fn bind_subjects_respects_max() {
        let (doc, _) = fixture_doc();
        let subjects = bind_subjects(&doc, "`robby.web` `doc.coverage_complete`", "a.md", 2);
        assert_eq!(subjects.len(), 2);
        assert_eq!(subjects[0], "path:a.md");
    }
}
