# `robby_cli::example_self_model`

[Self-model](../../README.md) · [robby_cli](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 15 |

## Entry

```mermaid
sequenceDiagram
    autonumber
    participant main as main()
    participant run as run
    main-->>run: run()
    alt match run()
    main-->>main: eprintln!('error: {err: }')
    end
```

## Types

```mermaid
classDiagram
    class Args {
        +root
        +ir
        +out
        +diagrams
        +document
        +stdout
    }
```

## Package diagram

```mermaid
flowchart TB
    n_c58f68f7fc449f63["example_self_model"]
    n_f3e9f97d91a4f4af(["robby_agent"])
    n_a331a60ee1568354(["robby_cli"])
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_c58f68f7fc449f63 --> n_356bf2c23da68961
    n_c58f68f7fc449f63 --> n_9c636b098ca4450e
    n_c58f68f7fc449f63 --> n_a331a60ee1568354
    n_c58f68f7fc449f63 --> n_f3e9f97d91a4f4af
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_c58f68f7fc449f63["example_self_model"]
        direction TB
        n_59c2c49c5fb01842["main"]
        n_32b42d0d55443cd4["run"]
    end
    n_59c2c49c5fb01842 --> n_32b42d0d55443cd4
```

## Dependency matrix

| | example_self_model | robby_ir | robby_render | robby_cli | robby_agent |
|---|---:|---:|---:|---:|---:|
| **example_self_model** | · | 1 | 6 | 1 | 1 |
| **robby_ir** | | · | | | |
| **robby_render** | | | · | | |
| **robby_cli** | | | | · | |
| **robby_agent** | | | | | · |
## References

- calls → [`robby_agent::model`](../../robby_agent/model/index.md) — parse
- calls → [`robby_cli`](../index.md) — extract
- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — failures
- calls → [`robby_render::pipeline`](../../robby_render/pipeline/index.md) — 6 items

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`Args`](args.md) | `code.struct` | — | `crates/robby-cli/examples/self-model.rs:27:1` |
| [`main`](main.md) | `code.function` | 2 | `crates/robby-cli/examples/self-model.rs:50:1` |
| [`run`](run.md) | `code.function` | 27 | `crates/robby-cli/examples/self-model.rs:60:1` |

## Imports

- `use std::path::PathBuf;`
- `use std::process::ExitCode;`
- `use anyhow::Result;`
- `use clap::Parser;`
- [`use robby_render::pipeline;`](../../robby_render/pipeline/index.md)

