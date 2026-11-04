# `robby_render::view::execution::sequence_includes_call_and_type_lifeline`

[Self-model](../../../../README.md) · [view::execution::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn sequence_includes_call_and_type_lifeline()` |
| location | [`crates/robby-render/src/view/execution.rs:155`](../../../../../../crates/robby-render/src/view/execution.rs#L155) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn sequence_includes_call_and_type_lifeline() {
        let doc = sample();
        let process =
            execution_process(&doc, &NodeId::symbol("rust", "demo::main")).expect("process");
        assert!(
            process
                .participants
                .iter()
                .any(|p| p.label.contains("main"))
        );
        assert!(process.participants.iter().any(|p| p.label.contains("run")));
        assert!(
            process.participants.iter().any(|p| p.label == "Foo"),
            "expected Foo type participant, got {:?}",
            process.participants
        );
        assert!(process.steps.len() >= 2);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant sequence_includes_call_and_type_lifeline as sequence_includes_call_and_type_lifeline()
    participant sample as sample
    participant NodeId__symbol as NodeId::symbol
    participant execution_process as execution_process
    participant expect as expect
    sequence_includes_call_and_type_lifeline-->>sample: sample()
    sequence_includes_call_and_type_lifeline-->>NodeId__symbol: NodeId::symbol('rust', 'demo::main')
    sequence_includes_call_and_type_lifeline-->>execution_process: execution_process(&doc, &NodeId::symbol('rust', 'demo::main'))
    sequence_includes_call_and_type_lifeline-->>expect: execution_process(&doc, &NodeId::symbol('rust', 'demo::main')).expect...
    sequence_includes_call_and_type_lifeline-->>sequence_includes_call_and_type_lifeline: assert!( process .participants .iter() .any(|p| p.label.contains('mai...
    sequence_includes_call_and_type_lifeline-->>sequence_includes_call_and_type_lifeline: assert!(process.participants.iter().any(|p| p.label.contains('run')))
    sequence_includes_call_and_type_lifeline-->>sequence_includes_call_and_type_lifeline: assert!( process.participants.iter().any(|p| p.label == 'Foo'), 'expe...
    sequence_includes_call_and_type_lifeline-->>sequence_includes_call_and_type_lifeline: assert!(process.steps.len() >= 2)
```

8 steps · 0 observed · 8 possible only

**Edges**

- calls → [`robby_render::view::execution::sample`](sample.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::execution::execution_process`](../execution-process.md)

## References

- calls → [`robby_ir::model`](../../../../robby_ir/model/index.md) — symbol
- calls → [`robby_render::view::execution`](../index.md) — execution_process
- calls → [`robby_render::view::execution::tests`](index.md) — sample

