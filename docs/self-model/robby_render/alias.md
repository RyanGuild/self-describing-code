# `robby_render::alias`

[Self-model](../README.md) · [robby_render](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn alias(id: &str) -> String` |
| location | [`crates/robby-render/src/lib.rs:212`](../../../crates/robby-render/src/lib.rs#L212) |
| provenance | extractor · confidence 1.00 |

> Mermaid participant and node ids allow only a restricted alphabet.

```rust
/// Mermaid participant and node ids allow only a restricted alphabet.
fn alias(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'p');
    }
    if out.is_empty() {
        out.push_str("p_empty");
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant alias as alias()
    participant String__with_capacity as String::with_capacity
    participant push as push
    participant insert as insert
    participant push_str as push_str
    alias-->>String__with_capacity: String::with_capacity(id.len())
    loop id.chars()
    alt ch.is_ascii_alphanumeric() || ch == '_'
    alias-->>push: out.push(ch)
    else else
    alias-->>push: out.push('_')
    end
    end
    alt out.chars().next().is_some_and(|c| c.is_ascii_digit())
    alias-->>insert: out.insert(0, 'p')
    end
    alt out.is_empty()
    alias-->>push_str: out.push_str('p_empty')
    end
    alias-->>alias: out
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::insert`](../robby_ir/model/index.md)

