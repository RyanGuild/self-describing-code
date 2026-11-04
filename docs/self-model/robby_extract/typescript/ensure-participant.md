# `robby_extract::typescript::ensure_participant`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn ensure_participant(process: &mut Process, name: &str)` |
| location | [`crates/robby-extract/src/typescript.rs:991`](../../../../crates/robby-extract/src/typescript.rs#L991) |
| provenance | extractor · confidence 1.00 |

```rust
fn ensure_participant(process: &mut Process, name: &str) {
    process.participant(
        sanitize(name),
        name.to_owned(),
        Kind::from(kind::code::FUNCTION),
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant ensure_participant as ensure_participant()
    participant sanitize as sanitize
    participant to_owned as to_owned
    participant Kind__from as Kind::from
    participant participant as participant
    ensure_participant-->>sanitize: sanitize(name)
    ensure_participant-->>to_owned: name.to_owned()
    ensure_participant-->>Kind__from: Kind::from(kind::code::FUNCTION)
    ensure_participant-->>participant: process.participant( sanitize(name), name.to_owned(), Kind::from(kind...
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_extract::typescript::sanitize`](sanitize.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)

## References

- calls → [`robby_extract::typescript`](index.md) — sanitize
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant

## Referenced by

- calls ← [`robby_extract::typescript::walk_expr`](walk-expr.md)

