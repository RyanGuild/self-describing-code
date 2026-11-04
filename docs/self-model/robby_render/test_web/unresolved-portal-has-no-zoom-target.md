# `robby_render::test_web::unresolved_portal_has_no_zoom_target`

[Self-model](../../README.md) · [test_web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn unresolved_portal_has_no_zoom_target()` |
| location | [`crates/robby-render/tests/web.rs:259`](../../../../crates/robby-render/tests/web.rs#L259) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn unresolved_portal_has_no_zoom_target() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    let b = scene
        .frames
        .values()
        .find(|f| f.id == "rust://demo::right::b")
        .expect("function b frame");
    let unresolved: Vec<_> = b.portals.iter().filter(|p| p.target.is_none()).collect();
    assert!(
        !unresolved.is_empty(),
        "expected unresolved portal on b, got {:?}",
        b.portals
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant unresolved_portal_has_no_zoom_target as unresolved_portal_has_no_zoom_target()
    participant demo_doc as demo_doc
    participant Scene__from_document as Scene::from_document
    participant values as values
    participant find as find
    participant expect as expect
    participant iter as iter
    participant is_none as is_none
    participant filter as filter
    participant collect as collect
    unresolved_portal_has_no_zoom_target-->>demo_doc: demo_doc()
    unresolved_portal_has_no_zoom_target-->>Scene__from_document: Scene::from_document(&doc)
    unresolved_portal_has_no_zoom_target-->>values: scene .frames .values()
    unresolved_portal_has_no_zoom_target-->>find: scene .frames .values() .find(|f| f.id == 'rust://demo::right::b')
    unresolved_portal_has_no_zoom_target-->>expect: scene .frames .values() .find(|f| f.id == 'rust://demo::right::b') .e...
    unresolved_portal_has_no_zoom_target-->>iter: b.portals.iter()
    unresolved_portal_has_no_zoom_target-->>is_none: p.target.is_none()
    unresolved_portal_has_no_zoom_target-->>filter: b.portals.iter().filter(|p| p.target.is_none())
    unresolved_portal_has_no_zoom_target-->>collect: b.portals.iter().filter(|p| p.target.is_none()).collect()
    unresolved_portal_has_no_zoom_target-->>unresolved_portal_has_no_zoom_target: assert!( !unresolved.is_empty(), 'expected unresolved portal on b, go...
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_render::test_web::demo_doc`](demo-doc.md)
- calls → [`robby_render::pipeline::from_document`](../pipeline/from-document.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)

## References

- calls → [`robby_render::pipeline`](../pipeline/index.md) — from_document
- calls → [`robby_render::test_web`](index.md) — demo_doc
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

