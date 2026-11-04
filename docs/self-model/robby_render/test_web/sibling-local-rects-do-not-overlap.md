# `robby_render::test_web::sibling_local_rects_do_not_overlap`

[Self-model](../../README.md) · [test_web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn sibling_local_rects_do_not_overlap()` |
| location | [`crates/robby-render/tests/web.rs:186`](../../../../crates/robby-render/tests/web.rs#L186) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn sibling_local_rects_do_not_overlap() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        let locals: Vec<_> = frame
            .children
            .iter()
            .filter_map(|id| scene.frames[id].local_in_parent)
            .collect();
        for (i, a) in locals.iter().enumerate() {
            for b in locals.iter().skip(i + 1) {
                let ax2 = a.x + a.w;
                let ay2 = a.y + a.h;
                let bx2 = b.x + b.w;
                let by2 = b.y + b.h;
                let overlap = !(ax2 <= b.x || bx2 <= a.x || ay2 <= b.y || by2 <= a.y);
                assert!(
                    !overlap,
                    "siblings overlap in {} local grid: {:?} vs {:?}",
                    frame.fqn, a, b
                );
            }
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant sibling_local_rects_do_not_overlap as sibling_local_rects_do_not_overlap()
    participant demo_doc as demo_doc
    participant Scene__from_document as Scene::from_document
    participant values as values
    participant iter as iter
    participant filter_map as filter_map
    participant collect as collect
    participant enumerate as enumerate
    participant skip as skip
    sibling_local_rects_do_not_overlap-->>demo_doc: demo_doc()
    sibling_local_rects_do_not_overlap-->>Scene__from_document: Scene::from_document(&doc)
    sibling_local_rects_do_not_overlap-->>values: scene.frames.values()
    loop scene.frames.values()
    sibling_local_rects_do_not_overlap-->>iter: frame .children .iter()
    sibling_local_rects_do_not_overlap-->>filter_map: frame .children .iter() .filter_map(|id| scene.frames[id].local_in_pa...
    sibling_local_rects_do_not_overlap-->>collect: frame .children .iter() .filter_map(|id| scene.frames[id].local_in_pa...
    sibling_local_rects_do_not_overlap-->>iter: locals.iter()
    sibling_local_rects_do_not_overlap-->>enumerate: locals.iter().enumerate()
    loop locals.iter().enumerate()
    sibling_local_rects_do_not_overlap-->>iter: locals.iter()
    sibling_local_rects_do_not_overlap-->>skip: locals.iter().skip(i + 1)
    loop locals.iter().skip(i + 1)
    sibling_local_rects_do_not_overlap-->>sibling_local_rects_do_not_overlap: assert!( !overlap, 'siblings overlap in {} local grid: {:?} vs {:?}',...
    end
    end
    end
```

11 steps · 0 observed · 11 possible only

**Edges**

- calls → [`robby_render::test_web::demo_doc`](demo-doc.md)
- calls → [`robby_render::pipeline::from_document`](../pipeline/from-document.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)

## References

- calls → [`robby_render::pipeline`](../pipeline/index.md) — from_document
- calls → [`robby_render::test_web`](index.md) — demo_doc
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

