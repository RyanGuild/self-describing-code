//! Bootstrap producers: walk source trees and emit Robby IR.
//!
//! - [`rust`] — `syn` walk over Cargo crates under `<root>/crates`
//! - [`typescript`] — oxc walk over packages with `package.json`
//!
//! [`extract`] merges both into one document. Fidelity limits of the Rust
//! producer still apply; the TypeScript producer resolves calls by short name
//! the same way.

mod rust;
mod typescript;

use std::path::Path;

use anyhow::Result;
use robby_ir::Document;

/// Extract a document describing Rust crates and TypeScript packages under `root`.
pub fn extract(root: &Path) -> Result<Document> {
    let mut doc = rust::extract(root)?;
    let ts = typescript::extract(root)?;
    merge_documents(&mut doc, ts);
    Ok(doc)
}

/// Count of source files (`.rs` + TypeScript) the extractor considers part of
/// the workspace.
pub fn source_file_count(root: &Path) -> Result<usize> {
    Ok(rust::source_file_count(root)? + typescript::source_file_count(root)?)
}

fn merge_documents(into: &mut Document, from: Document) {
    for file in from.files {
        into.declare_file(file);
    }
    for node in from.nodes.into_values() {
        into.merge(node);
    }
}
