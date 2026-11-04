# `robby_render::report::Section`

[Self-model](../../README.md) · [report](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/report.rs:9`](../../../../crates/robby-render/src/report.rs#L9) |
| provenance | extractor · confidence 1.00 |

> One titled block inside a report.

```rust
/// One titled block inside a report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: Option<String>,
    pub body: String,
}
```

**Fields and variants**

- `title` — `crates/robby-render/src/report.rs:12:5`
- `body` — `crates/robby-render/src/report.rs:13:5`

