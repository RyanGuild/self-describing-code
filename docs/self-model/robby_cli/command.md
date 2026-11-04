# `robby_cli::Command`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-cli/src/main.rs:27`](../../../crates/robby-cli/src/main.rs#L27) |
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
}
```

**Fields and variants**

- `Schema` — `crates/robby-cli/src/main.rs:29:5`
- `Check` — `crates/robby-cli/src/main.rs:35:5`
- `Render` — `crates/robby-cli/src/main.rs:46:5`
- `Extract` — `crates/robby-cli/src/main.rs:53:5`
- `Report` — `crates/robby-cli/src/main.rs:62:5`
- `Show` — `crates/robby-cli/src/main.rs:73:5`
- `EnrichComments` — `crates/robby-cli/src/main.rs:100:5`
- `Web` — `crates/robby-cli/src/main.rs:117:5`

