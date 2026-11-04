//! The `robby` command.
//!
//! Every subcommand delegates to [`robby_render::pipeline`], which is also what
//! `build.rs` calls. There is one code path that produces artifacts.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use robby_ir::{Document, Node, Severity};
use robby_render::pipeline;
use robby_render::tui;

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
    /// Print a node as a terminal tree, box graph, and process timeline.
    Show {
        /// Node path: `crate`, `crate::module`, or `crate::module::item`.
        /// Omit to print the whole workspace tree.
        path: Option<String>,
        /// IR document. Defaults to `.robby/robby.ir.json`.
        #[arg(long)]
        ir: Option<PathBuf>,
        /// Root used to resolve repository-relative source paths.
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Tree depth.
        #[arg(long, default_value_t = 4)]
        depth: usize,
        /// Target line width.
        #[arg(long, default_value_t = 96)]
        width: usize,
        /// Include imports, fields, and variants in the tree.
        #[arg(long)]
        all: bool,
        /// Which sections to print.
        #[arg(long, value_enum, default_value = "all")]
        view: ViewKind,
        /// Omit the source slice.
        #[arg(long)]
        no_source: bool,
    },
    /// Enrich comments via OpenRouter (decision + note); writes agent.decision into IR.
    EnrichComments {
        #[arg(long)]
        ir: PathBuf,
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Output path (defaults to overwriting `--ir`).
        #[arg(long)]
        out: Option<PathBuf>,
        #[arg(long)]
        decision_model: Option<String>,
        #[arg(long)]
        note_model: Option<String>,
        /// Max comments to process this run.
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Write Prezi-style scene JSON for the TypeScript viewer in `apps/site`.
    Web {
        #[arg(long)]
        ir: PathBuf,
        /// Root used to resolve repository-relative source paths.
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Directory for `scene.json` and `frames/` (default: `apps/site/public`).
        #[arg(long, default_value = "apps/site/public")]
        out: PathBuf,
        /// Rebuild when `--ir` changes; keep watching until Ctrl-C.
        #[arg(long)]
        watch: bool,
        /// Poll interval for `--watch` (milliseconds).
        #[arg(long, default_value_t = 750)]
        watch_interval_ms: u64,
    },
    /// Ingest foreign agent sessions into site `runs/` (same RunRecord shape).
    Ingest {
        #[command(subcommand)]
        source: IngestSource,
    },
}

