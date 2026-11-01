//! Bootstrap runner for the self-model.
//!
//! `build.rs` generates the artifact on every build. This example exists for
//! the case where that build is what is broken: you can still run
//!
//! ```sh
//! cargo run -p robby-cli --example self-model -- --ir fixtures/m0/hello.ir.json
//! ```
//!
//! and get the report, the diagrams, and the coverage verdict without trusting
//! the build script. Same library, same output, different trigger.
//!
//! Being a Rust example rather than a shell script matters: it compiles with
//! the workspace, it is type-checked, and it can be represented in the map.
//! A shell script beside the build would be a second pipeline that the
//! self-model cannot see — which is precisely the failure this project exists
//! to prevent.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use robby_render::pipeline;

#[derive(Parser)]
#[command(name = "self-model", about = "Bootstrap runner for Robby's self-model")]
struct Args {
    /// IR document to render.
    #[arg(long, default_value = "fixtures/m0/hello.ir.json")]
    ir: PathBuf,
    /// Root used to resolve repository-relative source paths.
    #[arg(long, default_value = ".")]
    source_root: PathBuf,
    /// Where to write the Markdown report.
    #[arg(long, default_value = "docs/robby-self-model.md")]
    out: PathBuf,
    /// Where to write the `.mmd` fragments.
    #[arg(long, default_value = ".robby/m0")]
    diagrams: PathBuf,
    /// Print the report to stdout instead of writing it.
    #[arg(long)]
    stdout: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let args = Args::parse();
    let loaded = pipeline::load(&args.ir, &args.source_root)?;

    println!("{}", loaded.summary());

    if !loaded.is_ok() {
        for diagnostic in loaded
            .diagnostics
            .iter()
            .filter(|d| d.severity == robby_ir::Severity::Error)
        {
            eprintln!("[{}] {}", diagnostic.code, diagnostic.message);
        }
        for (file, gap) in loaded.coverage.failures() {
            eprintln!(
                "unclaimed at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
            let _ = file;
        }
        eprintln!("coverage invariant violated");
        return Ok(ExitCode::FAILURE);
    }

    if args.stdout {
        print!(
            "{}",
            robby_render::markdown::report(
                &loaded.document,
                Some(&loaded.coverage),
                Some(&loaded.sources)
            )
        );
    } else {
        pipeline::write_report(&loaded, &args.out)?;
        println!("wrote {}", args.out.display());
    }

    let count = pipeline::render_files(&loaded.document, &args.diagrams)?;
    println!("wrote {count} diagram(s) to {}", args.diagrams.display());

    Ok(ExitCode::SUCCESS)
}
