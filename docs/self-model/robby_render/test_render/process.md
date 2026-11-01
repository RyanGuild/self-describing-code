# `robby_render::test_render::process`

[Self-model](../../README.md) · [test_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn process(steps: Vec<Step>) -> Process` |
| location | [`crates/robby-render/tests/render.rs:21`](../../../../crates/robby-render/tests/render.rs#L21) |
| provenance | extractor · confidence 1.00 |

```rust
fn process(steps: Vec<Step>) -> Process {
    let mut p = Process::new();
    p.participant("caller", "caller", Kind::from(code::FUNCTION));
    p.participant("io", "io::load", Kind::from(code::FUNCTION));
    for s in steps {
        p.step(s);
    }
    p
}
```

```mermaid
sequenceDiagram
    autonumber
    participant process as process()
    participant Process__new as Process::new
    participant participant as participant
    participant step as step
    process-->>Process__new: Process::new()
    process-->>participant: p.participant('caller', 'caller', Kind::from(code::FUNCTION))
    process-->>participant: p.participant('io', 'io::load', Kind::from(code::FUNCTION))
    loop steps
    process-->>step: p.step(s)
    end
    process-->>process: p
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::process::impl Process::step`](../../robby_ir/process/index.md)

