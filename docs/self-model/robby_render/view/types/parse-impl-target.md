# `robby_render::view::types::parse_impl_target`

[Self-model](../../../README.md) · [view::types](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn parse_impl_target(node: &Node) -> Option<(Option<String>, String)>` |
| location | [`crates/robby-render/src/view/types.rs:230`](../../../../../crates/robby-render/src/view/types.rs#L230) |
| provenance | extractor · confidence 1.00 |

```rust
fn parse_impl_target(node: &Node) -> Option<(Option<String>, String)> {
    let rest = node
        .fqn
        .rsplit_once("::impl ")
        .map(|(_, rest)| rest)
        .or_else(|| node.fqn.strip_prefix("impl "))
        .or_else(|| node.name.strip_prefix("impl "))?;

    if let Some((trait_part, self_part)) = rest.split_once(" for ") {
        Some((
            Some(trait_part.trim().to_owned()),
            self_part.trim().to_owned(),
        ))
    } else {
        Some((None, rest.trim().to_owned()))
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant parse_impl_target as parse_impl_target()
    participant rsplit_once as rsplit_once
    participant map as map
    participant strip_prefix as strip_prefix
    participant or_else as or_else
    participant split_once as split_once
    participant trim as trim
    participant to_owned as to_owned
    participant Some as Some
    parse_impl_target-->>rsplit_once: node .fqn .rsplit_once('::impl ')
    parse_impl_target-->>map: node .fqn .rsplit_once('::impl ') .map(|(_, rest)| rest)
    parse_impl_target-->>strip_prefix: node.fqn.strip_prefix('impl ')
    parse_impl_target-->>or_else: node .fqn .rsplit_once('::impl ') .map(|(_, rest)| rest) .or_else(|| ...
    parse_impl_target-->>strip_prefix: node.name.strip_prefix('impl ')
    parse_impl_target-->>or_else: node .fqn .rsplit_once('::impl ') .map(|(_, rest)| rest) .or_else(|| ...
    parse_impl_target-->>parse_impl_target: ? (propagate)
    parse_impl_target-->>split_once: rest.split_once(' for ')
    alt let Some((trait_part, self_part)) = rest.split_once(' for ')
    parse_impl_target-->>trim: trait_part.trim()
    parse_impl_target-->>to_owned: trait_part.trim().to_owned()
    parse_impl_target-->>Some: Some(trait_part.trim().to_owned())
    parse_impl_target-->>trim: self_part.trim()
    parse_impl_target-->>to_owned: self_part.trim().to_owned()
    parse_impl_target-->>Some: Some(( Some(trait_part.trim().to_owned()), self_part.trim().to_owned(...
    else else
    parse_impl_target-->>trim: rest.trim()
    parse_impl_target-->>to_owned: rest.trim().to_owned()
    parse_impl_target-->>Some: Some((None, rest.trim().to_owned()))
    end
```

17 steps · 0 observed · 17 possible only

## Referenced by

- calls ← [`robby_render::view::types::impl TypeModel::from_module`](index.md)

