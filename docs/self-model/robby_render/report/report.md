# `robby_render::report::Report`

[Self-model](../../README.md) · [report](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/report.rs:36`](../../../../crates/robby-render/src/report.rs#L36) |
| provenance | extractor · confidence 1.00 |

> Static artifact built from rendered views.

```rust
/// Static artifact built from rendered views.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report {
    pub sections: Vec<Section>,
}
```

**Fields and variants**

- `sections` — `crates/robby-render/src/report.rs:39:5`

