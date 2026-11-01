# `robby_render::pipeline::remove_markdown`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn remove_markdown(dir: &Path) -> std::io::Result<()>` |
| location | [`crates/robby-render/src/pipeline.rs:158`](../../../../crates/robby-render/src/pipeline.rs#L158) |
| provenance | extractor · confidence 1.00 |

> Delete generated Markdown so removed modules do not linger.

```rust
/// Delete generated Markdown so removed modules do not linger.
fn remove_markdown(dir: &Path) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            remove_markdown(&path)?;
            // Remove directories that became empty.
            let _ = fs::remove_dir(&path);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant remove_markdown as remove_markdown()
    participant Ok as Ok
    participant path as path
    participant fs__remove_dir as fs::remove_dir
    participant fs__remove_file as fs::remove_file
    alt !dir.is_dir()
    remove_markdown-->>Ok: Ok(())
    remove_markdown-->>remove_markdown: return
    end
    loop fs::read_dir(dir)?
    remove_markdown-->>path: entry?.path()
    alt path.is_dir()
    remove_markdown-->>remove_markdown: remove_markdown(&path)
    remove_markdown-->>remove_markdown: ? (propagate)
    remove_markdown-->>fs__remove_dir: fs::remove_dir(&path)
    else else
    alt path.extension().is_some_and(|ext| ext == 'md')
    remove_markdown-->>fs__remove_file: fs::remove_file(&path)
    remove_markdown-->>remove_markdown: ? (propagate)
    end
    end
    end
    remove_markdown-->>Ok: Ok(())
```

9 steps · 0 observed · 9 possible only

**Edges**

- calls → [`robby_render::pipeline::remove_markdown`](remove-markdown.md)

