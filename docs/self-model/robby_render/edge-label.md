# `robby_render::edge_label`

[Self-model](../README.md) · [robby_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn edge_label(kind: EdgeKind) -> &'static str` |
| location | [`crates/robby-render/src/lib.rs:175`](../../../crates/robby-render/src/lib.rs#L175) |
| provenance | extractor · confidence 1.00 |

```rust
fn edge_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Calls => "calls",
        EdgeKind::Uses => "uses",
        EdgeKind::Imports => "imports",
        EdgeKind::Implements => "implements",
        EdgeKind::Extends => "extends",
        EdgeKind::Returns => "returns",
        EdgeKind::Field => "field",
        EdgeKind::References => "refs",
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant edge_label as edge_label()
    alt match kind
    edge_label-->>edge_label: 'contains'
    else EdgeKind::Calls
    edge_label-->>edge_label: 'calls'
    else EdgeKind::Uses
    edge_label-->>edge_label: 'uses'
    else EdgeKind::Imports
    edge_label-->>edge_label: 'imports'
    else EdgeKind::Implements
    edge_label-->>edge_label: 'implements'
    else EdgeKind::Extends
    edge_label-->>edge_label: 'extends'
    else EdgeKind::Returns
    edge_label-->>edge_label: 'returns'
    else EdgeKind::Field
    edge_label-->>edge_label: 'field'
    else EdgeKind::References
    edge_label-->>edge_label: 'refs'
    end
```

9 steps · 0 observed · 9 possible only

