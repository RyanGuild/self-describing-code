# `robby_render::view::deps::topological_layers`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn topological_layers(     adj: &BTreeMap<String, BTreeSet<String>>,     cyclic: &BTreeSet<String>, ) -> Vec<Vec<String>>` |
| location | [`crates/robby-render/src/view/deps.rs:843`](../../../../../crates/robby-render/src/view/deps.rs#L843) |
| provenance | extractor · confidence 1.00 |

```rust
fn topological_layers(
    adj: &BTreeMap<String, BTreeSet<String>>,
    cyclic: &BTreeSet<String>,
) -> Vec<Vec<String>> {
    let mut indegree: BTreeMap<String, usize> = adj.keys().map(|k| (k.clone(), 0)).collect();
    let mut forward: BTreeMap<String, BTreeSet<String>> =
        adj.keys().map(|k| (k.clone(), BTreeSet::new())).collect();

    for (from, tos) in adj {
        for to in tos {
            if cyclic.contains(from) && cyclic.contains(to) {
                continue;
            }
            forward.get_mut(from).unwrap().insert(to.clone());
            *indegree.get_mut(to).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<String> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut layers = Vec::new();

    while !queue.is_empty() {
        let mut layer: Vec<String> = queue.drain(..).collect();
        layer.sort();
        let mut next = VecDeque::new();
        for v in &layer {
            for w in &forward[v] {
                let d = indegree.get_mut(w).unwrap();
                *d -= 1;
                if *d == 0 {
                    next.push_back(w.clone());
                }
            }
        }
        layers.push(layer);
        queue = next;
    }

    let placed: BTreeSet<_> = layers.iter().flatten().cloned().collect();
    let mut rest: Vec<String> = adj
        .keys()
        .filter(|k| !placed.contains(*k))
        .cloned()
        .collect();
    if !rest.is_empty() {
        rest.sort();
        layers.push(rest);
    }

    layers
}
```

```mermaid
sequenceDiagram
    autonumber
    participant topological_layers as topological_layers()
    participant keys as keys
    participant clone as clone
    participant map as map
    participant collect as collect
    participant BTreeSet__new as BTreeSet::new
    participant contains as contains
    participant get_mut as get_mut
    participant unwrap as unwrap
    participant insert as insert
    participant iter as iter
    participant filter as filter
    participant Vec__new as Vec::new
    participant is_empty as is_empty
    participant drain as drain
    participant sort as sort
    participant VecDeque__new as VecDeque::new
    participant push_back as push_back
    participant push as push
    participant flatten as flatten
    participant cloned as cloned
    topological_layers-->>keys: adj.keys()
    topological_layers-->>clone: k.clone()
    topological_layers-->>map: adj.keys().map(|k| (k.clone(), 0))
    topological_layers-->>collect: adj.keys().map(|k| (k.clone(), 0)).collect()
    topological_layers-->>keys: adj.keys()
    topological_layers-->>clone: k.clone()
    topological_layers-->>BTreeSet__new: BTreeSet::new()
    topological_layers-->>map: adj.keys().map(|k| (k.clone(), BTreeSet::new()))
    topological_layers-->>collect: adj.keys().map(|k| (k.clone(), BTreeSet::new())).collect()
    loop adj
    loop tos
    topological_layers-->>contains: cyclic.contains(from)
    topological_layers-->>contains: cyclic.contains(to)
    topological_layers-->>get_mut: forward.get_mut(from)
    topological_layers-->>unwrap: forward.get_mut(from).unwrap()
    topological_layers-->>clone: to.clone()
    topological_layers-->>insert: forward.get_mut(from).unwrap().insert(to.clone())
    topological_layers-->>get_mut: indegree.get_mut(to)
    topological_layers-->>unwrap: indegree.get_mut(to).unwrap()
    end
    end
    topological_layers-->>iter: indegree .iter()
    topological_layers-->>filter: indegree .iter() .filter(|(_, d)| **d == 0)
    topological_layers-->>clone: k.clone()
    topological_layers-->>map: indegree .iter() .filter(|(_, d)| **d == 0) .map(|(k, _)| k.clone())
    topological_layers-->>collect: indegree .iter() .filter(|(_, d)| **d == 0) .map(|(k, _)| k.clone()) ...
    topological_layers-->>Vec__new: Vec::new()
    topological_layers-->>is_empty: queue.is_empty()
    loop !queue.is_empty()
    topological_layers-->>drain: queue.drain(..)
    topological_layers-->>collect: queue.drain(..).collect()
    topological_layers-->>sort: layer.sort()
    topological_layers-->>VecDeque__new: VecDeque::new()
    loop &layer
    loop &forward[v]
    topological_layers-->>get_mut: indegree.get_mut(w)
    topological_layers-->>unwrap: indegree.get_mut(w).unwrap()
    alt *d == 0
    topological_layers-->>clone: w.clone()
    topological_layers-->>push_back: next.push_back(w.clone())
    end
    end
    end
    topological_layers-->>push: layers.push(layer)
    end
    topological_layers-->>iter: layers.iter()
    topological_layers-->>flatten: layers.iter().flatten()
    topological_layers-->>cloned: layers.iter().flatten().cloned()
    topological_layers-->>collect: layers.iter().flatten().cloned().collect()
    topological_layers-->>keys: adj .keys()
    topological_layers-->>contains: placed.contains(*k)
    topological_layers-->>filter: adj .keys() .filter(|k| !placed.contains(*k))
    topological_layers-->>cloned: adj .keys() .filter(|k| !placed.contains(*k)) .cloned()
    topological_layers-->>collect: adj .keys() .filter(|k| !placed.contains(*k)) .cloned() .collect()
    topological_layers-->>is_empty: rest.is_empty()
    alt !rest.is_empty()
    topological_layers-->>sort: rest.sort()
    topological_layers-->>push: layers.push(rest)
    end
```

45 steps · 0 observed · 45 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::deps::impl CouplingGraph::is_empty`](index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::deps::impl CouplingGraph::is_empty`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — insert
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::view::deps`](index.md) — is_empty
- calls → [`robby_render::view::scene`](../scene/index.md) — contains, new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::view::deps::package_edges`](package-edges.md)

