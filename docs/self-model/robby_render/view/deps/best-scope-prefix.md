# `robby_render::view::deps::best_scope_prefix`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn best_scope_prefix<'a>(     doc: &'a Document,     node: &Node,     scope: &BTreeSet<NodeId>, ) -> Option<&'a Node>` |
| location | [`crates/robby-render/src/view/deps.rs:741`](../../../../../crates/robby-render/src/view/deps.rs#L741) |
| provenance | extractor · confidence 1.00 |

```rust
fn best_scope_prefix<'a>(
    doc: &'a Document,
    node: &Node,
    scope: &BTreeSet<NodeId>,
) -> Option<&'a Node> {
    let mut best: Option<&Node> = None;
    let mut best_len = 0usize;
    for id in scope {
        let Some(candidate) = doc.get(id) else {
            continue;
        };
        if node.id == candidate.id || node.fqn == candidate.fqn {
            return Some(candidate);
        }
        let prefix = format!("{}::", candidate.fqn);
        if node.fqn.starts_with(&prefix) && candidate.fqn.len() > best_len {
            best_len = candidate.fqn.len();
            best = Some(candidate);
        }
    }
    best
}
```

```mermaid
sequenceDiagram
    autonumber
    participant best_scope_prefix as best_scope_prefix()
    participant get as get
    participant Some as Some
    participant starts_with as starts_with
    participant len as len
    loop scope
    best_scope_prefix-->>get: doc.get(id)
    alt node.id == candidate.id || node.fqn == candidate.fqn
    best_scope_prefix-->>Some: Some(candidate)
    best_scope_prefix-->>best_scope_prefix: return
    end
    best_scope_prefix-->>best_scope_prefix: format!('{}::', candidate.fqn)
    best_scope_prefix-->>starts_with: node.fqn.starts_with(&prefix)
    best_scope_prefix-->>len: candidate.fqn.len()
    alt node.fqn.starts_with(&prefix) && candidate.fqn.len() > best_len
    best_scope_prefix-->>len: candidate.fqn.len()
    best_scope_prefix-->>Some: Some(candidate)
    end
    end
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get

## Referenced by

- calls ← [`robby_render::view::deps::lift_to_scope`](lift-to-scope.md)
- calls ← [`robby_render::view::deps::owning_in_scope`](owning-in-scope.md)

