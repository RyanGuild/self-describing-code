# `robby_render::test_render::class_diagram_marks_traits_as_interfaces`

[Self-model](../../README.md) · [test_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn class_diagram_marks_traits_as_interfaces()` |
| location | [`crates/robby-render/tests/render.rs:469`](../../../../crates/robby-render/tests/render.rs#L469) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn class_diagram_marks_traits_as_interfaces() {
    let mut doc = Document::new("test");

    let mut module = Node::new(NodeId::from("rust://demo"), code::MODULE, "demo");
    module.kind = Kind::from(code::MODULE);
    doc.insert(module);

    let mut drawable = Node::new(
        NodeId::symbol("rust", "demo::Drawable"),
        code::TRAIT,
        "Drawable",
    );
    drawable.parent = Some(NodeId::from("rust://demo"));
    doc.insert(drawable);

    let mut method = Node::new(
        NodeId::symbol("rust", "demo::Drawable::draw"),
        code::METHOD,
        "draw",
    );
    method.parent = Some(NodeId::symbol("rust", "demo::Drawable"));
    doc.insert(method);

    let mut point = Node::new(NodeId::symbol("rust", "demo::Point"), code::STRUCT, "Point");
    point.parent = Some(NodeId::from("rust://demo"));
    point.children = vec![NodeId::symbol("rust", "demo::Point::x")];
    doc.insert(point);

    let mut field = Node::new(NodeId::symbol("rust", "demo::Point::x"), code::FIELD, "x");
    field.parent = Some(NodeId::symbol("rust", "demo::Point"));
    doc.insert(field);

    let mut impl_node = Node::new(
        NodeId::symbol("rust", "demo::impl Drawable for Point"),
        code::IMPL,
        "impl Drawable for Point",
    );
    impl_node.parent = Some(NodeId::from("rust://demo"));
    doc.insert(impl_node);

    if let Some(module) = doc.nodes.get_mut(&NodeId::from("rust://demo")) {
        module.children = vec![
            NodeId::symbol("rust", "demo::Drawable"),
            NodeId::symbol("rust", "demo::Point"),
            NodeId::symbol("rust", "demo::impl Drawable for Point"),
        ];
    }
    if let Some(drawable) = doc.nodes.get_mut(&NodeId::symbol("rust", "demo::Drawable")) {
        drawable.children = vec![NodeId::symbol("rust", "demo::Drawable::draw")];
    }

    let out = class_diagram(&doc, &NodeId::from("rust://demo"));
    assert!(out.contains("<<interface>>"));
    assert!(out.contains("class Drawable"));
    assert!(out.contains("+draw()"));
    assert!(out.contains("class Point"));
    assert!(out.contains("+x"));
    assert!(out.contains("Point ..|> Drawable : implements"));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant class_diagram_marks_traits_as_interfaces as class_diagram_marks_traits_as_interfaces()
    participant Document__new as Document::new
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant Kind__from as Kind::from
    participant insert as insert
    participant NodeId__symbol as NodeId::symbol
    participant Some as Some
    participant get_mut as get_mut
    participant class_diagram as class_diagram
    class_diagram_marks_traits_as_interfaces-->>Document__new: Document::new('test')
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new(NodeId::from('rust://demo'), code::MODULE, 'demo')
    class_diagram_marks_traits_as_interfaces-->>Kind__from: Kind::from(code::MODULE)
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(module)
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Drawable')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::Drawable'), code::TRAIT, 'Dr...
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>Some: Some(NodeId::from('rust://demo'))
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(drawable)
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Drawable::draw')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::Drawable::draw'), code::METH...
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Drawable')
    class_diagram_marks_traits_as_interfaces-->>Some: Some(NodeId::symbol('rust', 'demo::Drawable'))
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(method)
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Point')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::Point'), code::STRUCT, 'Point')
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>Some: Some(NodeId::from('rust://demo'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: vec![NodeId::symbol('rust', 'demo::Point::x')]
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(point)
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Point::x')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new(NodeId::symbol('rust', 'demo::Point::x'), code::FIELD, 'x')
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Point')
    class_diagram_marks_traits_as_interfaces-->>Some: Some(NodeId::symbol('rust', 'demo::Point'))
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(field)
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::impl Drawable for Point')
    class_diagram_marks_traits_as_interfaces-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::impl Drawable for Point'), c...
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>Some: Some(NodeId::from('rust://demo'))
    class_diagram_marks_traits_as_interfaces-->>insert: doc.insert(impl_node)
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>get_mut: doc.nodes.get_mut(&NodeId::from('rust://demo'))
    alt let Some(module) = doc.nodes.get_mut(&NodeId::from('rust://demo'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: vec![ NodeId::symbol('rust', 'demo::Drawable'), NodeId::symbol('rust'...
    end
    class_diagram_marks_traits_as_interfaces-->>NodeId__symbol: NodeId::symbol('rust', 'demo::Drawable')
    class_diagram_marks_traits_as_interfaces-->>get_mut: doc.nodes.get_mut(&NodeId::symbol('rust', 'demo::Drawable'))
    alt let Some(drawable) = doc.nodes.get_mut(&NodeId::symbol('rust', 'demo:...
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: vec![NodeId::symbol('rust', 'demo::Drawable::draw')]
    end
    class_diagram_marks_traits_as_interfaces-->>NodeId__from: NodeId::from('rust://demo')
    class_diagram_marks_traits_as_interfaces-->>class_diagram: class_diagram(&doc, &NodeId::from('rust://demo'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('<<interface>>'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('class Drawable'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('+draw()'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('class Point'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('+x'))
    class_diagram_marks_traits_as_interfaces-->>class_diagram_marks_traits_as_interfaces: assert!(out.contains('Point ..|> Drawable : implements'))
```

45 steps · 0 observed · 45 possible only

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
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::class_diagram`](../class-diagram.md)

## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert, symbol
- calls → [`robby_render`](../index.md) — class_diagram
- calls → [`robby_render::report`](../report/index.md) — new

