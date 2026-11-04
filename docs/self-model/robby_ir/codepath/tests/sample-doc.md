# `robby_ir::codepath::sample_doc`

[Self-model](../../../README.md) · [codepath::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn sample_doc() -> Document` |
| location | [`crates/robby-ir/src/codepath.rs:136`](../../../../../crates/robby-ir/src/codepath.rs#L136) |
| provenance | extractor · confidence 1.00 |

```rust
fn sample_doc() -> Document {
        let mut doc = Document::new("test");
        let mut main = Node::new(
            NodeId::symbol("rust", "demo::main"),
            kind::code::FUNCTION,
            "demo::main",
        );
        main.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::a"),
            EdgeKind::Calls,
        ));
        doc.insert(main);

        let mut a = Node::new(
            NodeId::symbol("rust", "demo::a"),
            kind::code::FUNCTION,
            "demo::a",
        );
        a.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::b"),
            EdgeKind::Calls,
        ));
        doc.insert(a);

        doc.insert(Node::new(
            NodeId::symbol("rust", "demo::b"),
            kind::code::FUNCTION,
            "demo::b",
        ));
        doc
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant sample_doc as sample_doc()
    participant Document__new as Document::new
    participant NodeId__symbol as NodeId::symbol
    participant Node__new as Node::new
    participant Edge__new as Edge::new
    participant push as push
    participant insert as insert
    sample_doc-->>Document__new: Document::new('test')
    sample_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::main')
    sample_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::main'), kind::code::FUNCTION...
    sample_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    sample_doc-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKind::Calls, )
    sample_doc-->>push: main.edges.push(Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKin...
    sample_doc-->>insert: doc.insert(main)
    sample_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    sample_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::a'), kind::code::FUNCTION, '...
    sample_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    sample_doc-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKind::Calls, )
    sample_doc-->>push: a.edges.push(Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKind::...
    sample_doc-->>insert: doc.insert(a)
    sample_doc-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    sample_doc-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::b'), kind::code::FUNCTION, '...
    sample_doc-->>insert: doc.insert(Node::new( NodeId::symbol('rust', 'demo::b'), kind::code::...
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_render::report::impl Report::push`](../../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_render::report::impl Report::push`](../../../robby_render/report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../../kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../model/index.md)

## References

- calls → [`robby_ir::kind`](../../kind/index.md) — new
- calls → [`robby_ir::model`](../../model/index.md) — insert, symbol
- calls → [`robby_render::report`](../../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_ir::codepath::builds_path_from_main`](builds-path-from-main.md)
- calls ← [`robby_ir::codepath::respects_max_depth`](respects-max-depth.md)

