# `robby_render::test_tree::every_internal_link_resolves`

[Self-model](../../README.md) · [test_tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn every_internal_link_resolves()` |
| location | [`crates/robby-render/tests/tree.rs:20`](../../../../crates/robby-render/tests/tree.rs#L20) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn every_internal_link_resolves() {
    let (_, pages) = pages();
    let known: BTreeSet<String> = pages
        .iter()
        .map(|page| page.path.to_string_lossy().replace('\\', "/"))
        .collect();

    let mut broken = Vec::new();
    for page in &pages {
        let dir = page.path.parent().unwrap_or(Path::new(""));
        for link in markdown_links(&page.body) {
            let target = link.split('#').next().unwrap_or("");
            if !target.ends_with(".md") {
                continue;
            }
            let resolved = normalize(&dir.join(target));
            if !known.contains(&resolved) {
                broken.push(format!("{}: {link} -> {resolved}", page.path.display()));
            }
        }
    }

    assert!(broken.is_empty(), "broken links:\n{}", broken.join("\n"));
}
```

```mermaid
sequenceDiagram
    autonumber
    participant every_internal_link_resolves as every_internal_link_resolves()
    participant pages as pages
    participant collect as collect
    participant Vec__new as Vec::new
    participant unwrap_or as unwrap_or
    participant normalize as normalize
    participant push as push
    every_internal_link_resolves-->>pages: pages()
    every_internal_link_resolves-->>collect: pages .iter() .map(|page| page.path.to_string_lossy().replace('\\', '...
    every_internal_link_resolves-->>Vec__new: Vec::new()
    loop &pages
    every_internal_link_resolves-->>unwrap_or: page.path.parent().unwrap_or(Path::new(''))
    loop markdown_links(&page.body)
    every_internal_link_resolves-->>unwrap_or: link.split(' ').next().unwrap_or('')
    alt !target.ends_with('.md')
    every_internal_link_resolves-->>every_internal_link_resolves: continue
    end
    every_internal_link_resolves-->>normalize: normalize(&dir.join(target))
    alt !known.contains(&resolved)
    every_internal_link_resolves-->>push: broken.push(format!('{}: {link} -> {resolved}', page.path.display()))
    end
    end
    end
    every_internal_link_resolves-->>every_internal_link_resolves: assert!(broken.is_empty(), 'broken links:\n{}', broken.join('\n'))
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_render::test_tree::pages`](pages.md)
- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)
- calls → [`robby_render::test_tree::normalize`](normalize.md)

