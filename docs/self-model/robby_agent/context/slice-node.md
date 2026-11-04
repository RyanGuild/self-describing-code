# `robby_agent::context::slice_node`

[Self-model](../../README.md) · [context](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn slice_node(sources: &BTreeMap<String, String>, node: &Node) -> Option<String>` |
| location | [`crates/robby-agent/src/context.rs:177`](../../../../crates/robby-agent/src/context.rs#L177) |
| provenance | extractor · confidence 1.00 |

```rust
fn slice_node(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    source
        .get(span.start.byte as usize..span.end.byte as usize)
        .map(|s| s.to_owned())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant slice_node as slice_node()
    participant as_ref as as_ref
    participant get as get
    participant to_owned as to_owned
    participant map as map
    slice_node-->>as_ref: node.span.as_ref()
    slice_node-->>slice_node: ? (propagate)
    slice_node-->>get: sources.get(&span.file)
    slice_node-->>slice_node: ? (propagate)
    slice_node-->>get: source .get(span.start.byte as usize..span.end.byte as usize)
    slice_node-->>to_owned: s.to_owned()
    slice_node-->>map: source .get(span.start.byte as usize..span.end.byte as usize) .map(|s...
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get

## Referenced by

- calls ← [`robby_agent::context::build_for_docs_owner`](build-for-docs-owner.md)
- calls ← [`robby_agent::context::code_window_for`](code-window-for.md)

