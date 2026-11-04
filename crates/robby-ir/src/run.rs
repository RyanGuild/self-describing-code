//! Pipeline run records and recipes.
//!
//! One JSON shape for invoke / store / watch (IR `agent.run` annotations and
//! site `runs/<id>.json`). See Project `docs/pipeline-agent-decision-types.md`.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::process::Process;

/// Stable recipe identifier, e.g. `robby.extract` or `cargo.test`.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct RecipeId(String);

impl RecipeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RecipeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for RecipeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for RecipeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Shared status for tests, extract, enrich, and future trace runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RunStatus {
    Queued,
    Running,
    Pass,
    Fail,
    Error,
    Cancelled,
}

impl RunStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Error => "error",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "queued" => Some(Self::Queued),
            "running" => Some(Self::Running),
            "pass" => Some(Self::Pass),
            "fail" => Some(Self::Fail),
            "error" => Some(Self::Error),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }

    /// Terminal success (gate / runner completed ok).
    pub fn is_success(self) -> bool {
        matches!(self, Self::Pass)
    }

    /// Terminal outcome (success or failure), not in-flight.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Pass | Self::Fail | Self::Error | Self::Cancelled
        )
    }
}

/// Catalog entry for an invokable procedure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Recipe {
    pub id: RecipeId,
    pub label: String,
    /// Policy / allowlist key for local invoke (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowlist: Option<String>,
}

/// One invocation: past, live, or queued.
///
/// Persist the same shape as an `agent.run` node payload and/or
/// `runs/<run_id>.json`. Optional [`Process`] is the **run** stage timeline
/// (extract → check → write), not the subject function body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RunRecord {
    pub run_id: String,
    pub recipe: RecipeId,
    /// Subject node id or stable id (tests).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub status: RunStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdout_tail: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub stats: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
}

impl RunRecord {
    pub fn new(run_id: impl Into<String>, recipe: impl Into<RecipeId>, status: RunStatus) -> Self {
        Self {
            run_id: run_id.into(),
            recipe: recipe.into(),
            subject: None,
            status,
            started_at: None,
            finished_at: None,
            exit_code: None,
            stdout_tail: None,
            stats: BTreeMap::new(),
            products: Vec::new(),
            process: None,
        }
    }

    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn with_process(mut self, process: Process) -> Self {
        self.process = Some(process);
        self
    }
}

/// Well-known recipe ids (first set + CI gates).
pub mod recipes {
    pub const EXTRACT: &str = "robby.extract";
    pub const CHECK: &str = "robby.check";
    pub const WEB: &str = "robby.web";
    pub const ENRICH_COMMENTS: &str = "robby.enrich_comments";
    pub const CARGO_TEST: &str = "cargo.test";
    pub const TRACE: &str = "robby.trace";
    /// View-triggered self-improvement review (map focus / rebuild).
    pub const REVIEW_VIEW: &str = "robby.review_view";
    /// Markdown-doc review (metrics loop sibling of [`REVIEW_VIEW`]).
    pub const REVIEW_DOC: &str = "robby.review_doc";
    /// Empirical gate spawned only after an allowed review.
    pub const METRIC_TEST: &str = "robby.metric_test";
    /// First model: propose a source patch for claimed metrics.
    pub const IMPROVE_PATCH: &str = "robby.improve_patch";
    /// Second, different model: review/integrate the proposed patch.
    pub const INTEGRATE_PATCH: &str = "robby.integrate_patch";

    /// Parent / umbrella for the GitHub Actions `rust` job.
    pub const CI_RUST: &str = "ci.rust";
    pub const CI_FMT: &str = "ci.fmt";
    pub const CI_CLIPPY: &str = "ci.clippy";
    pub const CI_TEST: &str = "ci.test";
    /// Generated paths stay untracked; working tree clean after CI.
    pub const CI_ARTIFACTS: &str = "ci.artifacts";
    /// Site biome + `tsc --noEmit` (`npm run check`).
    pub const CI_SITE: &str = "ci.site";

    /// Cursor cloud / Project worker (poll MCP / API → site `runs/`).
    pub const CURSOR_CLOUD: &str = "cursor.cloud";
    /// Cursor desktop / local agent session files (when available).
    pub const CURSOR_LOCAL: &str = "cursor.local";
    pub const OPENCODE_SESSION: &str = "opencode.session";
    pub const CLAUDE_CODE: &str = "claude.code";
    pub const CODEX_SESSION: &str = "codex.session";
    pub const EXTERNAL_GENERIC: &str = "external.generic";
}

