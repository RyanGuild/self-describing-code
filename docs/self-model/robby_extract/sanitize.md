# `robby_extract::sanitize`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn sanitize(input: &str) -> String` |
| location | [`crates/robby-extract/src/lib.rs:784`](../../../crates/robby-extract/src/lib.rs#L784) |
| provenance | extractor · confidence 1.00 |

```rust
fn sanitize(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push_str("p_empty");
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'p');
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant sanitize as sanitize()
    participant String__with_capacity as String::with_capacity
    participant push as push
    participant push_str as push_str
    participant insert as insert
    sanitize-->>String__with_capacity: String::with_capacity(input.len())
    loop input.chars()
    alt ch.is_ascii_alphanumeric() || ch == '_'
    sanitize-->>push: out.push(ch)
    else else
    sanitize-->>push: out.push('_')
    end
    end
    alt out.is_empty()
    sanitize-->>push_str: out.push_str('p_empty')
    end
    alt out.chars().next().is_some_and(|c| c.is_ascii_digit())
    sanitize-->>insert: out.insert(0, 'p')
    end
    sanitize-->>sanitize: out
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::insert`](../robby_ir/model/index.md)

