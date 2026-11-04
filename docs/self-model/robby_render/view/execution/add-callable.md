# `robby_render::view::execution::add_callable`

[Self-model](../../../README.md) · [view::execution](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn add_callable(process: &mut Process, node: &Node)` |
| location | [`crates/robby-render/src/view/execution.rs:80`](../../../../../crates/robby-render/src/view/execution.rs#L80) |
| provenance | extractor · confidence 1.00 |

```rust
fn add_callable(process: &mut Process, node: &Node) {
    process.participant(
        participant_id(node),
        node.fqn.clone(),
        node.kind.clone(),
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant add_callable as add_callable()
    participant participant_id as participant_id
    participant clone as clone
    participant participant as participant
    add_callable-->>participant_id: participant_id(node)
    add_callable-->>clone: node.fqn.clone()
    add_callable-->>clone: node.kind.clone()
    add_callable-->>participant: process.participant( participant_id(node), node.fqn.clone(), node.kin...
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::view::execution::participant_id`](participant-id.md)
- calls → [`robby_ir::process::impl Process::participant`](../../../robby_ir/process/index.md)

## References

- calls → [`robby_ir::process`](../../../robby_ir/process/index.md) — participant
- calls → [`robby_render::view::execution`](index.md) — participant_id

## Referenced by

- calls ← [`robby_render::view::execution::from_code_path`](from-code-path.md)

