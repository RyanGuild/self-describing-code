# `robby_render::render::markdown::cell`

[Self-model](../../../README.md) · [render::markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn cell(input: &str) -> String` |
| location | [`crates/robby-render/src/render/markdown.rs:186`](../../../../../crates/robby-render/src/render/markdown.rs#L186) |
| provenance | extractor · confidence 1.00 |

```rust
fn cell(input: &str) -> String {
    input.replace('|', "\\|").replace('\n', " ")
}
```

```mermaid
sequenceDiagram
    autonumber
    participant cell as cell()
    participant replace as replace
    cell-->>replace: input.replace('|', '\\|')
    cell-->>replace: input.replace('|', '\\|').replace('\n', ' ')
```

2 steps · 0 observed · 2 possible only

