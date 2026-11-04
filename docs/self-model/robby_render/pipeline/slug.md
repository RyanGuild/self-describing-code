# `robby_render::pipeline::slug`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn slug(input: &str) -> String` |
| location | [`crates/robby-render/src/pipeline.rs:188`](../../../../crates/robby-render/src/pipeline.rs#L188) |
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
    participant chars as chars
    participant is_ascii_alphanumeric as is_ascii_alphanumeric
    participant to_ascii_lowercase as to_ascii_lowercase
    participant push as push
    participant is_empty as is_empty
    participant ends_with as ends_with
    participant pop as pop
    participant crate__stable_id as crate::stable_id
    participant push_str as push_str
    slug-->>String__new: String::new()
    slug-->>chars: input.chars()
    loop input.chars()
    slug-->>is_ascii_alphanumeric: ch.is_ascii_alphanumeric()
    alt ch.is_ascii_alphanumeric()
    slug-->>to_ascii_lowercase: ch.to_ascii_lowercase()
    slug-->>push: out.push(ch.to_ascii_lowercase())
    else else
    slug-->>is_empty: out.is_empty()
    alt !prev_dash && !out.is_empty()
    slug-->>push: out.push('-')
    end
    end
    end
    slug-->>ends_with: out.ends_with('-')
    loop out.ends_with('-')
    slug-->>pop: out.pop()
    end
    slug-->>is_empty: out.is_empty()
    alt out.is_empty()
    slug-->>crate__stable_id: crate::stable_id(input)
    slug-->>push_str: out.push_str(&crate::stable_id(input))
    end
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::stable_id`](../stable-id.md)

## References

- calls → [`robby_render`](../index.md) — stable_id
- calls → [`robby_render::report`](../report/index.md) — new, push
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty

