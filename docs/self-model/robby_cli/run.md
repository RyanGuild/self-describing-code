# `robby_cli::run`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn run() -> Result<ExitCode>` |
| location | [`crates/robby-cli/src/main.rs:83`](../../../crates/robby-cli/src/main.rs#L83) |
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
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant run as run()
    participant Cli__parse as Cli::parse
    participant robby_ir__schema_json as robby_ir::schema_json
    participant std__fs__create_dir_all as std::fs::create_dir_all
    participant std__fs__write as std::fs::write
    participant Ok as Ok
    participant check as check
    participant pipeline__load as pipeline::load
    participant pipeline__render_files as pipeline::render_files
    participant extract as extract
    participant report as report
    run-->>Cli__parse: Cli::parse()
    alt match cli.command
    run-->>robby_ir__schema_json: robby_ir::schema_json()
    alt match out
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
    end
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby_ir::schema_json`](../robby_ir/schema-json.md)
- calls → [`robby_cli::check`](check.md)
- calls → [`robby_render::pipeline::load`](../robby_render/pipeline/load.md)
- calls → [`robby_render::pipeline::render_files`](../robby_render/pipeline/render-files.md)
- calls → [`robby_cli::extract`](extract.md)
- calls → [`robby_cli::report`](report.md)

