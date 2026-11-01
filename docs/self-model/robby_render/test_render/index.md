# `robby_render::test_render`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 14 |

```mermaid
flowchart TD
    subgraph n_092b7abcfcf260db["test_render [module]"]
        direction TB
        n_32370c79df1f48c9["fn step(from: &str, to: &str, label: &str, observed: bool, blocks: Vec<Block>) -> Step [function]"]
        n_016efff8aa636e20["fn process(steps: Vec<Step>) -> Process [function]"]
        n_f34932bdcafaaace["fn renders_a_flat_sequence() [function]"]
        n_7cea0c557227a35f["fn unobserved_steps_use_dashed_arrows() [function]"]
        n_85e74789245bccf1["fn opens_and_closes_alt_blocks() [function]"]
        n_8eba10265997f105["fn emits_else_for_a_second_branch() [function]"]
        n_ae1563fdedf3f116["fn nests_blocks_by_frame_stack() [function]"]
        n_3ff0d4fa507d354d["fn output_is_byte_identical_across_runs() [function]"]
        n_2e9e6864f59046a7["fn stable_ids_are_pinned() [function]"]
        n_7a167edd9760d943["fn flowchart_clusters_modules() [function]"]
    end
    n_3ff0d4fa507d354d -->|calls| n_016efff8aa636e20
    n_7cea0c557227a35f -->|calls| n_016efff8aa636e20
    n_85e74789245bccf1 -->|calls| n_016efff8aa636e20
    n_8eba10265997f105 -->|calls| n_016efff8aa636e20
    n_ae1563fdedf3f116 -->|calls| n_016efff8aa636e20
    n_f34932bdcafaaace -->|calls| n_016efff8aa636e20
```

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`step`](step.md) | `code.function` | 1 | `crates/robby-render/tests/render.rs:9:1` |
| [`process`](process.md) | `code.function` | 5 | `crates/robby-render/tests/render.rs:21:1` |
| [`renders_a_flat_sequence`](renders-a-flat-sequence.md) | `code.function` | 6 | `crates/robby-render/tests/render.rs:31:1` |
| [`unobserved_steps_use_dashed_arrows`](unobserved-steps-use-dashed-arrows.md) | `code.function` | 4 | `crates/robby-render/tests/render.rs:42:1` |
| [`opens_and_closes_alt_blocks`](opens-and-closes-alt-blocks.md) | `code.function` | 8 | `crates/robby-render/tests/render.rs:50:1` |
| [`emits_else_for_a_second_branch`](emits-else-for-a-second-branch.md) | `code.function` | 6 | `crates/robby-render/tests/render.rs:72:1` |
| [`nests_blocks_by_frame_stack`](nests-blocks-by-frame-stack.md) | `code.function` | 9 | `crates/robby-render/tests/render.rs:94:1` |
| [`output_is_byte_identical_across_runs`](output-is-byte-identical-across-runs.md) | `code.function` | 2 | `crates/robby-render/tests/render.rs:118:1` |
| [`stable_ids_are_pinned`](stable-ids-are-pinned.md) | `code.function` | 3 | `crates/robby-render/tests/render.rs:127:1` |
| [`flowchart_clusters_modules`](flowchart-clusters-modules.md) | `code.function` | 17 | `crates/robby-render/tests/render.rs:136:1` |

## Imports

- `use robby_ir::kind::code;`
- `use robby_ir::{`
- `use robby_render::{flowchart, sequence, stable_id};`

