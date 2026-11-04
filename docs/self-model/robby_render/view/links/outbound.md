# `robby_render::view::links::outbound`

[Self-model](../../../README.md) · [view::links](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn outbound(doc: &Document, root: &Node) -> Vec<OutboundRef>` |
| location | [`crates/robby-render/src/view/links.rs:44`](../../../../../crates/robby-render/src/view/links.rs#L44) |
| provenance | extractor · confidence 1.00 |

```rust
fn outbound(doc: &Document, root: &Node) -> Vec<OutboundRef> {
    let scope = subtree_ids(doc, root);
    let mut by_module: BTreeMap<NodeId, (EdgeKind, BTreeSet<String>)> = BTreeMap::new();

    for node in doc.subtree(&root.id) {
        for edge in &node.edges {
            let Some(target) = doc.get(&edge.to) else {
                continue;
            };
            if scope.contains(&target.id) {
                continue;
            }
            let Some(target_mod) = owning_module(doc, target) else {
                continue;
            };
            if scope.contains(&target_mod.id) {
                continue;
            }
            let entry = by_module
                .entry(target_mod.id.clone())
                .or_insert((edge.kind, BTreeSet::new()));
            entry.1.insert(target.name.clone());
        }
    }

    by_module
        .into_iter()
        .filter_map(|(module_id, (kind, items))| {
            let module = doc.get(&module_id)?;
            Some(OutboundRef {
                kind,
                kind_label: edge_label(kind),
                module_id,
                module_fqn: module.fqn.clone(),
                items: items.into_iter().collect(),
            })
        })
        .collect()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant outbound as outbound()
    participant subtree_ids as subtree_ids
    participant BTreeMap__new as BTreeMap::new
    participant subtree as subtree
    participant get as get
    participant contains as contains
    participant owning_module as owning_module
    participant clone as clone
    participant entry as entry
    participant BTreeSet__new as BTreeSet::new
    participant or_insert as or_insert
    participant insert as insert
    participant into_iter as into_iter
    participant edge_label as edge_label
    participant collect as collect
    participant Some as Some
    participant filter_map as filter_map
    outbound-->>subtree_ids: subtree_ids(doc, root)
    outbound-->>BTreeMap__new: BTreeMap::new()
    outbound-->>subtree: doc.subtree(&root.id)
    loop doc.subtree(&root.id)
    loop &node.edges
    outbound-->>get: doc.get(&edge.to)
    outbound-->>contains: scope.contains(&target.id)
    outbound-->>owning_module: owning_module(doc, target)
    outbound-->>contains: scope.contains(&target_mod.id)
    outbound-->>clone: target_mod.id.clone()
    outbound-->>entry: by_module .entry(target_mod.id.clone())
    outbound-->>BTreeSet__new: BTreeSet::new()
    outbound-->>or_insert: by_module .entry(target_mod.id.clone()) .or_insert((edge.kind, BTreeS...
    outbound-->>clone: target.name.clone()
    outbound-->>insert: entry.1.insert(target.name.clone())
    end
    end
    outbound-->>into_iter: by_module .into_iter()
    outbound-->>get: doc.get(&module_id)
    outbound-->>outbound: ? (propagate)
    outbound-->>edge_label: edge_label(kind)
    outbound-->>clone: module.fqn.clone()
    outbound-->>into_iter: items.into_iter()
    outbound-->>collect: items.into_iter().collect()
    outbound-->>Some: Some(OutboundRef { kind, kind_label: edge_label(kind), module_id, mod...
    outbound-->>filter_map: by_module .into_iter() .filter_map(|(module_id, (kind, items))| { let...
    outbound-->>collect: by_module .into_iter() .filter_map(|(module_id, (kind, items))| { let...
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_render::view::links::subtree_ids`](subtree-ids.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_ir::model::impl Document::subtree`](../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::links::owning_module`](owning-module.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::util::edge_label`](../../util/edge-label.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get, insert, subtree
- calls → [`robby_render::util`](../../util/index.md) — edge_label
- calls → [`robby_render::view::links`](index.md) — owning_module, subtree_ids
- calls → [`robby_render::view::scene`](../scene/index.md) — contains, new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::links::impl LinkGraph::for_node`](index.md)

