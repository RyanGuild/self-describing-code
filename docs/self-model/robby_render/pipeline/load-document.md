# `robby_render::pipeline::load_document`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn load_document(path: &Path) -> Result<Document>` |
| location | [`crates/robby-render/src/pipeline.rs:49`](../../../../crates/robby-render/src/pipeline.rs#L49) |
| provenance | extractor · confidence 1.00 |

> Read and parse an IR document.

```rust
/// Read and parse an IR document.
pub fn load_document(path: &Path) -> Result<Document> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant load_document as load_document()
    participant fs__read_to_string as fs::read_to_string
    participant Ok as Ok
    load_document-->>fs__read_to_string: fs::read_to_string(path)
    load_document-->>load_document: ? (propagate)
    load_document-->>Ok: Ok(serde_json::from_str(&text)?)
```

3 steps · 0 observed · 3 possible only

