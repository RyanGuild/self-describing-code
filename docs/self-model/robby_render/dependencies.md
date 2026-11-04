# `robby_render::dependencies`

[Self-model](../README.md) · [robby_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn dependencies(doc: &Document, root: &NodeId) -> String` |
| location | [`crates/robby-render/src/lib.rs:51`](../../../crates/robby-render/src/lib.rs#L51) |
| provenance | extractor · confidence 1.00 |

> Nested package-structure digraph (Mermaid) for a module/crate root.

```rust
/// Nested package-structure digraph (Mermaid) for a module/crate root.
pub fn dependencies(doc: &Document, root: &NodeId) -> String {
    let graph = PackageStructure::from_root(doc, root);
    if graph.is_empty() {
        return String::from("flowchart TB\n");
    }
    MermaidRenderer.package_deps(&graph)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant dependencies as dependencies()
    participant PackageStructure__from_root as PackageStructure::from_root
    participant is_empty as is_empty
    participant String__from as String::from
    participant package_deps as package_deps
    dependencies-->>PackageStructure__from_root: PackageStructure::from_root(doc, root)
    dependencies-->>is_empty: graph.is_empty()
    alt graph.is_empty()
    dependencies-->>String__from: String::from('flowchart TB\n')
    dependencies-->>dependencies: return
    end
    dependencies-->>package_deps: MermaidRenderer.package_deps(&graph)
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby_render::view::deps::impl CouplingGraph::from_root`](view/deps/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](view/links/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../robby_ir/kind/index.md)
- calls → [`robby_render::render::Renderer::package_deps`](render/index.md)

## References

- calls → [`robby_ir::kind`](../robby_ir/kind/index.md) — from
- calls → [`robby_render::render`](render/index.md) — package_deps
- calls → [`robby_render::view::deps`](view/deps/index.md) — from_root
- calls → [`robby_render::view::links`](view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_render::test_render::dependencies_emits_package_weighted_and_dsm`](test_render/dependencies-emits-package-weighted-and-dsm.md)

