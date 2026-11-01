//! The `robby` command.
//!
//! Every subcommand delegates to [`robby_render::pipeline`], which is also what
//! `build.rs` calls. There is one code path that produces artifacts.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};
use robby_ir::Severity;
use robby_render::pipeline;

#[derive(Parser)]
#[command(
    name = "robby",
    version,
    about = "Robby's self-model: total representation, progressive rendering"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the IR JSON Schema (source of truth for every producer).
    Schema {
        /// Write to a file instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Validate a document and enforce the coverage invariant.
    Check {
        #[arg(long)]
        ir: PathBuf,
        /// Root used to resolve repository-relative source paths.
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Emit a machine-readable report on stdout.
        #[arg(long)]
        json: bool,
    },
    /// Render every diagram in a document to Mermaid files.
    Render {
        #[arg(long)]
        ir: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Extract the workspace with syn and write the IR document.
    Extract {
        /// Workspace root to walk.
        #[arg(long, default_value = ".")]
        root: PathBuf,
        /// Write the document here instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Render the whole self-model to one Markdown file.
    Report {
        #[arg(long)]
        ir: PathBuf,
        /// Root used to resolve repository-relative source paths.
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Write to a file instead of stdout.
        #[arg(long)]
        out: Option<PathBuf>,
    },
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
    let cli = Cli::parse();
    match cli.command {
        Command::Schema { out } => {
            let schema = robby_ir::schema_json();
            match out {
                Some(path) => {
                    if let Some(parent) = path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&path, schema)?;
                    println!("wrote {}", path.display());
                }
                None => println!("{schema}"),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            ir,
            source_root,
            json,
        } => check(&ir, &source_root, json),
        Command::Render { ir, out } => {
            let loaded = pipeline::load(&ir, &PathBuf::from("."))?;
            let count = pipeline::render_files(&loaded.document, &out)?;
            println!("wrote {count} diagram(s) to {}", out.display());
            Ok(ExitCode::SUCCESS)
        }
        Command::Extract { root, out } => extract(&root, out),
        Command::Report {
            ir,
            source_root,
            out,
        } => report(&ir, &source_root, out),
    }
}

fn check(ir: &std::path::Path, source_root: &std::path::Path, json: bool) -> Result<ExitCode> {
    let loaded = pipeline::load(ir, source_root)?;

    if json {
        let payload = serde_json::json!({
            "diagnostics": loaded.diagnostics,
            "coverage": loaded.coverage,
            "ok": loaded.is_ok(),
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        print_human(&loaded);
    }

    Ok(if loaded.is_ok() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn print_human(loaded: &pipeline::Loaded) {
    for diagnostic in &loaded.diagnostics {
        let tag = match diagnostic.severity {
            Severity::Error => "error",
            Severity::Warning => "warn ",
            Severity::Info => "info ",
        };
        match &diagnostic.node {
            Some(node) => println!("{tag} [{}] {} ({})", diagnostic.code, diagnostic.message, node),
            None => println!("{tag} [{}] {}", diagnostic.code, diagnostic.message),
        }
    }

    println!();
    println!("coverage");
    for file in loaded.coverage.worst_first() {
        println!(
            "  code {:>6.1}%  {:>6}/{:<6} claimed  {:>5} trivia  {}",
            file.code_ratio() * 100.0,
            file.covered_bytes,
            file.covered_bytes + file.uncovered_bytes(),
            file.trivia_bytes(),
            file.file
        );
        for gap in file.gaps.iter().filter(|g| g.reason.is_failure()) {
            println!(
                "           missing at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
        }
    }

    println!();
    if loaded.is_ok() {
        println!("ok: every byte is owned or classified");
    } else {
        println!("FAIL: unclaimed code is invisible to the map");
    }
}

fn extract(root: &std::path::Path, out: Option<PathBuf>) -> Result<ExitCode> {
    let doc = robby_extract::extract(root)?;
    let json = serde_json::to_string_pretty(&doc)?;
    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, json)?;
            println!(
                "wrote {} ({} nodes, {} files)",
                path.display(),
                doc.nodes.len(),
                doc.files.len()
            );
        }
        None => println!("{json}"),
    }
    Ok(ExitCode::SUCCESS)
}

fn report(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: Option<PathBuf>,
) -> Result<ExitCode> {
    let loaded = pipeline::load(ir, source_root)?;
    match out {
        Some(path) => {
            let pages = pipeline::write_report(&loaded, &path)?;
            println!("wrote {pages} page(s) to {}", path.display());
        }
        None => print!(
            "{}",
            robby_render::markdown::report(
                &loaded.document,
                Some(&loaded.coverage),
                Some(&loaded.sources)
            )
        ),
    }
    Ok(ExitCode::SUCCESS)
}
