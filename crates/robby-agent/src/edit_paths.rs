//! Edit-path allow/deny policy for improve → integrate patches.
//!
//! Generated self-model markdown, IR dumps, Mermaid fragments, and site scene
//! artifacts are **outputs**. Small models often treat `doc.coverage_complete`
//! as a cue to edit those files; this module forbids that surface and prefers
//! Rust/TS source under `crates/` and `apps/`.

use std::path::{Component, Path};

fn extension_is(path: &str, ext: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(ext))
}

/// True when a proposed patch path must never be written.
///
/// Forbidden:
/// - `docs/self-model/**` (generated report tree)
/// - `.robby/**` (IR, diagrams, patches, reviews)
/// - `*.mmd` Mermaid fragments
/// - site scene / frames / runs / build / pipelines public artifacts
pub fn is_forbidden_edit_path(path: &str) -> bool {
    let normalized = normalize_repo_path(path);
    if normalized.is_empty() {
        return true;
    }

    if normalized.starts_with("docs/self-model/") || normalized == "docs/self-model" {
        return true;
    }
    if normalized.starts_with(".robby/") || normalized == ".robby" {
        return true;
    }
    if extension_is(&normalized, "mmd") {
        return true;
    }
    if is_generated_site_artifact(&normalized) {
        return true;
    }
    false
}

/// Preferred fix surface for metric improvements (source crates / apps).
pub fn is_preferred_source_path(path: &str) -> bool {
    let normalized = normalize_repo_path(path);
    normalized.starts_with("crates/")
        || normalized.starts_with("apps/site/src/")
        || normalized.starts_with("apps/") && !normalized.starts_with("apps/site/public/")
        || normalized.starts_with("fixtures/")
        || normalized.starts_with("schema/")
        || normalized.starts_with("scripts/")
}

/// Subject strings that are markdown / generated-doc binders, not source.
pub fn is_generated_or_markdown_subject(subject: &str) -> bool {
    let raw = subject.strip_prefix("path:").unwrap_or(subject);
    let normalized = normalize_repo_path(raw);
    if normalized.starts_with("docs/self-model/") || normalized == "docs/self-model" {
        return true;
    }
    if normalized.starts_with(".robby/") {
        return true;
    }
    if extension_is(&normalized, "mmd") {
        return true;
    }
    if is_generated_site_artifact(&normalized) {
        return true;
    }
    // Hand-written Project markdown can bind reviews, but it is not the
    // `doc.coverage_complete` fix surface (coverage is source-extract).
    if normalized.starts_with("docs/") && extension_is(&normalized, "md") {
        return true;
    }
    false
}

/// Drop forbidden file edits from a proposal; return how many were removed.
pub fn filter_forbidden_files<T, F>(files: &mut Vec<T>, path_of: F) -> usize
where
    F: Fn(&T) -> &str,
{
    let before = files.len();
    files.retain(|f| !is_forbidden_edit_path(path_of(f)));
    before.saturating_sub(files.len())
}

/// Prompt snippet shared by improve / integrate system messages.
pub fn patch_path_rules_prompt() -> &'static str {
    "Edit SOURCE only: prefer crates/**/*.rs, apps/site/src/**/*.{ts,tsx}, fixtures/, schema/.\n\
     Forbidden (generated outputs — never edit or invent):\n\
     - docs/self-model/** (regenerated Markdown report; not the fix surface)\n\
     - .robby/** (IR JSON, diagrams, patches, reviews)\n\
     - **/*.mmd Mermaid fragments\n\
     - apps/site/public/scene*.json, frames/, runs/, build.json, pipelines.json\n\
     Metric `doc.coverage_complete` measures source-extract CoverageReport gaps \
     in Rust/TS — it does NOT mean edit Markdown. Closing gaps means improving \
     extractors / spans / source under crates/ or apps/, then re-extract."
}

fn is_generated_site_artifact(normalized: &str) -> bool {
    if !normalized.starts_with("apps/site/public/") {
        return false;
    }
    let rest = &normalized["apps/site/public/".len()..];
    rest.starts_with("frames/")
        || rest.starts_with("runs/")
        || rest == "scene.json"
        || rest == "scene-entrypoints.json"
        || (rest.starts_with("scene") && extension_is(rest, "json"))
        || rest == "build.json"
        || rest == "pipelines.json"
}

/// Normalize to forward-slash, strip `./`, `a/`/`b/` diff prefixes, and `\\`.
pub fn normalize_repo_path(path: &str) -> String {
    let trimmed = path.trim().trim_start_matches("./");
    let trimmed = trimmed
        .strip_prefix("a/")
        .or_else(|| trimmed.strip_prefix("b/"))
        .unwrap_or(trimmed);
    let as_path = Path::new(trimmed);
    let mut parts = Vec::new();
    for component in as_path.components() {
        match component {
            Component::Normal(s) => parts.push(s.to_string_lossy().replace('\\', "/")),
            Component::ParentDir => {
                let _ = parts.pop();
            }
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {}
        }
    }
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbids_self_model_and_robby_and_mmd() {
        assert!(is_forbidden_edit_path("docs/self-model/README.md"));
        assert!(is_forbidden_edit_path("docs/self-model/robby_ir/index.md"));
        assert!(is_forbidden_edit_path(".robby/robby.ir.json"));
        assert!(is_forbidden_edit_path(".robby/diagrams/seq-x.mmd"));
        assert!(is_forbidden_edit_path("crates/x/foo.mmd"));
        assert!(is_forbidden_edit_path("apps/site/public/scene.json"));
        assert!(is_forbidden_edit_path("apps/site/public/frames/a.json"));
        assert!(is_forbidden_edit_path("a/docs/self-model/x.md"));
    }

    #[test]
    fn allows_source_crates_and_site_src() {
        assert!(!is_forbidden_edit_path("crates/robby-ir/src/lib.rs"));
        assert!(!is_forbidden_edit_path("apps/site/src/viewer.ts"));
        assert!(!is_forbidden_edit_path("fixtures/m0/hello.rs"));
        assert!(!is_forbidden_edit_path("schema/pipelines.json"));
        assert!(!is_forbidden_edit_path("docs/00-bootstrap.md"));
        assert!(is_preferred_source_path("crates/robby-extract/src/rust.rs"));
        assert!(is_preferred_source_path("apps/site/src/runRecord.ts"));
    }

    #[test]
    fn markdown_subjects_flagged_for_coverage_retarget() {
        assert!(is_generated_or_markdown_subject(
            "path:docs/self-model/README.md"
        ));
        assert!(is_generated_or_markdown_subject(
            "path:docs/00-bootstrap.md"
        ));
        assert!(!is_generated_or_markdown_subject(
            "rust://robby_ir::metrics"
        ));
        assert!(!is_generated_or_markdown_subject(
            "crates/robby-ir/src/lib.rs"
        ));
    }

    #[test]
    fn filter_removes_forbidden() {
        let mut files = vec![
            "crates/a.rs".to_owned(),
            "docs/self-model/x.md".to_owned(),
            ".robby/x.json".to_owned(),
        ];
        let removed = filter_forbidden_files(&mut files, String::as_str);
        assert_eq!(removed, 2);
        assert_eq!(files, vec!["crates/a.rs".to_owned()]);
    }
}
