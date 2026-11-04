# `robby_ir::codepath::respects_max_depth`

[Self-model](../../../README.md) · [codepath::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn respects_max_depth()` |
| location | [`crates/robby-ir/src/codepath.rs:182`](../../../../../crates/robby-ir/src/codepath.rs#L182) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn respects_max_depth() {
        let doc = sample_doc();
        let path =
            CodePath::from_entry(&doc, &NodeId::symbol("rust", "demo::main"), 1).expect("path");
        assert_eq!(path.edges.len(), 1);
        assert_eq!(path.edges[0].to, NodeId::symbol("rust", "demo::a"));
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant respects_max_depth as respects_max_depth()
    participant sample_doc as sample_doc
    participant NodeId__symbol as NodeId::symbol
    participant CodePath__from_entry as CodePath::from_entry
    participant expect as expect
    respects_max_depth-->>sample_doc: sample_doc()
    respects_max_depth-->>NodeId__symbol: NodeId::symbol('rust', 'demo::main')
    respects_max_depth-->>CodePath__from_entry: CodePath::from_entry(&doc, &NodeId::symbol('rust', 'demo::main'), 1)
    respects_max_depth-->>expect: CodePath::from_entry(&doc, &NodeId::symbol('rust', 'demo::main'), 1)....
    respects_max_depth-->>respects_max_depth: assert_eq!(path.edges.len(), 1)
    respects_max_depth-->>respects_max_depth: assert_eq!(path.edges[0].to, NodeId::symbol('rust', 'demo::a'))
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::codepath::sample_doc`](sample-doc.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::codepath::impl CodePath::from_entry`](../index.md)

## References

- calls → [`robby_ir::codepath`](../index.md) — from_entry
- calls → [`robby_ir::codepath::tests`](index.md) — sample_doc
- calls → [`robby_ir::model`](../../model/index.md) — symbol

