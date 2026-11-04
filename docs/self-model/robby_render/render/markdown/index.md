# `robby_render::render::markdown`

[Self-model](../../../README.md) · [robby_render](../../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/render/mod.rs:4`](../../../../../crates/robby-render/src/render/mod.rs#L4) |
| provenance | extractor · confidence 1.00 |
| nodes | 25 |

## Types

```mermaid
classDiagram
    class MarkdownRenderer {
        +mermaid
        +fence()
        +new()
    }
    class Renderer {
        <<interface>>
    }
    MarkdownRenderer ..|> Renderer : implements
```

## Package diagram

```mermaid
flowchart TB
    n_6f40e2fa7b42a40f["markdown"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_6f40e2fa7b42a40f --> n_356bf2c23da68961
    n_6f40e2fa7b42a40f --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_6f40e2fa7b42a40f["markdown"]
        direction TB
        n_8ff5a307b7777fb9["comments"]
        n_c85b951c411a59cf["coupling_deps"]
        n_b788aee11cf3dbed["externals"]
        n_27f283958f7b3bc6["fence"]
        n_2f40f9b922edd53a["hierarchy"]
        n_302e6be037d2c928["links"]
        n_5e80afda1320de5a["meta"]
        n_ea8679fdc5a30ae1["new"]
        n_4a581ee9a238d7f8["package_deps"]
        n_d957da81bd0d16e4["process"]
        n_cf84a0e43b973e14["sibling_graph"]
        n_5ac99e1c84ebd9ad["trim_comment"]
        n_1da2b6c82b3cdf34["types"]
    end
    n_1da2b6c82b3cdf34 --> n_27f283958f7b3bc6
    n_27f283958f7b3bc6 --> n_ea8679fdc5a30ae1
    n_2f40f9b922edd53a --> n_ea8679fdc5a30ae1
    n_302e6be037d2c928 --> n_ea8679fdc5a30ae1
    n_4a581ee9a238d7f8 --> n_27f283958f7b3bc6
    n_5e80afda1320de5a --> n_ea8679fdc5a30ae1
    n_8ff5a307b7777fb9 --> n_5ac99e1c84ebd9ad
    n_8ff5a307b7777fb9 --> n_ea8679fdc5a30ae1
    n_b788aee11cf3dbed --> n_ea8679fdc5a30ae1
    n_c85b951c411a59cf --> n_27f283958f7b3bc6
    n_cf84a0e43b973e14 --> n_27f283958f7b3bc6
    n_d957da81bd0d16e4 --> n_27f283958f7b3bc6
```

## Dependency matrix

| | markdown | robby_ir | robby_render |
|---|---:|---:|---:|
| **markdown** | · | 2 | 17 |
| **robby_ir** | | · | |
| **robby_render** | | | · |
## References

- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — from
- calls → [`robby_render::render::mermaid`](../mermaid/index.md) — dsm_markdown_table
- calls → [`robby_render::render::tui`](../tui/index.md) — default
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::view::comments`](../../view/comments/index.md) — apply_highlights, as_str
- calls → [`robby_render::view::links`](../../view/links/index.md) — is_empty
- calls → [`robby_render::view::select`](../../view/select/index.md) — collect

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`MarkdownRenderer`](markdownrenderer.md) | `code.struct` | — | `crates/robby-render/src/render/markdown.rs:16:1` |
| [`impl MarkdownRenderer`](impl-markdownrenderer.md) | `code.impl` | — | `crates/robby-render/src/render/markdown.rs:21:1` |
| [`impl Renderer for MarkdownRenderer`](impl-renderer-for-markdownrenderer.md) | `code.impl` | — | `crates/robby-render/src/render/markdown.rs:42:1` |
| [`cell`](cell.md) | `code.function` | 2 | `crates/robby-render/src/render/markdown.rs:186:1` |
| [`trim_comment`](trim-comment.md) | `code.function` | 12 | `crates/robby-render/src/render/markdown.rs:190:1` |

## Imports

- `use std::fmt::Write as _;`
- `use crate::view::{`
- `use super::Renderer;`
- `use super::mermaid::{self, MermaidRenderer};`

