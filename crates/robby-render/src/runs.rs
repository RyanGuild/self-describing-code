//! Site `runs/` persistence for shared [`robby_ir::RunRecord`] values.
//!
//! One JSON shape for extract / web / tests / reviews / external agents.
//! Writers put `runs/<run_id>.json` and maintain `runs/index.json`; the
//! Mac-local viewer polls the same files. Do **not** invent a parallel DTO.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use robby_ir::run::{RecipeId, RunRecord, RunStatus, recipes};
use robby_ir::sample::OpportunitySample;
use robby_ir::{Document, Kind, Process, Step, StepKind, kind::agent};

/// Thin index row so the dock can list without fetching every record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunIndexEntry {
    pub run_id: String,
    pub recipe: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
}

impl From<&RunRecord> for RunIndexEntry {
    fn from(record: &RunRecord) -> Self {
        Self {
            run_id: record.run_id.clone(),
            recipe: record.recipe.as_str().to_owned(),
            status: record.status.as_str().to_owned(),
            subject: record.subject.clone(),
            started_at: record.started_at,
            finished_at: record.finished_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RunsIndex {
    #[serde(default)]
    pub runs: Vec<RunIndexEntry>,
}

impl RunsIndex {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("runs index serializes")
    }
}

/// Filesystem-backed store under a site `public/` (or any out) directory.
#[derive(Debug, Clone)]
pub struct SiteRunsStore {
    root: PathBuf,
}

impl SiteRunsStore {
    pub fn new(site_out: impl Into<PathBuf>) -> Self {
        Self {
            root: site_out.into(),
        }
    }

    pub fn runs_dir(&self) -> PathBuf {
        self.root.join("runs")
    }

    pub fn index_path(&self) -> PathBuf {
        self.runs_dir().join("index.json")
    }

    pub fn record_path(&self, run_id: &str) -> PathBuf {
        self.runs_dir()
            .join(format!("{}.json", run_file_stem(run_id)))
    }

    /// Ensure `runs/` + empty index exist (idempotent; does not wipe records).
    pub fn ensure_layout(&self) -> Result<()> {
        fs::create_dir_all(self.runs_dir()).context("create runs/")?;
        let index_path = self.index_path();
        if !index_path.exists() {
            self.write_index(&RunsIndex::default())?;
        }
        Ok(())
    }

    pub fn read_index(&self) -> Result<RunsIndex> {
        let path = self.index_path();
        if !path.exists() {
            return Ok(RunsIndex::default());
        }
        let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))
    }

    pub fn write_index(&self, index: &RunsIndex) -> Result<()> {
        fs::create_dir_all(self.runs_dir())?;
        let raw = serde_json::to_string_pretty(index)?;
        fs::write(self.index_path(), format!("{raw}\n"))?;
        Ok(())
    }

