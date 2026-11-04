# `robby_extract::merge_documents`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn merge_documents(into: &mut Document, from: Document)` |
| location | [`crates/robby-extract/src/lib.rs:103`](../../../crates/robby-extract/src/lib.rs#L103) |
| provenance | extractor · confidence 1.00 |

```rust
fn merge_documents(into: &mut Document, from: Document) {
    for file in from.files {
        into.declare_file(file);
    }
    for node in from.nodes.into_values() {
        into.merge(node);
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant merge_documents as merge_documents()
    participant declare_file as declare_file
    participant into_values as into_values
    participant merge as merge
    loop from.files
    merge_documents-->>declare_file: into.declare_file(file)
    end
    merge_documents-->>into_values: from.nodes.into_values()
    loop from.nodes.into_values()
    merge_documents-->>merge: into.merge(node)
    end
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::declare_file`](../robby_ir/model/index.md)
- calls → [`robby_ir::model::impl Document::merge`](../robby_ir/model/index.md)

## References

- calls → [`robby_ir::model`](../robby_ir/model/index.md) — declare_file, merge

## Referenced by

- calls ← [`robby_extract::extract`](extract.md)
- calls ← [`robby_extract::extract_with`](extract-with.md)

