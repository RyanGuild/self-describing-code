# `robby_render::test_tree::markdown_links`

[Self-model](../../README.md) · [test_tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn markdown_links(body: &str) -> Vec<String>` |
| location | [`crates/robby-render/tests/tree.rs:98`](../../../../crates/robby-render/tests/tree.rs#L98) |
| provenance | extractor · confidence 1.00 |

```rust
fn markdown_links(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut fenced = false;

    for line in body.lines() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            match after.find(')') {
                Some(end) => {
                    links.push(after[..end].to_owned());
                    rest = &after[end + 1..];
                }
                None => break,
            }
        }
    }

    links
}
```

```mermaid
sequenceDiagram
    autonumber
    participant markdown_links as markdown_links()
    participant Vec__new as Vec::new
    participant push as push
    markdown_links-->>Vec__new: Vec::new()
    markdown_links-->>markdown_links: false
    loop body.lines()
    alt line.trim_start().starts_with('```')
    markdown_links-->>markdown_links: fenced = !fenced
    markdown_links-->>markdown_links: continue
    end
    alt fenced
    markdown_links-->>markdown_links: continue
    end
    markdown_links-->>markdown_links: line
    loop let Some(start) = rest.find('](')
    markdown_links-->>markdown_links: &rest[start + 2..]
    alt match after.find(')')
    markdown_links-->>push: links.push(after[..end].to_owned())
    markdown_links-->>markdown_links: rest = &after[end + 1..]
    else None
    markdown_links-->>markdown_links: break
    end
    end
    end
    markdown_links-->>markdown_links: links
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

