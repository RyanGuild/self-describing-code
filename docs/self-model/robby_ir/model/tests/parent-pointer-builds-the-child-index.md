# `robby_ir::model::parent_pointer_builds_the_child_index`

[Self-model](../../../README.md) · [model::tests](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn parent_pointer_builds_the_child_index()` |
| location | [`crates/robby-ir/src/model.rs:735`](../../../../../crates/robby-ir/src/model.rs#L735) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
    fn parent_pointer_builds_the_child_index() {
        let mut doc = Document::new("test");
        doc.insert(Node::new("parent", "code.module", "parent"));

        let mut child = Node::new("child", "code.function", "child");
        child.parent = Some(NodeId::from("parent"));
        doc.insert(child);

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
    participant parent_pointer_builds_the_child_index as parent_pointer_builds_the_child_index()
    participant Document__new as Document::new
    participant Node__new as Node::new
    participant insert as insert
    participant NodeId__from as NodeId::from
    participant Some as Some
    parent_pointer_builds_the_child_index-->>Document__new: Document::new('test')
    parent_pointer_builds_the_child_index-->>Node__new: Node::new('parent', 'code.module', 'parent')
    parent_pointer_builds_the_child_index-->>insert: doc.insert(Node::new('parent', 'code.module', 'parent'))
    parent_pointer_builds_the_child_index-->>Node__new: Node::new('child', 'code.function', 'child')
    parent_pointer_builds_the_child_index-->>NodeId__from: NodeId::from('parent')
    parent_pointer_builds_the_child_index-->>Some: Some(NodeId::from('parent'))
    parent_pointer_builds_the_child_index-->>insert: doc.insert(child)
    parent_pointer_builds_the_child_index-->>parent_pointer_builds_the_child_index: assert_eq!( doc.get(&NodeId::from('parent')).unwrap().children, vec![...
    parent_pointer_builds_the_child_index-->>parent_pointer_builds_the_child_index: assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error))
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl Document::insert`](../index.md)
- calls → [`robby_ir::model::impl Edge::new`](../index.md)
- calls → [`robby_ir::model::impl From<&str> for NodeId::from`](../index.md)
- calls → [`robby_ir::model::impl Document::insert`](../index.md)

## References

- calls → [`robby_ir::model`](../index.md) — from, insert, new

