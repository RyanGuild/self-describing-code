# `robby_render::pipeline::write_report`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn write_report(loaded: &Loaded, out_dir: &Path) -> std::io::Result<usize>` |
| location | [`crates/robby-render/src/pipeline.rs:135`](../../../../crates/robby-render/src/pipeline.rs#L135) |
| provenance | extractor · confidence 1.00 |

> Render the self-model as a linked tree of Markdown pages.
> 
> Returns the number of pages written. Stale pages are removed first so a
> deleted module cannot leave a ghost page behind.

```rust
/// Render the self-model as a linked tree of Markdown pages.
///
/// Returns the number of pages written. Stale pages are removed first so a
/// deleted module cannot leave a ghost page behind.
pub fn write_report(loaded: &Loaded, out_dir: &Path) -> std::io::Result<usize> {
    let out_rel = out_dir
        .strip_prefix(&loaded.root)
        .unwrap_or(out_dir)
        .to_string_lossy()
        .replace('\\', "/");

    let pages = crate::tree::tree(
        &loaded.document,
        Some(&loaded.coverage),
        Some(&loaded.sources),
        &out_rel,
    );

    fs::create_dir_all(out_dir)?;
    remove_markdown(out_dir)?;

    for page in &pages {
        let path = out_dir.join(&page.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, &page.body)?;
    }

    Ok(pages.len())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant write_report as write_report()
    participant strip_prefix as strip_prefix
    participant unwrap_or as unwrap_or
    participant to_string_lossy as to_string_lossy
    participant replace as replace
    participant Some as Some
    participant crate__tree__tree as crate::tree::tree
    participant fs__create_dir_all as fs::create_dir_all
    participant remove_markdown as remove_markdown
    participant join as join
    participant parent as parent
    participant fs__write as fs::write
    participant len as len
    participant Ok as Ok
    write_report-->>strip_prefix: out_dir .strip_prefix(&loaded.root)
    write_report-->>unwrap_or: out_dir .strip_prefix(&loaded.root) .unwrap_or(out_dir)
    write_report-->>to_string_lossy: out_dir .strip_prefix(&loaded.root) .unwrap_or(out_dir) .to_string_lo...
    write_report-->>replace: out_dir .strip_prefix(&loaded.root) .unwrap_or(out_dir) .to_string_lo...
    write_report-->>Some: Some(&loaded.coverage)
    write_report-->>Some: Some(&loaded.sources)
    write_report-->>crate__tree__tree: crate::tree::tree( &loaded.document, Some(&loaded.coverage), Some(&lo...
    write_report-->>fs__create_dir_all: fs::create_dir_all(out_dir)
    write_report-->>write_report: ? (propagate)
    write_report-->>remove_markdown: remove_markdown(out_dir)
    write_report-->>write_report: ? (propagate)
    loop &pages
    write_report-->>join: out_dir.join(&page.path)
    write_report-->>parent: path.parent()
    alt let Some(parent) = path.parent()
    write_report-->>fs__create_dir_all: fs::create_dir_all(parent)
    write_report-->>write_report: ? (propagate)
    end
    write_report-->>fs__write: fs::write(&path, &page.body)
    write_report-->>write_report: ? (propagate)
    end
    write_report-->>len: pages.len()
    write_report-->>Ok: Ok(pages.len())
```

19 steps · 0 observed · 19 possible only

**Edges**

- calls → [`robby_render::tui::tree`](../tui/tree.md)
- calls → [`robby_render::pipeline::remove_markdown`](remove-markdown.md)

## References

- calls → [`robby_render::pipeline`](index.md) — remove_markdown
- calls → [`robby_render::tui`](../tui/index.md) — tree

## Referenced by

- calls ← [`robby_cli::build::main`](../../robby_cli/build/main.md)
- calls ← [`robby_cli::example_self_model::run`](../../robby_cli/example_self_model/run.md)
- calls ← [`robby_cli::report`](../../robby_cli/report.md)

