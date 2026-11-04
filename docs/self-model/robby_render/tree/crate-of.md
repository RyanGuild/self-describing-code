# `robby_render::tree::crate_of`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn crate_of(node: &Node) -> Option<String>` |
| location | [`crates/robby-render/src/tree.rs:674`](../../../../crates/robby-render/src/tree.rs#L674) |
| provenance | extractor · confidence 1.00 |

```rust
fn crate_of(node: &Node) -> Option<String> {
    // Rust FQNs use `::`; TypeScript packages use `/` after the package name.
    let fqn = &node.fqn;
    if let Some((head, _)) = fqn.split_once("::") {
        return Some(head.to_owned());
    }
    if let Some((head, _)) = fqn.split_once('/') {
        return Some(head.to_owned());
    }
    Some(fqn.clone())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant crate_of as crate_of()
    participant split_once as split_once
    participant to_owned as to_owned
    participant Some as Some
    participant clone as clone
    crate_of-->>split_once: fqn.split_once('::')
    alt let Some((head, _)) = fqn.split_once('::')
    crate_of-->>to_owned: head.to_owned()
    crate_of-->>Some: Some(head.to_owned())
    crate_of-->>crate_of: return
    end
    crate_of-->>split_once: fqn.split_once('/')
    alt let Some((head, _)) = fqn.split_once('/')
    crate_of-->>to_owned: head.to_owned()
    crate_of-->>Some: Some(head.to_owned())
    crate_of-->>crate_of: return
    end
    crate_of-->>clone: fqn.clone()
    crate_of-->>Some: Some(fqn.clone())
```

10 steps · 0 observed · 10 possible only

## Referenced by

- calls ← [`robby_render::tree::nav`](nav.md)
- calls ← [`robby_render::tree::page_of_node`](page-of-node.md)

