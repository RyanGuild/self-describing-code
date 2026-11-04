# `robby_ir::coverage::preview`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn preview(source: &str, start: u32, end: u32) -> String` |
| location | [`crates/robby-ir/src/coverage.rs:287`](../../../../crates/robby-ir/src/coverage.rs#L287) |
| provenance | extractor · confidence 1.00 |

```rust
fn preview(source: &str, start: u32, end: u32) -> String {
    let slice = &source[start as usize..end as usize];
    let collapsed: String = slice.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.chars().count() > 120 {
        let truncated: String = collapsed.chars().take(117).collect();
        format!("{truncated}...")
    } else {
        collapsed
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant preview as preview()
    participant split_whitespace as split_whitespace
    participant collect as collect
    participant join as join
    participant chars as chars
    participant count as count
    participant take as take
    preview-->>split_whitespace: slice.split_whitespace()
    preview-->>collect: slice.split_whitespace().collect::<Vec<_>>()
    preview-->>join: slice.split_whitespace().collect::<Vec<_>>().join(' ')
    preview-->>chars: collapsed.chars()
    preview-->>count: collapsed.chars().count()
    alt collapsed.chars().count() > 120
    preview-->>chars: collapsed.chars()
    preview-->>take: collapsed.chars().take(117)
    preview-->>collect: collapsed.chars().take(117).collect()
    preview-->>preview: format!('{truncated}...')
    end
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_ir::coverage::make_gap`](make-gap.md)

