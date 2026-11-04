# `robby_render::tree::rel_link`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn rel_link(from: &Path, to: &Path) -> String` |
| location | [`crates/robby-render/src/tree.rs:760`](../../../../crates/robby-render/src/tree.rs#L760) |
| provenance | extractor · confidence 1.00 |

> Relative link from one page to another. Both are relative to the output root.

```rust
/// Relative link from one page to another. Both are relative to the output root.
fn rel_link(from: &Path, to: &Path) -> String {
    let from_dir: Vec<String> = from
        .parent()
        .map(|parent| {
            parent
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .collect()
        })
        .unwrap_or_default();
    let to_parts: Vec<String> = to
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .collect();

    let mut common = 0;
    while common < from_dir.len()
        && common + 1 < to_parts.len()
        && from_dir[common] == to_parts[common]
    {
        common += 1;
    }

    let mut out = String::new();
    for _ in common..from_dir.len() {
        out.push_str("../");
    }
    out.push_str(&to_parts[common..].join("/"));
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant rel_link as rel_link()
    participant parent as parent
    participant components as components
    participant as_os_str as as_os_str
    participant to_string_lossy as to_string_lossy
    participant to_string as to_string
    participant map as map
    participant collect as collect
    participant unwrap_or_default as unwrap_or_default
    participant len as len
    participant String__new as String::new
    participant push_str as push_str
    participant join as join
    rel_link-->>parent: from .parent()
    rel_link-->>components: parent .components()
    rel_link-->>as_os_str: c.as_os_str()
    rel_link-->>to_string_lossy: c.as_os_str().to_string_lossy()
    rel_link-->>to_string: c.as_os_str().to_string_lossy().to_string()
    rel_link-->>map: parent .components() .map(|c| c.as_os_str().to_string_lossy().to_stri...
    rel_link-->>collect: parent .components() .map(|c| c.as_os_str().to_string_lossy().to_stri...
    rel_link-->>map: from .parent() .map(|parent| { parent .components() .map(|c| c.as_os_...
    rel_link-->>unwrap_or_default: from .parent() .map(|parent| { parent .components() .map(|c| c.as_os_...
    rel_link-->>components: to .components()
    rel_link-->>as_os_str: c.as_os_str()
    rel_link-->>to_string_lossy: c.as_os_str().to_string_lossy()
    rel_link-->>to_string: c.as_os_str().to_string_lossy().to_string()
    rel_link-->>map: to .components() .map(|c| c.as_os_str().to_string_lossy().to_string())
    rel_link-->>collect: to .components() .map(|c| c.as_os_str().to_string_lossy().to_string()...
    rel_link-->>len: from_dir.len()
    rel_link-->>len: to_parts.len()
    rel_link-->>String__new: String::new()
    rel_link-->>len: from_dir.len()
    loop common..from_dir.len()
    rel_link-->>push_str: out.push_str('../')
    end
    rel_link-->>join: to_parts[common..].join('/')
    rel_link-->>push_str: out.push_str(&to_parts[common..].join('/'))
```

22 steps · 0 observed · 22 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::report::impl Report::new`](../report/index.md)

## References

- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

