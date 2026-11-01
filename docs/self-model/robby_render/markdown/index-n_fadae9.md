# `robby_render::markdown::index`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn index(doc: &Document) -> String` |
| location | [`crates/robby-render/src/markdown.rs:233`](../../../../crates/robby-render/src/markdown.rs#L233) |
| provenance | extractor · confidence 1.00 |

> A compact index of every node, for the top of the report.

```rust
/// A compact index of every node, for the top of the report.
pub fn index(doc: &Document) -> String {
    let mut out = String::from("| Node | Kind | Location |\n|---|---|---|\n");
    let ids: Vec<&NodeId> = doc.nodes.keys().collect();
    for id in ids {
        let node = &doc.nodes[id];
        let location = node
            .span
            .as_ref()
            .map(|s| s.location())
            .unwrap_or_else(|| "—".to_owned());
        let _ = writeln!(
            out,
            "| `{}` | `{}` | `{}` |",
            node.fqn,
            node.kind,
            location
        );
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant index as index()
    participant String__from as String::from
    participant collect as collect
    participant unwrap_or_else as unwrap_or_else
    index-->>String__from: String::from('| Node | Kind | Location |\n|---|---|---|\n')
    index-->>collect: doc.nodes.keys().collect()
    loop ids
    index-->>index: &doc.nodes[id]
    index-->>unwrap_or_else: node .span .as_ref() .map(|s| s.location()) .unwrap_or_else(|| '—'.to...
    index-->>index: writeln!( out, '| `{}` | `{}` | `{}` |', node.fqn, node.kind, location )
    end
    index-->>index: out
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)

