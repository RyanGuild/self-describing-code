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

/// Well-known recipe ids (first set).
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
}
