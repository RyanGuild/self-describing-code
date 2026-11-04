# `robby_render::view::types`

[Self-model](../../../README.md) · [view](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/view/mod.rs:18`](../../../../../crates/robby-render/src/view/mod.rs#L18) |
| provenance | extractor · confidence 1.00 |
| nodes | 32 |

## Types

```mermaid
classDiagram
    class TypeModel {
        +classes
        +relations
        +from_module()
        +is_empty()
    }
    class ClassView {
        +id
        +name
        +stereotype
        +members
    }
    class ClassStereotype {
        <<enumeration>>
        Interface
        Enumeration
    }
    class ClassMember {
        <<enumeration>>
        Field
        Method
        Variant
    }
    class TypeRelation {
        +from_id
        +to_id
        +kind
    }
    class RelationKind {
        <<enumeration>>
        Implements
        Extends
    }
```

## Package diagram

```mermaid
flowchart TB
    n_f7b09b6570bc03a0["types"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_f7b09b6570bc03a0 --> n_356bf2c23da68961
    n_f7b09b6570bc03a0 --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_f7b09b6570bc03a0["types"]
        direction TB
        n_93c94be41cee362e["class_from_node"]
        n_c7c36b72a1b4ae90["from_module"]
        n_116ac437ecee9baf["impl_methods"]
        n_d317bd47c5c14e11["is_empty"]
        n_796acf395b92fc32["parse_impl_target"]
    end
    n_c7c36b72a1b4ae90 --> n_116ac437ecee9baf
    n_c7c36b72a1b4ae90 --> n_796acf395b92fc32
    n_c7c36b72a1b4ae90 -->|"2"| n_93c94be41cee362e
```

## Dependency matrix

| | types | robby_ir | robby_render |
|---|---:|---:|---:|
| **types** | · | 11 | 41 |
| **robby_ir** | | · | |
| **robby_render** | | | · |
## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — insert
- calls → [`robby_ir::run`](../../../robby_ir/run/index.md) — get
- calls → [`robby_render::report`](../../report/index.md) — push
- calls → [`robby_render::util`](../../util/index.md) — class_id, is_type, strip_generics, text
- calls → [`robby_render::view::comments`](../comments/index.md) — as_str
- calls → [`robby_render::view::scene`](../scene/index.md) — new
- calls → [`robby_render::view::select`](../select/index.md) — collect

## Referenced by

- calls ← [`robby_render::class_diagram`](../../class-diagram.md)
- calls ← [`robby_render::view::select::collect`](../select/collect.md)

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`TypeModel`](typemodel.md) | `code.struct` | — | `crates/robby-render/src/view/types.rs:9:1` |
| [`ClassView`](classview.md) | `code.struct` | — | `crates/robby-render/src/view/types.rs:15:1` |
| [`ClassStereotype`](classstereotype.md) | `code.enum` | — | `crates/robby-render/src/view/types.rs:24:1` |
| [`ClassMember`](classmember.md) | `code.enum` | — | `crates/robby-render/src/view/types.rs:30:1` |
| [`TypeRelation`](typerelation.md) | `code.struct` | — | `crates/robby-render/src/view/types.rs:37:1` |
| [`RelationKind`](relationkind.md) | `code.enum` | — | `crates/robby-render/src/view/types.rs:44:1` |
| [`impl TypeModel`](impl-typemodel.md) | `code.impl` | — | `crates/robby-render/src/view/types.rs:50:1` |
| [`class_from_node`](class-from-node.md) | `code.function` | 24 | `crates/robby-render/src/view/types.rs:166:1` |
| [`impl_methods`](impl-methods.md) | `code.function` | 9 | `crates/robby-render/src/view/types.rs:218:1` |
| [`parse_impl_target`](parse-impl-target.md) | `code.function` | 17 | `crates/robby-render/src/view/types.rs:230:1` |

## Imports

- `use std::collections::{BTreeMap, BTreeSet};`
- [`use robby_ir::{Document, EdgeKind, Node, NodeId, kind};`](../../../robby_ir/index.md)
- `use crate::util::{class_id, is_type, strip_generics, text};`

