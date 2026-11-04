# `robby_ir::coverage::all_spans`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn all_spans(doc: &Document) -> BTreeMap<String, Vec<Span>>` |
| location | [`crates/robby-ir/src/coverage.rs:209`](../../../../crates/robby-ir/src/coverage.rs#L209) |
| provenance | extractor · confidence 1.00 |

> Every span in the document, per file.
> 
> This is the default claim set: a byte is represented when *some* node owns
> it, at any nesting level. It catches missing items, missing files, and code
> outside any item. It deliberately does **not** try to catch shallow nesting —
> a function span covers its own body, and proving that every statement got a
> node needs the AST. That check lives in the extractor.

```rust
/// Every span in the document, per file.
///
/// This is the default claim set: a byte is represented when *some* node owns
/// it, at any nesting level. It catches missing items, missing files, and code
/// outside any item. It deliberately does **not** try to catch shallow nesting —
/// a function span covers its own body, and proving that every statement got a
/// node needs the AST. That check lives in the extractor.
pub fn all_spans(doc: &Document) -> BTreeMap<String, Vec<Span>> {
    let mut out: BTreeMap<String, Vec<Span>> = BTreeMap::new();
    for node in doc.nodes.values() {
        if let Some(span) = &node.span {
            out.entry(span.file.clone()).or_default().push(span.clone());
        }
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant all_spans as all_spans()
    participant BTreeMap__new as BTreeMap::new
    participant values as values
    participant clone as clone
    participant entry as entry
    participant or_default as or_default
    participant push as push
    all_spans-->>BTreeMap__new: BTreeMap::new()
    all_spans-->>values: doc.nodes.values()
    loop doc.nodes.values()
    alt let Some(span) = &node.span
    all_spans-->>clone: span.file.clone()
    all_spans-->>entry: out.entry(span.file.clone())
    all_spans-->>or_default: out.entry(span.file.clone()).or_default()
    all_spans-->>clone: span.clone()
    all_spans-->>push: out.entry(span.file.clone()).or_default().push(span.clone())
    end
    end
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_ir::kind`](../kind/index.md) — new
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_ir::coverage::document_coverage`](document-coverage.md)
- calls ← [`robby_render::pipeline::load_sources`](../../robby_render/pipeline/load-sources.md)

