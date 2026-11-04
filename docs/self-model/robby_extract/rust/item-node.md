# `robby_extract::rust::item_node`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_node(kind_name: &str, fqn: &str, span: Span, parent: &NodeId) -> Node` |
| location | [`crates/robby-extract/src/rust.rs:369`](../../../../crates/robby-extract/src/rust.rs#L369) |
| provenance | extractor · confidence 1.00 |

```rust
fn item_node(kind_name: &str, fqn: &str, span: Span, parent: &NodeId) -> Node {
    let mut node = Node::new(NodeId::symbol("rust", fqn), kind_name, fqn);
    node.kind = Kind::from(kind_name);
    node.span = Some(span);
    node.parent = Some(parent.clone());
    node.lang = Lang::Rust;
    node.provenance = Provenance::new(Source::Extractor);
    node
}
```

```mermaid
sequenceDiagram
    autonumber
    participant item_node as item_node()
    participant NodeId__symbol as NodeId::symbol
    participant Node__new as Node::new
    participant Kind__from as Kind::from
    participant Some as Some
    participant clone as clone
    participant Provenance__new as Provenance::new
    item_node-->>NodeId__symbol: NodeId::symbol('rust', fqn)
    item_node-->>Node__new: Node::new(NodeId::symbol('rust', fqn), kind_name, fqn)
    item_node-->>Kind__from: Kind::from(kind_name)
    item_node-->>Some: Some(span)
    item_node-->>clone: parent.clone()
    item_node-->>Some: Some(parent.clone())
    item_node-->>Provenance__new: Provenance::new(Source::Extractor)
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)
- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)

## References

- calls → [`robby_extract::rust`](index.md) — new
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — symbol

## Referenced by

- calls ← [`robby_extract::rust::walk_items`](walk-items.md)

