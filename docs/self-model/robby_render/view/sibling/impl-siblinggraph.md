# `robby_render::view::sibling::impl SiblingGraph`

[Self-model](../../../README.md) · [view::sibling](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/view/sibling.rs:33`](../../../../../crates/robby-render/src/view/sibling.rs#L33) |
| provenance | extractor · confidence 1.00 |

```rust
impl SiblingGraph {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self {
                root_id: String::new(),
                root_label: String::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
            };
        };

        let visible: Vec<&Node> = root_node
            .children
            .iter()
            .filter_map(|child| doc.get(child))
            .filter(|node| is_structural(&node.kind))
            .collect();

        let nodes: Vec<SiblingNode> = visible
            .iter()
            .map(|node| {
                let sig = node.signature.as_deref().unwrap_or(&node.name);
                SiblingNode {
                    id: node.id.clone(),
                    stable_id: stable_id(node.id.as_str()),
                    label: text(&format!("{}  [{}]", sig, short_kind(&node.kind))),
                    name: node.name.clone(),
                    kind_tag: short_kind(&node.kind).to_owned(),
                }
            })
            .collect();

        let ids: Vec<&NodeId> = visible.iter().map(|n| &n.id).collect();
        let mut edges: Vec<(String, String, EdgeKind, bool)> = Vec::new();
        for node in &visible {
            for edge in &node.edges {
                if !ids.contains(&&edge.to) {
                    continue;
                }
                edges.push((
                    stable_id(node.id.as_str()),
                    stable_id(edge.to.as_str()),
                    edge.kind,
                    edge.resolved,
                ));
            }
        }
        edges.sort();
        edges.dedup();

        let edges = edges
            .into_iter()
            .map(|(from, to, kind, resolved)| SiblingEdge {
                from,
                to,
                label: edge_label(kind),
                resolved,
                kind,
            })
            .collect();

        let sig = root_node.signature.as_deref().unwrap_or(&root_node.name);
        Self {
            root_id: stable_id(root.as_str()),
            root_label: text(&format!("{}  [{}]", sig, short_kind(&root_node.kind))),
            nodes,
            edges,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
```

## References

- calls → [`robby_ir::run`](../../../robby_ir/run/index.md) — get
- calls → [`robby_render`](../../index.md) — stable_id
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::util`](../../util/index.md) — edge_label, is_structural, short_kind, text
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::scene`](../scene/index.md) — contains, new
- calls → [`robby_render::view::select`](../select/index.md) — collect

### `from_root`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_root(2)` |
| location | [`crates/robby-render/src/view/sibling.rs:34`](../../../../../crates/robby-render/src/view/sibling.rs#L34) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self {
                root_id: String::new(),
                root_label: String::new(),
                nodes: Vec::new(),
                edges: Vec::new(),
            };
        };

        let visible: Vec<&Node> = root_node
            .children
            .iter()
            .filter_map(|child| doc.get(child))
            .filter(|node| is_structural(&node.kind))
            .collect();

        let nodes: Vec<SiblingNode> = visible
            .iter()
            .map(|node| {
                let sig = node.signature.as_deref().unwrap_or(&node.name);
                SiblingNode {
                    id: node.id.clone(),
                    stable_id: stable_id(node.id.as_str()),
                    label: text(&format!("{}  [{}]", sig, short_kind(&node.kind))),
                    name: node.name.clone(),
                    kind_tag: short_kind(&node.kind).to_owned(),
                }
            })
            .collect();

        let ids: Vec<&NodeId> = visible.iter().map(|n| &n.id).collect();
        let mut edges: Vec<(String, String, EdgeKind, bool)> = Vec::new();
        for node in &visible {
            for edge in &node.edges {
                if !ids.contains(&&edge.to) {
                    continue;
                }
                edges.push((
                    stable_id(node.id.as_str()),
                    stable_id(edge.to.as_str()),
                    edge.kind,
                    edge.resolved,
                ));
            }
        }
        edges.sort();
        edges.dedup();

        let edges = edges
            .into_iter()
            .map(|(from, to, kind, resolved)| SiblingEdge {
                from,
                to,
                label: edge_label(kind),
                resolved,
                kind,
            })
            .collect();

        let sig = root_node.signature.as_deref().unwrap_or(&root_node.name);
        Self {
            root_id: stable_id(root.as_str()),
            root_label: text(&format!("{}  [{}]", sig, short_kind(&root_node.kind))),
            nodes,
            edges,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from_root as from_root()
    participant get as get
    participant iter as iter
    participant filter_map as filter_map
    participant is_structural as is_structural
    participant filter as filter
    participant collect as collect
    participant as_deref as as_deref
    participant unwrap_or as unwrap_or
    participant clone as clone
    participant as_str as as_str
    participant stable_id as stable_id
    participant text as text
    participant short_kind as short_kind
    participant to_owned as to_owned
    participant map as map
    participant Vec__new as Vec::new
    participant contains as contains
    participant push as push
    participant sort as sort
    participant dedup as dedup
    participant into_iter as into_iter
    participant edge_label as edge_label
    from_root-->>get: doc.get(root)
    from_root-->>iter: root_node .children .iter()
    from_root-->>get: doc.get(child)
    from_root-->>filter_map: root_node .children .iter() .filter_map(|child| doc.get(child))
    from_root-->>is_structural: is_structural(&node.kind)
    from_root-->>filter: root_node .children .iter() .filter_map(|child| doc.get(child)) .filt...
    from_root-->>collect: root_node .children .iter() .filter_map(|child| doc.get(child)) .filt...
    from_root-->>iter: visible .iter()
    from_root-->>as_deref: node.signature.as_deref()
    from_root-->>unwrap_or: node.signature.as_deref().unwrap_or(&node.name)
    from_root-->>clone: node.id.clone()
    from_root-->>as_str: node.id.as_str()
    from_root-->>stable_id: stable_id(node.id.as_str())
    from_root-->>from_root: format!('{} [{}]', sig, short_kind(&node.kind))
    from_root-->>text: text(&format!('{} [{}]', sig, short_kind(&node.kind)))
    from_root-->>clone: node.name.clone()
    from_root-->>short_kind: short_kind(&node.kind)
    from_root-->>to_owned: short_kind(&node.kind).to_owned()
    from_root-->>map: visible .iter() .map(|node| { let sig = node.signature.as_deref().unw...
    from_root-->>collect: visible .iter() .map(|node| { let sig = node.signature.as_deref().unw...
    from_root-->>iter: visible.iter()
    from_root-->>map: visible.iter().map(|n| &n.id)
    from_root-->>collect: visible.iter().map(|n| &n.id).collect()
    from_root-->>Vec__new: Vec::new()
    loop &visible
    loop &node.edges
    from_root-->>contains: ids.contains(&&edge.to)
    from_root-->>as_str: node.id.as_str()
    from_root-->>stable_id: stable_id(node.id.as_str())
    from_root-->>as_str: edge.to.as_str()
    from_root-->>stable_id: stable_id(edge.to.as_str())
    from_root-->>push: edges.push(( stable_id(node.id.as_str()), stable_id(edge.to.as_str())...
    end
    end
    from_root-->>sort: edges.sort()
    from_root-->>dedup: edges.dedup()
    from_root-->>into_iter: edges .into_iter()
    from_root-->>edge_label: edge_label(kind)
    from_root-->>map: edges .into_iter() .map(|(from, to, kind, resolved)| SiblingEdge { fr...
    from_root-->>collect: edges .into_iter() .map(|(from, to, kind, resolved)| SiblingEdge { fr...
    from_root-->>as_deref: root_node.signature.as_deref()
    from_root-->>unwrap_or: root_node.signature.as_deref().unwrap_or(&root_node.name)
    from_root-->>as_str: root.as_str()
    from_root-->>stable_id: stable_id(root.as_str())
    from_root-->>from_root: format!('{} [{}]', sig, short_kind(&root_node.kind))
    from_root-->>text: text(&format!('{} [{}]', sig, short_kind(&root_node.kind)))
```

42 steps · 0 observed · 42 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)
- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)
- calls → [`robby_render::util::is_structural`](../../util/is-structural.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::util::text`](../../util/text.md)
- calls → [`robby_render::util::short_kind`](../../util/short-kind.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../scene/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::report::impl Report::push`](../../report/index.md)
- calls → [`robby_render::util::edge_label`](../../util/edge-label.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../comments/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::util::text`](../../util/text.md)

### `is_empty`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn is_empty(1)` |
| location | [`crates/robby-render/src/view/sibling.rs:103`](../../../../../crates/robby-render/src/view/sibling.rs#L103) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant is_empty as is_empty()
    is_empty-->>is_empty: self.nodes.is_empty()
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::view::sibling::impl SiblingGraph::is_empty`](index.md)

