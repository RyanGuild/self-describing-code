//! `robby.improve_patch` — first model proposes a source patch for metrics.

use std::fmt::Write as _;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::model::{ChatClient, ChatMessage, DEFAULT_IMPROVE_MODEL, DEFAULT_INTEGRATE_MODEL};

/// Env / CLI defaults for the improve model (must differ from integrate).
pub fn default_improve_model() -> String {
    std::env::var("ROBBY_IMPROVE_MODEL").unwrap_or_else(|_| DEFAULT_IMPROVE_MODEL.into())
}

/// Env / CLI defaults for the integrate model.
pub fn default_integrate_model() -> String {
    std::env::var("ROBBY_INTEGRATE_MODEL").unwrap_or_else(|_| DEFAULT_INTEGRATE_MODEL.into())
}

/// Bail if improve and integrate would share one model slug.
pub fn ensure_distinct_models(improve: &str, integrate: &str) -> Result<()> {
    if improve.trim() == integrate.trim() {
        bail!(
            "improve and integrate models must differ (both are `{improve}`); \
             set ROBBY_IMPROVE_MODEL / ROBBY_INTEGRATE_MODEL or CLI flags"
        );
    }
    Ok(())
}

/// Context fed to the improve model.
#[derive(Debug, Clone, Default)]
pub struct ImproveContext {
    pub subject: Option<String>,
    pub subject_name: Option<String>,
    pub metric_scores: Vec<(String, String)>,
    pub decision_notes: Vec<String>,
    pub source_path: Option<String>,
    pub source_window: Option<String>,
    pub metric_stdout: Option<String>,
    /// Source files with CoverageReport unknown gaps (retarget away from MD).
    pub coverage_gap_files: Vec<String>,
}

/// One file edit in a proposed patch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchFile {
    pub path: String,
    pub unified_diff: String,
}

/// Structured improve-model product.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedPatch {
    pub summary: String,
    #[serde(default)]
    pub files: Vec<PatchFile>,
    #[serde(default)]
    pub metrics_targeted: Vec<String>,
    /// Model that authored the proposal.
    #[serde(default)]
    pub model: String,
}

impl ProposedPatch {
    pub fn unified_diff_text(&self) -> String {
        let mut out = String::new();
        if !self.summary.is_empty() {
            let _ = write!(out, "# {}\n\n", self.summary);
        }
        for file in &self.files {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&file.unified_diff);
            if !file.unified_diff.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }

    pub fn is_empty(&self) -> bool {
        self.files.iter().all(|f| f.unified_diff.trim().is_empty())
    }
}

pub struct ImproveModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}

impl<'a, C: ChatClient + ?Sized> ImproveModel<'a, C> {
    pub fn new(client: &'a C, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
        }
    }

    pub fn propose(&self, ctx: &ImproveContext) -> Result<ProposedPatch> {
        let messages = [
            ChatMessage::system(format!(
                "You improve a codebase so IR metric scores move in the desired direction.\n\
                 Reply with JSON only:\n\
                 {{\"summary\":\"...\",\"metrics_targeted\":[\"doc.coverage_complete\"],\
                 \"files\":[{{\"path\":\"crates/.../file.rs\",\"unified_diff\":\"--- a/...\\n+++ b/...\\n@@ ...\"}}]}}\n\
                 Rules:\n\
                 {}\n\
                 - Prefer a small, concrete unified diff against Rust/TS source. Empty files array only if nothing actionable.\n\
                 - Do not propose Markdown under docs/ as the fix for doc.coverage_complete — that metric is source extract coverage; generated self-model pages are outputs.",
                crate::edit_paths::patch_path_rules_prompt()
            )),
            ChatMessage::user(format_improve_user(ctx)),
        ];
        let raw = self.client.chat(&self.model, &messages, true)?;
        let mut patch = parse_proposed_patch(&raw)?;
        patch.model.clone_from(&self.model);
        if patch.metrics_targeted.is_empty() {
            patch.metrics_targeted = ctx.metric_scores.iter().map(|(id, _)| id.clone()).collect();
        }
        let removed =
            crate::edit_paths::filter_forbidden_files(&mut patch.files, |f| f.path.as_str());
        if removed > 0 {
            let note = format!(
                "stripped {removed} forbidden generated-path edit(s) (docs/self-model, .robby, *.mmd, scene artifacts)"
            );
            if patch.summary.is_empty() {
                patch.summary = note;
            } else {
                patch.summary = format!("{}\n({note})", patch.summary);
            }
        }
        Ok(patch)
    }
}

