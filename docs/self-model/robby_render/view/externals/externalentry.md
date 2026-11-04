# `robby_render::view::externals::ExternalEntry`

[Self-model](../../../README.md) · [view::externals](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/view/externals.rs:14`](../../../../../crates/robby-render/src/view/externals.rs#L14) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalEntry {
    pub package: String,
    pub spec: String,
    pub importers: Vec<String>,
}
```

**Fields and variants**

- `package` — `crates/robby-render/src/view/externals.rs:16:5`
- `spec` — `crates/robby-render/src/view/externals.rs:17:5`
- `importers` — `crates/robby-render/src/view/externals.rs:18:5`