    pub fn read(&self, run_id: &str) -> Result<RunRecord> {
        let path = self.record_path(run_id);
        let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))
    }

    pub fn put(&self, record: &RunRecord) -> Result<()> {
        self.ensure_layout()?;
        let path = self.record_path(&record.run_id);
        let raw = serde_json::to_string_pretty(record)?;
        fs::write(&path, format!("{raw}\n"))
            .with_context(|| format!("write {}", path.display()))?;

        let mut index = self.read_index()?;
        let entry = RunIndexEntry::from(record);
        if let Some(slot) = index.runs.iter_mut().find(|e| e.run_id == record.run_id) {
            *slot = entry;
        } else {
            index.runs.insert(0, entry);
        }
        sort_index_newest_first(&mut index);
        self.write_index(&index)?;
        Ok(())
    }

    /// Soft-queue `robby.review_view` for each subject in a seeded opportunity
    /// sample (not every IR node). Copies `sample_seed` / `sample_n` stats onto
    /// every queued record so Allow → metric_test can stay scoped to the draw.
    pub fn queue_sampled_review_views(
        &self,
        sample: &OpportunitySample,
        trigger: &str,
    ) -> Result<Vec<RunRecord>> {
        let sample_stats = sample.stats();
        let mut out = Vec::with_capacity(sample.subjects.len());
        for opportunity in &sample.subjects {
            let mut record = self.queue_review_view(
                opportunity.subject.as_str(),
                trigger,
                Some(opportunity.name.as_str()),
            )?;
            for (key, value) in &sample_stats {
                record.stats.insert(key.clone(), value.clone());
            }
            record.stats.insert(
                "opportunity_reason".to_owned(),
                opportunity.reason.as_str().to_owned(),
            );
            self.put(&record)?;
            out.push(record);
        }
        Ok(out)
    }

    /// Soft-queue a `robby.review_view` for `subject` (one pending per subject).
    pub fn queue_review_view(
        &self,
        subject: &str,
        trigger: &str,
        subject_name: Option<&str>,
    ) -> Result<RunRecord> {
        self.ensure_layout()?;
        let index = self.read_index()?;
        for entry in &index.runs {
            if entry.recipe != recipes::REVIEW_VIEW {
                continue;
            }
            if entry.subject.as_deref() != Some(subject) {
                continue;
            }
            if entry.status != RunStatus::Queued.as_str() {
                continue;
            }
            let mut existing = self.read(&entry.run_id)?;
            let now = now_ms();
            existing.started_at = Some(now);
            existing
                .stats
                .insert("trigger".to_owned(), trigger.to_owned());
            existing
                .stats
                .insert("ui_gate".to_owned(), "awaiting_allow".to_owned());
            if let Some(name) = subject_name {
                existing
                    .stats
                    .insert("subject_name".to_owned(), name.to_owned());
            }
            self.put(&existing)?;
            return Ok(existing);
        }

        let run_id = mint_run_id("review");
        let mut record = RunRecord::new(
            run_id,
            RecipeId::from(recipes::REVIEW_VIEW),
            RunStatus::Queued,
        )
        .with_subject(subject.to_owned());
        record.started_at = Some(now_ms());
        record
            .stats
            .insert("trigger".to_owned(), trigger.to_owned());
        record
            .stats
            .insert("ui_gate".to_owned(), "awaiting_allow".to_owned());
        if let Some(name) = subject_name {
            record
                .stats
                .insert("subject_name".to_owned(), name.to_owned());
        }
        record.process = Some(review_view_process(false, false));
        self.put(&record)?;
        Ok(record)
    }

    /// User Allow / Continue: gate → bind/score/emit scored `agent.decision` products.
    ///
    /// Completes the review as `pass` with metric annotations (`metric`,
    /// `test_id`, `score`). Does **not** spawn `robby.metric_test` (downstream
    /// of this gate stays optional / separate).
    pub fn allow(&self, run_id: &str) -> Result<RunRecord> {
        let mut record = self.read(run_id)?;
        if record.status == RunStatus::Cancelled {
            bail!("run {run_id} is cancelled");
        }
        if record.status.is_terminal() {
            bail!(
                "run {run_id} is already terminal ({})",
                record.status.as_str()
            );
        }
        record
            .stats
            .insert("ui_gate".to_owned(), "allowed".to_owned());

        let decisions = scored_review_decisions(&record);
        let decisions_name = format!("{run_id}.decisions.json");
        let decisions_path = self.runs_dir().join(&decisions_name);
        let raw = serde_json::to_string_pretty(&decisions)?;
        fs::write(&decisions_path, format!("{raw}\n"))
            .with_context(|| format!("write {}", decisions_path.display()))?;

        record.products = decisions
            .iter()
            .map(|d| format!("ir:agent.decision:{}", d.metric))
            .collect();
        record.products.push(format!("path:runs/{decisions_name}"));
        record
            .stats
            .insert("findings".to_owned(), decisions.len().to_string());
        record
            .stats
            .insert("metrics_claimed".to_owned(), decisions.len().to_string());
        for d in &decisions {
            record
                .stats
                .insert(format!("metric.{}", d.metric), d.score.clone());
        }

        record.process = Some(review_view_process(true, true));
        record.status = RunStatus::Pass;
        record.finished_at = Some(now_ms());
        record.exit_code = Some(0);
        record.stdout_tail = Some(format!(
            "review_view emitted {} scored agent.decision product(s)",
            decisions.len()
        ));
        self.put(&record)?;
        Ok(record)
    }

    /// User Cancel: terminal `cancelled`; never spawn metric_test / enrich.
    pub fn cancel(&self, run_id: &str) -> Result<RunRecord> {
        let mut record = self.read(run_id)?;
        if record.status == RunStatus::Cancelled {
            return Ok(record);
        }
        if record.status.is_terminal() {
            bail!(
                "run {run_id} is already terminal ({})",
                record.status.as_str()
            );
        }
        record.status = RunStatus::Cancelled;
        record.finished_at = Some(now_ms());
        record
            .stats
            .insert("ui_gate".to_owned(), "cancelled".to_owned());
        record.products.clear();
        self.put(&record)?;
        Ok(record)
    }

    /// True when this review must not start `robby.metric_test` / enrich-like work.
    pub fn blocks_downstream(record: &RunRecord) -> bool {
        matches!(
            record.status,
            RunStatus::Queued | RunStatus::Cancelled | RunStatus::Error
        ) || record
            .stats
            .get("ui_gate")
            .is_some_and(|g| g == "awaiting_allow" || g == "paused" || g == "cancelled")
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

/// Monotonic suffix so burst mints in the same millisecond never share a path.
///
/// Earlier `prefix-{ms}` collided under `sample-opportunities --queue` (N>1),
/// overwriting `runs/review-<ms>.json` and losing subjects.
static RUN_ID_SEQ: AtomicU64 = AtomicU64::new(0);

/// Mint a filesystem-safe run id unique under same-ms burst minting.
///
/// Format: `{prefix}-{unix_ms}-{seq}` — opaque to consumers (lookup / filename only).
pub fn mint_run_id(prefix: &str) -> String {
    let seq = RUN_ID_SEQ.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}-{}-{seq}", now_ms())
}

