# `robby_cli::example_self_model::Args`

[Self-model](../../README.md) · [example_self_model](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-cli/examples/self-model.rs:27`](../../../../crates/robby-cli/examples/self-model.rs#L27) |
| provenance | extractor · confidence 1.00 |

```rust
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
```

**Fields and variants**

- `root` — `crates/robby-cli/examples/self-model.rs:30:5`
- `ir` — `crates/robby-cli/examples/self-model.rs:33:5`
- `out` — `crates/robby-cli/examples/self-model.rs:36:5`
- `diagrams` — `crates/robby-cli/examples/self-model.rs:39:5`
- `document` — `crates/robby-cli/examples/self-model.rs:42:5`
- `stdout` — `crates/robby-cli/examples/self-model.rs:45:5`

