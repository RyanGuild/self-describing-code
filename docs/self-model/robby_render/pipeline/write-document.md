# `robby_render::pipeline::write_document`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn write_document(doc: &Document, out: &Path) -> Result<()>` |
| location | [`crates/robby-render/src/pipeline.rs:117`](../../../../crates/robby-render/src/pipeline.rs#L117) |
| provenance | extractor · confidence 1.00 |

> Write the IR document itself as a committed artifact.

```rust
/// Write the IR document itself as a committed artifact.
pub fn write_document(doc: &Document, out: &Path) -> Result<()> {
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out, serde_json::to_string_pretty(doc)?)?;
    Ok(())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant write_document as write_document()
    participant fs__create_dir_all as fs::create_dir_all
    participant fs__write as fs::write
    participant Ok as Ok
    alt let Some(parent) = out.parent()
    write_document-->>fs__create_dir_all: fs::create_dir_all(parent)
    write_document-->>write_document: ? (propagate)
    end
    write_document-->>fs__write: fs::write(out, serde_json::to_string_pretty(doc)?)
    write_document-->>write_document: ? (propagate)
    write_document-->>Ok: Ok(())
```

5 steps · 0 observed · 5 possible only

