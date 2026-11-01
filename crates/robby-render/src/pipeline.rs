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

use crate::markdown;

/// A document plus everything derived from it.
pub struct Loaded {
    pub document: Document,
    pub sources: BTreeMap<String, String>,
    pub coverage: CoverageReport,
    pub diagnostics: Vec<Diagnostic>,
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
        let unclaimed: u32 = self.coverage.files.iter().map(|f| f.uncovered_bytes()).sum();
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
    for file in coverage::frontier_spans(doc).keys() {
        let path = root.join(file);
        if let Ok(text) = fs::read_to_string(&path) {
            sources.insert(file.clone(), text);
        }
    }
    sources
}

/// Load a document and everything derived from it.
pub fn load(ir: &Path, root: &Path) -> Result<Loaded> {
    let document = load_document(ir)?;
    let sources = load_sources(&document, root);
    let coverage = coverage::document_coverage(&document, &sources);
    let diagnostics = validate(&document);
    Ok(Loaded {
        document,
        sources,
        coverage,
        diagnostics,
    })
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
        fs::write(out.join(format!("graph-{}.mmd", slug(root.as_str()))), mermaid)?;
        count += 1;
    }

    Ok(count)
}

/// Render the single-file Markdown report.
pub fn write_report(loaded: &Loaded, out: &Path) -> std::io::Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = markdown::report(
        &loaded.document,
        Some(&loaded.coverage),
        Some(&loaded.sources),
    );
    fs::write(out, text)
}

/// Files whose change must invalidate a cached build.
pub fn watched_files(loaded: &Loaded) -> Vec<PathBuf> {
    loaded
        .sources
        .keys()
        .map(PathBuf::from)
        .collect::<Vec<_>>()
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
