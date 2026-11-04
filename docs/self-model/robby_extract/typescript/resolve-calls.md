# `robby_extract::typescript::resolve_calls`

[Self-model](../../README.md) · [typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn resolve_calls(doc: &mut Document, pending: Vec<PendingCall>)` |
| location | [`crates/robby-extract/src/typescript.rs:1038`](../../../../crates/robby-extract/src/typescript.rs#L1038) |
| provenance | extractor · confidence 1.00 |

```rust
fn resolve_calls(doc: &mut Document, pending: Vec<PendingCall>) {
    let mut by_name: BTreeMap<String, Vec<NodeId>> = BTreeMap::new();
    for node in doc.nodes.values() {
        if matches!(
            node.kind.as_str(),
            kind::code::FUNCTION | kind::code::METHOD
        ) {
            by_name
                .entry(node.name.clone())
                .or_default()
                .push(node.id.clone());
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
    participant values as values
    participant clone as clone
    participant entry as entry
    participant or_default as or_default
    participant push as push
    participant get as get
    participant len as len
    participant get_mut as get_mut
    participant Edge__new as Edge::new
    participant Some as Some
    participant contains as contains
    resolve_calls-->>BTreeMap__new: BTreeMap::new()
    resolve_calls-->>values: doc.nodes.values()
    loop doc.nodes.values()
    resolve_calls-->>resolve_calls: matches!( node.kind.as_str(), kind::code::FUNCTION | kind::code::METH...
    alt matches!( node.kind.as_str(), kind::code::FUNCTION | kind::code::METH...
    resolve_calls-->>clone: node.name.clone()
    resolve_calls-->>entry: by_name .entry(node.name.clone())
    resolve_calls-->>or_default: by_name .entry(node.name.clone()) .or_default()
    resolve_calls-->>clone: node.id.clone()
    resolve_calls-->>push: by_name .entry(node.name.clone()) .or_default() .push(node.id.clone())
    end
    end
    loop pending
    resolve_calls-->>get: by_name.get(&call.name)
    resolve_calls-->>clone: targets[0].clone()
    resolve_calls-->>len: targets.len()
    resolve_calls-->>get_mut: doc.nodes.get_mut(&call.from)
    alt let Some(node) = doc.nodes.get_mut(&call.from)
    resolve_calls-->>Edge__new: Edge::new(target, EdgeKind::Calls)
    resolve_calls-->>Some: Some(call.span)
    resolve_calls-->>contains: node.edges.contains(&edge)
    alt !node.edges.contains(&edge)
    resolve_calls-->>push: node.edges.push(edge)
    end
    end
    end
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](../rust/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::scene`](../../robby_render/view/scene/index.md) — contains

## Referenced by

- calls ← [`robby_extract::typescript::extract`](extract.md)