/// First-party recipe catalog (pipelines.json / `agent.pipeline` ids).
///
/// CI recipes map 1:1 to steps in `.github/workflows/ci.yml` so Actions runs
/// can later be absorbed as [`RunRecord`]s with the same `recipe` strings.
pub fn builtin_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            id: RecipeId::from(recipes::EXTRACT),
            label: "Extract IR".into(),
            allowlist: Some("local".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CHECK),
            label: "Check IR".into(),
            allowlist: Some("local".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::WEB),
            label: "Write site".into(),
            allowlist: Some("local".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::ENRICH_COMMENTS),
            label: "Enrich comments".into(),
            allowlist: Some("openrouter".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CARGO_TEST),
            label: "Cargo test (subject)".into(),
            allowlist: Some("test".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::TRACE),
            label: "Trace (planned)".into(),
            allowlist: None,
        },
        Recipe {
            id: RecipeId::from(recipes::REVIEW_VIEW),
            label: "Review view".into(),
            allowlist: Some("local".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::REVIEW_DOC),
            label: "Review doc".into(),
            allowlist: Some("local".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::METRIC_TEST),
            label: "Metric test".into(),
            allowlist: Some("test".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::IMPROVE_PATCH),
            label: "Improve patch".into(),
            allowlist: Some("openrouter".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::INTEGRATE_PATCH),
            label: "Integrate patch".into(),
            allowlist: Some("openrouter".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CURSOR_CLOUD),
            label: "Cursor cloud agent".into(),
            allowlist: Some("cursor".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CURSOR_LOCAL),
            label: "Cursor local agent".into(),
            allowlist: Some("cursor".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::OPENCODE_SESSION),
            label: "OpenCode session".into(),
            allowlist: Some("opencode".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CLAUDE_CODE),
            label: "Claude Code".into(),
            allowlist: Some("claude".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CODEX_SESSION),
            label: "Codex session".into(),
            allowlist: Some("codex".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::EXTERNAL_GENERIC),
            label: "External agent".into(),
            allowlist: None,
        },
        Recipe {
            id: RecipeId::from(recipes::CI_RUST),
            label: "CI rust job".into(),
            allowlist: Some("github-actions".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CI_FMT),
            label: "CI rustfmt".into(),
            allowlist: Some("github-actions".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CI_CLIPPY),
            label: "CI clippy".into(),
            allowlist: Some("github-actions".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CI_TEST),
            label: "CI cargo test".into(),
            allowlist: Some("github-actions".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CI_ARTIFACTS),
            label: "CI no tracked generated artifacts".into(),
            allowlist: Some("github-actions".into()),
        },
        Recipe {
            id: RecipeId::from(recipes::CI_SITE),
            label: "CI site check".into(),
            allowlist: Some("github-actions".into()),
        },
    ]
}

/// Pretty-printed pipelines catalog JSON (`{ "recipes": [ … ] }`).
pub fn pipelines_json() -> String {
    #[derive(Serialize)]
    struct Catalog<'a> {
        recipes: &'a [Recipe],
    }
    let catalog = builtin_recipes();
    let mut out = serde_json::to_string_pretty(&Catalog { recipes: &catalog })
        .expect("pipelines catalog serializes");
    out.push('\n');
    out
}

/// How a Mac-local runtime discovers and refreshes foreign [`RunRecord`]s.
///
/// The IR names the source; Vite / `robby ingest --watch` / future `robby serve`
/// perform the fetch. Credentials stay on the host — never in this document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LiveRunTransport {
    /// Re-read a local JSON dump on an interval (Cursor MCP dump, NDJSON inbox).
    FilePoll {
        /// Repo-relative or absolute path (e.g. `.robby/external/cursor/agents.json`).
        path: String,
        /// Poll cadence in milliseconds while any run from this source is live.
        interval_ms: u64,
    },
    /// HTTP GET returning an agents envelope or RunRecord list.
    HttpPoll {
        url: String,
        interval_ms: u64,
    },
    /// React to filesystem mtime (Claude / OpenCode session dirs later).
    FileWatch {
        path: String,
    },
}