fn sort_index_newest_first(index: &mut RunsIndex) {
    index.runs.sort_by(|a, b| {
        let a_t = a.finished_at.or(a.started_at).unwrap_or(0);
        let b_t = b.finished_at.or(b.started_at).unwrap_or(0);
        b_t.cmp(&a_t)
    });
}

/// Scored `agent.decision` product for a view review (site JSON + RunRecord products).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredDecisionProduct {
    pub kind: String,
    pub verdict: String,
    pub metric: String,
    pub score: String,
    pub test_id: String,
    pub direction: String,
    pub subject: String,
    pub run_id: String,
    pub trigger: String,
    pub source_recipe: String,
    pub reason: String,
}

/// Catalog ids + harness names used when Allow scores a `robby.review_view`.
pub mod review_metrics {
    pub const UI_RUNRECORD_SHAPE: &str = "ui.runrecord_shape_compliance";
    pub const UI_RUNRECORD_SHAPE_TEST: &str = "metric_test.ui.runrecord_shape_compliance";
    pub const AGENT_PROCESS_STAGE: &str = "agent.action_or_process_stage_rate";
    pub const AGENT_PROCESS_STAGE_TEST: &str = "metric_test.agent.action_or_process_stage_rate";
}

/// Scripted scores measurable from the RunRecord itself (no IR load required).
fn scored_review_decisions(record: &RunRecord) -> Vec<ScoredDecisionProduct> {
    let subject = record
        .subject
        .clone()
        .unwrap_or_else(|| "workspace:".to_owned());
    let trigger = record
        .stats
        .get("trigger")
        .cloned()
        .unwrap_or_else(|| "frame_focus".to_owned());
    let stage_rate = if record.process.as_ref().is_some_and(|p| !p.steps.is_empty()) {
        "1"
    } else {
        "0"
    };

    vec![
        ScoredDecisionProduct {
            kind: agent::DECISION.to_owned(),
            verdict: "relevant".to_owned(),
            metric: review_metrics::UI_RUNRECORD_SHAPE.to_owned(),
            score: "1".to_owned(),
            test_id: review_metrics::UI_RUNRECORD_SHAPE_TEST.to_owned(),
            direction: "higher_better".to_owned(),
            subject: subject.clone(),
            run_id: record.run_id.clone(),
            trigger: trigger.clone(),
            source_recipe: recipes::REVIEW_VIEW.to_owned(),
            reason: "Allow gate produced a shared RunRecord-shaped review product".to_owned(),
        },
        ScoredDecisionProduct {
            kind: agent::DECISION.to_owned(),
            verdict: "relevant".to_owned(),
            metric: review_metrics::AGENT_PROCESS_STAGE.to_owned(),
            score: stage_rate.to_owned(),
            test_id: review_metrics::AGENT_PROCESS_STAGE_TEST.to_owned(),
            direction: "higher_better".to_owned(),
            subject,
            run_id: record.run_id.clone(),
            trigger,
            source_recipe: recipes::REVIEW_VIEW.to_owned(),
            reason: "Review Process lists bind_subjects / score_metrics / emit_decisions"
                .to_owned(),
        },
    ]
}

