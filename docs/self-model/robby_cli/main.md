# `robby_cli::main`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn main() -> ExitCode` |
| location | [`crates/robby-cli/src/main.rs:149`](../../../crates/robby-cli/src/main.rs#L149) |
| provenance | extractor · confidence 1.00 |

```rust
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::FAILURE
        }
    }
}
```

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

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_cli::run`](run.md)

## References

- calls → [`robby_cli`](index.md) — run

