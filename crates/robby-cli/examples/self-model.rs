//! Bootstrap runner for the self-model.
//!
//! `build.rs` generates the artifacts on every build. This example exists for
//! the case where that build is what is broken: you can still run
//!
//! ```sh
//! cargo run -p robby-cli --example self-model
//! ```
//!
//! and get the map, the diagrams, and the coverage verdict without trusting the
//! build script. Same library, same output, different trigger.
//!
//! It writes the artifacts *before* reporting failure, so a broken map is still
//! inspectable. The exit code stays non-zero: the gate is not weakened, only
//! made legible.
//!
//! Being a Rust example rather than a shell script matters: it compiles with
//! the workspace, it is type-checked, and it can be represented in the map.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;
use robby_render::pipeline;

#[derive(Parser)]
#[command(name = "self-model", about = "Bootstrap runner for Robby's self-model")]
struct Args {
    /// Workspace root to extract. Ignored when `--ir` is given.
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Use an existing IR document instead of extracting.
    #[arg(long)]
    ir: Option<PathBuf>,
    /// Where to write the Markdown report.
    #[arg(long, default_value = "docs/self-model")]
    out: PathBuf,
    /// Where to write the `.mmd` fragments.
    #[arg(long, default_value = ".robby/diagrams")]
    diagrams: PathBuf,
    /// Where to write the IR document.
    #[arg(long, default_value = ".robby/robby.ir.json")]
    document: PathBuf,
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

    let loaded = if let Some(ir) = &args.ir {
        pipeline::load(ir, &args.root)?
    } else {
        let document = robby_extract::extract(&args.root)?;
        pipeline::from_document(document, &args.root)
    };

    println!("{}", loaded.summary());

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
        pipeline::write_document(&loaded.document, &args.document)?;
        let pages = pipeline::write_report(&loaded, &args.out)?;
        println!("wrote {}", args.document.display());
        println!("wrote {pages} page(s) to {}", args.out.display());
    }

    let count = pipeline::render_files(&loaded.document, &args.diagrams)?;
    println!("wrote {count} diagram(s) to {}", args.diagrams.display());

    if loaded.is_ok() {
        return Ok(ExitCode::SUCCESS);
    }

    for diagnostic in loaded
        .diagnostics
        .iter()
        .filter(|d| d.severity == robby_ir::Severity::Error)
    {
        eprintln!("[{}] {}", diagnostic.code, diagnostic.message);
    }
    for (_file, gap) in loaded.coverage.failures() {
        eprintln!("unclaimed at {}  {:?}", gap.span.location(), gap.preview);
    }
    eprintln!("coverage invariant violated");
    Ok(ExitCode::FAILURE)
}
