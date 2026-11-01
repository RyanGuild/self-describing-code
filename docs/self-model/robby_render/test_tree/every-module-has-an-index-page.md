# `robby_render::test_tree::every_module_has_an_index_page`

[Self-model](../../README.md) · [test_tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn every_module_has_an_index_page()` |
| location | [`crates/robby-render/tests/tree.rs:63`](../../../../crates/robby-render/tests/tree.rs#L63) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn every_module_has_an_index_page() {
    let (doc, pages) = pages();
    let indexes = pages
        .iter()
        .filter(|page| page.path.file_name().is_some_and(|name| name == "index.md"))
        .count();

    let modules = doc
        .nodes
        .values()
        .filter(|node| {
            matches!(
                node.kind.as_str(),
                robby_ir::kind::code::MODULE | robby_ir::kind::code::CRATE
            )
        })
        .count();

    assert_eq!(
        indexes, modules,
        "every module and crate should have exactly one index page"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant every_module_has_an_index_page as every_module_has_an_index_page()
    participant pages as pages
    participant count as count
    every_module_has_an_index_page-->>pages: pages()
    every_module_has_an_index_page-->>count: pages .iter() .filter(|page| page.path.file_name().is_some_and(|name|...
    every_module_has_an_index_page-->>count: doc .nodes .values() .filter(|node| { matches!( node.kind.as_str(), r...
    every_module_has_an_index_page-->>every_module_has_an_index_page: assert_eq!( indexes, modules, 'every module and crate should have exa...
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::test_tree::pages`](pages.md)

