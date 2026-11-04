//! The `robby` command.
//!
//! Every subcommand delegates to [`robby_render::pipeline`], which is also what
//! `build.rs` calls. There is one code path that produces artifacts.

use std::ffi::OsStr;
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
    /// Review a Project markdown doc against IR metrics (`robby.review_doc`).
    ///
    /// Headless first slice: bind subjects, score catalog metrics, emit
    /// `agent.decision` products + a `RunRecord` (no LLM).
    ReviewDoc {
        /// Markdown plan/report to review.
        #[arg(long)]
        doc: PathBuf,
        #[arg(long)]
        ir: PathBuf,
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Write enriched IR here (defaults to overwriting `--ir`).
        #[arg(long)]
        out: Option<PathBuf>,
        /// Site/public (or any) directory for `runs/<id>.json` (optional).
        #[arg(long)]
        runs_out: Option<PathBuf>,
        /// Override minted run id.
        #[arg(long)]
        run_id: Option<String>,
        /// Metric id to claim (repeatable). Defaults to first-slice pair.
        #[arg(long = "metric")]
        metrics: Vec<String>,
        /// Cap on IR subjects bound beside the doc path.
        #[arg(long, default_value_t = 5)]
        max_subjects: usize,
        /// Directory for `.robby/reviews/<stem>.json` (default: repo `.robby/reviews`).
        #[arg(long)]
        reviews_out: Option<PathBuf>,
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
    /// Seeded random opportunity sample for self-improvement (not every function).
    ///
    /// Ranker models are deferred; this always uses strategy `random`.
    SampleOpportunities {
        #[arg(long)]
        ir: PathBuf,
        /// RNG seed recorded as `sample_seed` (default: 0).
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Subset size (default: 8).
        #[arg(long, default_value_t = robby_ir::DEFAULT_SAMPLE_N)]
        n: usize,
        /// Emit machine-readable JSON on stdout.
        #[arg(long)]
        json: bool,
        /// Soft-queue `robby.review_view` for each drawn subject under site out.
        #[arg(long)]
        queue: bool,
        /// Site public dir used with `--queue` (default: `apps/site/public`).
        #[arg(long, default_value = "apps/site/public")]
        out: PathBuf,
    },
    /// Run empirical IR metric harnesses; write a `robby.metric_test` RunRecord.
    MetricTest {
        #[arg(long, default_value = ".robby/robby.ir.json")]
        ir: PathBuf,
        /// Root used to resolve repository-relative source paths.
        #[arg(long, default_value = ".")]
        source_root: PathBuf,
        /// Catalog metric id (repeatable). Defaults: doc.coverage_complete + edge.resolve_rate.
        #[arg(long = "metric")]
        metrics: Vec<String>,
        /// Optional subject node id / fqn (scopes edge.resolve_rate).
        #[arg(long)]
        subject: Option<String>,
        /// Site `public/` (or any out dir) for `runs/<id>.json`.
        #[arg(long, default_value = "apps/site/public")]
        runs_out: PathBuf,
        /// Parent review run_id (Allow path / coordination via shared RunRecord).
        #[arg(long)]
        parent_run: Option<String>,
        /// Allow a queued `robby.review_view` then spawn metric_test products.
        #[arg(long)]
        allow_run: Option<String>,
        /// Print the RunRecord JSON to stdout as well as writing runs/.
        #[arg(long)]
        json: bool,
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
        Command::ReviewDoc {
            doc,
            ir,
            source_root,
            out,
            runs_out,
            run_id,
            metrics,
            max_subjects,
            reviews_out,
        } => review_doc_cmd(
            &doc,
            &ir,
            &source_root,
            out.as_deref(),
            runs_out.as_deref(),
            run_id,
            metrics,
            max_subjects,
            reviews_out.as_deref(),
        ),
        Command::Web {
            ir,
            source_root,
            out,
            watch,
            watch_interval_ms,
        } => web(&ir, &source_root, &out, watch, watch_interval_ms),
        Command::Ingest { source } => match source {
            IngestSource::Cursor { from, out } => ingest_cursor(&from, &out),
        },
        Command::SampleOpportunities {
            ir,
            seed,
            n,
            json,
            queue,
            out,
        } => sample_opportunities_cmd(&ir, seed, n, json, queue, &out),
        Command::MetricTest {
            ir,
            source_root,
            metrics,
            subject,
            runs_out,
            parent_run,
            allow_run,
            json,
        } => metric_test(
            &ir,
            &source_root,
            metrics,
            subject.as_deref(),
            &runs_out,
            parent_run.as_deref(),
            allow_run.as_deref(),
            json,
        ),
    }
}

