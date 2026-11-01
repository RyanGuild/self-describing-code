# `robby_ir`

[Self-model](../README.md)

| | |
|---|---|
| kind | `code.crate` |
| provenance | extractor · confidence 1.00 |
| nodes | 303 |

```mermaid
flowchart TD
    subgraph n_356bf2c23da68961["robby_ir [crate]"]
        direction TB
        n_965af55f4081b03f["coverage [module]"]
        n_611ba0cee1ac191f["kind [module]"]
        n_48c15551e2e91996["model [module]"]
        n_f5e61214f719e7ae["process [module]"]
        n_9053aa1513b08ea5["span [module]"]
        n_7f203267d92a1682["fn schema() -> schemars::Schema [function]"]
        n_ca6f58cffa1dcfd5["fn schema_json() -> String [function]"]
        n_46b0c6f667b1fb40["test_coverage [module]"]
    end
```

## Modules

- [`coverage`](coverage/index.md) — 50 nodes
- [`kind`](kind/index.md) — 50 nodes
- [`model`](model/index.md) — 112 nodes
- [`process`](process/index.md) — 43 nodes
- [`span`](span/index.md) — 19 nodes
- [`test_coverage`](test_coverage/index.md) — 21 nodes

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`schema`](schema.md) | `code.function` | 1 | `crates/robby-ir/src/lib.rs:34:1` |
| [`schema_json`](schema-json.md) | `code.function` | 1 | `crates/robby-ir/src/lib.rs:42:1` |

## Imports

- `pub use coverage::{`
- `pub use kind::Kind;`
- `pub use model::{`
- `pub use process::{Block, BlockKind, Participant, Process, Step, StepKind};`
- `pub use span::{Position, Span};`