/// A live feed that produces [`RunRecord`]s for a recipe family at runtime.
///
/// Site artifact: `runs/sources.json`. Viewer / Vite middleware polls these
/// so the Runs dock stays current without re-extracting the IR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LiveRunSource {
    /// Stable id, usually the recipe id (`cursor.cloud`).
    pub id: String,
    pub recipe: RecipeId,
    pub label: String,
    pub transport: LiveRunTransport,
    /// Runtime adapter key (`cursor_agents_v1`, `run_records_v1`, …).
    pub adapter: String,
    /// When false, runtime skips this source (opt-in team browser later).
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl LiveRunSource {
    /// Default Mac-local Cursor cloud dump path used by Vite + ingest watch.
    pub fn cursor_cloud_file_poll() -> Self {
        Self {
            id: recipes::CURSOR_CLOUD.to_owned(),
            recipe: RecipeId::from(recipes::CURSOR_CLOUD),
            label: "Cursor cloud / Project workers".to_owned(),
            transport: LiveRunTransport::FilePoll {
                path: ".robby/external/cursor/agents.json".to_owned(),
                interval_ms: 5_000,
            },
            adapter: "cursor_agents_v1".to_owned(),
            enabled: true,
        }
    }
}

/// Built-in live sources for a trusted MacBook host (slice 1: Cursor file dump).
pub fn default_live_run_sources() -> Vec<LiveRunSource> {
    vec![LiveRunSource::cursor_cloud_file_poll()]
}

/// Lookup over a recipe catalog (pipelines.json, in-memory, …).
pub trait RecipeCatalog {
    fn get(&self, id: &RecipeId) -> Option<&Recipe>;
}

impl RecipeCatalog for [Recipe] {
    fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        self.iter().find(|r| &r.id == id)
    }
}

impl RecipeCatalog for Vec<Recipe> {
    fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        RecipeCatalog::get(self.as_slice(), id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_status_round_trips() {
        for status in [
            RunStatus::Queued,
            RunStatus::Running,
            RunStatus::Pass,
            RunStatus::Fail,
            RunStatus::Error,
            RunStatus::Cancelled,
        ] {
            assert_eq!(RunStatus::parse(status.as_str()), Some(status));
        }
        assert!(RunStatus::Pass.is_success());
        assert!(RunStatus::Fail.is_terminal());
        assert!(!RunStatus::Running.is_terminal());
    }

    #[test]
    fn run_record_serializes_recipe_string() {
        let record = RunRecord::new("01TEST", recipes::EXTRACT, RunStatus::Pass)
            .with_subject("rust://robby_ir");
        let json = serde_json::to_value(&record).unwrap();
        assert_eq!(json["recipe"], recipes::EXTRACT);
        assert_eq!(json["status"], "pass");
        assert_eq!(json["subject"], "rust://robby_ir");
    }

    #[test]
    fn builtin_catalog_includes_ci_recipes() {
        let catalog = builtin_recipes();
        for id in [
            recipes::CI_FMT,
            recipes::CI_CLIPPY,
            recipes::CI_TEST,
            recipes::CI_ARTIFACTS,
            recipes::CI_SITE,
            recipes::CI_RUST,
            recipes::IMPROVE_PATCH,
            recipes::INTEGRATE_PATCH,
        ] {
            assert!(
                catalog.get(&RecipeId::from(id)).is_some(),
                "missing recipe {id}"
            );
        }
        let json: serde_json::Value = serde_json::from_str(&pipelines_json()).unwrap();
        assert!(json["recipes"].as_array().unwrap().len() >= 6);
    }

    #[test]
    fn schema_pipelines_json_matches_builtin() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schema/pipelines.json");
        let on_disk = std::fs::read_to_string(&path).expect("schema/pipelines.json");
        let expected: serde_json::Value = serde_json::from_str(&pipelines_json()).unwrap();
        let actual: serde_json::Value = serde_json::from_str(&on_disk).unwrap();
        assert_eq!(
            actual, expected,
            "schema/pipelines.json out of sync with builtin_recipes(); refresh from pipelines_json()"
        );
    }

    #[test]
    fn live_run_source_round_trips() {
        let src = LiveRunSource::cursor_cloud_file_poll();
        let json = serde_json::to_value(&src).unwrap();
        assert_eq!(json["id"], recipes::CURSOR_CLOUD);
        assert_eq!(json["adapter"], "cursor_agents_v1");
        assert_eq!(json["transport"]["kind"], "file_poll");
        assert_eq!(
            json["transport"]["path"],
            ".robby/external/cursor/agents.json"
        );
        let back: LiveRunSource = serde_json::from_value(json).unwrap();
        assert_eq!(back, src);
    }
}