/// Deterministic offline proposal when OpenRouter is unavailable.
pub fn stub_propose(ctx: &ImproveContext) -> ProposedPatch {
    let metrics: Vec<String> = ctx.metric_scores.iter().map(|(id, _)| id.clone()).collect();
    let subject = ctx
        .subject_name
        .as_deref()
        .or(ctx.subject.as_deref())
        .unwrap_or("subject");
    let path = stub_source_path(ctx);
    let score_line = ctx
        .metric_scores
        .iter()
        .map(|(id, score)| format!("{id}={score}"))
        .collect::<Vec<_>>()
        .join(", ");
    let body = format!(
        "Robby improve stub for `{subject}` ({score_line}). Source path `{path}` \
         (never docs/self-model). Replace with an OpenRouter proposal when OPENROUTER_API_KEY is set."
    );
    let diff = format!(
        "--- a/{path}\n+++ b/{path}\n@@ -1,1 +1,3 @@\n+// robby.improve_patch stub: {subject}\n+// metrics: {score_line}\n"
    );
    ProposedPatch {
        summary: body,
        files: vec![PatchFile {
            path,
            unified_diff: diff,
        }],
        metrics_targeted: metrics,
        model: "stub".into(),
    }
}

fn stub_source_path(ctx: &ImproveContext) -> String {
    if let Some(path) = ctx.source_path.as_deref()
        && !crate::edit_paths::is_forbidden_edit_path(path)
        && !std::path::Path::new(path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
    {
        return path.to_owned();
    }
    if let Some(gap) = ctx.coverage_gap_files.first()
        && !crate::edit_paths::is_forbidden_edit_path(gap)
    {
        return gap.clone();
    }
    "crates/robby-extract/src/lib.rs".into()
}

fn format_improve_user(ctx: &ImproveContext) -> String {
    let mut parts = Vec::new();
    if let Some(s) = &ctx.subject {
        parts.push(format!("subject: {s}"));
    }
    if let Some(n) = &ctx.subject_name {
        parts.push(format!("subject_name: {n}"));
    }
    if !ctx.metric_scores.is_empty() {
        let scores = ctx
            .metric_scores
            .iter()
            .map(|(id, v)| format!("{id}={v}"))
            .collect::<Vec<_>>()
            .join(", ");
        parts.push(format!("metric_scores: {scores}"));
        if ctx
            .metric_scores
            .iter()
            .any(|(id, _)| id == "doc.coverage_complete" || id.starts_with("doc.coverage"))
        {
            parts.push(
                "coverage_note: doc.coverage_complete is SOURCE extract CoverageReport::is_complete \
                 (Rust/TS byte gaps). Generated Markdown under docs/self-model/ is report OUTPUT — \
                 do not edit it to \"fix\" coverage. Improve extractors/spans/source instead."
                    .into(),
            );
        }
    }
    if let Some(tail) = &ctx.metric_stdout {
        parts.push(format!("metric_stdout:\n{tail}"));
    }
    if !ctx.decision_notes.is_empty() {
        parts.push(format!(
            "decision_products:\n{}",
            ctx.decision_notes.join("\n")
        ));
    }
    if !ctx.coverage_gap_files.is_empty() {
        parts.push(format!(
            "coverage_gap_source_files (edit these / their extractors):\n{}",
            ctx.coverage_gap_files.join("\n")
        ));
    }
    if let Some(path) = &ctx.source_path {
        parts.push(format!("source_path: {path}"));
    }
    if let Some(window) = &ctx.source_window {
        parts.push(format!("source_window:\n{window}"));
    }
    parts.join("\n\n")
}

fn parse_proposed_patch(raw: &str) -> Result<ProposedPatch> {
    let value = extract_json(raw)?;
    let summary = value
        .get("summary")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_owned();
    let metrics_targeted = value
        .get("metrics_targeted")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();
    let files: Vec<PatchFile> = value
        .get("files")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|f| {
                    let path = f.get("path")?.as_str()?.to_owned();
                    let unified_diff = f.get("unified_diff")?.as_str()?.to_owned();
                    Some(PatchFile { path, unified_diff })
                })
                .collect()
        })
        .unwrap_or_default();
    if summary.is_empty() && files.is_empty() {
        bail!("improve model returned empty proposal: {raw}");
    }
    Ok(ProposedPatch {
        summary: if summary.is_empty() {
            "improve patch".into()
        } else {
            summary
        },
        files,
        metrics_targeted,
        model: String::new(),
    })
}

