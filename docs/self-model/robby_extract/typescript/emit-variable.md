# `robby_extract::typescript::emit_variable`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn emit_variable<'a>(     doc: &mut Document,     decl: &VariableDeclarator<'a>,     parent: &NodeId,     ctx: &Ctx<'_>,     pending: &mut Vec<PendingCall>, )` |
| location | [`crates/robby-extract/src/typescript.rs:542`](../../../../crates/robby-extract/src/typescript.rs#L542) |
| provenance | extractor · confidence 1.00 |

```rust
fn emit_variable<'a>(
    doc: &mut Document,
    decl: &VariableDeclarator<'a>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let bindings = decl.id.get_binding_identifiers();
    let Some(id) = bindings.first() else {
        return;
    };
    let name = id.name.as_str();
    // Arrow / function expressions assigned to const.
    if let Some(Expression::ArrowFunctionExpression(arrow)) = &decl.init {
        let fqn = format!("{}/{}", ctx.module, name);
        let mut node = item_node(
            kind::code::FUNCTION,
            &fqn,
            oxc_span(decl.span, ctx),
            parent,
        );
        let mut process = Process::new();
        process.participant(sanitize(name), format!("{name}()"), Kind::from(kind::code::FUNCTION));
        let from = NodeId::symbol("typescript", &fqn);
        match &arrow.body {
            ArrowFunctionBody::FunctionBody(body) => {
                walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx);
            }
            other => {
                if let Some(expr) = other.as_expression() {
                    walk_expr(expr, &mut process, &[], pending, &from, ctx);
                }
            }
        }
        node.process = Some(process);
        doc.merge(node);
    } else if let Some(Expression::FunctionExpression(f)) = &decl.init {
        let fqn = format!("{}/{}", ctx.module, name);
        let mut node = item_node(
            kind::code::FUNCTION,
            &fqn,
            oxc_span(decl.span, ctx),
            parent,
        );
        node.process = Some(build_function_process(f, &fqn, pending, ctx));
        doc.merge(node);
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant emit_variable as emit_variable()
    participant get_binding_identifiers as get_binding_identifiers
    participant first as first
    participant as_str as as_str
    participant oxc_span as oxc_span
    participant item_node as item_node
    participant Process__new as Process::new
    participant sanitize as sanitize
    participant Kind__from as Kind::from
    participant participant as participant
    participant NodeId__symbol as NodeId::symbol
    participant walk_stmts as walk_stmts
    participant as_expression as as_expression
    participant walk_expr as walk_expr
    participant Some as Some
    participant merge as merge
    participant build_function_process as build_function_process
    emit_variable-->>get_binding_identifiers: decl.id.get_binding_identifiers()
    emit_variable-->>first: bindings.first()
    emit_variable-->>as_str: id.name.as_str()
    alt let Some(Expression::ArrowFunctionExpression(arrow)) = &decl.init
    emit_variable-->>emit_variable: format!('{}/{}', ctx.module, name)
    emit_variable-->>oxc_span: oxc_span(decl.span, ctx)
    emit_variable-->>item_node: item_node( kind::code::FUNCTION, &fqn, oxc_span(decl.span, ctx), pare...
    emit_variable-->>Process__new: Process::new()
    emit_variable-->>sanitize: sanitize(name)
    emit_variable-->>emit_variable: format!('{name}()')
    emit_variable-->>Kind__from: Kind::from(kind::code::FUNCTION)
    emit_variable-->>participant: process.participant(sanitize(name), format!('{name}()'), Kind::from(k...
    emit_variable-->>NodeId__symbol: NodeId::symbol('typescript', &fqn)
    alt match &arrow.body
    emit_variable-->>walk_stmts: walk_stmts(&body.statements, &mut process, &[], pending, &from, ctx)
    else other
    emit_variable-->>as_expression: other.as_expression()
    alt let Some(expr) = other.as_expression()
    emit_variable-->>walk_expr: walk_expr(expr, &mut process, &[], pending, &from, ctx)
    end
    end
    emit_variable-->>Some: Some(process)
    emit_variable-->>merge: doc.merge(node)
    else else
    alt let Some(Expression::FunctionExpression(f)) = &decl.init
    emit_variable-->>emit_variable: format!('{}/{}', ctx.module, name)
    emit_variable-->>oxc_span: oxc_span(decl.span, ctx)
    emit_variable-->>item_node: item_node( kind::code::FUNCTION, &fqn, oxc_span(decl.span, ctx), pare...
    emit_variable-->>build_function_process: build_function_process(f, &fqn, pending, ctx)
    emit_variable-->>Some: Some(build_function_process(f, &fqn, pending, ctx))
    emit_variable-->>merge: doc.merge(node)
    end
    end
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::typescript::oxc_span`](oxc-span.md)
- calls → [`robby_extract::typescript::item_node`](item-node.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_extract::typescript::sanitize`](sanitize.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_extract::typescript::walk_stmts`](walk-stmts.md)
- calls → [`robby_extract::typescript::walk_expr`](walk-expr.md)
- calls → [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)
- calls → [`robby_extract::typescript::oxc_span`](oxc-span.md)
- calls → [`robby_extract::typescript::item_node`](item-node.md)
- calls → [`robby_extract::typescript::build_function_process`](build-function-process.md)
- calls → [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_extract::typescript`](index.md) — 6 items
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str, from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — merge, symbol
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant

## Referenced by

- calls ← [`robby_extract::typescript::walk_declaration`](walk-declaration.md)
- calls ← [`robby_extract::typescript::walk_statements`](walk-statements.md)

