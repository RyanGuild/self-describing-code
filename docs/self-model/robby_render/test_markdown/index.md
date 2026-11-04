# `robby_render::test_markdown`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 12 |

## Package diagram

```mermaid
flowchart TB
    n_94a590d1410dd3da["test_markdown"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_94a590d1410dd3da --> n_356bf2c23da68961
    n_94a590d1410dd3da --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_94a590d1410dd3da["test_markdown"]
        direction TB
        n_5cc8cf51165d164e["fences_are_balanced"]
        n_3c079b97f8351ae3["fixture"]
        n_40fd118f8ee57918["report_embeds_source_and_diagram_together"]
        n_74c96613b6250856["report_is_byte_stable"]
        n_bac59c77f333c3ad["report_without_sources_still_renders"]
    end
    n_40fd118f8ee57918 --> n_3c079b97f8351ae3
    n_5cc8cf51165d164e --> n_3c079b97f8351ae3
    n_74c96613b6250856 --> n_3c079b97f8351ae3
    n_bac59c77f333c3ad --> n_3c079b97f8351ae3
```

## Dependency matrix

| | test_markdown | robby_ir | robby_render |
|---|---:|---:|---:|
| **test_markdown** | · | 12 | 13 |
| **robby_ir** | | · | |
| **robby_render** | | | · |
## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert, symbol
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant
- calls → [`robby_render::markdown`](../markdown/index.md) — report
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::test_render`](../test_render/index.md) — step

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`SOURCE`](source.md) | `code.const` | — | `crates/robby-render/tests/markdown.rs:10:1` |
| [`fixture`](fixture.md) | `code.function` | 31 | `crates/robby-render/tests/markdown.rs:12:1` |
| [`report_is_byte_stable`](report-is-byte-stable.md) | `code.function` | 6 | `crates/robby-render/tests/markdown.rs:47:1` |
| [`report_embeds_source_and_diagram_together`](report-embeds-source-and-diagram-together.md) | `code.function` | 8 | `crates/robby-render/tests/markdown.rs:55:1` |
| [`fences_are_balanced`](fences-are-balanced.md) | `code.function` | 6 | `crates/robby-render/tests/markdown.rs:67:1` |
| [`report_without_sources_still_renders`](report-without-sources-still-renders.md) | `code.function` | 4 | `crates/robby-render/tests/markdown.rs:75:1` |
| [`unused_participant_import_is_referenced`](unused-participant-import-is-referenced.md) | `code.function` | 3 | `crates/robby-render/tests/markdown.rs:83:1` |

## Imports

- `use std::collections::BTreeMap;`
- [`use robby_ir::kind::code;`](../../robby_ir/kind/code/index.md)
- [`use robby_ir::{Document, Kind, Node, NodeId, Participant, Process, Source, Step, StepKind};`](../../robby_ir/index.md)
- [`use robby_render::markdown::report;`](../markdown/index.md)

