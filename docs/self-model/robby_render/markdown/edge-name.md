# `robby_render::markdown::edge_name`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn edge_name(kind: EdgeKind) -> &'static str` |
| location | [`crates/robby-render/src/markdown.rs:219`](../../../../crates/robby-render/src/markdown.rs#L219) |
| provenance | extractor · confidence 1.00 |

```rust
pub(crate) fn edge_name(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Calls => "calls",
        EdgeKind::Uses => "uses",
        EdgeKind::Imports => "imports",
        EdgeKind::Implements => "implements",
        EdgeKind::Extends => "extends",
        EdgeKind::Returns => "returns",
        EdgeKind::Field => "field",
        EdgeKind::References => "references",
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant edge_name as edge_name()
    alt match kind
    edge_name-->>edge_name: 'contains'
    else EdgeKind::Calls
    edge_name-->>edge_name: 'calls'
    else EdgeKind::Uses
    edge_name-->>edge_name: 'uses'
    else EdgeKind::Imports
    edge_name-->>edge_name: 'imports'
    else EdgeKind::Implements
    edge_name-->>edge_name: 'implements'
    else EdgeKind::Extends
    edge_name-->>edge_name: 'extends'
    else EdgeKind::Returns
    edge_name-->>edge_name: 'returns'
    else EdgeKind::Field
    edge_name-->>edge_name: 'field'
    else EdgeKind::References
    edge_name-->>edge_name: 'references'
    end
```

9 steps · 0 observed · 9 possible only

