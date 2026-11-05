//! `robby.improve_patch` + `robby.integrate_patch` RunRecords (no LLM).
//!
//! LLM proposal/review lives in `robby-agent`. This module persists shared
//! [`RunRecord`]s, patch files under a patches dir, and product links so the
//! site Runs dock can show the improve → integrate chain after Allow.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use robby_ir::run::{RecipeId, RunRecord, RunStatus, recipes};
use robby_ir::{Kind, Process, Step, StepKind, kind::agent};

use crate::runs::SiteRunsStore;

/// Result of writing improve then integrate runs.
#[derive(Debug, Clone)]
pub struct ImproveLoopResult {
    pub improve: RunRecord,
    pub integrate: RunRecord,
    pub proposal_path: PathBuf,
    pub integrated_path: Option<PathBuf>,
}

/// Persist an improve proposal RunRecord + patch file.
pub fn write_improve_run(
    store: &SiteRunsStore,
    patches_dir: &Path,
    subject: Option<&str>,
    parent_run_id: Option<&str>,
    metric_test_run_id: Option<&str>,
    improve_model: &str,
    summary: &str,
    unified_diff: &str,
    metrics_targeted: &[String],
) -> Result<(RunRecord, PathBuf)> {
    let started = now_ms();
    let run_id = crate::runs::mint_run_id("improve");
    fs::create_dir_all(patches_dir).with_context(|| format!("mkdir {}", patches_dir.display()))?;
    let rel_name = format!("{run_id}.propose.diff");
    let proposal_path = patches_dir.join(&rel_name);
    let body = format!("# {summary}\n\n{unified_diff}");
    fs::write(&proposal_path, body)
        .with_context(|| format!("write {}", proposal_path.display()))?;

    let status = if unified_diff.trim().is_empty() {
        RunStatus::Fail
    } else {
        RunStatus::Pass
    };

    let mut record = RunRecord::new(
        run_id.clone(),
        RecipeId::from(recipes::IMPROVE_PATCH),
        status,
    );
    if let Some(subject) = subject {
        record.subject = Some(subject.to_owned());
    }
    record.started_at = Some(started);
    record.finished_at = Some(now_ms());
    record.exit_code = Some(i32::from(!status.is_success()));
    record.stdout_tail = Some(summary.to_owned());
    record.process = Some(improve_process());
    record
        .stats
        .insert("improve_model".to_owned(), improve_model.to_owned());
    record
        .stats
        .insert("metrics_targeted".to_owned(), metrics_targeted.join(","));
    record
        .stats
        .insert("mode".to_owned(), "propose_patch".to_owned());
    if let Some(parent) = parent_run_id {
        record
            .stats
            .insert("parent_run_id".to_owned(), parent.to_owned());
        record.products.push(format!("run:{parent}"));
    }
    if let Some(metric_id) = metric_test_run_id {
        record
            .stats
            .insert("metric_test_run_id".to_owned(), metric_id.to_owned());
        let product = format!("run:{metric_id}");
        if !record.products.contains(&product) {
            record.products.push(product);
        }
    }
    let path_product = format!("path:{}", display_patch_product(&proposal_path));
    record.products.push(path_product);
    record
        .products
        .push(format!("recipe:{}", recipes::IMPROVE_PATCH));
    store.put(&record)?;
    Ok((record, proposal_path))
}

