//! The I/O pipeline, shared by the CLI and by `build.rs`.
//!
//! The renderers in this crate are pure functions of the IR. This module is the
//! only place that touches the filesystem, so there is exactly one code path
//! that produces committed artifacts. A shell script beside the build would be
//! a second path — unrepresented code, outside the map, able to drift.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use robby_ir::{CoverageReport, Diagnostic, Document, Severity, coverage, validate};

/// A document plus everything derived from it.
pub struct Loaded {
    pub document: Document,
    pub sources: BTreeMap<String, String>,
    pub coverage: CoverageReport,
    pub diagnostics: Vec<Diagnostic>,
    /// Workspace root the source paths are relative to.
    pub root: PathBuf,
}

impl Loaded {
    /// True when the coverage gate passes and there are no error diagnostics.
    pub fn is_ok(&self) -> bool {
        self.coverage.is_complete()
            && !self
                .diagnostics
                .iter()
                .any(|d| d.severity == Severity::Error)
    }

    /// One-line summary for build output and CI logs.
    pub fn summary(&self) -> String {
        let files = self.coverage.files.len();
        let claimed: u32 = self.coverage.files.iter().map(|f| f.covered_bytes).sum();
        let unclaimed: u32 = self
            .coverage
            .files
            .iter()
            .map(robby_ir::FileCoverage::uncovered_bytes)
            .sum();
        format!(
            "{files} file(s), {claimed} bytes claimed, {unclaimed} unclaimed, {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}

/// Read and parse an IR document.
pub fn load_document(path: &Path) -> Result<Document> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

/// Read every source file the document references.
///
/// A missing file is not an error here: the coverage report will show it as
/// fully unclaimed, which is the more useful failure.
pub fn load_sources(doc: &Document, root: &Path) -> BTreeMap<String, String> {
    let mut sources = BTreeMap::new();

    // A document that declares its files is authoritative: a declared file that
    // cannot be read is a missing piece of the map, not something to skip.
    let files: Vec<String> = if doc.files.is_empty() {
        coverage::all_spans(doc).keys().cloned().collect()
    } else {
        doc.files.clone()
    };

    for file in files {
        let path = root.join(&file);
        if let Ok(text) = fs::read_to_string(&path) {
            sources.insert(file, text);
        }
    }
    sources
}

/// Load a document and everything derived from it.
pub fn load(ir: &Path, root: &Path) -> Result<Loaded> {
    let document = load_document(ir)?;
    Ok(from_document(document, root))
}

/// Compute everything derived from an in-memory document.
pub fn from_document(mut document: Document, root: &Path) -> Loaded {
    // Refresh entrypoint call trees so older IR files and post-edit graphs
    // stay consistent with current `Calls` edges.
    document.rebuild_code_paths();
    let sources = load_sources(&document, root);
    let coverage = coverage::document_coverage(&document, &sources);
    let diagnostics = validate(&document);
    Loaded {
        document,
        sources,
        coverage,
        diagnostics,
        root: root.to_path_buf(),
    }
}

/// Write one `.mmd` file per process and per root graph.
pub fn render_files(doc: &Document, out: &Path) -> std::io::Result<usize> {
    fs::create_dir_all(out)?;
    let mut count = 0;

    for (id, mermaid) in crate::all_sequences(doc) {
        fs::write(out.join(format!("seq-{}.mmd", slug(id.as_str()))), mermaid)?;
        count += 1;
    }
    for root in &doc.roots {
        let mermaid = crate::flowchart(doc, root);
        fs::write(
            out.join(format!("graph-{}.mmd", slug(root.as_str()))),
            mermaid,
        )?;
        count += 1;
    }

    Ok(count)
}

/// Write the IR document itself as a committed artifact.
pub fn write_document(doc: &Document, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out, serde_json::to_string_pretty(doc)?)?;
    Ok(())
}

/// Render the self-model as a linked tree of Markdown pages.
///
/// Returns the number of pages written. Stale pages are removed first so a
/// deleted module cannot leave a ghost page behind.
pub fn write_report(loaded: &Loaded, out_dir: &Path) -> std::io::Result<usize> {
    let out_rel = out_dir
        .strip_prefix(&loaded.root)
        .unwrap_or(out_dir)
        .to_string_lossy()
        .replace('\\', "/");

    let pages = crate::tree::tree(
        &loaded.document,
        Some(&loaded.coverage),
        Some(&loaded.sources),
        &out_rel,
    );

    fs::create_dir_all(out_dir)?;
    remove_markdown(out_dir)?;

    for page in &pages {
        let path = out_dir.join(&page.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, &page.body)?;
    }

    Ok(pages.len())
}

/// Delete generated Markdown so removed modules do not linger.
fn remove_markdown(dir: &Path) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            remove_markdown(&path)?;
            // Remove directories that became empty.
            let _ = fs::remove_dir(&path);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}

/// Files whose change must invalidate a cached build.
pub fn watched_files(loaded: &Loaded) -> Vec<PathBuf> {
    loaded.sources.keys().map(PathBuf::from).collect::<Vec<_>>()
}

fn slug(input: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        out.push_str(&crate::stable_id(input));
    }
    out
}
