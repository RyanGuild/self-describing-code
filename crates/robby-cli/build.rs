//! Generate the self-model artifact as part of the build.
//!
//! This is the enforcement point for "total representation": if any byte of
//! source is unclaimed, the build fails. Robby cannot be compiled from a
//! codebase that is invisible to its own map.
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

    let ir = env::var("ROBBY_IR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| root.join("fixtures/m0/hello.ir.json"));

    println!("cargo:rerun-if-changed={}", ir.display());
    println!("cargo:rerun-if-changed={}", manifest.join("build.rs").display());
    println!("cargo:rerun-if-env-changed=ROBBY_IR");

    let loaded = pipeline::load(&ir, &root).unwrap_or_else(|err| {
        panic!("robby: cannot load IR {}: {err}", ir.display());
    });

    for file in pipeline::watched_files(&loaded) {
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
        for (file, gap) in loaded.coverage.failures() {
            eprintln!(
                "robby: unclaimed code at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
            let _ = file;
        }
        panic!(
            "robby: coverage invariant violated — {}",
            loaded.summary()
        );
    }

    let out = root.join("docs/robby-self-model.md");
    pipeline::write_report(&loaded, &out).unwrap_or_else(|err| {
        panic!("robby: cannot write {}: {err}", out.display());
    });

}
