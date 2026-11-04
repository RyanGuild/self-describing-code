# `robby_render::view::deps::impl PackageStructure`

[Self-model](../../../README.md) · [view::deps](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/view/deps.rs:103`](../../../../../crates/robby-render/src/view/deps.rs#L103) |
| provenance | extractor · confidence 1.00 |

```rust
impl PackageStructure {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self::empty();
        };

        let packages: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| {
                is_container(node) && (node.id == root_node.id || is_under_fqn(root_node, node))
            })
            .collect();

        if packages.is_empty() {
            return Self::empty();
        }

        let tree = build_package_tree(root_node, &packages);
        let scope: BTreeSet<NodeId> = packages.iter().map(|p| p.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.subtree(root).into_iter(), false, root_node);

        Self {
            root: tree,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }

    pub fn workspace(doc: &Document) -> Self {
        let crates: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE))
            .collect();
        if crates.is_empty() {
            return Self::empty();
        }

        let mut children: Vec<PackageNode> = crates.iter().map(|c| package_leaf(c)).collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));

        let root = PackageNode {
            id: NodeId::from("workspace://root"),
            stable_id: stable_id("workspace://root"),
            name: "workspace".into(),
            fqn: "workspace".into(),
            children,
        };

        let scope: BTreeSet<NodeId> = crates.iter().map(|c| c.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.nodes.values(), true, crates[0]);

        Self {
            root,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }

    pub fn empty() -> Self {
        Self {
            root: PackageNode {
                id: NodeId::from(""),
                stable_id: String::new(),
                name: String::new(),
                fqn: String::new(),
                children: Vec::new(),
            },
            externals: Vec::new(),
            edges: Vec::new(),
            layers: Vec::new(),
            cycles: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty() && self.modules.is_empty() && self.edges.is_empty()
    }

    pub fn module_by_stable<'a>(&'a self, id: &str) -> Option<&'a DepModule> {
        self.modules.iter().find(|m| m.stable_id == id)
    }
}
```

## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — subtree
- calls → [`robby_ir::run`](../../../robby_ir/run/index.md) — get
- calls → [`robby_render`](../../index.md) — stable_id
- calls → [`robby_render::util`](../../util/index.md) — is_container
- calls → [`robby_render::view::deps`](index.md) — build_package_tree, is_under_fqn, package_edges, package_leaf
- calls → [`robby_render::view::scene`](../scene/index.md) — new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::render::mermaid::dsm_markdown_table`](../../render/mermaid/dsm-markdown-table.md)
- calls ← [`robby_render::render::tui::ascii_package_structure`](../../render/tui/ascii-package-structure.md)

