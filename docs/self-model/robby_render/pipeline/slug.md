# `robby_render::pipeline::slug`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn slug(input: &str) -> String` |
| location | [`crates/robby-render/src/pipeline.rs:185`](../../../../crates/robby-render/src/pipeline.rs#L185) |
| provenance | extractor · confidence 1.00 |

```rust
fn slug(input: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        out.push_str(&crate::stable_id(input));
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant slug as slug()
    participant String__new as String::new
    participant push as push
    participant pop as pop
    participant push_str as push_str
    slug-->>String__new: String::new()
    slug-->>slug: false
    loop input.chars()
    alt ch.is_ascii_alphanumeric()
    slug-->>push: out.push(ch.to_ascii_lowercase())
    slug-->>slug: prev_dash = false
    else else
    alt !prev_dash && !out.is_empty()
    slug-->>push: out.push('-')
    slug-->>slug: prev_dash = true
    end
    end
    end
    loop out.ends_with('-')
    slug-->>pop: out.pop()
    end
    alt out.is_empty()
    slug-->>push_str: out.push_str(&crate::stable_id(input))
    end
    slug-->>slug: out
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

