# `robby_extract::extract_file`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn extract_file(     doc: &mut Document,     krate: &Krate,     file: &Path,     pending: &mut Vec<PendingCall>, ) -> Result<()>` |
| location | [`crates/robby-extract/src/lib.rs:146`](../../../crates/robby-extract/src/lib.rs#L146) |
| provenance | extractor · confidence 1.00 |

```rust
fn extract_file(
    doc: &mut Document,
    krate: &Krate,
    file: &Path,
    pending: &mut Vec<PendingCall>,
) -> Result<()> {
    let source = fs::read_to_string(file).with_context(|| format!("reading {}", file.display()))?;
    let rel = file
        .strip_prefix(krate.dir.parent().and_then(|p| p.parent()).unwrap_or(&krate.dir))
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/");

    doc.declare_file(rel.clone());

    let ast = syn::parse_file(&source).with_context(|| format!("parsing {rel}"))?;
    let index = LineIndex::new(&source);
    let path = module_path(krate, file);
    let module_id = NodeId::symbol("rust", &path);

    // File-backed modules hang off their crate so the document has one root per
    // crate. A `mod foo;` declaration may already have set a deeper parent; the
    // merge keeps whatever is there.
    let mut module = Node::new(module_id.clone(), kind::code::MODULE, path.clone());
    module.provenance = Provenance::new(Source::Extractor);
    if path != krate.name {
        module.parent = Some(NodeId::symbol("rust", &krate.name));
    }
    doc.merge(module);

    let ctx = Ctx {
        file: &rel,
        source: &source,
        index: &index,
        module: path.clone(),
    };
    walk_items(doc, &ast.items, &module_id, &ctx, pending);
    Ok(())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant extract_file as extract_file()
    participant with_context as with_context
    participant replace as replace
    participant declare_file as declare_file
    participant LineIndex__new as LineIndex::new
    participant module_path as module_path
    participant NodeId__symbol as NodeId::symbol
    participant Node__new as Node::new
    participant merge as merge
    participant walk_items as walk_items
    participant Ok as Ok
    extract_file-->>with_context: fs::read_to_string(file).with_context(|| format!('reading {}', file.d...
    extract_file-->>extract_file: ? (propagate)
    extract_file-->>replace: file .strip_prefix(krate.dir.parent().and_then(|p| p.parent()).unwrap...
    extract_file-->>declare_file: doc.declare_file(rel.clone())
    extract_file-->>with_context: syn::parse_file(&source).with_context(|| format!('parsing {rel}'))
    extract_file-->>extract_file: ? (propagate)
    extract_file-->>LineIndex__new: LineIndex::new(&source)
    extract_file-->>module_path: module_path(krate, file)
    extract_file-->>NodeId__symbol: NodeId::symbol('rust', &path)
    extract_file-->>Node__new: Node::new(module_id.clone(), kind::code::MODULE, path.clone())
    extract_file-->>extract_file: module.provenance = Provenance::new(Source::Extractor)
    alt path != krate.name
    extract_file-->>extract_file: module.parent = Some(NodeId::symbol('rust', &krate.name))
    end
    extract_file-->>merge: doc.merge(module)
    extract_file-->>extract_file: Ctx { file: &rel, source: &source, index: &index, module: path.clone(...
    extract_file-->>walk_items: walk_items(doc, &ast.items, &module_id, &ctx, pending)
    extract_file-->>Ok: Ok(())
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::declare_file`](../robby_ir/model/index.md)
- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_extract::module_path`](module-path.md)
- calls → [`robby_ir::model::impl NodeId::symbol`](../robby_ir/model/index.md)
- calls → [`robby_extract::impl LineIndex::new`](index.md)
- calls → [`robby_ir::model::impl Document::merge`](../robby_ir/model/index.md)
- calls → [`robby_extract::walk_items`](walk-items.md)