#[derive(Subcommand)]
enum IngestSource {
    /// Mirror Cursor cloud / Project workers from a JSON dump (`list-cloud-agents`).
    Cursor {
        /// Path to agents JSON (`{ "agents": [...] }` or a bare array). `-` = stdin.
        #[arg(long)]
        from: PathBuf,
        /// Site public directory that already holds (or will hold) `runs/`.
        #[arg(long, default_value = "apps/site/public")]
        out: PathBuf,
        /// Re-read `--from` on an interval (Mac-local live panel). Ctrl-C to stop.
        #[arg(long)]
        watch: bool,
        /// Poll interval for `--watch` (milliseconds).
        #[arg(long, default_value_t = 5_000)]
        interval_ms: u64,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ViewKind {
    All,
    Entry,
    Types,
    Dependencies,
    References,
    Tree,
    Graph,
    Process,
    Source,
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
        Command::Show {
            path,
            ir,
            source_root,
            depth,
            width,
            all,
            view,
            no_source,
        } => show(
            path.as_deref(),
            ir.as_deref(),
            &source_root,
            depth,
            width,
            all,
            view,
            no_source,
        ),
        Command::EnrichComments {
            ir,
            source_root,
            out,
            decision_model,
            note_model,
            limit,
        } => enrich_comments(
            &ir,
            &source_root,
            out.as_deref(),
            decision_model,
            note_model,
            limit,
        ),
        Command::Web {
            ir,
            source_root,
            out,
            watch,
            watch_interval_ms,
        } => web(&ir, &source_root, &out, watch, watch_interval_ms),
        Command::Ingest { source } => match source {
            IngestSource::Cursor {
                from,
                out,
                watch,
                interval_ms,
            } => ingest_cursor(&from, &out, watch, interval_ms),
        },
    }
}

fn ingest_cursor(
    from: &std::path::Path,
    out: &std::path::Path,
    watch: bool,
    interval_ms: u64,
) -> Result<ExitCode> {
    // Ensure sources.json exists so the viewer knows this live feed.
    robby_render::ensure_runs_layout(out)?;
    let mut last_raw = String::new();
    loop {
        let json = if from.as_os_str() == "-" {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        } else if from.is_file() {
            std::fs::read_to_string(from)
                .with_context(|| format!("read cursor ingest file {}", from.display()))?
        } else if watch {
            // Wait for the dump file to appear (Vite / agent writes it).
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
            continue;
        } else {
            bail!("{} does not exist", from.display());
        };

        if json != last_raw {
            let records = robby_render::ingest_cursor_json(out, &json)?;
            println!(
                "ingested {} cursor.cloud run(s) into {}/runs/",
                records.len(),
                out.display()
            );
            for record in &records {
                println!(
                    "  {}  {}  {}",
                    record.status.as_str(),
                    record.run_id,
                    record.recipe.as_str()
                );
            }
            last_raw = json;
        }

        if !watch || from.as_os_str() == "-" {
            return Ok(ExitCode::SUCCESS);
        }
        std::thread::sleep(std::time::Duration::from_millis(interval_ms));
    }
}

fn web(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: &std::path::Path,
    watch: bool,
    watch_interval_ms: u64,
) -> Result<ExitCode> {
    let count = write_web(ir, source_root, out)?;
    println!(
        "wrote {count} frame(s) to {}\nserve with: cd apps/site && npm install && npm run dev",
        out.display()
    );
    if !watch {
        return Ok(ExitCode::SUCCESS);
    }

    let mut last_mtime = ir_mtime(ir);
    println!(
        "watching {} every {watch_interval_ms}ms (Ctrl-C to stop)",
        ir.display()
    );
    loop {
        std::thread::sleep(std::time::Duration::from_millis(watch_interval_ms));
        let mtime = ir_mtime(ir);
        if mtime == last_mtime {
            continue;
        }
        last_mtime = mtime;
        match write_web(ir, source_root, out) {
            Ok(count) => println!("rebuilt {count} frame(s) → {}", out.display()),
            Err(err) => eprintln!("watch rebuild failed: {err:#}"),
        }
    }
}

fn write_web(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: &std::path::Path,
) -> Result<usize> {
    let loaded = pipeline::load(ir, source_root)?;
    robby_render::write_site(&loaded.document, &loaded.sources, out)
}

fn ir_mtime(ir: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(ir).and_then(|m| m.modified()).ok()
}

fn enrich_comments(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: Option<&std::path::Path>,
    decision_model: Option<String>,
    note_model: Option<String>,
    limit: Option<usize>,
) -> Result<ExitCode> {
    let mut loaded = pipeline::load(ir, source_root)?;
    let client = robby_agent::OpenRouterClient::from_env()?;
    let mut opts = robby_agent::EnrichOpts::default();
    if let Some(m) = decision_model {
        opts.decision_model = m;
    }
    if let Some(m) = note_model {
        opts.note_model = m;
    }
    opts.limit = limit;

    let stats =
        robby_agent::enrich_comments(&mut loaded.document, &loaded.sources, &client, &opts)?;
    let out_path = out.unwrap_or(ir);
    pipeline::write_document(&loaded.document, out_path)?;
    println!(
        "enriched {} comment(s): {} relevant, {} noise, {} noted → {}",
        stats.decided,
        stats.relevant,
        stats.noise,
        stats.noted,
        out_path.display()
    );
    Ok(ExitCode::SUCCESS)
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
            Some(node) => println!(
                "{tag} [{}] {} ({})",
                diagnostic.code, diagnostic.message, node
            ),
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

#[allow(clippy::too_many_arguments)]
fn show(
    path: Option<&str>,
    ir: Option<&std::path::Path>,
    source_root: &std::path::Path,
    depth: usize,
    width: usize,
    all: bool,
    view: ViewKind,
    no_source: bool,
) -> Result<ExitCode> {
    let ir = ir.map_or_else(default_ir, PathBuf::from);
    if !ir.exists() {
        bail!(
            "{} does not exist; run `cargo run -p robby-cli -- extract --out {}` first",
            ir.display(),
            ir.display()
        );
    }

    let loaded = pipeline::load(&ir, source_root)?;
    let opts = tui::ShowOptions {
        depth,
        width,
        all,
        tree: matches!(view, ViewKind::All | ViewKind::Tree),
        graph: matches!(view, ViewKind::Graph),
        process: matches!(view, ViewKind::All | ViewKind::Process),
        source: !no_source && matches!(view, ViewKind::All | ViewKind::Source),
        entry: matches!(view, ViewKind::All | ViewKind::Entry),
        types: matches!(view, ViewKind::All | ViewKind::Types),
        dependencies: matches!(view, ViewKind::All | ViewKind::Dependencies),
        externals: matches!(view, ViewKind::All | ViewKind::Dependencies),
        comments: matches!(view, ViewKind::All),
        references: matches!(view, ViewKind::All | ViewKind::References),
    };

    match path {
        Some(path) => {
            let node = resolve(&loaded.document, path)?;
            print!(
                "{}",
                tui::show(&loaded.document, node, Some(&loaded.sources), &opts)
            );
        }
        None => print!("{}", workspace(&loaded.document, &opts)),
    }

    Ok(ExitCode::SUCCESS)
}

fn default_ir() -> PathBuf {
    PathBuf::from(".robby/robby.ir.json")
}

fn workspace(doc: &Document, opts: &tui::ShowOptions) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "Robby · {} nodes · {} files · {} crates",
        doc.nodes.len(),
        doc.files.len(),
        doc.roots.len()
    );
    let _ = writeln!(out, "{}", "─".repeat(opts.width.clamp(24, 78)));
    out.push('\n');

    for root in &doc.roots {
        out.push_str(&tui::tree(doc, root, opts));
        out.push_str("\n\n");
    }
    out
}

/// Resolve an exact fqn, an id, or a unique suffix / bare name.
fn resolve<'a>(doc: &'a Document, path: &str) -> Result<&'a Node> {
    if let Some(node) = doc
        .nodes
        .values()
        .find(|node| node.fqn == path || node.id.as_str() == path)
    {
        return Ok(node);
    }

    let suffix = format!("::{path}");
    let mut matches: Vec<&Node> = doc
        .nodes
        .values()
        .filter(|node| node.fqn.ends_with(&suffix) || node.name == path)
        .collect();

    match matches.len() {
        0 => bail!("no node matches `{path}`"),
        1 => Ok(matches.remove(0)),
        _ => {
            matches.sort_by(|a, b| a.fqn.cmp(&b.fqn));
            let mut message = format!("`{path}` is ambiguous ({} matches):", matches.len());
            for node in matches.iter().take(20) {
                let _ = write!(message, "\n  {}  [{}]", node.fqn, node.kind);
            }
            if matches.len() > 20 {
                let _ = write!(message, "\n  … {} more", matches.len() - 20);
            }
            bail!(message)
        }
    }
}
