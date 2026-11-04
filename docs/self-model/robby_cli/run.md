# `robby_cli::run`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn run() -> Result<ExitCode>` |
| location | [`crates/robby-cli/src/main.rs:159`](../../../crates/robby-cli/src/main.rs#L159) |
| provenance | extractor · confidence 1.00 |

```rust
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
        } => enrich_comments(&ir, &source_root, out.as_deref(), decision_model, note_model, limit),
        Command::Web {
            ir,
            source_root,
            out,
            watch,
            watch_interval_ms,
        } => web(&ir, &source_root, &out, watch, watch_interval_ms),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant run as run()
    participant Cli__parse as Cli::parse
    participant robby_ir__schema_json as robby_ir::schema_json
    participant parent as parent
    participant std__fs__create_dir_all as std::fs::create_dir_all
    participant std__fs__write as std::fs::write
    participant Ok as Ok
    participant check as check
    participant PathBuf__from as PathBuf::from
    participant pipeline__load as pipeline::load
    participant pipeline__render_files as pipeline::render_files
    participant extract as extract
    participant report as report
    participant as_deref as as_deref
    participant show as show
    participant enrich_comments as enrich_comments
    participant web as web
    run-->>Cli__parse: Cli::parse()
    alt match cli.command
    run-->>robby_ir__schema_json: robby_ir::schema_json()
    alt match out
    run-->>parent: path.parent()
    alt let Some(parent) = path.parent()
    run-->>std__fs__create_dir_all: std::fs::create_dir_all(parent)
    run-->>run: ? (propagate)
    end
    run-->>std__fs__write: std::fs::write(&path, schema)
    run-->>run: ? (propagate)
    run-->>run: println!('wrote {}', path.display())
    else None
    run-->>run: println!('{schema}')
    end
    run-->>Ok: Ok(ExitCode::SUCCESS)
    else Command::Check { ir, source_root, json, }
    run-->>check: check(&ir, &source_root, json)
    else Command::Render { ir, out }
    run-->>PathBuf__from: PathBuf::from('.')
    run-->>pipeline__load: pipeline::load(&ir, &PathBuf::from('.'))
    run-->>run: ? (propagate)
    run-->>pipeline__render_files: pipeline::render_files(&loaded.document, &out)
    run-->>run: ? (propagate)
    run-->>run: println!('wrote {count} diagram(s) to {}', out.display())
    run-->>Ok: Ok(ExitCode::SUCCESS)
    else Command::Extract { root, out }
    run-->>extract: extract(&root, out)
    else Command::Report { ir, source_root, out, }
    run-->>report: report(&ir, &source_root, out)
    else Command::Show { path, ir, source_root, depth, width, all, view, no_so...
    run-->>as_deref: path.as_deref()
    run-->>as_deref: ir.as_deref()
    run-->>show: show( path.as_deref(), ir.as_deref(), &source_root, depth, width, all...
    else Command::EnrichComments { ir, source_root, out, decision_model, note_...
    run-->>as_deref: out.as_deref()
    run-->>enrich_comments: enrich_comments(&ir, &source_root, out.as_deref(), decision_model, no...
    else Command::Web { ir, source_root, out, watch, watch_interval_ms, }
    run-->>web: web(&ir, &source_root, &out, watch, watch_interval_ms)
    end
```

26 steps · 0 observed · 26 possible only

**Edges**

- calls → [`robby_ir::run::impl RunStatus::parse`](../robby_ir/run/index.md)
- calls → [`robby_ir::schema_json`](../robby_ir/schema-json.md)
- calls → [`robby_cli::check`](check.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../robby_ir/kind/index.md)
- calls → [`robby_render::pipeline::load`](../robby_render/pipeline/load.md)
- calls → [`robby_render::pipeline::render_files`](../robby_render/pipeline/render-files.md)
- calls → [`robby_cli::extract`](extract.md)
- calls → [`robby_cli::report`](report.md)
- calls → [`robby_cli::show`](show.md)
- calls → [`robby_cli::enrich_comments`](enrich-comments.md)
- calls → [`robby_cli::web`](web.md)

## References

- calls → [`robby_cli`](index.md) — 6 items
- calls → [`robby_ir`](../robby_ir/index.md) — schema_json
- calls → [`robby_ir::kind`](../robby_ir/kind/index.md) — from
- calls → [`robby_ir::run`](../robby_ir/run/index.md) — parse
- calls → [`robby_render::pipeline`](../robby_render/pipeline/index.md) — load, render_files

## Referenced by

- calls ← [`robby_cli::main`](main.md)