fn ingest_cursor(from: &std::path::Path, out: &std::path::Path) -> Result<ExitCode> {
    let json = if from.as_os_str() == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        std::fs::read_to_string(from)
            .with_context(|| format!("read cursor ingest file {}", from.display()))?
    };
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
    Ok(ExitCode::SUCCESS)
}

fn sample_opportunities_cmd(
    ir: &std::path::Path,
    seed: u64,
    n: usize,
    json: bool,
    queue: bool,
    out: &std::path::Path,
) -> Result<ExitCode> {
    let raw = std::fs::read_to_string(ir)?;
    let doc: Document = serde_json::from_str(&raw)?;
    let sample = robby_ir::sample_opportunities(&doc, seed, n);

    if queue {
        let store = robby_render::runs::SiteRunsStore::new(out);
        let records = store.queue_sampled_review_views(&sample, "cli.sample_opportunities")?;
        eprintln!(
            "queued {} robby.review_view run(s) under {}/runs",
            records.len(),
            out.display()
        );
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&sample)?);
    } else {
        println!(
            "strategy={} seed={} n={} candidates={} drawn={}",
            sample.strategy,
            sample.seed,
            sample.n,
            sample.candidate_count,
            sample.subjects.len()
        );
        for opportunity in &sample.subjects {
            println!(
                "{}	{}	{}",
                opportunity.reason.as_str(),
                opportunity.subject,
                opportunity.name
            );
        }
    }
    Ok(ExitCode::SUCCESS)
}

#[allow(clippy::too_many_arguments)]
fn metric_test(
    ir: &std::path::Path,
    source_root: &std::path::Path,
    metrics: Vec<String>,
    subject: Option<&str>,
    runs_out: &std::path::Path,
    parent_run: Option<&str>,
    allow_run: Option<&str>,
    json: bool,
) -> Result<ExitCode> {
    let loaded = pipeline::load(ir, source_root)?;
    let metric_ids = if metrics.is_empty() {
        vec![
            robby_ir::metrics::ids::DOC_COVERAGE_COMPLETE.to_owned(),
            robby_ir::metrics::ids::EDGE_RESOLVE_RATE.to_owned(),
        ]
    } else {
        metrics
    };

    let store = robby_render::SiteRunsStore::new(runs_out);
    store.ensure_layout()?;

    if let Some(review_id) = allow_run {
        let (review, metric_run) = robby_render::allow_and_spawn_metric_test(
            &store,
            review_id,
            &loaded.document,
            &loaded.coverage,
            &metric_ids,
        )?;
        print_metric_result(&metric_run, json);
        println!(
            "allowed review {} → metric_test {} ({})",
            review.run_id,
            metric_run.run_id,
            metric_run.status.as_str()
        );
        return Ok(exit_for_status(metric_run.status));
    }

    let req = robby_render::MetricTestRequest {
        metrics: metric_ids,
        subject: subject.map(str::to_owned),
        parent_run_id: parent_run.map(str::to_owned),
        sample_seed: None,
        sample_n: None,
    };
    let record =
        robby_render::write_metric_test_run(runs_out, &loaded.document, &loaded.coverage, &req)?;
    print_metric_result(&record, json);

    if let Some(parent_id) = parent_run {
        if let Ok(mut parent) = store.read(parent_id) {
            if !robby_render::SiteRunsStore::blocks_downstream(&parent) {
                let product = format!("run:{}", record.run_id);
                if !parent.products.contains(&product) {
                    parent.products.push(product);
                    parent
                        .products
                        .push(format!("recipe:{}", robby_ir::run::recipes::METRIC_TEST));
                    parent
                        .stats
                        .insert("metric_test_run_id".to_owned(), record.run_id.clone());
                    store.put(&parent)?;
                }
            }
        }
    }

    Ok(exit_for_status(record.status))
}

