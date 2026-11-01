//! Generate Robby's self-model as part of the build.
//!
//! This is the enforcement point for "total representation". The build script
//! extracts the workspace with `syn`, enforces the coverage invariant, and only
//! then writes the artifacts. If any byte of source is unclaimed, the build
//! fails — Robby cannot be compiled from a codebase that is invisible to its
//! own map.
//!
//! `examples/self-model.rs` calls the same pipeline. It exists because of a
//! real bootstrap problem: when the generation code itself is broken, you
//! cannot build the thing that regenerates the artifact. The example is the
//! escape hatch, and being Rust it is inside the map rather than beside it.

use std::env;
use std::path::PathBuf;

use robby_render::pipeline;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest.join("../..");

    println!("cargo:rerun-if-changed={}", manifest.join("build.rs").display());

    let document = robby_extract::extract(&root).unwrap_or_else(|err| {
        panic!("robby: extraction failed: {err:#}");
    });
    let loaded = pipeline::from_document(document, &root);

    // Any change to a mapped source file must regenerate the artifacts.
    for file in &loaded.document.files {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
    }

    if !loaded.is_ok() {
        for diagnostic in loaded
            .diagnostics
            .iter()
            .filter(|d| d.severity == robby_ir::Severity::Error)
        {
            eprintln!("robby: [{}] {}", diagnostic.code, diagnostic.message);
        }
        for (_file, gap) in loaded.coverage.failures() {
            eprintln!(
                "robby: unclaimed code at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
        }
        panic!("robby: coverage invariant violated — {}", loaded.summary());
    }

    pipeline::write_document(&loaded.document, &root.join(".robby/robby.ir.json"))
        .unwrap_or_else(|err| panic!("robby: cannot write IR: {err:#}"));
    pipeline::render_files(&loaded.document, &root.join(".robby/diagrams"))
        .unwrap_or_else(|err| panic!("robby: cannot render diagrams: {err:#}"));
    let pages = pipeline::write_report(&loaded, &root.join("docs/self-model"))
        .unwrap_or_else(|err| panic!("robby: cannot write report: {err:#}"));
    let _ = pages;
}
