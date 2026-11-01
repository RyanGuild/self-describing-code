# `robby_extract::build_process_sig`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn build_process_sig(     sig: &syn::Signature,     block: &syn::Block,     ctx: &Ctx<'_>,     fqn: &str,     pending: &mut Vec<PendingCall>, ) -> Process` |
| location | [`crates/robby-extract/src/lib.rs:560`](../../../crates/robby-extract/src/lib.rs#L560) |
| provenance | extractor · confidence 1.00 |

```rust
fn build_process_sig(
    sig: &syn::Signature,
    block: &syn::Block,
    ctx: &Ctx<'_>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
) -> Process {
    let me = fqn.rsplit("::").next().unwrap_or(fqn).to_owned();
    let mut builder = ProcBuilder {
        ctx,
        process: Process::new(),
        blocks: Vec::new(),
        me: me.clone(),
        me_id: fqn.to_owned(),
        pending,
    };
    builder.process.participant(
        sanitize(&me),
        format!("{}()", me),
        Kind::from(kind::code::FUNCTION),
    );
    let _ = sig;
    walk_stmts(&mut builder, &block.stmts);
    builder.process
}
```

```mermaid
sequenceDiagram
    autonumber
    participant build_process_sig as build_process_sig()
    participant to_owned as to_owned
    participant participant as participant
    participant walk_stmts as walk_stmts
    build_process_sig-->>to_owned: fqn.rsplit('::').next().unwrap_or(fqn).to_owned()
    build_process_sig-->>build_process_sig: ProcBuilder { ctx, process: Process::new(), blocks: Vec::new(), me: m...
    build_process_sig-->>participant: builder.process.participant( sanitize(&me), format!('{}()', me), Kind...
    build_process_sig-->>build_process_sig: sig
    build_process_sig-->>walk_stmts: walk_stmts(&mut builder, &block.stmts)
    build_process_sig-->>build_process_sig: builder.process
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::process::impl Process::participant`](../robby_ir/process/index.md)
- calls → [`robby_extract::walk_stmts`](walk-stmts.md)

