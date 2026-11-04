# `robby_render::view::externals`

[Self-model](../../../README.md) · [robby_render](../../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/view/mod.rs:10`](../../../../../crates/robby-render/src/view/mod.rs#L10) |
| provenance | extractor · confidence 1.00 |
| nodes | 16 |

## Types

```mermaid
classDiagram
    class ExternalsView {
        +entries
        +from_root()
        +is_empty()
    }
    class ExternalEntry {
        +package
        +spec
        +importers
    }
```

## Package diagram

```mermaid
flowchart TB
    n_575b84f39392c791["externals"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_575b84f39392c791 --> n_356bf2c23da68961
    n_575b84f39392c791 --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_575b84f39392c791["externals"]
        direction TB
        n_00fad7cf58583263["from_root"]
        n_32c1f67a7062b22e["import_spec"]
        n_c274af82ed9c19c4["is_empty"]
        n_a570dbeb379084c2["is_external_package"]
        n_99d441e4191b295f["owning_container_fqn"]
    end
    n_00fad7cf58583263 --> n_32c1f67a7062b22e
    n_00fad7cf58583263 --> n_99d441e4191b295f
    n_00fad7cf58583263 --> n_a570dbeb379084c2
```

## Dependency matrix

| | externals | robby_ir | robby_render |
|---|---:|---:|---:|
| **externals** | · | 6 | 7 |
| **robby_ir** | | · | |
| **robby_render** | | | · |
## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get, insert, subtree
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::links`](../links/index.md) — is_module_kind
- calls → [`robby_render::view::scene`](../scene/index.md) — new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`ExternalsView`](externalsview.md) | `code.struct` | — | `crates/robby-render/src/view/externals.rs:9:1` |
| [`ExternalEntry`](externalentry.md) | `code.struct` | — | `crates/robby-render/src/view/externals.rs:14:1` |
| [`impl ExternalsView`](impl-externalsview.md) | `code.impl` | — | `crates/robby-render/src/view/externals.rs:21:1` |
| [`is_external_package`](is-external-package.md) | `code.function` | 8 | `crates/robby-render/src/view/externals.rs:72:1` |
| [`import_spec`](import-spec.md) | `code.function` | 4 | `crates/robby-render/src/view/externals.rs:85:1` |
| [`owning_container_fqn`](owning-container-fqn.md) | `code.function` | 11 | `crates/robby-render/src/view/externals.rs:90:1` |

## Imports

- `use std::collections::{BTreeMap, BTreeSet};`
- [`use robby_ir::{Document, EdgeKind, Node, NodeId, kind};`](../../../robby_ir/index.md)
- `use super::links::is_module_kind;`

