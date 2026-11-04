# `robby_render::tree::page_paths`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn page_paths(doc: &Document) -> Vec<PathBuf>` |
| location | [`crates/robby-render/src/tree.rs:854`](../../../../crates/robby-render/src/tree.rs#L854) |
| provenance | extractor · confidence 1.00 |

> Every page path a document will produce, for tests and stale-file cleanup.

```rust
/// Every page path a document will produce, for tests and stale-file cleanup.
pub fn page_paths(doc: &Document) -> Vec<PathBuf> {
    tree(doc, None, None, "docs/self-model")
        .into_iter()
        .map(|page| page.path)
        .collect()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant page_paths as page_paths()
    participant tree as tree
    participant into_iter as into_iter
    participant map as map
    participant collect as collect
    page_paths-->>tree: tree(doc, None, None, 'docs/self-model')
    page_paths-->>into_iter: tree(doc, None, None, 'docs/self-model') .into_iter()
    page_paths-->>map: tree(doc, None, None, 'docs/self-model') .into_iter() .map(|page| pag...
    page_paths-->>collect: tree(doc, None, None, 'docs/self-model') .into_iter() .map(|page| pag...
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::tree::tree`](tree.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)

## References

- calls → [`robby_render::tree`](index.md) — tree
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

