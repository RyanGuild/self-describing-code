# `robby_render::markdown::heading_depth`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn heading_depth(doc: &Document, node: &Node) -> usize` |
| location | [`crates/robby-render/src/markdown.rs:238`](../../../../crates/robby-render/src/markdown.rs#L238) |
| provenance | extractor · confidence 1.00 |

```rust
fn heading_depth(doc: &Document, node: &Node) -> usize {
    let mut depth = 3;
    let mut current = node.parent.clone();
    while let Some(parent) = current {
        depth += 1;
        current = doc.get(&parent).and_then(|p| p.parent.clone());
    }
    depth
}
```

```mermaid
sequenceDiagram
    autonumber
    participant heading_depth as heading_depth()
    participant clone as clone
    participant get as get
    participant and_then as and_then
    heading_depth-->>clone: node.parent.clone()
    loop let Some(parent) = current
    heading_depth-->>get: doc.get(&parent)
    heading_depth-->>clone: p.parent.clone()
    heading_depth-->>and_then: doc.get(&parent).and_then(|p| p.parent.clone())
    end
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get

## Referenced by

- calls ← [`robby_render::markdown::node_section`](node-section.md)

