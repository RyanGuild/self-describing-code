# `robby_render::tree::first_line`

[Self-model](../../README.md) · [tree](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn first_line(text: &str) -> String` |
| location | [`crates/robby-render/src/tree.rs:809`](../../../../crates/robby-render/src/tree.rs#L809) |
| provenance | extractor · confidence 1.00 |

```rust
fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or("").trim().to_owned()
}
```

```mermaid
sequenceDiagram
    autonumber
    participant first_line as first_line()
    participant lines as lines
    participant next as next
    participant unwrap_or as unwrap_or
    participant trim as trim
    participant to_owned as to_owned
    first_line-->>lines: text.lines()
    first_line-->>next: text.lines().next()
    first_line-->>unwrap_or: text.lines().next().unwrap_or('')
    first_line-->>trim: text.lines().next().unwrap_or('').trim()
    first_line-->>to_owned: text.lines().next().unwrap_or('').trim().to_owned()
```

5 steps · 0 observed · 5 possible only

## Referenced by

- calls ← [`robby_render::tree::module_page`](module-page.md)

