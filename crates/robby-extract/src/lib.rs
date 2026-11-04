//! Bootstrap producers: walk source trees and emit Robby IR.
//!
//! - [`rust`] — `syn` walk over Cargo crates under `<root>/crates`
//! - [`typescript`] — oxc walk over packages with `package.json`
//!
//! [`extract`] merges both into one document via [`LanguageExtractor`].
//! Fidelity limits of the Rust producer still apply; the TypeScript producer
//! resolves calls by short name the same way.

mod rust;
mod typescript;

use std::path::Path;

use anyhow::Result;
use robby_ir::{Document, Lang};

/// One language's producer. Future crates (`robby-extract-rust`,
/// `robby-extract-typescript`) should keep this signature.
pub trait LanguageExtractor {
    fn language(&self) -> Lang;
    fn extract(&self, root: &Path) -> Result<Document>;
    fn source_file_count(&self, root: &Path) -> Result<usize>;
}

/// Syn-based Rust extractor.
#[derive(Debug, Default, Clone, Copy)]
pub struct RustExtractor;

impl LanguageExtractor for RustExtractor {
    fn language(&self) -> Lang {
        Lang::Rust
    }

    fn extract(&self, root: &Path) -> Result<Document> {
        rust::extract(root)
    }

    fn source_file_count(&self, root: &Path) -> Result<usize> {
        rust::source_file_count(root)
    }
}

/// Oxc-based TypeScript / JS extractor.
#[derive(Debug, Default, Clone, Copy)]
pub struct TypeScriptExtractor;

impl LanguageExtractor for TypeScriptExtractor {
    fn language(&self) -> Lang {
        Lang::TypeScript
    }

    fn extract(&self, root: &Path) -> Result<Document> {
        typescript::extract(root)
    }

    fn source_file_count(&self, root: &Path) -> Result<usize> {
        typescript::source_file_count(root)
    }
}

/// Extract a document describing Rust crates and TypeScript packages under `root`.
pub fn extract(root: &Path) -> Result<Document> {
    let mut doc = RustExtractor.extract(root)?;
    let ts = TypeScriptExtractor.extract(root)?;
    merge_documents(&mut doc, ts);
    Ok(doc)
}

/// Merge language extractors in order (first is the base document).
pub fn extract_with<'a>(
    root: &Path,
    extractors: impl IntoIterator<Item = &'a dyn LanguageExtractor>,
) -> Result<Document> {
    let mut iter = extractors.into_iter();
    let Some(first) = iter.next() else {
        return Ok(Document::new("robby-extract"));
    };
    let mut doc = first.extract(root)?;
    for extractor in iter {
        merge_documents(&mut doc, extractor.extract(root)?);
    }
    Ok(doc)
}

/// Count of source files (`.rs` + TypeScript) the extractor considers part of
/// the workspace.
pub fn source_file_count(root: &Path) -> Result<usize> {
    Ok(RustExtractor.source_file_count(root)? + TypeScriptExtractor.source_file_count(root)?)
}

pub fn source_file_count_with<'a>(
    root: &Path,
    extractors: impl IntoIterator<Item = &'a dyn LanguageExtractor>,
) -> Result<usize> {
    let mut total = 0;
    for extractor in extractors {
        total += extractor.source_file_count(root)?;
    }
    Ok(total)
}

fn merge_documents(into: &mut Document, from: Document) {
    for file in from.files {
        into.declare_file(file);
    }
    for node in from.nodes.into_values() {
        into.merge(node);
    }
}
