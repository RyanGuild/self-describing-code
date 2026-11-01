# `robby_render::pipeline::render_files`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn render_files(doc: &Document, out: &Path) -> std::io::Result<usize>` |
| location | [`crates/robby-render/src/pipeline.rs:99`](../../../../crates/robby-render/src/pipeline.rs#L99) |
| provenance | extractor · confidence 1.00 |

> Write one `.mmd` file per process and per root graph.

```rust
/// Write one `.mmd` file per process and per root graph.
pub fn render_files(doc: &Document, out: &Path) -> std::io::Result<usize> {
    fs::create_dir_all(out)?;
    let mut count = 0;

    for (id, mermaid) in crate::all_sequences(doc) {
        fs::write(out.join(format!("seq-{}.mmd", slug(id.as_str()))), mermaid)?;
        count += 1;
    }
    for root in &doc.roots {
        let mermaid = crate::flowchart(doc, root);
        fs::write(out.join(format!("graph-{}.mmd", slug(root.as_str()))), mermaid)?;
        count += 1;
    }

    Ok(count)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant render_files as render_files()
    participant fs__create_dir_all as fs::create_dir_all
    participant fs__write as fs::write
    participant crate__flowchart as crate::flowchart
    participant Ok as Ok
    render_files-->>fs__create_dir_all: fs::create_dir_all(out)
    render_files-->>render_files: ? (propagate)
    render_files-->>render_files: 0
    loop crate::all_sequences(doc)
    render_files-->>fs__write: fs::write(out.join(format!('seq-{}.mmd', slug(id.as_str()))), mermaid)
    render_files-->>render_files: ? (propagate)
    render_files-->>render_files: count += 1
    end
    loop &doc.roots
    render_files-->>crate__flowchart: crate::flowchart(doc, root)
    render_files-->>fs__write: fs::write(out.join(format!('graph-{}.mmd', slug(root.as_str()))), mer...
    render_files-->>render_files: ? (propagate)
    render_files-->>render_files: count += 1
    end
    render_files-->>Ok: Ok(count)
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::flowchart`](../flowchart.md)

