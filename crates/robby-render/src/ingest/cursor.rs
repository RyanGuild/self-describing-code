//! Cursor cloud agent → [`RunRecord`] adapter.
//!
//! Input matches Cursor MCP `list-cloud-agents` (+ optional `get-events`).
//! Primary host is the operator MacBook: dump stubs and write into site
//! `public/runs/` via [`SiteRunsStore`].

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use robby_ir::kind::agent;
use robby_ir::run::{RecipeId, RunRecord, RunStatus, recipes};
use robby_ir::{Kind, Process, Step, StepKind};

use crate::runs::SiteRunsStore;

/// One row from `list-cloud-agents` / run-info (MCP camelCase JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorAgentStub {
    pub bc_id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub created_at_ms: Option<i64>,
    #[serde(default)]
    pub updated_at_ms: Option<i64>,
    #[serde(default)]
    pub last_message_activity_at_ms: Option<i64>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub branch_name: Option<String>,
    #[serde(default)]
    pub repo_url: Option<String>,
    #[serde(default)]
    pub original_model_name: Option<String>,
    #[serde(default)]
    pub private_worker_id: Option<String>,
    #[serde(default)]
    pub use_private_worker: Option<bool>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub subject: Option<String>,
}

/// Dashboard event from `get-events` / `events.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorEvent {
    #[serde(default)]
    pub public_id: Option<String>,
    #[serde(default)]
    pub created_at_ms: Option<i64>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub log_level: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub link_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CursorIngestFile {
    #[serde(default)]
    pub agents: Vec<CursorAgentStub>,
    #[serde(default)]
    pub events: Vec<CursorEvent>,
    #[serde(default)]
    pub events_by_bc_id: BTreeMap<String, Vec<CursorEvent>>,
}

/// Map Cursor lifecycle strings onto shared [`RunStatus`].
pub fn map_cursor_status(raw: &str) -> RunStatus {
    match raw.trim().to_ascii_uppercase().as_str() {
        "RUNNING" | "WAITING_FOR_BACKGROUND_WORK" => RunStatus::Running,
        "NOT_YET_STARTED" | "UNSPECIFIED" => RunStatus::Queued,
        "IDLE" => RunStatus::Pass,
        "ARCHIVED" | "EXPIRED" => RunStatus::Cancelled,
        _ => RunStatus::Error, // includes ERROR and unknown
    }
}

pub fn cursor_run_id(bc_id: &str) -> String {
    format!("cursor:{bc_id}")
}

