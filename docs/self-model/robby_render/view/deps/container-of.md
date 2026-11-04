# `robby_render::view::deps::container_of`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn container_of<'a>(doc: &'a Document, id: &NodeId) -> Option<&'a Node>` |
| location | [`crates/robby-render/src/view/deps.rs:714`](../../../../../crates/robby-render/src/view/deps.rs#L714) |
| provenance | extractor · confidence 1.00 |

```rust
fn container_of<'a>(doc: &'a Document, id: &NodeId) -> Option<&'a Node> {
    let mut current = doc.get(id)?;
    if is_container(current) {
        return Some(current);
    }
    while let Some(parent_id) = &current.parent {
        let parent = doc.get(parent_id)?;
        if is_container(parent) {
            return Some(parent);
        }
        current = parent;
    }
    None
}
```

```mermaid
sequenceDiagram
    autonumber
    participant container_of as container_of()
    participant get as get
    participant is_container as is_container
    participant Some as Some
    container_of-->>get: doc.get(id)
    container_of-->>container_of: ? (propagate)
    container_of-->>is_container: is_container(current)
    alt is_container(current)
    container_of-->>Some: Some(current)
    container_of-->>container_of: return
    end
    loop let Some(parent_id) = &current.parent
    container_of-->>get: doc.get(parent_id)
    container_of-->>container_of: ? (propagate)
    container_of-->>is_container: is_container(parent)
    alt is_container(parent)
    container_of-->>Some: Some(parent)
    container_of-->>container_of: return
    end
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::util::is_container`](../../util/is-container.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::util::is_container`](../../util/is-container.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get
- calls → [`robby_render::util`](../../util/index.md) — is_container

## Referenced by

- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](index.md)
- calls ← [`robby_render::view::deps::package_edges`](package-edges.md)

