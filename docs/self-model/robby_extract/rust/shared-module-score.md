# `robby_extract::rust::shared_module_score`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn shared_module_score(a: &str, b: &str) -> usize` |
| location | [`crates/robby-extract/src/rust.rs:951`](../../../../crates/robby-extract/src/rust.rs#L951) |
| provenance | extractor · confidence 1.00 |

```rust
fn shared_module_score(a: &str, b: &str) -> usize {
    let a_parts: Vec<&str> = a.split("::").collect();
    let b_parts: Vec<&str> = b.split("::").collect();
    a_parts
        .iter()
        .zip(b_parts.iter())
        .take_while(|(x, y)| x == y)
        .count()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant shared_module_score as shared_module_score()
    participant split as split
    participant collect as collect
    participant iter as iter
    participant zip as zip
    participant take_while as take_while
    participant count as count
    shared_module_score-->>split: a.split('::')
    shared_module_score-->>collect: a.split('::').collect()
    shared_module_score-->>split: b.split('::')
    shared_module_score-->>collect: b.split('::').collect()
    shared_module_score-->>iter: a_parts .iter()
    shared_module_score-->>iter: b_parts.iter()
    shared_module_score-->>zip: a_parts .iter() .zip(b_parts.iter())
    shared_module_score-->>take_while: a_parts .iter() .zip(b_parts.iter()) .take_while(|(x, y)| x == y)
    shared_module_score-->>count: a_parts .iter() .zip(b_parts.iter()) .take_while(|(x, y)| x == y) .co...
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)

## References

- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_extract::rust::pick_call_target`](pick-call-target.md)

