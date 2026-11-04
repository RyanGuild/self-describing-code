# `robby_extract::rust::walk_stmts`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn walk_stmts(builder: &mut ProcBuilder<'_, '_>, stmts: &[Stmt])` |
| location | [`crates/robby-extract/src/rust.rs:622`](../../../../crates/robby-extract/src/rust.rs#L622) |
| provenance | extractor · confidence 1.00 |

```rust
fn walk_stmts(builder: &mut ProcBuilder<'_, '_>, stmts: &[Stmt]) {
    for stmt in stmts {
        match stmt {
            Stmt::Local(local) => match &local.init {
                Some(init) => walk_expr(builder, &init.expr),
                None => {
                    let span = span_of(local, builder.ctx);
                    builder.emit(
                        &builder.me.clone(),
                        label_of(builder.ctx, span.clone()),
                        StepKind::Note,
                        span,
                    );
                }
            },
            Stmt::Expr(expr, _) => walk_expr(builder, expr),
            Stmt::Macro(m) => {
                let span = span_of(m, builder.ctx);
                let me = builder.me.clone();
                builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note, span);
            }
            Stmt::Item(item) => {
                let span = span_of(item, builder.ctx);
                let me = builder.me.clone();
                builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note, span);
            }
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant walk_stmts as walk_stmts()
    participant walk_expr as walk_expr
    participant span_of as span_of
    participant clone as clone
    participant label_of as label_of
    participant emit as emit
    loop stmts
    alt match stmt
    alt match &local.init
    walk_stmts-->>walk_expr: walk_expr(builder, &init.expr)
    else None
    walk_stmts-->>span_of: span_of(local, builder.ctx)
    walk_stmts-->>clone: builder.me.clone()
    walk_stmts-->>clone: span.clone()
    walk_stmts-->>label_of: label_of(builder.ctx, span.clone())
    walk_stmts-->>emit: builder.emit( &builder.me.clone(), label_of(builder.ctx, span.clone()...
    end
    else Stmt::Expr(expr, _)
    walk_stmts-->>walk_expr: walk_expr(builder, expr)
    else Stmt::Macro(m)
    walk_stmts-->>span_of: span_of(m, builder.ctx)
    walk_stmts-->>clone: builder.me.clone()
    walk_stmts-->>clone: span.clone()
    walk_stmts-->>label_of: label_of(builder.ctx, span.clone())
    walk_stmts-->>emit: builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note...
    else Stmt::Item(item)
    walk_stmts-->>span_of: span_of(item, builder.ctx)
    walk_stmts-->>clone: builder.me.clone()
    walk_stmts-->>clone: span.clone()
    walk_stmts-->>label_of: label_of(builder.ctx, span.clone())
    walk_stmts-->>emit: builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note...
    end
    end
```

17 steps · 0 observed · 17 possible only

**Edges**

- calls → [`robby_extract::rust::walk_expr`](walk-expr.md)
- calls → [`robby_extract::rust::span_of`](span-of.md)
- calls → [`robby_extract::rust::label_of`](label-of.md)
- calls → [`robby_extract::rust::impl ProcBuilder<'_, '_>::emit`](index.md)
- calls → [`robby_extract::rust::walk_expr`](walk-expr.md)
- calls → [`robby_extract::rust::span_of`](span-of.md)
- calls → [`robby_extract::rust::label_of`](label-of.md)
- calls → [`robby_extract::rust::impl ProcBuilder<'_, '_>::emit`](index.md)
- calls → [`robby_extract::rust::span_of`](span-of.md)
- calls → [`robby_extract::rust::label_of`](label-of.md)
- calls → [`robby_extract::rust::impl ProcBuilder<'_, '_>::emit`](index.md)

## References

- calls → [`robby_extract::rust`](index.md) — emit, label_of, span_of, walk_expr

## Referenced by

- calls ← [`robby_extract::rust::build_process_sig`](build-process-sig.md)
- calls ← [`robby_extract::rust::walk_expr`](walk-expr.md)

