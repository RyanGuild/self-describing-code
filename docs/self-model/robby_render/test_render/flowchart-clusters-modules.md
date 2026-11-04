# `robby_render::test_render::flowchart_clusters_modules`

[Self-model](../../README.md) · [test_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn flowchart_clusters_modules()` |
| location | [`crates/robby-render/tests/render.rs:140`](../../../../crates/robby-render/tests/render.rs#L140) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn flowchart_clusters_modules() {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    module.provenance.source = Source::Extractor;
    doc.insert(module);

    let mut a = Node::new(NodeId::symbol("rust", "demo::a"), code::FUNCTION, "demo::a");
    a.parent = Some(NodeId::from("rust://demo"));
    a.signature = Some("fn a()".to_owned());
    doc.insert(a);

    let mut b = Node::new(NodeId::symbol("rust", "demo::b"), code::FUNCTION, "demo::b");
    b.parent = Some(NodeId::from("rust://demo"));
    b.edges.push(Edge::new(
        NodeId::symbol("rust", "demo::a"),
        EdgeKind::Calls,
    ));
    doc.insert(b);

    let out = flowchart(&doc, &NodeId::from("rust://demo"));

    assert!(out.starts_with("flowchart TD\n"));
    assert!(out.contains("subgraph"));
    assert!(out.contains("|calls|"));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant flowchart_clusters_modules as flowchart_clusters_modules()
    participant Document__new as Document::new
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant Kind__from as Kind::from
    participant insert as insert
    participant NodeId__symbol as NodeId::symbol
    participant Some as Some
    participant to_owned as to_owned
    participant Edge__new as Edge::new
    participant push as push
    participant flowchart as flowchart
    flowchart_clusters_modules-->>Document__new: Document::new('test')
    flowchart_clusters_modules-->>NodeId__from: NodeId::from('rust://demo')
    flowchart_clusters_modules-->>Node__new: Node::new(NodeId::from('rust://demo'), code::MODULE, 'demo')
    flowchart_clusters_modules-->>Kind__from: Kind::from(code::MODULE)
    flowchart_clusters_modules-->>insert: doc.insert(module)
    flowchart_clusters_modules-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    flowchart_clusters_modules-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::a'), code::FUNCTION, 'demo::a')
    flowchart_clusters_modules-->>NodeId__from: NodeId::from('rust://demo')
    flowchart_clusters_modules-->>Some: Some(NodeId::from('rust://demo'))
    flowchart_clusters_modules-->>to_owned: 'fn a()'.to_owned()
    flowchart_clusters_modules-->>Some: Some('fn a()'.to_owned())
    flowchart_clusters_modules-->>insert: doc.insert(a)
    flowchart_clusters_modules-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    flowchart_clusters_modules-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::b'), code::FUNCTION, 'demo::b')
    flowchart_clusters_modules-->>NodeId__from: NodeId::from('rust://demo')
    flowchart_clusters_modules-->>Some: Some(NodeId::from('rust://demo'))
    flowchart_clusters_modules-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    flowchart_clusters_modules-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKind::Calls, )
    flowchart_clusters_modules-->>push: b.edges.push(Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKind::...
    flowchart_clusters_modules-->>insert: doc.insert(b)
    flowchart_clusters_modules-->>NodeId__from: NodeId::from('rust://demo')
    flowchart_clusters_modules-->>flowchart: flowchart(&doc, &NodeId::from('rust://demo'))
    flowchart_clusters_modules-->>flowchart_clusters_modules: assert!(out.starts_with('flowchart TD\n'))
    flowchart_clusters_modules-->>flowchart_clusters_modules: assert!(out.contains('subgraph'))
    flowchart_clusters_modules-->>flowchart_clusters_modules: assert!(out.contains('|calls|'))
```

25 steps · 0 observed · 25 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::flowchart`](../flowchart.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert, symbol
- calls → [`robby_render`](../index.md) — flowchart
- calls → [`robby_render::report`](../report/index.md) — new, push

