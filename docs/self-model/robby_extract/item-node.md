# `robby_extract::item_node`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn item_node(kind_name: &str, fqn: &str, span: Span, parent: &NodeId) -> Node` |
| location | [`crates/robby-extract/src/lib.rs:383`](../../../crates/robby-extract/src/lib.rs#L383) |
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
    participant Node__new as Node::new
    item_node-->>Node__new: Node::new(NodeId::symbol('rust', fqn), kind_name, fqn)
    item_node-->>item_node: node.kind = Kind::from(kind_name)
    item_node-->>item_node: node.span = Some(span)
    item_node-->>item_node: node.parent = Some(parent.clone())
    item_node-->>item_node: node.lang = Lang::Rust
    item_node-->>item_node: node.provenance = Provenance::new(Source::Extractor)
    item_node-->>item_node: node
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)

