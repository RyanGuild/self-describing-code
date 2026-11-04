# `robby_render::pipeline::Loaded`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/pipeline.rs:17`](../../../../crates/robby-render/src/pipeline.rs#L17) |
| provenance | extractor · confidence 1.00 |

> A document plus everything derived from it.
> 
> This is the checked snapshot between extract and render: coverage and
> structural diagnostics are always computed here so write paths share one gate.

```rust
/// A document plus everything derived from it.
///
/// This is the checked snapshot between extract and render: coverage and
/// structural diagnostics are always computed here so write paths share one gate.
pub struct Loaded {
    pub document: Document,
    pub sources: BTreeMap<String, String>,
    pub coverage: CoverageReport,
    pub diagnostics: Vec<Diagnostic>,
    /// Workspace root the source paths are relative to.
    pub root: PathBuf,
}
```

**Fields and variants**

- `document` — `crates/robby-render/src/pipeline.rs:22:5`
- `sources` — `crates/robby-render/src/pipeline.rs:23:5`
- `coverage` — `crates/robby-render/src/pipeline.rs:24:5`
- `diagnostics` — `crates/robby-render/src/pipeline.rs:25:5`
- `root` — `crates/robby-render/src/pipeline.rs:26:5`

