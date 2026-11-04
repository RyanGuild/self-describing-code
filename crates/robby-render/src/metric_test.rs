//! `robby.metric_test` recipe: measure catalog metrics into a [`RunRecord`].
//!
//! Computers live in [`robby_ir::metrics`] (SDC-23). This module runs them as
//! recipe `robby.metric_test`, persists shared [`RunRecord`] JSON under
//! `runs/`, and spawns from an allowed `robby.review_view` only when
//! [`SiteRunsStore::blocks_downstream`] is false.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Result, bail};
use robby_ir::metrics::{self, MetricMeasurement, ids};
use robby_ir::run::{RecipeId, RunRecord, RunStatus, recipes};
use robby_ir::{CoverageReport, Document, Kind, NodeId, Process, Step, StepKind, kind::agent};

use crate::runs::SiteRunsStore;

/// Inputs for one metric_test invocation.
#[derive(Debug, Clone)]
pub struct MetricTestRequest {
    /// Catalog ids, e.g. `doc.coverage_complete`. Defaults: first-slice pair.
    pub metrics: Vec<String>,
    /// Optional subject node id / fqn (scopes `edge.resolve_rate` via `_at`).
    pub subject: Option<String>,
    /// Parent review `run_id` when spawned from Allow.
    pub parent_run_id: Option<String>,
    pub sample_seed: Option<u64>,
    pub sample_n: Option<usize>,
}

impl Default for MetricTestRequest {
    fn default() -> Self {
        Self {
            metrics: vec![
                ids::DOC_COVERAGE_COMPLETE.to_owned(),
                ids::EDGE_RESOLVE_RATE.to_owned(),
            ],
            subject: None,
            parent_run_id: None,
            sample_seed: None,
            sample_n: None,
        }
    }
}

/// Build (do not yet persist) a completed `robby.metric_test` [`RunRecord`].
pub fn build_metric_test_run(
    doc: &Document,
    coverage: &CoverageReport,
    req: &MetricTestRequest,
) -> Result<RunRecord> {
    let started = now_ms();
    let metric_ids = if req.metrics.is_empty() {
        MetricTestRequest::default().metrics
    } else {
        req.metrics.clone()
    };

    let mut observations = Vec::new();
    let mut errors = Vec::new();
    for id in &metric_ids {
        match compute_one(id, doc, coverage, req.subject.as_deref()) {
            Ok(obs) => observations.push(obs),
            Err(err) => errors.push(err),
        }
    }
    if observations.is_empty() && !errors.is_empty() {
        bail!("{}", errors.join("; "));
    }

    let all_pass = observations.iter().all(measurement_passes) && errors.is_empty();
    let status = if all_pass {
        RunStatus::Pass
    } else if observations.is_empty() {
        RunStatus::Error
    } else {
        RunStatus::Fail
    };

    let run_id = mint_run_id("metric");
    let mut record = RunRecord::new(run_id, RecipeId::from(recipes::METRIC_TEST), status);
    if let Some(subject) = &req.subject {
        record.subject = Some(subject.clone());
    } else if let Some(root) = doc.roots.first() {
        record.subject = Some(root.as_str().to_owned());
    }
    record.started_at = Some(started);
    record.finished_at = Some(now_ms());
    record.exit_code = Some(i32::from(!all_pass));
    record.process = Some(metric_test_process());
    record.stdout_tail = Some(summarize(&observations, &errors));
    metrics::write_into_stats(&mut record, &observations);
    record
        .stats
        .insert("metrics_tested".to_owned(), observations.len().to_string());
    record.stats.insert(
        "pass".to_owned(),
        if all_pass {
            "true".to_owned()
        } else {
            "false".to_owned()
        },
    );
    if let Some(seed) = req.sample_seed {
        record
            .stats
            .insert("sample_seed".to_owned(), seed.to_string());
    }
    if let Some(n) = req.sample_n {
        record.stats.insert("sample_n".to_owned(), n.to_string());
    }
    if !errors.is_empty() {
        record.stats.insert("errors".to_owned(), errors.join("; "));
    }
    for obs in &observations {
        record
            .stats
            .insert(format!("{}.test_id", obs.id), test_id_for(&obs.id));
    }
    record.products = observations
        .iter()
        .map(|o| format!("metric:{}:{}", o.id, o.score_string()))
        .collect();
    if let Some(parent) = &req.parent_run_id {
        record
            .stats
            .insert("parent_run_id".to_owned(), parent.clone());
        record.products.push(format!("run:{parent}"));
    }
    Ok(record)
}

