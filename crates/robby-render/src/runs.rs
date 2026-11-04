//! Site `runs/` persistence for shared [`robby_ir::RunRecord`] values.
//!
//! One JSON shape for extract / web / tests / reviews / external agents.
//! Writers put `runs/<run_id>.json` and maintain `runs/index.json`; the
//! Mac-local viewer polls the same files. Do **not** invent a parallel DTO.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
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
        record.process = Some(review_view_process(false));
        self.put(&record)?;
        Ok(record)
    }

    /// User Allow / Continue: `queued` → `running` (LLM/score may be a no-op).
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
        record.status = RunStatus::Running;
        record
            .stats
            .insert("ui_gate".to_owned(), "allowed".to_owned());
        if let Some(process) = record.process.as_mut() {
            *process = review_view_process(true);
        }
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

fn mint_run_id(prefix: &str) -> String {
    format!("{prefix}-{}", now_ms())
}

fn sort_index_newest_first(index: &mut RunsIndex) {
    index.runs.sort_by(|a, b| {
        let a_t = a.finished_at.or(a.started_at).unwrap_or(0);
        let b_t = b.finished_at.or(b.started_at).unwrap_or(0);
        b_t.cmp(&a_t)
    });
}

fn review_view_process(observed_gate: bool) -> Process {
    let mut process = Process::new();
    process.participant("review", "review_view", Kind::from(agent::ACTION));
    process.participant("ui", "ui_gate", Kind::from(agent::ACTION));
    let stages = [
        ("queue", false),
        ("await_ui_gate", observed_gate),
        ("bind_subjects", false),
        ("score_metrics", false),
        ("emit_decisions", false),
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

    #[test]
    fn queue_sampled_review_views_stamps_sample_stats() {
        use robby_ir::sample::{
            Opportunity, OpportunityReason, OpportunitySample, STRATEGY_RANDOM, STAT_SAMPLE_N,
            STAT_SAMPLE_SEED,
        };
        use robby_ir::NodeId;

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
    fn allow_moves_queued_to_running_without_metric_test() {
        let store = temp_store();
        let queued = store
            .queue_review_view("sid-c", "user", Some("Charlie"))
            .unwrap();
        let running = store.allow(&queued.run_id).unwrap();
        assert_eq!(running.status, RunStatus::Running);
        assert_eq!(
            running.stats.get("ui_gate").map(String::as_str),
            Some("allowed")
        );
        assert_eq!(running.products, Vec::<String>::new());
        assert!(!SiteRunsStore::blocks_downstream(&running));
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
