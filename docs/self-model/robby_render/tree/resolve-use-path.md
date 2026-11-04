# `robby_render::tree::resolve_use_path`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn resolve_use_path<'a>(doc: &'a Document, text: &str) -> Option<&'a Node>` |
| location | [`crates/robby-render/src/tree.rs:631`](../../../../crates/robby-render/src/tree.rs#L631) |
| provenance | extractor · confidence 1.00 |

```rust
fn resolve_use_path<'a>(doc: &'a Document, text: &str) -> Option<&'a Node> {
    let path = text
        .trim()
        .trim_start_matches("pub ")
        .trim_start_matches("use ")
        .trim_end_matches(';')
        .trim();
    let path = path
        .split_once('{')
        .map(|(prefix, _)| prefix)
        .unwrap_or(path)
        .split_once(" as ")
        .map(|(prefix, _)| prefix)
        .unwrap_or(path)
        .trim()
        .trim_end_matches("::")
        .trim();
    if path.is_empty() {
        return None;
    }

    let mut candidates = vec![path.to_owned()];
    let mut rest = path;
    while let Some((prefix, _)) = rest.rsplit_once("::") {
        candidates.push(prefix.to_owned());
        rest = prefix;
    }

    for candidate in candidates {
        let id = NodeId::symbol("rust", &candidate);
        if let Some(node) = doc.get(&id) {
            if is_module(node) {
                return Some(node);
            }
        }
    }
    None
}
```

```mermaid
sequenceDiagram
    autonumber
    participant resolve_use_path as resolve_use_path()
    participant trim as trim
    participant trim_start_matches as trim_start_matches
    participant trim_end_matches as trim_end_matches
    participant split_once as split_once
    participant map as map
    participant unwrap_or as unwrap_or
    participant is_empty as is_empty
    participant rsplit_once as rsplit_once
    participant to_owned as to_owned
    participant push as push
    participant NodeId__symbol as NodeId::symbol
    participant get as get
    participant is_module as is_module
    participant Some as Some
    resolve_use_path-->>trim: text .trim()
    resolve_use_path-->>trim_start_matches: text .trim() .trim_start_matches('pub ')
    resolve_use_path-->>trim_start_matches: text .trim() .trim_start_matches('pub ') .trim_start_matches('use ')
    resolve_use_path-->>trim_end_matches: text .trim() .trim_start_matches('pub ') .trim_start_matches('use ') ...
    resolve_use_path-->>trim: text .trim() .trim_start_matches('pub ') .trim_start_matches('use ') ...
    resolve_use_path-->>split_once: path .split_once('{')
    resolve_use_path-->>map: path .split_once('{') .map(|(prefix, _)| prefix)
    resolve_use_path-->>unwrap_or: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path)
    resolve_use_path-->>split_once: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>map: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>unwrap_or: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>trim: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>trim_end_matches: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>trim: path .split_once('{') .map(|(prefix, _)| prefix) .unwrap_or(path) .sp...
    resolve_use_path-->>is_empty: path.is_empty()
    alt path.is_empty()
    resolve_use_path-->>resolve_use_path: return
    end
    resolve_use_path-->>resolve_use_path: vec![path.to_owned()]
    resolve_use_path-->>rsplit_once: rest.rsplit_once('::')
    loop let Some((prefix, _)) = rest.rsplit_once('::')
    resolve_use_path-->>to_owned: prefix.to_owned()
    resolve_use_path-->>push: candidates.push(prefix.to_owned())
    end
    loop candidates
    resolve_use_path-->>NodeId__symbol: NodeId::symbol('rust', &candidate)
    resolve_use_path-->>get: doc.get(&id)
    alt let Some(node) = doc.get(&id)
    resolve_use_path-->>is_module: is_module(node)
    alt is_module(node)
    resolve_use_path-->>Some: Some(node)
    resolve_use_path-->>resolve_use_path: return
    end
    end
    end
```

25 steps · 0 observed · 25 possible only

**Edges**

- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_render::tree::is_module`](is-module.md)

## References

- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get, symbol
- calls → [`robby_render::report`](../report/index.md) — push
- calls → [`robby_render::tree`](index.md) — is_module
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty

## Referenced by

- calls ← [`robby_render::tree::linked_import`](linked-import.md)

