# `robby_ir::coverage::has_span_descendant`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn has_span_descendant(doc: &Document, id: &crate::model::NodeId) -> bool` |
| location | [`crates/robby-ir/src/coverage.rs:247`](../../../../crates/robby-ir/src/coverage.rs#L247) |
| provenance | extractor · confidence 1.00 |

```rust
fn has_span_descendant(doc: &Document, id: &crate::model::NodeId) -> bool {
    let Some(node) = doc.get(id) else {
        return false;
    };
    node.children.iter().any(|child| {
        doc.get(child).is_some_and(|c| c.span.is_some()) || has_span_descendant(doc, child)
    })
}
```

```mermaid
sequenceDiagram
    autonumber
    participant has_span_descendant as has_span_descendant()
    participant get as get
    participant iter as iter
    participant is_some as is_some
    participant is_some_and as is_some_and
    participant any as any
    has_span_descendant-->>get: doc.get(id)
    has_span_descendant-->>iter: node.children.iter()
    has_span_descendant-->>get: doc.get(child)
    has_span_descendant-->>is_some: c.span.is_some()
    has_span_descendant-->>is_some_and: doc.get(child).is_some_and(|c| c.span.is_some())
    has_span_descendant-->>has_span_descendant: has_span_descendant(doc, child)
    has_span_descendant-->>any: node.children.iter().any(|child| { doc.get(child).is_some_and(|c| c.s...
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../run/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../run/index.md)
- calls → [`robby_ir::coverage::has_span_descendant`](has-span-descendant.md)

## References

- calls → [`robby_ir::run`](../run/index.md) — get

## Referenced by

- calls ← [`robby_ir::coverage::frontier_spans`](frontier-spans.md)

