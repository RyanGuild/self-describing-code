# `robby_render::test_tree::normalize`

[Self-model](../../README.md) · [test_tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn normalize(path: &Path) -> String` |
| location | [`crates/robby-render/tests/tree.rs:126`](../../../../crates/robby-render/tests/tree.rs#L126) |
| provenance | extractor · confidence 1.00 |

```rust
fn normalize(path: &Path) -> String {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                parts.pop();
            }
            Component::CurDir => {}
            Component::Normal(part) => parts.push(part.to_string_lossy().to_string()),
            _ => {}
        }
    }
    parts.join("/")
}
```

```mermaid
sequenceDiagram
    autonumber
    participant normalize as normalize()
    participant Vec__new as Vec::new
    participant pop as pop
    participant push as push
    participant join as join
    normalize-->>Vec__new: Vec::new()
    loop path.components()
    alt match component
    normalize-->>pop: parts.pop()
    else Component::Normal(part)
    normalize-->>push: parts.push(part.to_string_lossy().to_string())
    end
    end
    normalize-->>join: parts.join('/')
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

