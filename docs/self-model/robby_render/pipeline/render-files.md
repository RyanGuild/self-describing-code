# `robby_render::pipeline::render_files`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn render_files(doc: &Document, out: &Path) -> std::io::Result<usize>` |
| location | [`crates/robby-render/src/pipeline.rs:102`](../../../../crates/robby-render/src/pipeline.rs#L102) |
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
    participant crate__all_sequences as crate::all_sequences
    participant join as join
    participant fs__write as fs::write
    participant crate__flowchart as crate::flowchart
    participant Ok as Ok
    render_files-->>fs__create_dir_all: fs::create_dir_all(out)
    render_files-->>render_files: ? (propagate)
    render_files-->>crate__all_sequences: crate::all_sequences(doc)
    loop crate::all_sequences(doc)
    render_files-->>render_files: format!('seq-{}.mmd', slug(id.as_str()))
    render_files-->>join: out.join(format!('seq-{}.mmd', slug(id.as_str())))
    render_files-->>fs__write: fs::write(out.join(format!('seq-{}.mmd', slug(id.as_str()))), mermaid)
    render_files-->>render_files: ? (propagate)
    end
    loop &doc.roots
    render_files-->>crate__flowchart: crate::flowchart(doc, root)
    render_files-->>render_files: format!('graph-{}.mmd', slug(root.as_str()))
    render_files-->>join: out.join(format!('graph-{}.mmd', slug(root.as_str())))
    render_files-->>fs__write: fs::write(out.join(format!('graph-{}.mmd', slug(root.as_str()))), mer...
    render_files-->>render_files: ? (propagate)
    end
    render_files-->>Ok: Ok(count)
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_render::all_sequences`](../all-sequences.md)
- calls → [`robby_render::flowchart`](../flowchart.md)

## References

- calls → [`robby_render`](../index.md) — all_sequences, flowchart

## Referenced by

- calls ← [`robby_cli::build::main`](../../robby_cli/build/main.md)
- calls ← [`robby_cli::example_self_model::run`](../../robby_cli/example_self_model/run.md)
- calls ← [`robby_cli::run`](../../robby_cli/run.md)