/// Persist a metric_test run under `site_out/runs/`.
pub fn write_metric_test_run(
    site_out: &Path,
    doc: &Document,
    coverage: &CoverageReport,
    req: &MetricTestRequest,
) -> Result<RunRecord> {
    let record = build_metric_test_run(doc, coverage, req)?;
    let store = SiteRunsStore::new(site_out);
    store.put(&record)?;
    Ok(record)
}

/// After Allow on a review: spawn metric_test products iff not blocked.
///
/// Returns `(review, metric_test)`. Links `run:<metric_id>` on the review
/// products list. Bare [`SiteRunsStore::allow`] stays product-light; this is
/// the Allow → empirical gate path.
pub fn allow_and_spawn_metric_test(
    store: &SiteRunsStore,
    review_run_id: &str,
    doc: &Document,
    coverage: &CoverageReport,
    metrics: &[String],
) -> Result<(RunRecord, RunRecord)> {
    // Allow first (queued → running). Never start metric_test from queued /
    // cancelled / awaiting_allow — `blocks_downstream` encodes that gate.
    let mut review = store.allow(review_run_id)?;
    if SiteRunsStore::blocks_downstream(&review) {
        bail!(
            "review {review_run_id} blocks downstream (status={}, ui_gate={:?})",
            review.status.as_str(),
            review.stats.get("ui_gate")
        );
    }

    let req = MetricTestRequest {
        metrics: if metrics.is_empty() {
            MetricTestRequest::default().metrics
        } else {
            metrics.to_vec()
        },
        subject: review.subject.clone(),
        parent_run_id: Some(review.run_id.clone()),
        sample_seed: None,
        sample_n: None,
    };
    let metric_run = build_metric_test_run(doc, coverage, &req)?;
    store.put(&metric_run)?;

    review.products.push(format!("run:{}", metric_run.run_id));
    review
        .products
        .push(format!("recipe:{}", recipes::METRIC_TEST));
    for product in &metric_run.products {
        if product.starts_with("metric:") {
            review.products.push(product.clone());
        }
    }
    review
        .stats
        .insert("metric_test_run_id".to_owned(), metric_run.run_id.clone());
    review.stats.insert(
        "metrics_tested".to_owned(),
        metric_run
            .stats
            .get("metrics_tested")
            .cloned()
            .unwrap_or_else(|| "0".to_owned()),
    );
    if let Some(process) = review.process.as_mut() {
        for step in &mut process.steps {
            if matches!(
                step.label.as_str(),
                "bind_subjects" | "score_metrics" | "emit_decisions"
            ) {
                step.observed = true;
            }
        }
    }
    // Review completes when the empirical gate finishes.
    review.status = if metric_run.status.is_success() {
        RunStatus::Pass
    } else if matches!(metric_run.status, RunStatus::Error) {
        RunStatus::Error
    } else {
        RunStatus::Fail
    };
    review.finished_at = metric_run.finished_at;
    review.exit_code = metric_run.exit_code;
    store.put(&review)?;

    Ok((review, metric_run))
}

fn compute_one(
    id: &str,
    doc: &Document,
    coverage: &CoverageReport,
    subject: Option<&str>,
) -> Result<MetricMeasurement, String> {
    if id == ids::EDGE_RESOLVE_RATE
        && let Some(subject) = subject
    {
        let node_id = resolve_subject(doc, subject)
            .ok_or_else(|| format!("subject `{subject}` not found for edge.resolve_rate"))?;
        return metrics::edge_resolve_rate_at(doc, &node_id)
            .ok_or_else(|| format!("subject `{subject}` missing from document"));
    }
    metrics::compute(id, doc, coverage).map_err(|e| e.to_string())
}

fn resolve_subject(doc: &Document, subject: &str) -> Option<NodeId> {
    if let Some(node) = doc.nodes.get(&NodeId::new(subject)) {
        return Some(node.id.clone());
    }
    doc.nodes.values().find_map(|n| {
        if n.fqn == subject || n.id.as_str() == subject {
            Some(n.id.clone())
        } else {
            None
        }
    })
}

