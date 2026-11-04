# `robby_render::render::markdown::trim_comment`

[Self-model](../../../README.md) · [render::markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn trim_comment(text: &str, max: usize) -> String` |
| location | [`crates/robby-render/src/render/markdown.rs:190`](../../../../../crates/robby-render/src/render/markdown.rs#L190) |
| provenance | extractor · confidence 1.00 |

```rust
fn trim_comment(text: &str, max: usize) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max {
        return format!("\"{flat}\"");
    }
    let truncated: String = flat.chars().take(max.saturating_sub(1)).collect();
    format!("\"{truncated}…\"")
}
```

```mermaid
sequenceDiagram
    autonumber
    participant trim_comment as trim_comment()
    participant split_whitespace as split_whitespace
    participant collect as collect
    participant join as join
    participant chars as chars
    participant count as count
    participant saturating_sub as saturating_sub
    participant take as take
    trim_comment-->>split_whitespace: text.split_whitespace()
    trim_comment-->>collect: text.split_whitespace().collect::<Vec<_>>()
    trim_comment-->>join: text.split_whitespace().collect::<Vec<_>>().join(' ')
    trim_comment-->>chars: flat.chars()
    trim_comment-->>count: flat.chars().count()
    alt flat.chars().count() <= max
    trim_comment-->>trim_comment: format!('\'{flat}\'')
    trim_comment-->>trim_comment: return
    end
    trim_comment-->>chars: flat.chars()
    trim_comment-->>saturating_sub: max.saturating_sub(1)
    trim_comment-->>take: flat.chars().take(max.saturating_sub(1))
    trim_comment-->>collect: flat.chars().take(max.saturating_sub(1)).collect()
    trim_comment-->>trim_comment: format!('\'{truncated}…\'')
```

12 steps · 0 observed · 12 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../../view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../../view/select/collect.md)

## References

- calls → [`robby_render::view::select`](../../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_render::render::markdown::impl Renderer for MarkdownRenderer::comments`](index.md)

