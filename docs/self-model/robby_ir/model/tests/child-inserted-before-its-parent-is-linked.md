# `robby_ir::model::child_inserted_before_its_parent_is_linked`

[Self-model](../../../README.md) · [model::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn child_inserted_before_its_parent_is_linked()` |
| location | [`crates/robby-ir/src/model.rs:751`](../../../../../crates/robby-ir/src/model.rs#L751) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn child_inserted_before_its_parent_is_linked() {
        let mut doc = Document::new("test");
        let mut child = Node::new("child", "code.function", "child");
        child.parent = Some(NodeId::from("parent"));
        doc.insert(child);
        doc.insert(Node::new("parent", "code.module", "parent"));

        assert_eq!(
            doc.get(&NodeId::from("parent")).unwrap().children,
            vec![NodeId::from("child")]
        );
        assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error));
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant child_inserted_before_its_parent_is_linked as child_inserted_before_its_parent_is_linked()
    participant Document__new as Document::new
    participant Node__new as Node::new
    participant NodeId__from as NodeId::from
    participant Some as Some
    participant insert as insert
    child_inserted_before_its_parent_is_linked-->>Document__new: Document::new('test')
    child_inserted_before_its_parent_is_linked-->>Node__new: Node::new('child', 'code.function', 'child')
    child_inserted_before_its_parent_is_linked-->>NodeId__from: NodeId::from('parent')
    child_inserted_before_its_parent_is_linked-->>Some: Some(NodeId::from('parent'))
    child_inserted_before_its_parent_is_linked-->>insert: doc.insert(child)
    child_inserted_before_its_parent_is_linked-->>Node__new: Node::new('parent', 'code.module', 'parent')
    child_inserted_before_its_parent_is_linked-->>insert: doc.insert(Node::new('parent', 'code.module', 'parent'))
    child_inserted_before_its_parent_is_linked-->>child_inserted_before_its_parent_is_linked: assert_eq!( doc.get(&NodeId::from('parent')).unwrap().children, vec![...
    child_inserted_before_its_parent_is_linked-->>child_inserted_before_its_parent_is_linked: assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error))
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl From<&str> for NodeId::from`](../index.md)
- calls → [`robby_ir::model::impl Document::insert`](../index.md)
- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl Document::insert`](../index.md)

## References

- calls → [`robby_ir::model`](../index.md) — from, insert, new

