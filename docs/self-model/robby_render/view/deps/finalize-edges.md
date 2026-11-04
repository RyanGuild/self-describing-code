# `robby_render::view::deps::finalize_edges`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn finalize_edges(weights: BTreeMap<(String, String), usize>) -> Vec<WeightedEdge>` |
| location | [`crates/robby-render/src/view/deps.rs:580`](../../../../../crates/robby-render/src/view/deps.rs#L580) |
| provenance | extractor · confidence 1.00 |

```rust
fn finalize_edges(weights: BTreeMap<(String, String), usize>) -> Vec<WeightedEdge> {
    let adjacency: BTreeMap<String, BTreeSet<String>> = {
        let mut adj: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for ((from, to), _) in &weights {
            adj.entry(from.clone()).or_default().insert(to.clone());
            adj.entry(to.clone()).or_default();
        }
        adj
    };
    let sccs = tarjan(&adjacency);
    let mut edge_in_cycle: BTreeSet<(String, String)> = BTreeSet::new();
    for scc in &sccs {
        if scc.len() < 2 {
            continue;
        }
        let set: BTreeSet<_> = scc.iter().cloned().collect();
        for ((from, to), _) in &weights {
            if set.contains(from) && set.contains(to) {
                edge_in_cycle.insert((from.clone(), to.clone()));
            }
        }
    }
    let mut edges: Vec<WeightedEdge> = weights
        .into_iter()
        .map(|((from, to), weight)| WeightedEdge {
            in_cycle: edge_in_cycle.contains(&(from.clone(), to.clone())),
            from,
            to,
            weight,
        })
        .collect();
    edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));
    edges
}
```

```mermaid
sequenceDiagram
    autonumber
    participant finalize_edges as finalize_edges()
    participant BTreeMap__new as BTreeMap::new
    participant clone as clone
    participant entry as entry
    participant or_default as or_default
    participant insert as insert
    participant tarjan as tarjan
    participant BTreeSet__new as BTreeSet::new
    participant len as len
    participant iter as iter
    participant cloned as cloned
    participant collect as collect
    participant contains as contains
    participant into_iter as into_iter
    participant map as map
    participant cmp as cmp
    participant sort_by as sort_by
    finalize_edges-->>BTreeMap__new: BTreeMap::new()
    loop &weights
    finalize_edges-->>clone: from.clone()
    finalize_edges-->>entry: adj.entry(from.clone())
    finalize_edges-->>or_default: adj.entry(from.clone()).or_default()
    finalize_edges-->>clone: to.clone()
    finalize_edges-->>insert: adj.entry(from.clone()).or_default().insert(to.clone())
    finalize_edges-->>clone: to.clone()
    finalize_edges-->>entry: adj.entry(to.clone())
    finalize_edges-->>or_default: adj.entry(to.clone()).or_default()
    end
    finalize_edges-->>tarjan: tarjan(&adjacency)
    finalize_edges-->>BTreeSet__new: BTreeSet::new()
    loop &sccs
    finalize_edges-->>len: scc.len()
    finalize_edges-->>iter: scc.iter()
    finalize_edges-->>cloned: scc.iter().cloned()
    finalize_edges-->>collect: scc.iter().cloned().collect()
    loop &weights
    finalize_edges-->>contains: set.contains(from)
    finalize_edges-->>contains: set.contains(to)
    alt set.contains(from) && set.contains(to)
    finalize_edges-->>clone: from.clone()
    finalize_edges-->>clone: to.clone()
    finalize_edges-->>insert: edge_in_cycle.insert((from.clone(), to.clone()))
    end
    end
    end
    finalize_edges-->>into_iter: weights .into_iter()
    finalize_edges-->>clone: from.clone()
    finalize_edges-->>clone: to.clone()
    finalize_edges-->>contains: edge_in_cycle.contains(&(from.clone(), to.clone()))
    finalize_edges-->>map: weights .into_iter() .map(|((from, to), weight)| WeightedEdge { in_cy...
    finalize_edges-->>collect: weights .into_iter() .map(|((from, to), weight)| WeightedEdge { in_cy...
    finalize_edges-->>cmp: (&a.from, &a.to).cmp(&(&b.from, &b.to))
    finalize_edges-->>sort_by: edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)))
```

28 steps · 0 observed · 28 possible only

**Edges**

- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::deps::tarjan`](tarjan.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — insert
- calls → [`robby_render::view::deps`](index.md) — tarjan
- calls → [`robby_render::view::scene`](../scene/index.md) — contains, new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::deps::impl CouplingGraph::from_root`](index.md)
- calls ← [`robby_render::view::deps::impl CouplingGraph::workspace`](index.md)

