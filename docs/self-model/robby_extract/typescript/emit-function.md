# `robby_extract::typescript::emit_function`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn emit_function<'a>(     doc: &mut Document,     f: &Function<'a>,     parent: &NodeId,     ctx: &Ctx<'_>,     pending: &mut Vec<PendingCall>, )` |
| location | [`crates/robby-extract/src/typescript.rs:591`](../../../../crates/robby-extract/src/typescript.rs#L591) |
| provenance | extractor · confidence 1.00 |

```rust
fn emit_function<'a>(
    doc: &mut Document,
    f: &Function<'a>,
    parent: &NodeId,
    ctx: &Ctx<'_>,
    pending: &mut Vec<PendingCall>,
) {
    let Some(id) = &f.id else {
        return;
    };
    let name = id.name.as_str();
    let fqn = format!("{}/{}", ctx.module, name);
    let mut node = item_node(kind::code::FUNCTION, &fqn, oxc_span(f.span, ctx), parent);
    node.process = Some(build_function_process(f, &fqn, pending, ctx));
    doc.merge(node);
}
```

```mermaid
sequenceDiagram
    autonumber
    participant emit_function as emit_function()
    participant as_str as as_str
    participant oxc_span as oxc_span
    participant item_node as item_node
    participant build_function_process as build_function_process
    participant Some as Some
    participant merge as merge
    emit_function-->>as_str: id.name.as_str()
    emit_function-->>emit_function: format!('{}/{}', ctx.module, name)
    emit_function-->>oxc_span: oxc_span(f.span, ctx)
    emit_function-->>item_node: item_node(kind::code::FUNCTION, &fqn, oxc_span(f.span, ctx), parent)
    emit_function-->>build_function_process: build_function_process(f, &fqn, pending, ctx)
    emit_function-->>Some: Some(build_function_process(f, &fqn, pending, ctx))
    emit_function-->>merge: doc.merge(node)
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::typescript::oxc_span`](oxc-span.md)
- calls → [`robby_extract::typescript::item_node`](item-node.md)
- calls → [`robby_extract::typescript::build_function_process`](build-function-process.md)
- calls → [`robby_ir::model::impl Document::merge`](../../robby_ir/model/index.md)

## References

- calls → [`robby_extract::typescript`](index.md) — build_function_process, item_node, oxc_span
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — merge

## Referenced by

- calls ← [`robby_extract::typescript::walk_declaration`](walk-declaration.md)
- calls ← [`robby_extract::typescript::walk_statements`](walk-statements.md)

