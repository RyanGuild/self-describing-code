# `robby_render::test_render::process`

[Self-model](../../README.md) · [test_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn process(steps: Vec<Step>) -> Process` |
| location | [`crates/robby-render/tests/render.rs:25`](../../../../crates/robby-render/tests/render.rs#L25) |
| provenance | extractor · confidence 1.00 |

```rust
fn process(steps: Vec<Step>) -> Process {
    let mut p = Process::new();
    p.participant("caller", "caller", Kind::from(code::FUNCTION));
    p.participant("io", "io::load", Kind::from(code::FUNCTION));
    for s in steps {
        p.step(s);
    }
    p
}
```

```mermaid
sequenceDiagram
    autonumber
    participant process as process()
    participant Process__new as Process::new
    participant Kind__from as Kind::from
    participant participant as participant
    participant step as step
    process-->>Process__new: Process::new()
    process-->>Kind__from: Kind::from(code::FUNCTION)
    process-->>participant: p.participant('caller', 'caller', Kind::from(code::FUNCTION))
    process-->>Kind__from: Kind::from(code::FUNCTION)
    process-->>participant: p.participant('io', 'io::load', Kind::from(code::FUNCTION))
    loop steps
    process-->>step: p.step(s)
    end
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../robby_ir/process/index.md)
- calls → [`robby_render::test_render::step`](step.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::process`](../../robby_ir/process/index.md) — participant
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::test_render`](index.md) — step

## Referenced by

- calls ← [`robby_render::markdown::report`](../markdown/report.md)
- calls ← [`robby_render::sequence`](../sequence.md)
- calls ← [`robby_render::test_render::emits_else_for_a_second_branch`](emits-else-for-a-second-branch.md)
- calls ← [`robby_render::test_render::nests_blocks_by_frame_stack`](nests-blocks-by-frame-stack.md)
- calls ← [`robby_render::test_render::opens_and_closes_alt_blocks`](opens-and-closes-alt-blocks.md)
- calls ← [`robby_render::test_render::output_is_byte_identical_across_runs`](output-is-byte-identical-across-runs.md)
- calls ← [`robby_render::test_render::renders_a_flat_sequence`](renders-a-flat-sequence.md)
- calls ← [`robby_render::test_render::unobserved_steps_use_dashed_arrows`](unobserved-steps-use-dashed-arrows.md)
- calls ← [`robby_render::tree::module_page`](../tree/module-page.md)
- calls ← [`robby_render::tui::timeline`](../tui/timeline.md)
- calls ← [`robby_render::web::impl FrameDetail::from_node_in_doc`](../web/index.md)

