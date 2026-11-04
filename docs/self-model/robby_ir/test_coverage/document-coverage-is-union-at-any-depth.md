# `robby_ir::test_coverage::document_coverage_is_union_at_any_depth`

[Self-model](../../README.md) · [test_coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn document_coverage_is_union_at_any_depth()` |
| location | [`crates/robby-ir/tests/coverage.rs:187`](../../../../crates/robby-ir/tests/coverage.rs#L187) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn document_coverage_is_union_at_any_depth() {
    let source = "fn a() {}\nfn b() {}\n";
    let mut doc = Document::new("test");
    let mut parent = Node::new(NodeId::from("rust://demo"), kind::code::MODULE, "demo");
    parent.span = Some(Span::new(FILE, source, 0, source.len() as u32));
    doc.insert(parent);

    let mut sources = BTreeMap::new();
    sources.insert(FILE.to_owned(), source.to_owned());

    let report = document_coverage(&doc, &sources);
    assert!(
        report.is_complete(),
        "a parent span is a legitimate claim under the union model"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant document_coverage_is_union_at_any_depth as document_coverage_is_union_at_any_depth()
    participant Document__new as Document::new
    participant NodeId__from as NodeId::from
    participant Node__new as Node::new
    participant len as len
    participant Span__new as Span::new
    participant Some as Some
    participant insert as insert
    participant BTreeMap__new as BTreeMap::new
    participant to_owned as to_owned
    participant document_coverage as document_coverage
    document_coverage_is_union_at_any_depth-->>Document__new: Document::new('test')
    document_coverage_is_union_at_any_depth-->>NodeId__from: NodeId::from('rust://demo')
    document_coverage_is_union_at_any_depth-->>Node__new: Node::new(NodeId::from('rust://demo'), kind::code::MODULE, 'demo')
    document_coverage_is_union_at_any_depth-->>len: source.len()
    document_coverage_is_union_at_any_depth-->>Span__new: Span::new(FILE, source, 0, source.len() as u32)
    document_coverage_is_union_at_any_depth-->>Some: Some(Span::new(FILE, source, 0, source.len() as u32))
    document_coverage_is_union_at_any_depth-->>insert: doc.insert(parent)
    document_coverage_is_union_at_any_depth-->>BTreeMap__new: BTreeMap::new()
    document_coverage_is_union_at_any_depth-->>to_owned: FILE.to_owned()
    document_coverage_is_union_at_any_depth-->>to_owned: source.to_owned()
    document_coverage_is_union_at_any_depth-->>insert: sources.insert(FILE.to_owned(), source.to_owned())
    document_coverage_is_union_at_any_depth-->>document_coverage: document_coverage(&doc, &sources)
    document_coverage_is_union_at_any_depth-->>document_coverage_is_union_at_any_depth: assert!( report.is_complete(), 'a parent span is a legitimate claim u...
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../model/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::model::impl Document::insert`](../model/index.md)
- calls → [`robby_ir::coverage::document_coverage`](../coverage/document-coverage.md)

## References

- calls → [`robby_ir::coverage`](../coverage/index.md) — document_coverage
- calls → [`robby_ir::kind`](../kind/index.md) — from, new
- calls → [`robby_ir::model`](../model/index.md) — insert

