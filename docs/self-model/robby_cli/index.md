# `robby_cli`

[Self-model](../README.md)

| | |
|---|---|
| kind | `code.crate` |
| provenance | extractor · confidence 1.00 |
| nodes | 41 |

```mermaid
flowchart TD
    subgraph n_a331a60ee1568354["robby_cli [crate]"]
        direction TB
        n_874b29800ed524d0["build [module]"]
        n_c58f68f7fc449f63["example_self_model [module]"]
        n_099bba204f6689a2["Cli [struct]"]
        n_07cb3a5c0ad73319["Command [enum]"]
        n_eb50e92e5843791d["fn main() -> ExitCode [function]"]
        n_5fb1bc1f5dbdeb45["fn run() -> Result<ExitCode> [function]"]
        n_0b19cbdc9c790728["fn check(ir: &std::path::Path, source_root: &std::path::Path, json: bool) -> Result<ExitCode> [function]"]
        n_e3b7ee2025a4b7a1["fn print_human(loaded: &pipeline::Loaded) [function]"]
        n_b685ee67ac4f2f21["fn extract(root: &std::path::Path, out: Option<PathBuf>) -> Result<ExitCode> [function]"]
        n_310da90f0204c1aa["fn report( ir: &std::path::Path, source_root: &std::path::Path, out: Option<PathBuf>, ) -> Result<ExitCode> [function]"]
    end
    n_0b19cbdc9c790728 -->|calls| n_e3b7ee2025a4b7a1
    n_5fb1bc1f5dbdeb45 -->|calls| n_0b19cbdc9c790728
    n_5fb1bc1f5dbdeb45 -->|calls| n_310da90f0204c1aa
    n_5fb1bc1f5dbdeb45 -->|calls| n_b685ee67ac4f2f21
    n_b685ee67ac4f2f21 -->|calls| n_b685ee67ac4f2f21
```

## Modules

- [`build`](build/index.md) — 5 nodes
- [`example_self_model`](example_self_model/index.md) — 15 nodes

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`Cli`](cli.md) | `code.struct` | — | `crates/robby-cli/src/main.rs:14:1` |
| [`Command`](command.md) | `code.enum` | — | `crates/robby-cli/src/main.rs:25:1` |
| [`main`](main.md) | `code.function` | 3 | `crates/robby-cli/src/main.rs:73:1` |
| [`run`](run.md) | `code.function` | 18 | `crates/robby-cli/src/main.rs:83:1` |
| [`check`](check.md) | `code.function` | 6 | `crates/robby-cli/src/main.rs:120:1` |
| [`print_human`](print-human.md) | `code.function` | 12 | `crates/robby-cli/src/main.rs:141:1` |
| [`extract`](extract.md) | `code.function` | 11 | `crates/robby-cli/src/main.rs:182:1` |
| [`report`](report.md) | `code.function` | 7 | `crates/robby-cli/src/main.rs:203:1` |

## Imports

- `use std::path::PathBuf;`
- `use std::process::ExitCode;`
- `use anyhow::Result;`
- `use clap::{Parser, Subcommand};`
- `use robby_ir::Severity;`
- `use robby_render::pipeline;`

