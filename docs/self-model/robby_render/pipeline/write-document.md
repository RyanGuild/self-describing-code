# `robby_render::pipeline::write_document`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn write_document(doc: &Document, out: &Path) -> Result<()>` |
| location | [`crates/robby-render/src/pipeline.rs:131`](../../../../crates/robby-render/src/pipeline.rs#L131) |
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
    participant parent as parent
    participant fs__create_dir_all as fs::create_dir_all
    participant serde_json__to_string_pretty as serde_json::to_string_pretty
    participant fs__write as fs::write
    participant Ok as Ok
    write_document-->>parent: out.parent()
    alt let Some(parent) = out.parent()
    write_document-->>fs__create_dir_all: fs::create_dir_all(parent)
    write_document-->>write_document: ? (propagate)
    end
    write_document-->>serde_json__to_string_pretty: serde_json::to_string_pretty(doc)
    write_document-->>write_document: ? (propagate)
    write_document-->>fs__write: fs::write(out, serde_json::to_string_pretty(doc)?)
    write_document-->>write_document: ? (propagate)
    write_document-->>Ok: Ok(())
```

8 steps · 0 observed · 8 possible only

## Referenced by

- calls ← [`robby_cli::build::main`](../../robby_cli/build/main.md)
- calls ← [`robby_cli::enrich_comments`](../../robby_cli/enrich-comments.md)
- calls ← [`robby_cli::example_self_model::run`](../../robby_cli/example_self_model/run.md)

