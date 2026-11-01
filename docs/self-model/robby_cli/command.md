# `robby_cli::Command`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-cli/src/main.rs:25`](../../../crates/robby-cli/src/main.rs#L25) |
| provenance | extractor · confidence 1.00 |

```rust
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
```

**Fields and variants**

- `Schema` — `crates/robby-cli/src/main.rs:27:5`
- `Check` — `crates/robby-cli/src/main.rs:33:5`
- `Render` — `crates/robby-cli/src/main.rs:44:5`
- `Extract` — `crates/robby-cli/src/main.rs:51:5`
- `Report` — `crates/robby-cli/src/main.rs:60:5`

