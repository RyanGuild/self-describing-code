# `robby_render::pipeline::load_sources`

[Self-model](../../README.md) · [pipeline](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn load_sources(doc: &Document, root: &Path) -> BTreeMap<String, String>` |
| location | [`crates/robby-render/src/pipeline.rs:55`](../../../../crates/robby-render/src/pipeline.rs#L55) |
| provenance | extractor · confidence 1.00 |

> Read every source file the document references.
> 
> A missing file is not an error here: the coverage report will show it as
> fully unclaimed, which is the more useful failure.

```rust
/// Read every source file the document references.
///
/// A missing file is not an error here: the coverage report will show it as
/// fully unclaimed, which is the more useful failure.
pub fn load_sources(doc: &Document, root: &Path) -> BTreeMap<String, String> {
    let mut sources = BTreeMap::new();

    // A document that declares its files is authoritative: a declared file that
    // cannot be read is a missing piece of the map, not something to skip.
    let files: Vec<String> = if doc.files.is_empty() {
        coverage::all_spans(doc).keys().cloned().collect()
    } else {
        doc.files.clone()
    };

    for file in files {
        let path = root.join(&file);
        if let Ok(text) = fs::read_to_string(&path) {
            sources.insert(file, text);
        }
    }
    sources
}
```

```mermaid
sequenceDiagram
    autonumber
    participant load_sources as load_sources()
    participant BTreeMap__new as BTreeMap::new
    participant is_empty as is_empty
    participant coverage__all_spans as coverage::all_spans
    participant keys as keys
    participant cloned as cloned
    participant collect as collect
    participant clone as clone
    participant join as join
    participant fs__read_to_string as fs::read_to_string
    participant insert as insert
    load_sources-->>BTreeMap__new: BTreeMap::new()
    load_sources-->>is_empty: doc.files.is_empty()
    alt doc.files.is_empty()
    load_sources-->>coverage__all_spans: coverage::all_spans(doc)
    load_sources-->>keys: coverage::all_spans(doc).keys()
    load_sources-->>cloned: coverage::all_spans(doc).keys().cloned()
    load_sources-->>collect: coverage::all_spans(doc).keys().cloned().collect()
    else else
    load_sources-->>clone: doc.files.clone()
    end
    loop files
    load_sources-->>join: root.join(&file)
    load_sources-->>fs__read_to_string: fs::read_to_string(&path)
    alt let Ok(text) = fs::read_to_string(&path)
    load_sources-->>insert: sources.insert(file, text)
    end
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_render::report::impl Report::new`](../report/index.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_ir::coverage::all_spans`](../../robby_ir/coverage/all-spans.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_ir::model::impl Document::insert`](../../robby_ir/model/index.md)

## References

- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — all_spans
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_render::pipeline::from_document`](from-document.md)

