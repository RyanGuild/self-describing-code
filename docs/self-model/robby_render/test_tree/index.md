# `robby_render::test_tree`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 12 |

```mermaid
flowchart TD
    subgraph n_2790416a72a81c1b["test_tree [module]"]
        direction TB
        n_13a9263f385f07c0["fn workspace() -> PathBuf [function]"]
        n_9ed0f9b7842fbcc9["fn pages() -> (robby_ir::Document, Vec<tree::Page>) [function]"]
        n_197157e39be2dfd1["fn every_internal_link_resolves() [function]"]
        n_1a89d16c1861baae["fn no_page_is_a_wall_of_text() [function]"]
        n_32cf4d2ecb5a0a21["fn every_module_has_an_index_page() [function]"]
        n_4080fdf0b68d4a87["fn page_names_never_collide_with_index() [function]"]
        n_323013ead64fbe04["fn markdown_links(body: &str) -> Vec<String> [function]"]
        n_2bde649864bf1146["fn normalize(path: &Path) -> String [function]"]
    end
    n_197157e39be2dfd1 -->|calls| n_2bde649864bf1146
    n_197157e39be2dfd1 -->|calls| n_9ed0f9b7842fbcc9
    n_1a89d16c1861baae -->|calls| n_9ed0f9b7842fbcc9
    n_32cf4d2ecb5a0a21 -->|calls| n_9ed0f9b7842fbcc9
    n_4080fdf0b68d4a87 -->|calls| n_9ed0f9b7842fbcc9
    n_9ed0f9b7842fbcc9 -->|calls| n_13a9263f385f07c0
```

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`workspace`](workspace.md) | `code.function` | 1 | `crates/robby-render/tests/tree.rs:9:1` |
| [`pages`](pages.md) | `code.function` | 4 | `crates/robby-render/tests/tree.rs:13:1` |
| [`every_internal_link_resolves`](every-internal-link-resolves.md) | `code.function` | 9 | `crates/robby-render/tests/tree.rs:20:1` |
| [`no_page_is_a_wall_of_text`](no-page-is-a-wall-of-text.md) | `code.function` | 3 | `crates/robby-render/tests/tree.rs:46:1` |
| [`every_module_has_an_index_page`](every-module-has-an-index-page.md) | `code.function` | 4 | `crates/robby-render/tests/tree.rs:63:1` |
| [`page_names_never_collide_with_index`](page-names-never-collide-with-index.md) | `code.function` | 4 | `crates/robby-render/tests/tree.rs:88:1` |
| [`markdown_links`](markdown-links.md) | `code.function` | 11 | `crates/robby-render/tests/tree.rs:98:1` |
| [`normalize`](normalize.md) | `code.function` | 4 | `crates/robby-render/tests/tree.rs:126:1` |

## Imports

- `use std::collections::BTreeSet;`
- `use std::path::{Component, Path, PathBuf};`
- `use robby_render::tree;`