fn print_metric_result(record: &robby_ir::RunRecord, json: bool) {
    if json {
        match serde_json::to_string_pretty(record) {
            Ok(raw) => println!("{raw}"),
            Err(err) => eprintln!("serialize run: {err}"),
        }
    } else {
        println!(
            "{} {} · {} · exit {:?}",
            record.recipe,
            record.status.as_str(),
            record.run_id,
            record.exit_code
        );
        if let Some(tail) = &record.stdout_tail {
            println!("{tail}");
        }
        println!("wrote {}", format!("runs/{}.json", record.run_id));
    }
}

fn exit_for_status(status: robby_ir::RunStatus) -> ExitCode {
    if status.is_success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
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

#[allow(clippy::too_many_arguments)]
fn review_doc_cmd(
    doc_path: &std::path::Path,
    ir: &std::path::Path,
    source_root: &std::path::Path,
    out: Option<&std::path::Path>,
    runs_out: Option<&std::path::Path>,
    run_id: Option<String>,
    metrics: Vec<String>,
    max_subjects: usize,
    reviews_out: Option<&std::path::Path>,
) -> Result<ExitCode> {
    let mut loaded = pipeline::load(ir, source_root)?;
    let markdown = std::fs::read_to_string(doc_path)
        .with_context(|| format!("read markdown {}", doc_path.display()))?;

    let doc_display = doc_path
        .strip_prefix(source_root)
        .unwrap_or(doc_path)
        .to_string_lossy()
        .replace('\\', "/");

    let opts = robby_agent::ReviewDocOpts {
        doc_path: doc_display.clone(),
        run_id,
        metrics,
        max_subjects,
    };
    let result = robby_agent::review_doc(&mut loaded.document, &markdown, &loaded.sources, &opts)?;

    let out_path = out.unwrap_or(ir);
    pipeline::write_document(&loaded.document, out_path)?;

    let artifact_rel = robby_agent::review_artifact_relpath(&doc_display);
    let artifact_path = match reviews_out {
        Some(dir) => {
            let stem = std::path::Path::new(&artifact_rel).file_name().map_or_else(
                || std::ffi::OsString::from("review.json"),
                OsStr::to_os_string,
            );
            dir.join(stem)
        }
        None => source_root.join(&artifact_rel),
    };
    if let Some(parent) = artifact_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let artifact = serde_json::json!({
        "recipe": "robby.review_doc",
        "run_id": result.record.run_id,
        "doc": doc_display,
        "subjects": result.subjects,
        "stats": {
            "findings": result.stats.findings,
            "relevant": result.stats.relevant,
            "metrics_claimed": result.stats.metrics_claimed,
            "metrics_tested": result.stats.metrics_tested,
            "subjects": result.stats.subjects,
        },
        "findings": result.findings.iter().map(|f| serde_json::json!({
            "decision_id": f.decision_id.as_str(),
            "metric": f.metric,
            "score": f.score,
            "subject": f.subject,
            "verdict": f.verdict.as_str(),
            "test_id": f.test_id,
        })).collect::<Vec<_>>(),
        "record": result.record,
    });
    std::fs::write(
        &artifact_path,
        format!("{}\n", serde_json::to_string_pretty(&artifact)?),
    )?;

    if let Some(runs_dir) = runs_out {
        let store = robby_render::SiteRunsStore::new(runs_dir);
        store.put(&result.record)?;
    }

    println!(
        "robby.review_doc {}: {} finding(s), {} relevant, {} subject(s)",
        result.record.run_id, result.stats.findings, result.stats.relevant, result.stats.subjects
    );
    println!("  ir → {}", out_path.display());
    println!("  review → {}", artifact_path.display());
    if let Some(runs_dir) = runs_out {
        println!(
            "  run → {}/runs/{}.json",
            runs_dir.display(),
            result.record.run_id
        );
    }
    for finding in &result.findings {
        println!(
            "  {} {}={} ({})",
            finding.verdict.as_str(),
            finding.metric,
            finding.score,
            finding.test_id
        );
    }
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
