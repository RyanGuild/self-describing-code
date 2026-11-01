# `robby_cli::main`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn main() -> ExitCode` |
| location | [`crates/robby-cli/src/main.rs:73`](../../../crates/robby-cli/src/main.rs#L73) |
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
    alt match run()
    main-->>main: code
    else Err(err)
    main-->>main: eprintln!('error: {err: }')
    main-->>main: ExitCode::FAILURE
    end
```

3 steps · 0 observed · 3 possible only

