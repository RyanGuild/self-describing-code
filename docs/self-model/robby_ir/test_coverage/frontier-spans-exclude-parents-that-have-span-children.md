# `robby_ir::test_coverage::frontier_spans_exclude_parents_that_have_span_children`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn frontier_spans_exclude_parents_that_have_span_children()` |
| location | [`crates/robby-ir/tests/coverage.rs:173`](../../../../crates/robby-ir/tests/coverage.rs#L173) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn frontier_spans_exclude_parents_that_have_span_children() {
    let source = "fn a() {}\nfn b() {}\n";
    let mut doc = Document::new("test");
    let mut parent = Node::new(NodeId::from("rust://demo"), kind::code::MODULE, "demo");
    parent.span = Some(Span::new(FILE, source, 0, source.len() as u32));
    doc.insert(parent);

    let mut a = Node::new(
        NodeId::symbol("rust", "demo::a"),
        kind::code::FUNCTION,
        "demo::a",
    );
    a.parent = Some(NodeId::from("rust://demo"));
    a.span = Some(Span::new(FILE, source, 0, 9));
    doc.insert(a);

    let frontier = robby_ir::coverage::frontier_spans(&doc);
    let spans = frontier.get(FILE).expect("frontier");
    assert_eq!(spans.len(), 1, "only the leaf claims");
    assert_eq!(spans[0].byte_range(), (0, 9));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant frontier_spans_exclude_parents_that_have_span_children as frontier_spans_exclude_parents_that_have_span_children()
    participant Document__new as Document::new
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant len as len
    participant Span__new as Span::new
    participant Some as Some
    participant insert as insert
    participant NodeId__symbol as NodeId::symbol
    participant robby_ir__coverage__frontier_spans as robby_ir::coverage::frontier_spans
    participant get as get
    participant expect as expect
    frontier_spans_exclude_parents_that_have_span_children-->>Document__new: Document::new('test')
    frontier_spans_exclude_parents_that_have_span_children-->>NodeId__from: NodeId::from('rust://demo')
    frontier_spans_exclude_parents_that_have_span_children-->>Node__new: Node::new(NodeId::from('rust://demo'), kind::code::MODULE, 'demo')
    frontier_spans_exclude_parents_that_have_span_children-->>len: source.len()
    frontier_spans_exclude_parents_that_have_span_children-->>Span__new: Span::new(FILE, source, 0, source.len() as u32)
    frontier_spans_exclude_parents_that_have_span_children-->>Some: Some(Span::new(FILE, source, 0, source.len() as u32))
    frontier_spans_exclude_parents_that_have_span_children-->>insert: doc.insert(parent)
    frontier_spans_exclude_parents_that_have_span_children-->>NodeId__symbol: NodeId::symbol('rust', 'demo::a')
    frontier_spans_exclude_parents_that_have_span_children-->>Node__new: Node::new( NodeId::symbol('rust', 'demo::a'), kind::code::FUNCTION, '...
    frontier_spans_exclude_parents_that_have_span_children-->>NodeId__from: NodeId::from('rust://demo')
    frontier_spans_exclude_parents_that_have_span_children-->>Some: Some(NodeId::from('rust://demo'))
    frontier_spans_exclude_parents_that_have_span_children-->>Span__new: Span::new(FILE, source, 0, 9)
    frontier_spans_exclude_parents_that_have_span_children-->>Some: Some(Span::new(FILE, source, 0, 9))
    frontier_spans_exclude_parents_that_have_span_children-->>insert: doc.insert(a)
    frontier_spans_exclude_parents_that_have_span_children-->>robby_ir__coverage__frontier_spans: robby_ir::coverage::frontier_spans(&doc)
    frontier_spans_exclude_parents_that_have_span_children-->>get: frontier.get(FILE)
    frontier_spans_exclude_parents_that_have_span_children-->>expect: frontier.get(FILE).expect('frontier')
    frontier_spans_exclude_parents_that_have_span_children-->>frontier_spans_exclude_parents_that_have_span_children: assert_eq!(spans.len(), 1, 'only the leaf claims')
    frontier_spans_exclude_parents_that_have_span_children-->>frontier_spans_exclude_parents_that_have_span_children: assert_eq!(spans[0].byte_range(), (0, 9))
```

19 steps · 0 observed · 19 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../model/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../model/index.md)
- calls → [`robby_ir::coverage::frontier_spans`](../coverage/frontier-spans.md)
- calls → [`robby_ir::model::impl Document::get`](../model/index.md)

## References

- calls → [`robby_ir::coverage`](../coverage/index.md) — frontier_spans
- calls → [`robby_ir::kind`](../kind/index.md) — from, new
- calls → [`robby_ir::model`](../model/index.md) — get, insert, symbol

