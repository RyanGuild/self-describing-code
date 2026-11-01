//! The `robby` command.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use robby_ir::{Document, Severity, coverage, model, validate};
use robby_render::{flowchart, stable_id};

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
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&path, schema)?;
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
        Command::Render { ir, out } => render(&ir, &out),
        Command::Report {
            ir,
            source_root,
            out,
        } => report(&ir, &source_root, out),
    }
}

fn load(ir: &Path) -> Result<Document> {
    let text = fs::read_to_string(ir)
        .with_context(|| format!("reading document {}", ir.display()))?;
    let doc: Document = serde_json::from_str(&text)
        .with_context(|| format!("parsing document {}", ir.display()))?;
    Ok(doc)
}

fn check(ir: &Path, source_root: &Path, json: bool) -> Result<ExitCode> {
    let doc = load(ir)?;
    let diagnostics = validate(&doc);
    let sources = load_sources(&doc, source_root);
    let report = coverage::document_coverage(&doc, &sources);
    let errors = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();

    if json {
        let payload = serde_json::json!({
            "diagnostics": diagnostics,
            "coverage": report,
            "ok": report.is_complete() && errors == 0,
        });
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        print_human(&diagnostics, &report);
    }

    Ok(if report.is_complete() && errors == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Read every file the document references.
fn load_sources(doc: &Document, source_root: &Path) -> BTreeMap<String, String> {
    let mut sources = BTreeMap::new();
    for file in coverage::frontier_spans(doc).keys() {
        let path = source_root.join(file);
        match fs::read_to_string(&path) {
            Ok(text) => {
                sources.insert(file.clone(), text);
            }
            Err(err) => eprintln!("warning: cannot read {}: {err}", path.display()),
        }
    }
    sources
}

fn report(ir: &Path, source_root: &Path, out: Option<PathBuf>) -> Result<ExitCode> {
    let doc = load(ir)?;
    let sources = load_sources(&doc, source_root);
    let coverage = coverage::document_coverage(&doc, &sources);
    let markdown = robby_render::markdown::report(&doc, Some(&coverage), Some(&sources));

    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, markdown)?;
            println!("wrote {}", path.display());
        }
        None => print!("{markdown}"),
    }

    Ok(ExitCode::SUCCESS)
}

fn print_human(diagnostics: &[model::Diagnostic], report: &coverage::CoverageReport) {
    for diagnostic in diagnostics {
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
    for file in report.worst_first() {
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
    if report.is_complete() {
        println!("ok: every byte is owned or classified");
    } else {
        println!("FAIL: unclaimed code is invisible to the map");
    }
}

fn render(ir: &Path, out: &Path) -> Result<ExitCode> {
    let doc = load(ir)?;
    fs::create_dir_all(out)?;

    let mut count = 0usize;

    for (id, mermaid) in robby_render::all_sequences(&doc) {
        let path = out.join(format!("seq-{}.mmd", slug(id.as_str())));
        fs::write(&path, mermaid)?;
        count += 1;
    }

    for root in &doc.roots {
        let mermaid = flowchart(&doc, root);
        let path = out.join(format!("graph-{}.mmd", slug(root.as_str())));
        fs::write(&path, mermaid)?;
        count += 1;
    }

    println!("wrote {count} diagram(s) to {}", out.display());
    Ok(ExitCode::SUCCESS)
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
        out.push_str(&stable_id(input));
    }
    out
}

/// Kept for symmetry with the renderer's id scheme.
#[allow(dead_code)]
fn id_for(input: &str) -> String {
    stable_id(input)
}
