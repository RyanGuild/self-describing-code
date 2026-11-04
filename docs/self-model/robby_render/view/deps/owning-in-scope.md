# `robby_render::view::deps::owning_in_scope`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn owning_in_scope<'a>(     doc: &'a Document,     id: &NodeId,     scope: &BTreeSet<NodeId>, ) -> Option<&'a Node>` |
| location | [`crates/robby-render/src/view/deps.rs:678`](../../../../../crates/robby-render/src/view/deps.rs#L678) |
| provenance | extractor · confidence 1.00 |

```rust
fn owning_in_scope<'a>(
    doc: &'a Document,
    id: &NodeId,
    scope: &BTreeSet<NodeId>,
) -> Option<&'a Node> {
    let node = doc.get(id)?;
    if let Some(m) = best_scope_prefix(doc, node, scope) {
        return Some(m);
    }
    let mut current = node;
    loop {
        if scope.contains(&current.id) {
            return Some(current);
        }
        let parent = current.parent.as_ref().and_then(|p| doc.get(p))?;
        current = parent;
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant owning_in_scope as owning_in_scope()
    participant get as get
    participant best_scope_prefix as best_scope_prefix
    participant Some as Some
    participant contains as contains
    participant as_ref as as_ref
    participant and_then as and_then
    owning_in_scope-->>get: doc.get(id)
    owning_in_scope-->>owning_in_scope: ? (propagate)
    owning_in_scope-->>best_scope_prefix: best_scope_prefix(doc, node, scope)
    alt let Some(m) = best_scope_prefix(doc, node, scope)
    owning_in_scope-->>Some: Some(m)
    owning_in_scope-->>owning_in_scope: return
    end
    loop loop
    owning_in_scope-->>contains: scope.contains(&current.id)
    alt scope.contains(&current.id)
    owning_in_scope-->>Some: Some(current)
    owning_in_scope-->>owning_in_scope: return
    end
    owning_in_scope-->>as_ref: current.parent.as_ref()
    owning_in_scope-->>get: doc.get(p)
    owning_in_scope-->>and_then: current.parent.as_ref().and_then(|p| doc.get(p))
    owning_in_scope-->>owning_in_scope: ? (propagate)
    end
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)
- calls → [`robby_render::view::deps::best_scope_prefix`](best-scope-prefix.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)

## References

- calls → [`robby_ir::run`](../../../robby_ir/run/index.md) — get
- calls → [`robby_render::view::deps`](index.md) — best_scope_prefix
- calls → [`robby_render::view::scene`](../scene/index.md) — contains

## Referenced by

- calls ← [`robby_render::view::deps::impl CouplingGraph::from_root`](index.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](index.md)
- calls ← [`robby_render::view::deps::package_edges`](package-edges.md)