fn measurement_passes(m: &MetricMeasurement) -> bool {
    match m.id.as_str() {
        ids::DOC_COVERAGE_COMPLETE | ids::EDGE_RESOLVE_RATE => (m.value - 1.0).abs() < 1e-9,
        _ => true,
    }
}

fn test_id_for(metric: &str) -> String {
    format!("harness.{metric}")
}

fn metric_test_process() -> Process {
    let mut process = Process::new();
    process.participant("metric", "metric_test", Kind::from(agent::ACTION));
    process.participant("ir", "Document", Kind::from(agent::OBSERVATION));
    for label in ["load_ir", "compute", "write_run"] {
        process.step(Step {
            from: "metric".to_owned(),
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

fn summarize(observations: &[MetricMeasurement], errors: &[String]) -> String {
    let mut parts: Vec<String> = observations
        .iter()
        .map(|o| {
            format!(
                "{}={} ({})",
                o.id,
                o.score_string(),
                if measurement_passes(o) {
                    "pass"
                } else {
                    "fail"
                }
            )
        })
        .collect();
    for err in errors {
        parts.push(format!("error:{err}"));
    }
    parts.join("; ")
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

fn mint_run_id(prefix: &str) -> String {
    format!("{prefix}-{}", now_ms())
}

#[cfg(test)]
mod tests {
    use super::*;
    use robby_ir::model::{Edge, EdgeKind, Node};
    use std::fs;

    fn temp_store() -> SiteRunsStore {
        let dir = std::env::temp_dir().join(format!(
            "robby-metric-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        SiteRunsStore::new(dir)
    }

    fn tiny_doc() -> Document {
        let mut doc = Document::new("test");
        let mut a = Node::new("rust://a", robby_ir::kind::code::FUNCTION, "a");
        a.edges.push(Edge::new("rust://b", EdgeKind::Calls));
        let b = Node::new("rust://b", robby_ir::kind::code::FUNCTION, "b");
        doc.merge(a);
        doc.merge(b);
        doc.roots.push(NodeId::new("rust://a"));
        doc
    }

    fn empty_coverage() -> CoverageReport {
        CoverageReport { files: vec![] }
    }

    #[test]
    fn build_pass_for_resolved_edges() {
        let doc = tiny_doc();
        let req = MetricTestRequest {
            metrics: vec![ids::EDGE_RESOLVE_RATE.to_owned()],
            ..Default::default()
        };
        let run = build_metric_test_run(&doc, &empty_coverage(), &req).unwrap();
        assert_eq!(run.recipe.as_str(), recipes::METRIC_TEST);
        assert_eq!(run.status, RunStatus::Pass);
        assert!(run.products.iter().any(|p| p.starts_with("metric:")));
        assert_eq!(
            run.stats.get(ids::EDGE_RESOLVE_RATE).map(String::as_str),
            Some("1")
        );
    }

    #[test]
    fn allow_spawns_metric_test_product() {
        let store = temp_store();
        let queued = store
            .queue_review_view("rust://a", "frame_focus", Some("a"))
            .unwrap();
        let doc = tiny_doc();
        let (review, metric) = allow_and_spawn_metric_test(
            &store,
            &queued.run_id,
            &doc,
            &empty_coverage(),
            &[ids::EDGE_RESOLVE_RATE.to_owned()],
        )
        .unwrap();
        assert!(!SiteRunsStore::blocks_downstream(&review));
        assert!(
            review
                .products
                .iter()
                .any(|p| p.contains(recipes::METRIC_TEST) || p.starts_with("run:metric-")),
            "products={:?}",
            review.products
        );
        assert_eq!(metric.recipe.as_str(), recipes::METRIC_TEST);
        assert_eq!(
            metric.stats.get("parent_run_id").map(String::as_str),
            Some(review.run_id.as_str())
        );
        let loaded = store.read(&metric.run_id).unwrap();
        assert_eq!(loaded.run_id, metric.run_id);
    }

    #[test]
    fn cancel_still_blocks_spawn() {
        let store = temp_store();
        let queued = store
            .queue_review_view("rust://a", "frame_focus", None)
            .unwrap();
        store.cancel(&queued.run_id).unwrap();
        let doc = tiny_doc();
        let err = allow_and_spawn_metric_test(
            &store,
            &queued.run_id,
            &doc,
            &empty_coverage(),
            &[ids::EDGE_RESOLVE_RATE.to_owned()],
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("blocks downstream") || err.to_string().contains("cancelled"),
            "{err}"
        );
    }
}
