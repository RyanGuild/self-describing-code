# `robby_ir::contract::absorb_trait_marks_matching_steps_observed`

[Self-model](../../../README.md) · [contract::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn absorb_trait_marks_matching_steps_observed()` |
| location | [`crates/robby-ir/src/contract.rs:98`](../../../../../crates/robby-ir/src/contract.rs#L98) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn absorb_trait_marks_matching_steps_observed() {
        let mut static_proc = Process::new();
        static_proc.participant("a", "A", kind::Kind::new(kind::code::FUNCTION));
        static_proc.participant("b", "B", kind::Kind::new(kind::code::FUNCTION));
        static_proc.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "call".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: Vec::new(),
        });

        let mut observed = Process::new();
        observed.participant("a", "A", kind::Kind::new(kind::code::FUNCTION));
        observed.participant("b", "B", kind::Kind::new(kind::code::FUNCTION));
        observed.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "call".into(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: Vec::new(),
        });

        Absorb::absorb(&mut static_proc, &observed);
        assert!(static_proc.steps[0].observed);
        assert_eq!(static_proc.steps.len(), 1);
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant absorb_trait_marks_matching_steps_observed as absorb_trait_marks_matching_steps_observed()
    participant Process__new as Process::new
    participant kind__Kind__new as kind::Kind::new
    participant participant as participant
    participant into as into
    participant Vec__new as Vec::new
    participant step as step
    participant Absorb__absorb as Absorb::absorb
    absorb_trait_marks_matching_steps_observed-->>Process__new: Process::new()
    absorb_trait_marks_matching_steps_observed-->>kind__Kind__new: kind::Kind::new(kind::code::FUNCTION)
    absorb_trait_marks_matching_steps_observed-->>participant: static_proc.participant('a', 'A', kind::Kind::new(kind::code::FUNCTION))
    absorb_trait_marks_matching_steps_observed-->>kind__Kind__new: kind::Kind::new(kind::code::FUNCTION)
    absorb_trait_marks_matching_steps_observed-->>participant: static_proc.participant('b', 'B', kind::Kind::new(kind::code::FUNCTION))
    absorb_trait_marks_matching_steps_observed-->>into: 'a'.into()
    absorb_trait_marks_matching_steps_observed-->>into: 'b'.into()
    absorb_trait_marks_matching_steps_observed-->>into: 'call'.into()
    absorb_trait_marks_matching_steps_observed-->>Vec__new: Vec::new()
    absorb_trait_marks_matching_steps_observed-->>step: static_proc.step(Step { from: 'a'.into(), to: 'b'.into(), label: 'cal...
    absorb_trait_marks_matching_steps_observed-->>Process__new: Process::new()
    absorb_trait_marks_matching_steps_observed-->>kind__Kind__new: kind::Kind::new(kind::code::FUNCTION)
    absorb_trait_marks_matching_steps_observed-->>participant: observed.participant('a', 'A', kind::Kind::new(kind::code::FUNCTION))
    absorb_trait_marks_matching_steps_observed-->>kind__Kind__new: kind::Kind::new(kind::code::FUNCTION)
    absorb_trait_marks_matching_steps_observed-->>participant: observed.participant('b', 'B', kind::Kind::new(kind::code::FUNCTION))
    absorb_trait_marks_matching_steps_observed-->>into: 'a'.into()
    absorb_trait_marks_matching_steps_observed-->>into: 'b'.into()
    absorb_trait_marks_matching_steps_observed-->>into: 'call'.into()
    absorb_trait_marks_matching_steps_observed-->>Vec__new: Vec::new()
    absorb_trait_marks_matching_steps_observed-->>step: observed.step(Step { from: 'a'.into(), to: 'b'.into(), label: 'call'....
    absorb_trait_marks_matching_steps_observed-->>Absorb__absorb: Absorb::absorb(&mut static_proc, &observed)
    absorb_trait_marks_matching_steps_observed-->>absorb_trait_marks_matching_steps_observed: assert!(static_proc.steps[0].observed)
    absorb_trait_marks_matching_steps_observed-->>absorb_trait_marks_matching_steps_observed: assert_eq!(static_proc.steps.len(), 1)
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../process/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../process/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::step`](../../process/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../process/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::participant`](../../process/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::process::impl Process::step`](../../process/index.md)
- calls → [`robby_ir::contract::Absorb::absorb`](../index.md)

## References

- calls → [`robby_ir::contract`](../index.md) — absorb
- calls → [`robby_ir::kind`](../../kind/index.md) — new
- calls → [`robby_ir::process`](../../process/index.md) — participant, step