### `from_root`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_root(2)` |
| location | [`crates/robby-render/src/view/deps.rs:104`](../../../../../crates/robby-render/src/view/deps.rs#L104) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self::empty();
        };

        let packages: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| {
                is_container(node) && (node.id == root_node.id || is_under_fqn(root_node, node))
            })
            .collect();

        if packages.is_empty() {
            return Self::empty();
        }

        let tree = build_package_tree(root_node, &packages);
        let scope: BTreeSet<NodeId> = packages.iter().map(|p| p.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.subtree(root).into_iter(), false, root_node);

        Self {
            root: tree,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from_root as from_root()
    participant get as get
    participant values as values
    participant is_container as is_container
    participant is_under_fqn as is_under_fqn
    participant filter as filter
    participant collect as collect
    participant is_empty as is_empty
    participant Self__empty as Self::empty
    participant build_package_tree as build_package_tree
    participant iter as iter
    participant clone as clone
    participant map as map
    participant subtree as subtree
    participant into_iter as into_iter
    participant package_edges as package_edges
    from_root-->>get: doc.get(root)
    from_root-->>values: doc .nodes .values()
    from_root-->>is_container: is_container(node)
    from_root-->>is_under_fqn: is_under_fqn(root_node, node)
    from_root-->>filter: doc .nodes .values() .filter(|node| { is_container(node) && (node.id ...
    from_root-->>collect: doc .nodes .values() .filter(|node| { is_container(node) && (node.id ...
    from_root-->>is_empty: packages.is_empty()
    alt packages.is_empty()
    from_root-->>Self__empty: Self::empty()
    from_root-->>from_root: return
    end
    from_root-->>build_package_tree: build_package_tree(root_node, &packages)
    from_root-->>iter: packages.iter()
    from_root-->>clone: p.id.clone()
    from_root-->>map: packages.iter().map(|p| p.id.clone())
    from_root-->>collect: packages.iter().map(|p| p.id.clone()).collect()
    from_root-->>subtree: doc.subtree(root)
    from_root-->>into_iter: doc.subtree(root).into_iter()
    from_root-->>package_edges: package_edges(doc, &scope, doc.subtree(root).into_iter(), false, root...
```

17 steps · 0 observed · 17 possible only

**Edges**

- calls → [`robby_ir::run::RecipeCatalog::get`](../../../robby_ir/run/index.md)
- calls → [`robby_render::util::is_container`](../../util/is-container.md)
- calls → [`robby_render::view::deps::is_under_fqn`](is-under-fqn.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::deps::impl PackageStructure::is_empty`](index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::empty`](index.md)
- calls → [`robby_render::view::deps::build_package_tree`](build-package-tree.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_ir::model::impl Document::subtree`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::deps::package_edges`](package-edges.md)

### `workspace`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn workspace(1)` |
| location | [`crates/robby-render/src/view/deps.rs:136`](../../../../../crates/robby-render/src/view/deps.rs#L136) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn workspace(doc: &Document) -> Self {
        let crates: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE))
            .collect();
        if crates.is_empty() {
            return Self::empty();
        }

        let mut children: Vec<PackageNode> = crates.iter().map(|c| package_leaf(c)).collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));

        let root = PackageNode {
            id: NodeId::from("workspace://root"),
            stable_id: stable_id("workspace://root"),
            name: "workspace".into(),
            fqn: "workspace".into(),
            children,
        };

        let scope: BTreeSet<NodeId> = crates.iter().map(|c| c.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.nodes.values(), true, crates[0]);

        Self {
            root,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant workspace as workspace()
    participant values as values
    participant filter as filter
    participant collect as collect
    participant is_empty as is_empty
    participant Self__empty as Self::empty
    participant iter as iter
    participant package_leaf as package_leaf
    participant map as map
    participant cmp as cmp
    participant sort_by as sort_by
    participant NodeId__from as NodeId::from
    participant stable_id as stable_id
    participant into as into
    participant clone as clone
    participant package_edges as package_edges
    workspace-->>values: doc .nodes .values()
    workspace-->>workspace: matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE)
    workspace-->>filter: doc .nodes .values() .filter(|node| matches!(node.kind.as_str(), kind...
    workspace-->>collect: doc .nodes .values() .filter(|node| matches!(node.kind.as_str(), kind...
    workspace-->>is_empty: crates.is_empty()
    alt crates.is_empty()
    workspace-->>Self__empty: Self::empty()
    workspace-->>workspace: return
    end
    workspace-->>iter: crates.iter()
    workspace-->>package_leaf: package_leaf(c)
    workspace-->>map: crates.iter().map(|c| package_leaf(c))
    workspace-->>collect: crates.iter().map(|c| package_leaf(c)).collect()
    workspace-->>cmp: a.name.cmp(&b.name)
    workspace-->>sort_by: children.sort_by(|a, b| a.name.cmp(&b.name))
    workspace-->>NodeId__from: NodeId::from('workspace://root')
    workspace-->>stable_id: stable_id('workspace://root')
    workspace-->>into: 'workspace'.into()
    workspace-->>into: 'workspace'.into()
    workspace-->>iter: crates.iter()
    workspace-->>clone: c.id.clone()
    workspace-->>map: crates.iter().map(|c| c.id.clone())
    workspace-->>collect: crates.iter().map(|c| c.id.clone()).collect()
    workspace-->>values: doc.nodes.values()
    workspace-->>package_edges: package_edges(doc, &scope, doc.nodes.values(), true, crates[0])
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::deps::impl PackageStructure::is_empty`](index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::empty`](index.md)
- calls → [`robby_render::view::deps::package_leaf`](package-leaf.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../../robby_ir/kind/index.md)
- calls → [`robby_render::stable_id`](../../stable-id.md)
- calls → [`robby_render::view::select::collect`](../select/collect.md)
- calls → [`robby_render::view::deps::package_edges`](package-edges.md)

### `empty`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn empty(0)` |
| location | [`crates/robby-render/src/view/deps.rs:171`](../../../../../crates/robby-render/src/view/deps.rs#L171) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn empty() -> Self {
        Self {
            root: PackageNode {
                id: NodeId::from(""),
                stable_id: String::new(),
                name: String::new(),
                fqn: String::new(),
                children: Vec::new(),
            },
            externals: Vec::new(),
            edges: Vec::new(),
            layers: Vec::new(),
            cycles: Vec::new(),
            modules: Vec::new(),
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant empty as empty()
    participant NodeId__from as NodeId::from
    participant String__new as String::new
    participant Vec__new as Vec::new
    empty-->>NodeId__from: NodeId::from('')
    empty-->>String__new: String::new()
    empty-->>String__new: String::new()
    empty-->>String__new: String::new()
    empty-->>Vec__new: Vec::new()
    empty-->>Vec__new: Vec::new()
    empty-->>Vec__new: Vec::new()
    empty-->>Vec__new: Vec::new()
    empty-->>Vec__new: Vec::new()
    empty-->>Vec__new: Vec::new()
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../../robby_ir/kind/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../scene/index.md)

### `is_empty`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn is_empty(1)` |
| location | [`crates/robby-render/src/view/deps.rs:188`](../../../../../crates/robby-render/src/view/deps.rs#L188) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn is_empty(&self) -> bool {
        self.root.children.is_empty() && self.modules.is_empty() && self.edges.is_empty()
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant is_empty as is_empty()
    is_empty-->>is_empty: self.root.children.is_empty()
    is_empty-->>is_empty: self.modules.is_empty()
    is_empty-->>is_empty: self.edges.is_empty()
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_render::view::deps::impl PackageStructure::is_empty`](index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::is_empty`](index.md)
- calls → [`robby_render::view::deps::impl PackageStructure::is_empty`](index.md)

### `module_by_stable`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn module_by_stable(2)` |
| location | [`crates/robby-render/src/view/deps.rs:192`](../../../../../crates/robby-render/src/view/deps.rs#L192) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn module_by_stable<'a>(&'a self, id: &str) -> Option<&'a DepModule> {
        self.modules.iter().find(|m| m.stable_id == id)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant module_by_stable as module_by_stable()
    participant iter as iter
    participant find as find
    module_by_stable-->>iter: self.modules.iter()
    module_by_stable-->>find: self.modules.iter().find(|m| m.stable_id == id)
```

2 steps · 0 observed · 2 possible only

