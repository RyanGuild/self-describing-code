# `robby_render::test_web::child_world_rects_lie_inside_parent_world`

[Self-model](../../README.md) · [test_web](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn child_world_rects_lie_inside_parent_world()` |
| location | [`crates/robby-render/tests/web.rs:172`](../../../../crates/robby-render/tests/web.rs#L172) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn child_world_rects_lie_inside_parent_world() {
    let (doc, _) = demo_doc();
    let scene = Scene::from_document(&doc);
    for frame in scene.frames.values() {
        for child_id in &frame.children {
            let child = &scene.frames[child_id];
            assert!(
                frame.world.contains(&child.world),
                "{} ({:?}) not inside {} ({:?})",
                child.stable_id,
                child.world,
                frame.stable_id,
                frame.world
            );
        }
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant child_world_rects_lie_inside_parent_world as child_world_rects_lie_inside_parent_world()
    participant demo_doc as demo_doc
    participant Scene__from_document as Scene::from_document
    participant values as values
    child_world_rects_lie_inside_parent_world-->>demo_doc: demo_doc()
    child_world_rects_lie_inside_parent_world-->>Scene__from_document: Scene::from_document(&doc)
    child_world_rects_lie_inside_parent_world-->>values: scene.frames.values()
    loop scene.frames.values()
    loop &frame.children
    child_world_rects_lie_inside_parent_world-->>child_world_rects_lie_inside_parent_world: assert!( frame.world.contains(&child.world), '{} ({:?}) not inside {}...
    end
    end
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_render::test_web::demo_doc`](demo-doc.md)
- calls → [`robby_render::pipeline::from_document`](../pipeline/from-document.md)

## References

- calls → [`robby_render::pipeline`](../pipeline/index.md) — from_document
- calls → [`robby_render::test_web`](index.md) — demo_doc