/// Redact common secret shapes before persisting event/summary text.
pub fn redact(text: &str) -> String {
    const MAX: usize = 2_000;
    let prefixes = [
        "sk-", "ghp_", "gho_", "ghu_", "ghs_", "ghr_", "xoxb-", "xoxp-", "xoxa-", "xoxr-",
        "Bearer ",
    ];
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let rest = &text[i..];
        let mut hit = None;
        for prefix in prefixes {
            if rest.starts_with(prefix) {
                hit = Some(prefix);
                break;
            }
        }
        if let Some(prefix) = hit {
            out.push_str(prefix);
            out.push_str("***");
            i += prefix.len();
            while i < bytes.len() {
                let c = bytes[i] as char;
                if c.is_whitespace() || c == '"' || c == '\'' || c == ',' || c == '}' {
                    break;
                }
                i += 1;
            }
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    if out.len() > MAX {
        out.truncate(MAX);
        out.push('…');
    }
    out
}

fn process_from_events(events: &[CursorEvent]) -> Process {
    let mut process = Process::new();
    if events.is_empty() {
        process.participant("create", "create", Kind::from(agent::ACTION));
        process.participant("run", "run", Kind::from(agent::ACTION));
        process.step(Step {
            from: "create".into(),
            to: "run".into(),
            label: "run".into(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
        return process;
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut ids: Vec<String> = Vec::new();
    for ev in events {
        let id = ev
            .kind
            .clone()
            .or_else(|| ev.category.clone())
            .unwrap_or_else(|| "event".into());
        let label = ev
            .title
            .clone()
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| id.clone());
        if seen.insert(id.clone()) {
            process.participant(id.clone(), label, Kind::from(agent::OBSERVATION));
        }
        ids.push(id);
    }
    if ids.len() == 1 {
        process.participant("start", "start", Kind::from(agent::ACTION));
        process.step(Step {
            from: "start".into(),
            to: ids[0].clone(),
            label: ids[0].clone(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
        return process;
    }
    for window in ids.windows(2) {
        process.step(Step {
            from: window[0].clone(),
            to: window[1].clone(),
            label: window[1].clone(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
    }
    process
}

/// Convert one Cursor agent stub (+ events) into a shared [`RunRecord`].
pub fn agent_to_run_record(agent: &CursorAgentStub, events: &[CursorEvent]) -> RunRecord {
    let status = map_cursor_status(agent.status.as_deref().unwrap_or("UNSPECIFIED"));
    let started_at = agent.created_at_ms.unwrap_or(0);
    let activity = agent
        .updated_at_ms
        .or(agent.last_message_activity_at_ms)
        .unwrap_or(started_at);

    let mut stats = BTreeMap::new();
    stats.insert("adapter".into(), "cursor.cloud".into());
    stats.insert("bc_id".into(), agent.bc_id.clone());
    if let Some(src) = &agent.source {
        stats.insert("cursor.source".into(), src.clone());
    }
    if let Some(model) = &agent.original_model_name {
        stats.insert("model".into(), model.clone());
    }
    if let Some(branch) = &agent.branch_name {
        stats.insert("branch".into(), branch.clone());
    }
    if let Some(repo) = &agent.repo_url {
        stats.insert("repo".into(), repo.clone());
    }
    if let Some(url) = &agent.url {
        stats.insert("agent_url".into(), url.clone());
    }
    if let Some(role) = &agent.role {
        stats.insert("role".into(), role.clone());
    }
    if agent.use_private_worker == Some(true) {
        stats.insert("private_worker".into(), "true".into());
    }
    if let Some(wid) = &agent.private_worker_id {
        stats.insert("private_worker_id".into(), wid.clone());
    }
    stats.insert("event_count".into(), events.len().to_string());

    let mut products = Vec::new();
    if let Some(url) = &agent.url {
        products.push(format!("url:{url}"));
    }
    if let Some(branch) = &agent.branch_name {
        products.push(format!("branch:{branch}"));
    }

    let name = agent.name.clone().unwrap_or_else(|| agent.bc_id.clone());
    let mut stdout_parts = vec![format!("cursor.cloud · {name} · {}", status.as_str())];
    for ev in events.iter().rev().take(3) {
        if let Some(title) = &ev.title {
            stdout_parts.push(redact(title));
        } else if let Some(kind) = &ev.kind {
            stdout_parts.push(redact(kind));
        }
    }

    let mut record = RunRecord::new(
        cursor_run_id(&agent.bc_id),
        RecipeId::from(recipes::CURSOR_CLOUD),
        status,
    )
    .with_subject(
        agent
            .subject
            .clone()
            .unwrap_or_else(|| "workspace:".to_owned()),
    )
    .with_process(process_from_events(events));
    record.started_at = Some(started_at);
    record.finished_at = Some(activity);
    record.stdout_tail = Some(stdout_parts.join("\n"));
    record.stats = stats;
    record.products = products;
    record
}

/// Ingest stubs into `out/runs/`, upserting by `run_id`.
pub fn ingest_cursor_agents(
    out: &Path,
    agents: &[CursorAgentStub],
    events_by_bc_id: &BTreeMap<String, Vec<CursorEvent>>,
    shared_events: &[CursorEvent],
) -> Result<Vec<RunRecord>> {
    let store = SiteRunsStore::new(out);
    store.ensure_layout()?;
    let mut records = Vec::with_capacity(agents.len());
    for agent in agents {
        let events = events_by_bc_id
            .get(&agent.bc_id)
            .map_or(shared_events, Vec::as_slice);
        let scrubbed: Vec<CursorEvent> = events
            .iter()
            .map(|ev| {
                let mut e = ev.clone();
                if let Some(d) = e.detail.take() {
                    e.detail = Some(redact(&d));
                }
                if let Some(t) = e.title.take() {
                    e.title = Some(redact(&t));
                }
                e
            })
            .collect();
        let record = agent_to_run_record(agent, &scrubbed);
        store.put(&record)?;
        records.push(record);
    }
    Ok(records)
}

/// Parse Cursor ingest JSON and upsert into `out/runs/`.
pub fn ingest_cursor_json(out: &Path, json: &str) -> Result<Vec<RunRecord>> {
    let file: CursorIngestFile = if let Ok(file) = serde_json::from_str(json) {
        file
    } else if let Ok(agents) = serde_json::from_str::<Vec<CursorAgentStub>>(json) {
        CursorIngestFile {
            agents,
            events: Vec::new(),
            events_by_bc_id: BTreeMap::new(),
        }
    } else {
        let agent: CursorAgentStub =
            serde_json::from_str(json).context("cursor ingest JSON (agents envelope or stub)")?;
        CursorIngestFile {
            agents: vec![agent],
            events: Vec::new(),
            events_by_bc_id: BTreeMap::new(),
        }
    };
    ingest_cursor_agents(out, &file.agents, &file.events_by_bc_id, &file.events)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_lifecycle_status() {
        assert_eq!(map_cursor_status("RUNNING"), RunStatus::Running);
        assert_eq!(
            map_cursor_status("WAITING_FOR_BACKGROUND_WORK"),
            RunStatus::Running
        );
        assert_eq!(map_cursor_status("IDLE"), RunStatus::Pass);
        assert_eq!(map_cursor_status("ERROR"), RunStatus::Error);
        assert_eq!(map_cursor_status("ARCHIVED"), RunStatus::Cancelled);
        assert_eq!(map_cursor_status("NOT_YET_STARTED"), RunStatus::Queued);
    }

    #[test]
    fn run_id_and_file_stem() {
        assert_eq!(cursor_run_id("bc-abc"), "cursor:bc-abc");
        assert_eq!(crate::runs::run_file_stem("cursor:bc-abc"), "cursor_bc-abc");
    }

    #[test]
    fn redacts_token_prefixes() {
        let raw = "Authorization: Bearer tokensecret and ghp_ABCDEF1234 and sk-abcXYZ";
        let scrubbed = redact(raw);
        assert!(scrubbed.contains("Bearer ***"));
        assert!(scrubbed.contains("ghp_***"));
        assert!(scrubbed.contains("sk-***"));
        assert!(!scrubbed.contains("tokensecret"));
        assert!(!scrubbed.contains("ghp_ABCDEF1234"));
        assert!(!scrubbed.contains("sk-abcXYZ"));
    }

    #[test]
    fn agent_stub_becomes_run_record() {
        let agent = CursorAgentStub {
            bc_id: "bc-test".into(),
            name: Some("Demo".into()),
            status: Some("RUNNING".into()),
            source: Some("desktop".into()),
            created_at_ms: Some(1000),
            updated_at_ms: Some(2000),
            last_message_activity_at_ms: None,
            url: Some("https://cursor.com/agents/bc-test".into()),
            branch_name: None,
            repo_url: Some("github.com/RyanGuild/self-describing-code".into()),
            original_model_name: Some("default".into()),
            private_worker_id: Some("worker-1".into()),
            use_private_worker: Some(true),
            role: Some("worker".into()),
            subject: None,
        };
        let record = agent_to_run_record(&agent, &[]);
        assert_eq!(record.run_id, "cursor:bc-test");
        assert_eq!(record.recipe.as_str(), recipes::CURSOR_CLOUD);
        assert_eq!(record.status, RunStatus::Running);
        assert_eq!(record.subject.as_deref(), Some("workspace:"));
        assert_eq!(record.started_at, Some(1000));
        assert_eq!(record.finished_at, Some(2000));
        assert_eq!(
            record.stats.get("adapter").map(String::as_str),
            Some("cursor.cloud")
        );
        assert!(
            record
                .products
                .iter()
                .any(|p| p.starts_with("url:https://cursor.com/agents/"))
        );
    }

    #[test]
    fn ingest_writes_colon_safe_files() {
        let dir = std::env::temp_dir().join(format!(
            "robby-cursor-ingest-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let json = r#"{
          "agents": [{
            "bcId": "bc-file",
            "name": "File",
            "status": "RUNNING",
            "createdAtMs": 1,
            "updatedAtMs": 2
          }]
        }"#;
        let records = ingest_cursor_json(&dir, json).unwrap();
        assert_eq!(records.len(), 1);
        assert!(dir.join("runs/cursor_bc-file.json").is_file());
        assert!(!dir.join("runs/cursor:bc-file.json").exists());
        let store = SiteRunsStore::new(&dir);
        let loaded = store.read("cursor:bc-file").unwrap();
        assert_eq!(loaded.status, RunStatus::Running);
    }
}