/// Persist an integrate RunRecord + optional integrated patch file.
pub fn write_integrate_run(
    store: &SiteRunsStore,
    patches_dir: &Path,
    subject: Option<&str>,
    improve_run: &RunRecord,
    integrate_model: &str,
    verdict: &str,
    reason: &str,
    integrated_diff: Option<&str>,
) -> Result<(RunRecord, Option<PathBuf>)> {
    let started = now_ms();
    let run_id = crate::runs::mint_run_id("integrate");
    fs::create_dir_all(patches_dir).with_context(|| format!("mkdir {}", patches_dir.display()))?;

    let status = match verdict {
        "accept" | "revise" if integrated_diff.is_some_and(|d| !d.trim().is_empty()) => {
            RunStatus::Pass
        }
        "reject" | "accept" | "revise" => RunStatus::Fail,
        _ => RunStatus::Error,
    };

    let mut integrated_path = None;
    if let Some(diff) = integrated_diff.filter(|d| !d.trim().is_empty()) {
        let rel_name = format!("{run_id}.integrated.diff");
        let path = patches_dir.join(&rel_name);
        let body = format!("# integrate {verdict}: {reason}\n\n{diff}");
        fs::write(&path, &body).with_context(|| format!("write {}", path.display()))?;
        integrated_path = Some(path);
    }

    let mut record = RunRecord::new(run_id, RecipeId::from(recipes::INTEGRATE_PATCH), status);
    if let Some(subject) = subject.or(improve_run.subject.as_deref()) {
        record.subject = Some(subject.to_owned());
    }
    record.started_at = Some(started);
    record.finished_at = Some(now_ms());
    record.exit_code = Some(i32::from(!status.is_success()));
    record.stdout_tail = Some(format!("{verdict}: {reason}"));
    record.process = Some(integrate_process());
    record
        .stats
        .insert("integrate_model".to_owned(), integrate_model.to_owned());
    record
        .stats
        .insert("verdict".to_owned(), verdict.to_owned());
    record.stats.insert("reason".to_owned(), reason.to_owned());
    record
        .stats
        .insert("mode".to_owned(), "write_patch_file".to_owned());
    record
        .stats
        .insert("improve_run_id".to_owned(), improve_run.run_id.clone());
    if let Some(improve_model) = improve_run.stats.get("improve_model") {
        record
            .stats
            .insert("improve_model".to_owned(), improve_model.clone());
    }
    record
        .stats
        .insert("parent_run_id".to_owned(), improve_run.run_id.clone());
    record.products.push(format!("run:{}", improve_run.run_id));
    record
        .products
        .push(format!("recipe:{}", recipes::INTEGRATE_PATCH));
    if let Some(path) = &integrated_path {
        record
            .products
            .push(format!("path:{}", display_patch_product(path)));
    }
    store.put(&record)?;

    // Link improve → integrate.
    let mut improve = store.read(&improve_run.run_id)?;
    let link = format!("run:{}", record.run_id);
    if !improve.products.contains(&link) {
        improve.products.push(link);
        improve
            .products
            .push(format!("recipe:{}", recipes::INTEGRATE_PATCH));
        improve
            .stats
            .insert("integrate_run_id".to_owned(), record.run_id.clone());
        store.put(&improve)?;
    }

    Ok((record, integrated_path))
}

/// Link metric_test (and optional review) products to the improve run.
pub fn link_metric_to_improve(
    store: &SiteRunsStore,
    metric_run_id: &str,
    improve: &RunRecord,
) -> Result<()> {
    let mut metric = store.read(metric_run_id)?;
    let link = format!("run:{}", improve.run_id);
    if !metric.products.contains(&link) {
        metric.products.push(link);
        metric
            .products
            .push(format!("recipe:{}", recipes::IMPROVE_PATCH));
        metric
            .stats
            .insert("improve_run_id".to_owned(), improve.run_id.clone());
        store.put(&metric)?;
    }
    Ok(())
}

/// Convenience: write improve + integrate and link from metric_test.
pub fn spawn_improve_integrate(
    store: &SiteRunsStore,
    patches_dir: &Path,
    subject: Option<&str>,
    metric_run: &RunRecord,
    improve_model: &str,
    integrate_model: &str,
    summary: &str,
    unified_diff: &str,
    metrics_targeted: &[String],
    integrate_verdict: &str,
    integrate_reason: &str,
    integrated_diff: Option<&str>,
) -> Result<ImproveLoopResult> {
    if improve_model.trim() == integrate_model.trim() {
        bail!("improve_model and integrate_model must differ (got `{improve_model}`)");
    }
    let (improve, proposal_path) = write_improve_run(
        store,
        patches_dir,
        subject.or(metric_run.subject.as_deref()),
        Some(metric_run.run_id.as_str()),
        Some(metric_run.run_id.as_str()),
        improve_model,
        summary,
        unified_diff,
        metrics_targeted,
    )?;
    link_metric_to_improve(store, &metric_run.run_id, &improve)?;
    let (integrate, integrated_path) = write_integrate_run(
        store,
        patches_dir,
        subject.or(metric_run.subject.as_deref()),
        &improve,
        integrate_model,
        integrate_verdict,
        integrate_reason,
        integrated_diff,
    )?;
    Ok(ImproveLoopResult {
        improve,
        integrate,
        proposal_path,
        integrated_path,
    })
}

