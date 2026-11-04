# `robby_extract::typescript::push_step`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn push_step(process: &mut Process, frames: &[Block], kind: StepKind, from: &str, to: &str)` |
| location | [`crates/robby-extract/src/typescript.rs:979`](../../../../crates/robby-extract/src/typescript.rs#L979) |
| provenance | extractor · confidence 1.00 |

```rust
fn push_step(process: &mut Process, frames: &[Block], kind: StepKind, from: &str, to: &str) {
    process.step(Step {
        from: from.into(),
        to: to.into(),
        label: String::new(),
        kind,
        observed: false,
        span: None,
        blocks: frames.to_vec(),
    });
}
```

```mermaid
sequenceDiagram
    autonumber
    participant push_step as push_step()
    participant into as into
    participant String__new as String::new
    participant to_vec as to_vec
    participant step as step
    push_step-->>into: from.into()
    push_step-->>into: to.into()
    push_step-->>String__new: String::new()
    push_step-->>to_vec: frames.to_vec()
    push_step-->>step: process.step(Step { from: from.into(), to: to.into(), label: String::...
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_render::test_render::step`](../../robby_render/test_render/step.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_render::test_render`](../../robby_render/test_render/index.md) — step

## Referenced by

- calls ← [`robby_extract::typescript::walk_expr`](walk-expr.md)
- calls ← [`robby_extract::typescript::walk_stmts`](walk-stmts.md)