fn extract_json(raw: &str) -> Result<serde_json::Value> {
    let trimmed = raw.trim();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Ok(v);
    }
    let start = trimmed
        .find('{')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    let end = trimmed
        .rfind('}')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    serde_json::from_str(&trimmed[start..=end])
        .with_context(|| format!("invalid JSON in response: {raw}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChatClient, ChatMessage};
    use std::sync::Mutex;

    struct FakeClient {
        responses: Mutex<Vec<String>>,
    }

    impl ChatClient for FakeClient {
        fn chat(
            &self,
            _model: &str,
            _messages: &[ChatMessage],
            _json_mode: bool,
        ) -> Result<String> {
            let mut q = self.responses.lock().unwrap();
            if q.is_empty() {
                bail!("FakeClient exhausted");
            }
            Ok(q.remove(0))
        }
    }

    #[test]
    fn rejects_same_model_pair() {
        assert!(ensure_distinct_models("openai/gpt-4o", "openai/gpt-4o").is_err());
        assert!(ensure_distinct_models("openai/gpt-4o-mini", "openai/gpt-4o").is_ok());
    }

    #[test]
    fn propose_parses_files() {
        let client = FakeClient {
            responses: Mutex::new(vec![
                r#"{"summary":"fix edge","metrics_targeted":["edge.resolve_rate"],"files":[{"path":"crates/robby-ir/src/lib.rs","unified_diff":"--- a/crates/robby-ir/src/lib.rs\n+++ b/crates/robby-ir/src/lib.rs\n@@ -1 +1 @@\n-a\n+b\n"}]}"#
                    .into(),
            ]),
        };
        let model = ImproveModel::new(&client, "openai/gpt-4o-mini");
        let patch = model
            .propose(&ImproveContext {
                subject: Some("rust://a".into()),
                metric_scores: vec![("edge.resolve_rate".into(), "0.5".into())],
                ..Default::default()
            })
            .unwrap();
        assert_eq!(patch.files.len(), 1);
        assert_eq!(patch.model, "openai/gpt-4o-mini");
        assert_ne!(patch.unified_diff_text(), "");
    }

    #[test]
    fn stub_is_nonempty() {
        let patch = stub_propose(&ImproveContext {
            subject: Some("rust://a".into()),
            metric_scores: vec![("doc.coverage_complete".into(), "0".into())],
            ..Default::default()
        });
        assert!(!patch.is_empty());
        assert_eq!(patch.model, "stub");
        assert!(
            patch.files[0].path.starts_with("crates/"),
            "stub must target source crates, got {}",
            patch.files[0].path
        );
        assert!(!patch.files[0].path.contains("docs/self-model"));
        assert!(
            !std::path::Path::new(&patch.files[0].path)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
        );
    }

    #[test]
    fn propose_strips_forbidden_self_model_paths() {
        let client = FakeClient {
            responses: Mutex::new(vec![
                r#"{"summary":"bad md edit","metrics_targeted":["doc.coverage_complete"],"files":[{"path":"docs/self-model/README.md","unified_diff":"--- a/docs/self-model/README.md\n+++ b/docs/self-model/README.md\n@@ -1 +1 @@\n-a\n+b\n"},{"path":"crates/robby-ir/src/lib.rs","unified_diff":"--- a/crates/robby-ir/src/lib.rs\n+++ b/crates/robby-ir/src/lib.rs\n@@ -1 +1 @@\n-a\n+b\n"}]}"#
                    .into(),
            ]),
        };
        let model = ImproveModel::new(&client, "openai/gpt-4o-mini");
        let patch = model
            .propose(&ImproveContext {
                subject: Some("path:docs/self-model/README.md".into()),
                metric_scores: vec![("doc.coverage_complete".into(), "0".into())],
                coverage_gap_files: vec!["crates/robby-extract/src/rust.rs".into()],
                ..Default::default()
            })
            .unwrap();
        assert_eq!(patch.files.len(), 1);
        assert_eq!(patch.files[0].path, "crates/robby-ir/src/lib.rs");
        assert!(patch.summary.contains("stripped"));
    }
}
