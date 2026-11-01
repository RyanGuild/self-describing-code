# `robby_extract::resolve_calls`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn resolve_calls(doc: &mut Document, pending: Vec<PendingCall>)` |
| location | [`crates/robby-extract/src/lib.rs:806`](../../../crates/robby-extract/src/lib.rs#L806) |
| provenance | extractor · confidence 1.00 |

```rust
fn resolve_calls(doc: &mut Document, pending: Vec<PendingCall>) {
    let mut by_name: BTreeMap<String, Vec<NodeId>> = BTreeMap::new();
    for node in doc.nodes.values() {
        if matches!(
            node.kind.as_str(),
            kind::code::FUNCTION | kind::code::METHOD
        ) {
            by_name.entry(node.name.clone()).or_default().push(node.id.clone());
        }
    }

    for call in pending {
        let Some(targets) = by_name.get(&call.name) else {
            continue;
        };
        let target = targets[0].clone();
        let confidence = if targets.len() == 1 { 1.0 } else { 0.4 };
        if let Some(node) = doc.nodes.get_mut(&call.from) {
            let mut edge = Edge::new(target, EdgeKind::Calls);
            edge.span = Some(call.span);
            edge.confidence = confidence;
            if !node.edges.contains(&edge) {
                node.edges.push(edge);
            }
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant resolve_calls as resolve_calls()
    participant BTreeMap__new as BTreeMap::new
    participant push as push
    participant get as get
    participant clone as clone
    participant Edge__new as Edge::new
    resolve_calls-->>BTreeMap__new: BTreeMap::new()
    loop doc.nodes.values()
    alt matches!( node.kind.as_str(), kind::code::FUNCTION | kind::code::METH...
    resolve_calls-->>push: by_name.entry(node.name.clone()).or_default().push(node.id.clone())
    end
    end
    loop pending
    resolve_calls-->>get: by_name.get(&call.name)
    resolve_calls-->>clone: targets[0].clone()
    alt targets.len() == 1
    resolve_calls-->>resolve_calls: 1.0
    else else
    resolve_calls-->>resolve_calls: 0.4
    end
    alt let Some(node) = doc.nodes.get_mut(&call.from)
    resolve_calls-->>Edge__new: Edge::new(target, EdgeKind::Calls)
    resolve_calls-->>resolve_calls: edge.span = Some(call.span)
    resolve_calls-->>resolve_calls: edge.confidence = confidence
    alt !node.edges.contains(&edge)
    resolve_calls-->>push: node.edges.push(edge)
    end
    end
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_ir::model::impl Document::get`](../robby_ir/model/index.md)
- calls → [`robby_extract::impl LineIndex::new`](index.md)

