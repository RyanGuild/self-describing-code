# `robby_extract::walk_stmts`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn walk_stmts(builder: &mut ProcBuilder<'_, '_>, stmts: &[Stmt])` |
| location | [`crates/robby-extract/src/lib.rs:636`](../../../crates/robby-extract/src/lib.rs#L636) |
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
    participant emit as emit
    participant clone as clone
    loop stmts
    alt match stmt
    alt match &local.init
    walk_stmts-->>walk_expr: walk_expr(builder, &init.expr)
    else None
    walk_stmts-->>span_of: span_of(local, builder.ctx)
    walk_stmts-->>emit: builder.emit( &builder.me.clone(), label_of(builder.ctx, span.clone()...
    end
    walk_stmts-->>walk_expr: walk_expr(builder, expr)
    else Stmt::Macro(m)
    walk_stmts-->>span_of: span_of(m, builder.ctx)
    walk_stmts-->>clone: builder.me.clone()
    walk_stmts-->>emit: builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note...
    else Stmt::Item(item)
    walk_stmts-->>span_of: span_of(item, builder.ctx)
    walk_stmts-->>clone: builder.me.clone()
    walk_stmts-->>emit: builder.emit(&me, label_of(builder.ctx, span.clone()), StepKind::Note...
    end
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_extract::walk_expr`](walk-expr.md)
- calls → [`robby_extract::span_of`](span-of.md)
- calls → [`robby_extract::impl ProcBuilder<'_, '_>::emit`](index.md)
- calls → [`robby_extract::walk_expr`](walk-expr.md)
- calls → [`robby_extract::span_of`](span-of.md)
- calls → [`robby_extract::impl ProcBuilder<'_, '_>::emit`](index.md)
- calls → [`robby_extract::span_of`](span-of.md)
- calls → [`robby_extract::impl ProcBuilder<'_, '_>::emit`](index.md)

