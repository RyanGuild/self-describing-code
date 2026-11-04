# `robby_extract::typescript::build_function_process`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn build_function_process<'a>(     f: &Function<'a>,     fqn: &str,     pending: &mut Vec<PendingCall>,     ctx: &Ctx<'_>, ) -> Process` |
| location | [`crates/robby-extract/src/typescript.rs:732`](../../../../crates/robby-extract/src/typescript.rs#L732) |
| provenance | extractor · confidence 1.00 |

```rust
fn build_function_process<'a>(
    f: &Function<'a>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
    ctx: &Ctx<'_>,
) -> Process {
    let name = f
        .id
        .as_ref()
        .map(|i| i.name.as_str())
        .unwrap_or_else(|| fqn.rsplit('/').next().unwrap_or(fqn));
    let mut process = Process::new();
    process.participant(
        sanitize(name),
        format!("{name}()"),
        Kind::from(kind::code::FUNCTION),
    );
    let from = NodeId::symbol("typescript", fqn);
    if let Some(body) = &f.body {
        walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx);
    }
    process
}
```

```mermaid
sequenceDiagram
    autonumber
    participant build_function_process as build_function_process()
    participant as_ref as as_ref
    participant as_str as as_str
    participant map as map
    participant rsplit as rsplit
    participant next as next
    participant unwrap_or as unwrap_or
    participant unwrap_or_else as unwrap_or_else
    participant Process__new as Process::new
    participant sanitize as sanitize
    participant Kind__from as Kind::from
    participant participant as participant
    participant NodeId__symbol as NodeId::symbol
    participant walk_stmts as walk_stmts
    build_function_process-->>as_ref: f .id .as_ref()
    build_function_process-->>as_str: i.name.as_str()
    build_function_process-->>map: f .id .as_ref() .map(|i| i.name.as_str())
    build_function_process-->>rsplit: fqn.rsplit('/')
    build_function_process-->>next: fqn.rsplit('/').next()
    build_function_process-->>unwrap_or: fqn.rsplit('/').next().unwrap_or(fqn)
    build_function_process-->>unwrap_or_else: f .id .as_ref() .map(|i| i.name.as_str()) .unwrap_or_else(|| fqn.rspl...
    build_function_process-->>Process__new: Process::new()
    build_function_process-->>sanitize: sanitize(name)
    build_function_process-->>build_function_process: format!('{name}()')
    build_function_process-->>Kind__from: Kind::from(kind::code::FUNCTION)
    build_function_process-->>participant: process.participant( sanitize(name), format!('{name}()'), Kind::from(...
    build_function_process-->>NodeId__symbol: NodeId::symbol('typescript', fqn)
    alt let Some(body) = &f.body
    build_function_process-->>walk_stmts: walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx)
    end
```

14 steps · 0 observed · 14 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_extract::typescript::sanitize`](sanitize.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_extract::typescript::walk_stmts`](walk-stmts.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::typescript`](index.md) — sanitize, walk_stmts
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str, from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — symbol
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant

## Referenced by

- calls ← [`robby_extract::typescript::emit_function`](emit-function.md)
- calls ← [`robby_extract::typescript::emit_method`](emit-method.md)
- calls ← [`robby_extract::typescript::emit_variable`](emit-variable.md)

