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
    participant iter as iter
    participant to_string_lossy as to_string_lossy
    participant replace as replace
    participant map as map
    participant collect as collect
    participant Vec__new as Vec::new
    participant parent as parent
    participant Path__new as Path::new
    participant unwrap_or as unwrap_or
    participant markdown_links as markdown_links
    participant split as split
    participant next as next
    participant ends_with as ends_with
    participant join as join
    participant normalize as normalize
    participant contains as contains
    participant push as push
    every_internal_link_resolves-->>pages: pages()
    every_internal_link_resolves-->>iter: pages .iter()
    every_internal_link_resolves-->>to_string_lossy: page.path.to_string_lossy()
    every_internal_link_resolves-->>replace: page.path.to_string_lossy().replace('\\', '/')
    every_internal_link_resolves-->>map: pages .iter() .map(|page| page.path.to_string_lossy().replace('\\', '...
    every_internal_link_resolves-->>collect: pages .iter() .map(|page| page.path.to_string_lossy().replace('\\', '...
    every_internal_link_resolves-->>Vec__new: Vec::new()
    loop &pages
    every_internal_link_resolves-->>parent: page.path.parent()
    every_internal_link_resolves-->>Path__new: Path::new('')
    every_internal_link_resolves-->>unwrap_or: page.path.parent().unwrap_or(Path::new(''))
    every_internal_link_resolves-->>markdown_links: markdown_links(&page.body)
    loop markdown_links(&page.body)
    every_internal_link_resolves-->>split: link.split(' ')
    every_internal_link_resolves-->>next: link.split(' ').next()
    every_internal_link_resolves-->>unwrap_or: link.split(' ').next().unwrap_or('')
    every_internal_link_resolves-->>ends_with: target.ends_with('.md')
    every_internal_link_resolves-->>join: dir.join(target)
    every_internal_link_resolves-->>normalize: normalize(&dir.join(target))
    every_internal_link_resolves-->>contains: known.contains(&resolved)
    alt !known.contains(&resolved)
    every_internal_link_resolves-->>every_internal_link_resolves: format!('{}: {link} -> {resolved}', page.path.display())
    every_internal_link_resolves-->>push: broken.push(format!('{}: {link} -> {resolved}', page.path.display()))
    end
    end
    end
    every_internal_link_resolves-->>every_internal_link_resolves: assert!(broken.is_empty(), 'broken links:\n{}', broken.join('\n'))
```

21 steps · 0 observed · 21 possible only

**Edges**

- calls → [`robby_render::test_tree::pages`](pages.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::test_tree::markdown_links`](markdown-links.md)
- calls → [`robby_render::test_tree::normalize`](normalize.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../view/scene/index.md)
- calls → [`robby_render::report::impl Report::push`](../report/index.md)

## References

- calls → [`robby_render::report`](../report/index.md) — new, push
- calls → [`robby_render::test_tree`](index.md) — markdown_links, normalize, pages
- calls → [`robby_render::view::scene`](../view/scene/index.md) — contains
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

