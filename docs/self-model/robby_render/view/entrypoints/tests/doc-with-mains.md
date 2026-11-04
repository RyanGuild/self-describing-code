# `robby_render::view::entrypoints::doc_with_mains`

[Self-model](../../../../README.md) · [view::entrypoints::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn doc_with_mains() -> Document` |
| location | [`crates/robby-render/src/view/entrypoints.rs:131`](../../../../../../crates/robby-render/src/view/entrypoints.rs#L131) |
| provenance | extractor · confidence 1.00 |

```rust
fn doc_with_mains() -> Document {
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

        let mut other = Node::new(
            NodeId::symbol("rust", "demo::other"),
            kind::code::FUNCTION,
            "demo::other",
        );
        other.edges.push(Edge::new(
            NodeId::symbol("rust", "demo::b"),
            EdgeKind::Calls,
        ));
        doc.insert(other);

        doc.rebuild_code_paths();
        doc
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant doc_with_mains as doc_with_mains()
    participant Document__new as Document::new
    participant NodeId__symbol as NodeId::symbol
    participant Node__new as Node::new
    participant Edge__new as Edge::new
    participant push as push
    participant insert as insert
    participant rebuild_code_paths as rebuild_code_paths
    doc_with_mains-->>Document__new: Document::new('test')
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::main')
    doc_with_mains-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::main'), kind::code::FUNCTION...
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    doc_with_mains-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKind::Calls, )
    doc_with_mains-->>push: main.edges.push(Edge::new( NodeId::symbol('rust', 'demo::a'), EdgeKin...
    doc_with_mains-->>insert: doc.insert(main)
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    doc_with_mains-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::a'), kind::code::FUNCTION, '...
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    doc_with_mains-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKind::Calls, )
    doc_with_mains-->>push: a.edges.push(Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKind::...
    doc_with_mains-->>insert: doc.insert(a)
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    doc_with_mains-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::b'), kind::code::FUNCTION, '...
    doc_with_mains-->>insert: doc.insert(Node::new( NodeId::symbol('rust', 'demo::b'), kind::code::...
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::other')
    doc_with_mains-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::other'), kind::code::FUNCTIO...
    doc_with_mains-->>NodeId__symbol: NodeId::symbol('rust', 'demo::b')
    doc_with_mains-->>Edge__new: Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKind::Calls, )
    doc_with_mains-->>push: other.edges.push(Edge::new( NodeId::symbol('rust', 'demo::b'), EdgeKi...
    doc_with_mains-->>insert: doc.insert(other)
    doc_with_mains-->>rebuild_code_paths: doc.rebuild_code_paths()
```

23 steps · 0 observed · 23 possible only

**Edges**

- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../../../robby_ir/model/index.md)
- calls → [`robby_render::view::scene::impl Rect::new`](../../scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../../../report/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::rebuild_code_paths`](../../../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../../../../robby_ir/model/index.md) — insert, rebuild_code_paths, symbol
- calls → [`robby_render::report`](../../../report/index.md) — push
- calls → [`robby_render::view::scene`](../../scene/index.md) — new

## Referenced by

- calls ← [`robby_render::view::entrypoints::first_parent_wins_shared_callee`](first-parent-wins-shared-callee.md)
- calls ← [`robby_render::view::entrypoints::prefers_document_code_paths`](prefers-document-code-paths.md)

