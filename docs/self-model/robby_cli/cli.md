# `robby_cli::Cli`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-cli/src/main.rs:14`](../../../crates/robby-cli/src/main.rs#L14) |
| provenance | extractor · confidence 1.00 |

```rust
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
```

**Fields and variants**

- `command` — `crates/robby-cli/src/main.rs:21:5`