fn display_patch_product(path: &Path) -> String {
    // Prefer repo-relative `.robby/patches/...` when present in the path.
    let s = path.to_string_lossy();
    if let Some(idx) = s.find(".robby/patches/") {
        return s[idx..].replace('\\', "/");
    }
    if let Some(idx) = s.find(".robby\\patches\\") {
        return s[idx..].replace('\\', "/");
    }
    path.file_name().map_or_else(
        || s.replace('\\', "/"),
        |n| format!(".robby/patches/{}", n.to_string_lossy()),
    )
}

fn improve_process() -> Process {
    let mut process = Process::new();
    process.participant("improve", "improve_patch", Kind::from(agent::ACTION));
    process.participant("source", "workspace", Kind::from(agent::OBSERVATION));
    for label in ["gather_context", "propose_patch", "write_run"] {
        process.step(Step {
            from: "improve".to_owned(),
            to: "source".to_owned(),
            label: label.to_owned(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
    }
    process
}

fn integrate_process() -> Process {
    let mut process = Process::new();
    process.participant("integrate", "integrate_patch", Kind::from(agent::ACTION));
    process.participant("patch", "proposal", Kind::from(agent::OBSERVATION));
    for label in ["review_patch", "write_patch_file", "write_run"] {
        process.step(Step {
            from: "integrate".to_owned(),
            to: "patch".to_owned(),
            label: label.to_owned(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: vec![],
        });
    }
    process
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use robby_ir::run::recipes;

    fn temp_store() -> (SiteRunsStore, PathBuf) {
        let dir = std::env::temp_dir().join(format!(
            "robby-improve-loop-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        let patches = dir.join("patches");
        (SiteRunsStore::new(dir), patches)
    }

    #[test]
    fn spawn_writes_both_runs_and_links() {
        let (store, patches) = temp_store();
        let mut metric = RunRecord::new("metric-1", recipes::METRIC_TEST, RunStatus::Fail)
            .with_subject("rust://a");
        metric.products.push("metric:edge.resolve_rate:0.5".into());
        store.put(&metric).unwrap();

        let result = spawn_improve_integrate(
            &store,
            &patches,
            Some("rust://a"),
            &metric,
            "openai/gpt-4o-mini",
            "openai/gpt-4o",
            "stub summary",
            "--- a/x\n+++ b/x\n@@ -1 +1 @@\n-a\n+b\n",
            &["edge.resolve_rate".into()],
            "accept",
            "ok",
            Some("--- a/x\n+++ b/x\n@@ -1 +1 @@\n-a\n+b\n"),
        )
        .unwrap();

        assert_eq!(result.improve.recipe.as_str(), recipes::IMPROVE_PATCH);
        assert_eq!(result.integrate.recipe.as_str(), recipes::INTEGRATE_PATCH);
        assert_eq!(result.improve.status, RunStatus::Pass);
        assert_eq!(result.integrate.status, RunStatus::Pass);
        assert!(result.proposal_path.exists());
        assert!(result.integrated_path.as_ref().unwrap().exists());

        let metric = store.read("metric-1").unwrap();
        assert!(
            metric
                .products
                .iter()
                .any(|p| p.contains(recipes::IMPROVE_PATCH) || p.starts_with("run:improve-")),
            "{:?}",
            metric.products
        );
        let improve = store.read(&result.improve.run_id).unwrap();
        assert!(
            improve
                .products
                .iter()
                .any(|p| p.contains(recipes::INTEGRATE_PATCH)),
            "{:?}",
            improve.products
        );
    }

    #[test]
    fn rejects_same_model() {
        let (store, patches) = temp_store();
        let metric = RunRecord::new("metric-2", recipes::METRIC_TEST, RunStatus::Pass);
        store.put(&metric).unwrap();
        let err = spawn_improve_integrate(
            &store,
            &patches,
            None,
            &metric,
            "openai/gpt-4o",
            "openai/gpt-4o",
            "x",
            "diff",
            &[],
            "accept",
            "ok",
            Some("diff"),
        )
        .unwrap_err();
        assert!(err.to_string().contains("must differ"), "{err}");
    }
}
