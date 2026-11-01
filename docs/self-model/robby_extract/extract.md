# `robby_extract::extract`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn extract(root: &Path) -> Result<Document>` |
| location | [`crates/robby-extract/src/lib.rs:31`](../../../crates/robby-extract/src/lib.rs#L31) |
| provenance | extractor · confidence 1.00 |

> Extract a document describing every Rust crate under `<root>/crates`.

```rust
/// Extract a document describing every Rust crate under `<root>/crates`.
pub fn extract(root: &Path) -> Result<Document> {
    let mut doc = Document::new("robby-extract");
    let mut pending: Vec<PendingCall> = Vec::new();

    let crates = discover_crates(root)?;
    for krate in &crates {
        let mut crate_node = Node::new(
            NodeId::symbol("rust", &krate.name),
            kind::code::CRATE,
            krate.name.clone(),
        );
        crate_node.provenance = Provenance::new(Source::Extractor);
        doc.merge(crate_node);

        for file in &krate.files {
            extract_file(&mut doc, krate, file, &mut pending)?;
        }
    }

    resolve_calls(&mut doc, pending);
    Ok(doc)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant extract as extract()
    participant Document__new as Document::new
    participant Vec__new as Vec::new
    participant discover_crates as discover_crates
    participant Node__new as Node::new
    participant merge as merge
    participant extract_file as extract_file
    participant resolve_calls as resolve_calls
    participant Ok as Ok
    extract-->>Document__new: Document::new('robby-extract')
    extract-->>Vec__new: Vec::new()
    extract-->>discover_crates: discover_crates(root)
    extract-->>extract: ? (propagate)
    loop &crates
    extract-->>Node__new: Node::new( NodeId::symbol('rust', &krate.name), kind::code::CRATE, kr...
    extract-->>extract: crate_node.provenance = Provenance::new(Source::Extractor)
    extract-->>merge: doc.merge(crate_node)
    loop &krate.files
    extract-->>extract_file: extract_file(&mut doc, krate, file, &mut pending)
    extract-->>extract: ? (propagate)
    end
    end
    extract-->>resolve_calls: resolve_calls(&mut doc, pending)
    extract-->>Ok: Ok(doc)
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_extract::discover_crates`](discover-crates.md)
- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_ir::model::impl Document::merge`](../robby_ir/model/index.md)
- calls → [`robby_extract::extract_file`](extract-file.md)
- calls → [`robby_extract::resolve_calls`](resolve-calls.md)

