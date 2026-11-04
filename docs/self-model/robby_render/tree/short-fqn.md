# `robby_render::tree::short_fqn`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn short_fqn(node: &Node, crate_dir: &str) -> String` |
| location | [`crates/robby-render/src/tree.rs:820`](../../../../crates/robby-render/src/tree.rs#L820) |
| provenance | extractor · confidence 1.00 |

```rust
fn short_fqn(node: &Node, crate_dir: &str) -> String {
    node.fqn
        .strip_prefix(&format!("{crate_dir}::"))
        .unwrap_or(&node.fqn)
        .to_owned()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant short_fqn as short_fqn()
    participant strip_prefix as strip_prefix
    participant unwrap_or as unwrap_or
    participant to_owned as to_owned
    short_fqn-->>short_fqn: format!('{crate_dir}::')
    short_fqn-->>strip_prefix: node.fqn .strip_prefix(&format!('{crate_dir}::'))
    short_fqn-->>unwrap_or: node.fqn .strip_prefix(&format!('{crate_dir}::')) .unwrap_or(&node.fqn)
    short_fqn-->>to_owned: node.fqn .strip_prefix(&format!('{crate_dir}::')) .unwrap_or(&node.fq...
```

4 steps · 0 observed · 4 possible only

## Referenced by

- calls ← [`robby_render::tree::nav`](nav.md)

