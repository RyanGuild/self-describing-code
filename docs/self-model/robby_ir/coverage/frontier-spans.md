# `robby_ir::coverage::frontier_spans`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn frontier_spans(doc: &Document) -> BTreeMap<String, Vec<Span>>` |
| location | [`crates/robby-ir/src/coverage.rs:229`](../../../../crates/robby-ir/src/coverage.rs#L229) |
| provenance | extractor · confidence 1.00 |

> The spans that actually claim source, per file.
> 
> A node is a **frontier** node when no descendant carries a span. Only
> frontier spans count towards coverage: a parent's span is structural, and
> including it would make coverage trivially 100% for any file with a
> top-level item. The finest claim is the claim.

```rust
/// The spans that actually claim source, per file.
///
/// A node is a **frontier** node when no descendant carries a span. Only
/// frontier spans count towards coverage: a parent's span is structural, and
/// including it would make coverage trivially 100% for any file with a
/// top-level item. The finest claim is the claim.
pub fn frontier_spans(doc: &Document) -> BTreeMap<String, Vec<Span>> {
    let mut out: BTreeMap<String, Vec<Span>> = BTreeMap::new();
    for (id, node) in &doc.nodes {
        let Some(span) = &node.span else { continue };
        if has_span_descendant(doc, id) {
            continue;
        }
        out.entry(span.file.clone()).or_default().push(span.clone());
    }
    out
}
```

```mermaid
sequenceDiagram
    autonumber
    participant frontier_spans as frontier_spans()
    participant BTreeMap__new as BTreeMap::new
    participant has_span_descendant as has_span_descendant
    participant clone as clone
    participant entry as entry
    participant or_default as or_default
    participant push as push
    frontier_spans-->>BTreeMap__new: BTreeMap::new()
    loop &doc.nodes
    frontier_spans-->>has_span_descendant: has_span_descendant(doc, id)
    frontier_spans-->>clone: span.file.clone()
    frontier_spans-->>entry: out.entry(span.file.clone())
    frontier_spans-->>or_default: out.entry(span.file.clone()).or_default()
    frontier_spans-->>clone: span.clone()
    frontier_spans-->>push: out.entry(span.file.clone()).or_default().push(span.clone())
    end
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::coverage::has_span_descendant`](has-span-descendant.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_ir::coverage`](index.md) — has_span_descendant
- calls → [`robby_ir::kind`](../kind/index.md) — new
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_ir::test_coverage::frontier_spans_exclude_parents_that_have_span_children`](../test_coverage/frontier-spans-exclude-parents-that-have-span-children.md)

