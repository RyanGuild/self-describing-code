# `robby_render::tree::tree`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn tree(     doc: &Document,     coverage: Option<&CoverageReport>,     sources: Option<&BTreeMap<String, String>>,     out_rel: &str, ) -> Vec<Page>` |
| location | [`crates/robby-render/src/tree.rs:32`](../../../../crates/robby-render/src/tree.rs#L32) |
| provenance | extractor · confidence 1.00 |

> Build every page of the self-model.
> 
> `out_rel` is the output directory relative to the workspace root, e.g.
> `docs/self-model`. It is only used to compute links back to source files.

```rust
/// Build every page of the self-model.
///
/// `out_rel` is the output directory relative to the workspace root, e.g.
/// `docs/self-model`. It is only used to compute links back to source files.
pub fn tree(
    doc: &Document,
    coverage: Option<&CoverageReport>,
    sources: Option<&BTreeMap<String, String>>,
    out_rel: &str,
) -> Vec<Page> {
    let mut pages = Vec::new();

    let crates: Vec<&Node> = doc
        .nodes
        .values()
        .filter(|node| is_crate(node))
        .collect();

    pages.push(Page {
        path: PathBuf::from("README.md"),
        body: global_index(doc, coverage, &crates),
    });

    for krate in crates {
        let crate_dir = krate.name.clone();
        module_pages(doc, krate, &crate_dir, &mut pages, sources, out_rel);
    }

    pages
}
```

```mermaid
sequenceDiagram
    autonumber
    participant tree as tree()
    participant Vec__new as Vec::new
    participant collect as collect
    participant push as push
    participant clone as clone
    participant module_pages as module_pages
    tree-->>Vec__new: Vec::new()
    tree-->>collect: doc .nodes .values() .filter(|node| is_crate(node)) .collect()
    tree-->>push: pages.push(Page { path: PathBuf::from('README.md'), body: global_inde...
    loop crates
    tree-->>clone: krate.name.clone()
    tree-->>module_pages: module_pages(doc, krate, &crate_dir, &mut pages, sources, out_rel)
    end
    tree-->>tree: pages
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_render::tree::module_pages`](module-pages.md)

