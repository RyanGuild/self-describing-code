# `robby_render::tree::source_link`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn source_link(page: &Path, out_rel: &str, span: &robby_ir::Span) -> String` |
| location | [`crates/robby-render/src/tree.rs:792`](../../../../crates/robby-render/src/tree.rs#L792) |
| provenance | extractor · confidence 1.00 |

> A link from a generated page back to the exact source span.

```rust
/// A link from a generated page back to the exact source span.
fn source_link(page: &Path, out_rel: &str, span: &robby_ir::Span) -> String {
    let out_depth = out_rel.split('/').filter(|s| !s.is_empty()).count();
    let page_depth = page.parent().map(|p| p.components().count()).unwrap_or(0);
    let prefix = "../".repeat(out_depth + page_depth);
    format!(
        "[`{}:{}`]({}{}#L{})",
        span.file, span.start.line, prefix, span.file, span.start.line
    )
}
```

```mermaid
sequenceDiagram
    autonumber
    participant source_link as source_link()
    participant split as split
    participant is_empty as is_empty
    participant filter as filter
    participant count as count
    participant parent as parent
    participant components as components
    participant map as map
    participant unwrap_or as unwrap_or
    participant repeat as repeat
    source_link-->>split: out_rel.split('/')
    source_link-->>is_empty: s.is_empty()
    source_link-->>filter: out_rel.split('/').filter(|s| !s.is_empty())
    source_link-->>count: out_rel.split('/').filter(|s| !s.is_empty()).count()
    source_link-->>parent: page.parent()
    source_link-->>components: p.components()
    source_link-->>count: p.components().count()
    source_link-->>map: page.parent().map(|p| p.components().count())
    source_link-->>unwrap_or: page.parent().map(|p| p.components().count()).unwrap_or(0)
    source_link-->>repeat: '../'.repeat(out_depth + page_depth)
    source_link-->>source_link: format!( '[`{}:{}`]({}{} L{})', span.file, span.start.line, prefix, s...
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)

## References

- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty

