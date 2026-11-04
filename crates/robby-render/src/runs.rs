//! Site-facing [`RunRecord`] artifacts (`public/runs/`).
//!
//! Same JSON shape as the pipeline plan: recipe → run → products. The first
//! slice writes completed extract + web rows when `robby web` regenerates the
//! site so the viewer Runs mode is never empty of history.

use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use robby_ir::Document;

/// Canonical run status strings (tests + pipelines share these).
pub mod status {
    pub const QUEUED: &str = "queued";
    pub const RUNNING: &str = "running";
    pub const PASS: &str = "pass";
    pub const FAIL: &str = "fail";
    pub const ERROR: &str = "error";
    pub const CANCELLED: &str = "cancelled";
}

/// First-party recipe ids.
pub mod recipe {
    pub const EXTRACT: &str = "robby.extract";
    pub const WEB: &str = "robby.web";
}

/// One invocation — IR `agent.run` payload and/or `runs/<id>.json`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunRecord {
    pub run_id: String,
    pub recipe: String,
    pub subject: String,
    pub status: String,
    pub started_at: u64,
    pub finished_at: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout_tail: Option<String>,
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub stats: serde_json::Map<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<RunProcess>,
}

/// Coarse stage timeline for the invocation itself (not the subject body).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunProcess {
    pub participants: Vec<RunParticipant>,
    pub steps: Vec<RunStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunParticipant {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunStep {
    pub from: String,
    pub to: String,
    pub label: String,
    #[serde(default)]
    pub observed: bool,
}

/// Compact row for `runs/index.json` (viewer list).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunIndexEntry {
    pub run_id: String,
    pub recipe: String,
    pub subject: String,
    pub status: String,
    pub started_at: u64,
    pub finished_at: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunsIndex {
    pub runs: Vec<RunIndexEntry>,
}

impl RunRecord {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("run record serializes")
    }

    pub fn index_entry(&self) -> RunIndexEntry {
        RunIndexEntry {
            run_id: self.run_id.clone(),
            recipe: self.recipe.clone(),
            subject: self.subject.clone(),
            status: self.status.clone(),
            started_at: self.started_at,
            finished_at: self.finished_at,
        }
    }
}

impl RunsIndex {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("runs index serializes")
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn stage_process(stages: &[&str]) -> RunProcess {
    let participants: Vec<RunParticipant> = stages
        .iter()
        .map(|s| RunParticipant {
            id: (*s).to_owned(),
            label: (*s).to_owned(),
        })
        .collect();
    let mut steps = Vec::new();
    for window in stages.windows(2) {
        steps.push(RunStep {
            from: window[0].to_owned(),
            to: window[1].to_owned(),
            label: window[1].to_owned(),
            observed: true,
        });
    }
    RunProcess {
        participants,
        steps,
    }
}

/// Synthetic completed extract from the loaded Document (retrospective history).
pub fn synthetic_extract_run(doc: &Document, finished_at: u64) -> RunRecord {
    let started_at = finished_at.saturating_sub(5_000);
    let mut stats = serde_json::Map::new();
    stats.insert("nodes".into(), serde_json::json!(doc.nodes.len()));
    stats.insert("files".into(), serde_json::json!(doc.files.len()));
    stats.insert("roots".into(), serde_json::json!(doc.roots.len()));
    stats.insert("generator".into(), serde_json::json!(doc.generator));
    stats.insert("synthetic".into(), serde_json::json!(true));

    RunRecord {
        run_id: "synthetic-extract".to_owned(),
        recipe: recipe::EXTRACT.to_owned(),
        subject: "workspace:".to_owned(),
        status: status::PASS.to_owned(),
        started_at,
        finished_at,
        exit_code: Some(0),
        stdout_tail: Some(format!(
            "synthetic robby.extract from {} · {} nodes · {} files",
            doc.generator,
            doc.nodes.len(),
            doc.files.len()
        )),
        stats,
        products: vec!["path:.robby/robby.ir.json".to_owned()],
        process: Some(stage_process(&[
            "extract",
            "validate",
            "write_document",
        ])),
    }
}

/// Completed `robby.web` run for this site write.
pub fn web_build_run(doc: &Document, frames: usize, finished_at: u64) -> RunRecord {
    let started_at = finished_at.saturating_sub(500);
    let mut stats = serde_json::Map::new();
    stats.insert("frames".into(), serde_json::json!(frames));
    stats.insert("nodes".into(), serde_json::json!(doc.nodes.len()));
    stats.insert("files".into(), serde_json::json!(doc.files.len()));

    RunRecord {
        run_id: "latest-web".to_owned(),
        recipe: recipe::WEB.to_owned(),
        subject: "workspace:".to_owned(),
        status: status::PASS.to_owned(),
        started_at,
        finished_at,
        exit_code: Some(0),
        stdout_tail: Some(format!("wrote {frames} frame(s) to site public/")),
        stats,
        products: vec![
            "path:apps/site/public/scene.json".to_owned(),
            "path:apps/site/public/build.json".to_owned(),
            "path:apps/site/public/runs/index.json".to_owned(),
        ],
        process: Some(stage_process(&["load_ir", "scene", "frames", "runs"])),
    }
}

/// Write `runs/<id>.json` + `runs/index.json` (newest first).
pub fn write_runs(out: &Path, records: &[RunRecord]) -> Result<()> {
    let runs_dir = out.join("runs");
    fs::create_dir_all(&runs_dir)?;

    let mut index = RunsIndex { runs: Vec::new() };
    for record in records {
        fs::write(
            runs_dir.join(format!("{}.json", record.run_id)),
            record.to_json(),
        )?;
        index.runs.push(record.index_entry());
    }
    // Newest finished_at first for the list.
    index.runs.sort_by(|a, b| b.finished_at.cmp(&a.finished_at));
    fs::write(runs_dir.join("index.json"), index.to_json())?;
    Ok(())
}

/// Emit extract + web RunRecords under `out/runs` for a site rebuild.
pub fn write_site_runs(doc: &Document, out: &Path, frames: usize, rebuilt_at: u64) -> Result<()> {
    let finished = if rebuilt_at == 0 { now_ms() } else { rebuilt_at };
    let extract = synthetic_extract_run(doc, finished.saturating_sub(1_000));
    let web = web_build_run(doc, frames, finished);
    write_runs(out, &[extract, web])
}