fn review_view_process(observed_gate: bool, observed_score: bool) -> Process {
    let mut process = Process::new();
    process.participant("review", "review_view", Kind::from(agent::ACTION));
    process.participant("ui", "ui_gate", Kind::from(agent::ACTION));
    let stages = [
        ("queue", observed_gate || observed_score),
        ("await_ui_gate", observed_gate || observed_score),
        ("bind_subjects", observed_score),
        ("score_metrics", observed_score),
        ("emit_decisions", observed_score),
    ];
    for (label, observed) in stages {
        process.step(Step {
            from: "review".to_owned(),
            to: "ui".to_owned(),
            label: label.to_owned(),
            kind: StepKind::Call,
            observed,
            span: None,
            blocks: vec![],
        });
    }
    process
}

fn stage_process(stages: &[&str]) -> Process {
    let mut process = Process::new();
    for stage in stages {
        process.participant(*stage, *stage, Kind::from(agent::ACTION));
    }
    for window in stages.windows(2) {
        process.step(Step {
            from: window[0].to_owned(),
            to: window[1].to_owned(),
            label: window[1].to_owned(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
    }
    process
}

/// Filesystem-safe stem for `runs/<stem>.json`.
///
/// `run_id` may contain `:` (e.g. `cursor:bc-…`); Vite/static servers do not
/// reliably serve colon filenames, so on-disk names map `:` → `_`.
pub fn run_file_stem(run_id: &str) -> String {
    run_id.replace(':', "_")
}

/// Ensure empty `runs/index.json` beside scene payloads.
pub fn ensure_runs_layout(site_out: &Path) -> Result<()> {
    SiteRunsStore::new(site_out).ensure_layout()
}

/// Synthetic completed extract from the loaded Document (retrospective history).
pub fn synthetic_extract_run(doc: &Document, finished_at: i64) -> RunRecord {
    let started_at = finished_at.saturating_sub(5_000);
    let mut stats = BTreeMap::new();
    stats.insert("nodes".to_owned(), doc.nodes.len().to_string());
    stats.insert("files".to_owned(), doc.files.len().to_string());
    stats.insert("roots".to_owned(), doc.roots.len().to_string());
    stats.insert("generator".to_owned(), doc.generator.clone());
    stats.insert("synthetic".to_owned(), "true".to_owned());

    let mut record = RunRecord::new(
        "synthetic-extract",
        RecipeId::from(recipes::EXTRACT),
        RunStatus::Pass,
    )
    .with_subject("workspace:")
    .with_process(stage_process(&["extract", "validate", "write_document"]));
    record.started_at = Some(started_at);
    record.finished_at = Some(finished_at);
    record.exit_code = Some(0);
    record.stdout_tail = Some(format!(
        "synthetic robby.extract from {} · {} nodes · {} files",
        doc.generator,
        doc.nodes.len(),
        doc.files.len()
    ));
    record.stats = stats;
    record.products = vec!["path:.robby/robby.ir.json".to_owned()];
    record
}

/// Completed `robby.web` run for this site write.
pub fn web_build_run(doc: &Document, frames: usize, finished_at: i64) -> RunRecord {
    let started_at = finished_at.saturating_sub(500);
    let mut stats = BTreeMap::new();
    stats.insert("frames".to_owned(), frames.to_string());
    stats.insert("nodes".to_owned(), doc.nodes.len().to_string());
    stats.insert("files".to_owned(), doc.files.len().to_string());

    let mut record = RunRecord::new("latest-web", RecipeId::from(recipes::WEB), RunStatus::Pass)
        .with_subject("workspace:")
        .with_process(stage_process(&["load_ir", "scene", "frames", "runs"]));
    record.started_at = Some(started_at);
    record.finished_at = Some(finished_at);
    record.exit_code = Some(0);
    record.stdout_tail = Some(format!("wrote {frames} frame(s) to site public/"));
    record.stats = stats;
    record.products = vec![
        "path:apps/site/public/scene.json".to_owned(),
        "path:apps/site/public/build.json".to_owned(),
        "path:apps/site/public/runs/index.json".to_owned(),
    ];
    record
}

/// Write `runs/<id>.json` for each record and refresh those index rows.
///
/// Existing index entries whose `run_id` is not in `records` are preserved
/// (so a site rebuild does not wipe queued reviews).
pub fn write_runs(out: &Path, records: &[RunRecord]) -> Result<()> {
    let store = SiteRunsStore::new(out);
    store.ensure_layout()?;
    let keep_ids: std::collections::BTreeSet<&str> =
        records.iter().map(|r| r.run_id.as_str()).collect();

    let mut index = store.read_index()?;
    index.runs.retain(|e| !keep_ids.contains(e.run_id.as_str()));

    for record in records {
        let path = store.record_path(&record.run_id);
        let raw = serde_json::to_string_pretty(record)?;
        fs::write(&path, format!("{raw}\n"))
            .with_context(|| format!("write {}", path.display()))?;
        index.runs.push(RunIndexEntry::from(record));
    }
    sort_index_newest_first(&mut index);
    store.write_index(&index)?;
    Ok(())
}

/// Emit extract + web RunRecords under `out/runs` for a site rebuild.
pub fn write_site_runs(doc: &Document, out: &Path, frames: usize, rebuilt_at: u64) -> Result<()> {
    let finished = if rebuilt_at == 0 {
        now_ms()
    } else {
        rebuilt_at as i64
    };
    let extract = synthetic_extract_run(doc, finished.saturating_sub(1_000));
    let web = web_build_run(doc, frames, finished);
    write_runs(out, &[extract, web])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> SiteRunsStore {
        let dir = std::env::temp_dir().join(format!(
            "robby-runs-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        SiteRunsStore::new(dir)
    }

    #[test]
    fn queue_dedupes_pending_subject() {
        let store = temp_store();
        let a = store
            .queue_review_view("sid-a", "frame_focus", Some("Alpha"))
            .unwrap();
        let b = store
            .queue_review_view("sid-a", "frame_focus", Some("Alpha"))
            .unwrap();
        assert_eq!(a.run_id, b.run_id);
        assert_eq!(b.status, RunStatus::Queued);
        assert_eq!(b.recipe.as_str(), recipes::REVIEW_VIEW);
        let index = store.read_index().unwrap();
        assert_eq!(index.runs.len(), 1);
    }

    /// Regression: `prefix-{ms}` alone overwrote files when sample-opportunities
    /// queued N>1 subjects in the same millisecond (trial seed 42 lost 2/8).
    #[test]
    fn mint_run_id_unique_under_same_ms_burst() {
        let n = 32;
        let mut ids = std::collections::BTreeSet::new();
        let mut stems = std::collections::BTreeSet::new();
        for _ in 0..n {
            let id = mint_run_id("review");
            assert!(
                ids.insert(id.clone()),
                "duplicate run_id under burst mint: {id}"
            );
            assert!(
                stems.insert(run_file_stem(&id)),
                "duplicate file stem under burst mint: {id}"
            );
        }
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn queue_burst_writes_distinct_run_files() {
        let store = temp_store();
        let n = 16;
        let mut ids = std::collections::BTreeSet::new();
        for i in 0..n {
            let record = store
                .queue_review_view(&format!("sid-burst-{i}"), "burst", Some("Burst"))
                .unwrap();
            assert!(
                ids.insert(record.run_id.clone()),
                "duplicate run_id: {}",
                record.run_id
            );
            assert!(
                store.record_path(&record.run_id).is_file(),
                "missing file for {}",
                record.run_id
            );
        }
        let index = store.read_index().unwrap();
        assert_eq!(index.runs.len(), n);
        assert_eq!(ids.len(), n);
        let on_disk: usize = fs::read_dir(store.runs_dir())
            .unwrap()
            .filter_map(Result::ok)
            .filter(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("review-") && n.ends_with(".json"))
            })
            .count();
        assert_eq!(on_disk, n);
    }

    #[test]
    fn queue_sampled_review_views_stamps_sample_stats() {
        use robby_ir::NodeId;
        use robby_ir::sample::{
            Opportunity, OpportunityReason, OpportunitySample, STAT_SAMPLE_N, STAT_SAMPLE_SEED,
            STRATEGY_RANDOM,
        };

        let store = temp_store();
        let sample = OpportunitySample {
            seed: 42,
            n: 2,
            strategy: STRATEGY_RANDOM.to_owned(),
            candidate_count: 10,
            subjects: vec![
                Opportunity {
                    subject: NodeId::from("rust://a"),
                    reason: OpportunityReason::WorkspaceRoot,
                    name: "a".into(),
                },
                Opportunity {
                    subject: NodeId::from("rust://b"),
                    reason: OpportunityReason::FunctionWithProcess,
                    name: "b".into(),
                },
            ],
        };
        let records = store
            .queue_sampled_review_views(&sample, "cli.sample_opportunities")
            .unwrap();
        assert_eq!(records.len(), 2);
        for record in &records {
            assert_eq!(
                record.stats.get(STAT_SAMPLE_SEED).map(String::as_str),
                Some("42")
            );
            assert_eq!(
                record.stats.get(STAT_SAMPLE_N).map(String::as_str),
                Some("2")
            );
            assert!(record.stats.contains_key("opportunity_reason"));
        }
    }

    #[test]
    fn cancel_blocks_metric_test_products() {
        let store = temp_store();
        let queued = store
            .queue_review_view("sid-b", "frame_focus", None)
            .unwrap();
        let cancelled = store.cancel(&queued.run_id).unwrap();
        assert_eq!(cancelled.status, RunStatus::Cancelled);
        assert_eq!(cancelled.products, Vec::<String>::new());
        assert!(SiteRunsStore::blocks_downstream(&cancelled));
        assert!(
            !cancelled
                .products
                .iter()
                .any(|p| p.contains(recipes::METRIC_TEST))
        );
    }

    #[test]
    fn allow_emits_scored_decisions_without_metric_test() {
        let store = temp_store();
        let queued = store
            .queue_review_view("sid-c", "user", Some("Charlie"))
            .unwrap();
        let allowed = store.allow(&queued.run_id).unwrap();
        assert_eq!(allowed.status, RunStatus::Pass);
        assert_eq!(
            allowed.stats.get("ui_gate").map(String::as_str),
            Some("allowed")
        );
        assert!(!SiteRunsStore::blocks_downstream(&allowed));
        assert!(
            allowed
                .products
                .iter()
                .any(|p| p == &format!("ir:agent.decision:{}", review_metrics::UI_RUNRECORD_SHAPE)),
            "expected scored decision product, got {:?}",
            allowed.products
        );
        assert!(
            allowed
                .products
                .iter()
                .any(|p| p.contains(&format!("{}.decisions.json", allowed.run_id))),
            "expected decisions sidecar path, got {:?}",
            allowed.products
        );
        assert!(
            !allowed
                .products
                .iter()
                .any(|p| p.contains(recipes::METRIC_TEST)),
            "Allow must not spawn metric_test products yet"
        );

        let process = allowed.process.as_ref().expect("process");
        for label in ["bind_subjects", "score_metrics", "emit_decisions"] {
            assert!(
                process.steps.iter().any(|s| s.label == label && s.observed),
                "stage {label} should be observed after Allow"
            );
        }

        let sidecar = store
            .runs_dir()
            .join(format!("{}.decisions.json", allowed.run_id));
        let raw = fs::read_to_string(&sidecar).unwrap();
        let decisions: Vec<ScoredDecisionProduct> = serde_json::from_str(&raw).unwrap();
        assert_eq!(decisions.len(), 2);
        for d in &decisions {
            assert_eq!(d.kind, agent::DECISION);
            assert_eq!(d.source_recipe, recipes::REVIEW_VIEW);
            assert_eq!(d.run_id, allowed.run_id);
            assert_eq!(d.subject, "sid-c");
            assert_ne!(d.metric, "");
            assert_ne!(d.test_id, "");
            assert_ne!(d.score, "");
        }
        assert_eq!(
            allowed
                .stats
                .get(&format!("metric.{}", review_metrics::UI_RUNRECORD_SHAPE))
                .map(String::as_str),
            Some("1")
        );
    }

    #[test]
    fn write_site_runs_preserves_other_reviews() {
        let dir = std::env::temp_dir().join(format!(
            "robby-runs-preserve-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let store = SiteRunsStore::new(&dir);
        let review = store
            .queue_review_view("sid-keep", "frame_focus", None)
            .unwrap();

        let doc = Document::new("test");
        write_site_runs(&doc, &dir, 3, 1_000).unwrap();

        let index = store.read_index().unwrap();
        assert!(
            index.runs.iter().any(|e| e.run_id == review.run_id),
            "review should survive site rebuild, got {:?}",
            index.runs
        );
        assert!(index.runs.iter().any(|e| e.run_id == "synthetic-extract"));
        assert!(index.runs.iter().any(|e| e.run_id == "latest-web"));
    }
}
