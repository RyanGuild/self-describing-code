# `robby_extract::rust::build_process_sig`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn build_process_sig(     sig: &syn::Signature,     block: &syn::Block,     ctx: &Ctx<'_>,     fqn: &str,     pending: &mut Vec<PendingCall>, ) -> Process` |
| location | [`crates/robby-extract/src/rust.rs:546`](../../../../crates/robby-extract/src/rust.rs#L546) |
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
    participant rsplit as rsplit
    participant next as next
    participant unwrap_or as unwrap_or
    participant to_owned as to_owned
    participant Process__new as Process::new
    participant Vec__new as Vec::new
    participant clone as clone
    participant sanitize as sanitize
    participant Kind__from as Kind::from
    participant participant as participant
    participant walk_stmts as walk_stmts
    build_process_sig-->>rsplit: fqn.rsplit('::')
    build_process_sig-->>next: fqn.rsplit('::').next()
    build_process_sig-->>unwrap_or: fqn.rsplit('::').next().unwrap_or(fqn)
    build_process_sig-->>to_owned: fqn.rsplit('::').next().unwrap_or(fqn).to_owned()
    build_process_sig-->>Process__new: Process::new()
    build_process_sig-->>Vec__new: Vec::new()
    build_process_sig-->>clone: me.clone()
    build_process_sig-->>to_owned: fqn.to_owned()
    build_process_sig-->>sanitize: sanitize(&me)
    build_process_sig-->>build_process_sig: format!('{}()', me)
    build_process_sig-->>Kind__from: Kind::from(kind::code::FUNCTION)
    build_process_sig-->>participant: builder.process.participant( sanitize(&me), format!('{}()', me), Kind...
    build_process_sig-->>walk_stmts: walk_stmts(&mut builder, &block.stmts)
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)
- calls → [`robby_extract::rust::sanitize`](sanitize.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_extract::rust::walk_stmts`](walk-stmts.md)

## References

- calls → [`robby_extract::rust`](index.md) — new, sanitize, walk_stmts
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant

## Referenced by

- calls ← [`robby_extract::rust::build_process`](build-process.md)
- calls ← [`robby_extract::rust::walk_items`](walk-items.md)

