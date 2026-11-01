# `robby_render::markdown`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/lib.rs:8`](../../../../crates/robby-render/src/lib.rs#L8) |
| provenance | extractor · confidence 1.00 |
| nodes | 14 |

```mermaid
flowchart TD
    subgraph n_58704dd7dd5e44a7["markdown [module]"]
        direction TB
        n_d1bec2e55ee10959["fn report( doc: &Document, coverage: Option<&CoverageReport>, sources: Option<&BTreeMap<String, String>>, ) -> String [function]"]
        n_3f2b66ab38cc5d65["fn coverage_section(out: &mut String, report: &CoverageReport) [function]"]
        n_161877e5328c261f["fn node_section( out: &mut String, doc: &Document, node: &Node, sources: Option<&BTreeMap<String, String>>, ) [function]"]
        n_84ee9201a5b54929["fn heading_depth(doc: &Document, node: &Node) -> usize [function]"]
        n_7cb98ef1906d7e0b["fn cell(input: &str) -> String [function]"]
        n_1ab72dc7057894f4["fn source_name(source: robby_ir::Source) -> &'static str [function]"]
        n_6f44a6789c625aa4["fn edge_name(kind: EdgeKind) -> &'static str [function]"]
        n_fadae9084bd096ed["fn index(doc: &Document) -> String [function]"]
        n_742a0cec2e78e8fe["fn anchor(id: &NodeId) -> String [function]"]
    end
    n_161877e5328c261f -->|calls| n_84ee9201a5b54929
    n_d1bec2e55ee10959 -->|calls| n_161877e5328c261f
    n_d1bec2e55ee10959 -->|calls| n_3f2b66ab38cc5d65
```

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`report`](report.md) | `code.function` | 12 | `crates/robby-render/src/markdown.rs:14:1` |
| [`coverage_section`](coverage-section.md) | `code.function` | 12 | `crates/robby-render/src/markdown.rs:60:1` |
| [`node_section`](node-section.md) | `code.function` | 30 | `crates/robby-render/src/markdown.rs:102:1` |
| [`heading_depth`](heading-depth.md) | `code.function` | 5 | `crates/robby-render/src/markdown.rs:195:1` |
| [`cell`](cell.md) | `code.function` | 1 | `crates/robby-render/src/markdown.rs:205:1` |
| [`source_name`](source-name.md) | `code.function` | 5 | `crates/robby-render/src/markdown.rs:209:1` |
| [`edge_name`](edge-name.md) | `code.function` | 9 | `crates/robby-render/src/markdown.rs:219:1` |
| [`index`](index-n_fadae9.md) | `code.function` | 6 | `crates/robby-render/src/markdown.rs:233:1` |
| [`anchor`](anchor.md) | `code.function` | 1 | `crates/robby-render/src/markdown.rs:255:1` |

## Imports

- `use std::collections::BTreeMap;`
- `use std::fmt::Write as _;`
- `use robby_ir::{CoverageReport, Document, EdgeKind, Node, NodeId};`
- `use crate::{flowchart, sequence, stable_id};`

