# `robby_agent::enrich::noise_short_circuits_without_note_call`

[Self-model](../../../README.md) · [enrich::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn noise_short_circuits_without_note_call()` |
| location | [`crates/robby-agent/src/enrich.rs:233`](../../../../../crates/robby-agent/src/enrich.rs#L233) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn noise_short_circuits_without_note_call() {
        let (mut doc, sources) = mini_doc();
        let client = FakeClient::new(vec![r#"{"verdict":"noise","reason":"filler"}"#.into()]);
        let stats = enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default()).unwrap();
        assert_eq!(stats.noise, 1);
        assert_eq!(stats.noted, 0);
        let decision = doc
            .nodes
            .values()
            .find(|n| n.kind.as_str() == kind::agent::DECISION)
            .expect("decision");
        assert_eq!(
            decision
                .annotations
                .get("verdict")
                .map(std::string::String::as_str),
            Some("noise")
        );
        assert!(decision.docs.is_none());
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant noise_short_circuits_without_note_call as noise_short_circuits_without_note_call()
    participant mini_doc as mini_doc
    participant FakeClient__new as FakeClient::new
    participant EnrichOpts__default as EnrichOpts::default
    participant enrich_comments as enrich_comments
    participant unwrap as unwrap
    participant values as values
    participant as_str as as_str
    participant find as find
    participant expect as expect
    noise_short_circuits_without_note_call-->>mini_doc: mini_doc()
    noise_short_circuits_without_note_call-->>noise_short_circuits_without_note_call: vec![r '{'verdict':'noise','reason':'filler'}' .into()]
    noise_short_circuits_without_note_call-->>FakeClient__new: FakeClient::new(vec![r '{'verdict':'noise','reason':'filler'}' .into()])
    noise_short_circuits_without_note_call-->>EnrichOpts__default: EnrichOpts::default()
    noise_short_circuits_without_note_call-->>enrich_comments: enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default())
    noise_short_circuits_without_note_call-->>unwrap: enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default())....
    noise_short_circuits_without_note_call-->>noise_short_circuits_without_note_call: assert_eq!(stats.noise, 1)
    noise_short_circuits_without_note_call-->>noise_short_circuits_without_note_call: assert_eq!(stats.noted, 0)
    noise_short_circuits_without_note_call-->>values: doc .nodes .values()
    noise_short_circuits_without_note_call-->>as_str: n.kind.as_str()
    noise_short_circuits_without_note_call-->>find: doc .nodes .values() .find(|n| n.kind.as_str() == kind::agent::DECISION)
    noise_short_circuits_without_note_call-->>expect: doc .nodes .values() .find(|n| n.kind.as_str() == kind::agent::DECISI...
    noise_short_circuits_without_note_call-->>noise_short_circuits_without_note_call: assert_eq!( decision .annotations .get('verdict') .map(std::string::S...
    noise_short_circuits_without_note_call-->>noise_short_circuits_without_note_call: assert!(decision.docs.is_none())
```

14 steps · 0 observed · 14 possible only

**Edges**

- calls → [`robby_agent::enrich::mini_doc`](mini-doc.md)
- calls → [`robby_agent::enrich::impl FakeClient::new`](index.md)
- calls → [`robby_agent::enrich::impl Default for EnrichOpts::default`](../index.md)
- calls → [`robby_agent::enrich::enrich_comments`](../enrich-comments.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../../robby_ir/kind/index.md)

## References

- calls → [`robby_agent::enrich`](../index.md) — default, enrich_comments
- calls → [`robby_agent::enrich::tests`](index.md) — mini_doc, new
- calls → [`robby_ir::kind`](../../../robby_ir/kind/index.md) — as_str

