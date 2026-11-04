# `robby_render::test_tree::every_module_has_an_index_page`

[Self-model](../../README.md) · [test_tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn every_module_has_an_index_page()` |
| location | [`crates/robby-render/tests/tree.rs:66`](../../../../crates/robby-render/tests/tree.rs#L66) |
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
                robby_ir::kind::code::MODULE
                    | robby_ir::kind::code::CRATE
                    | robby_ir::kind::code::PACKAGE
            )
        })
        .count();

    assert_eq!(
        indexes, modules,
        "every module, crate, and package should have exactly one index page"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant every_module_has_an_index_page as every_module_has_an_index_page()
    participant pages as pages
    participant iter as iter
    participant file_name as file_name
    participant is_some_and as is_some_and
    participant filter as filter
    participant count as count
    participant values as values
    every_module_has_an_index_page-->>pages: pages()
    every_module_has_an_index_page-->>iter: pages .iter()
    every_module_has_an_index_page-->>file_name: page.path.file_name()
    every_module_has_an_index_page-->>is_some_and: page.path.file_name().is_some_and(|name| name == 'index.md')
    every_module_has_an_index_page-->>filter: pages .iter() .filter(|page| page.path.file_name().is_some_and(|name|...
    every_module_has_an_index_page-->>count: pages .iter() .filter(|page| page.path.file_name().is_some_and(|name|...
    every_module_has_an_index_page-->>values: doc .nodes .values()
    every_module_has_an_index_page-->>every_module_has_an_index_page: matches!( node.kind.as_str(), robby_ir::kind::code::MODULE | robby_ir...
    every_module_has_an_index_page-->>filter: doc .nodes .values() .filter(|node| { matches!( node.kind.as_str(), r...
    every_module_has_an_index_page-->>count: doc .nodes .values() .filter(|node| { matches!( node.kind.as_str(), r...
    every_module_has_an_index_page-->>every_module_has_an_index_page: assert_eq!( indexes, modules, 'every module, crate, and package shoul...
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::test_tree::pages`](pages.md)

## References

- calls → [`robby_render::test_tree`](index.md) — pages

