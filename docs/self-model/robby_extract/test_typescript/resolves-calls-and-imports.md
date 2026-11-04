# `robby_extract::test_typescript::resolves_calls_and_imports`

[Self-model](../../README.md) · [test_typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn resolves_calls_and_imports()` |
| location | [`crates/robby-extract/tests/typescript.rs:37`](../../../../crates/robby-extract/tests/typescript.rs#L37) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn resolves_calls_and_imports() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let draw = doc
        .nodes
        .values()
        .find(|n| n.fqn.ends_with("/Point/draw") || n.fqn.ends_with("Point/draw"))
        .expect("Point.draw");
    assert!(
        draw.edges.iter().any(|e| e.kind == EdgeKind::Calls),
        "draw should call helper: {:?}",
        draw.edges
    );

    let shapes = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::MODULE && n.fqn.contains("shapes"))
        .expect("shapes module");
    let import = doc
        .nodes
        .values()
        .find(|n| {
            n.kind.as_str() == kind::code::IMPORT
                && n.parent.as_ref() == Some(&shapes.id)
                && n.fqn.contains("import:./util")
        })
        .expect("relative import node");
    assert!(
        import.edges.iter().any(|e| e.kind == EdgeKind::Imports),
        "import should resolve: {:?}",
        import.edges
    );

    let point = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::CLASS && n.name == "Point")
        .expect("Point");
    assert!(
        point
            .edges
            .iter()
            .any(|e| e.kind == EdgeKind::Implements),
        "Point implements Drawable"
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant resolves_calls_and_imports as resolves_calls_and_imports()
    participant fixture as fixture
    participant robby_extract__extract as robby_extract::extract
    participant expect as expect
    participant values as values
    participant ends_with as ends_with
    participant find as find
    participant as_str as as_str
    participant contains as contains
    participant as_ref as as_ref
    participant Some as Some
    resolves_calls_and_imports-->>fixture: fixture()
    resolves_calls_and_imports-->>robby_extract__extract: robby_extract::extract(&fixture())
    resolves_calls_and_imports-->>expect: robby_extract::extract(&fixture()).expect('extract')
    resolves_calls_and_imports-->>values: doc .nodes .values()
    resolves_calls_and_imports-->>ends_with: n.fqn.ends_with('/Point/draw')
    resolves_calls_and_imports-->>ends_with: n.fqn.ends_with('Point/draw')
    resolves_calls_and_imports-->>find: doc .nodes .values() .find(|n| n.fqn.ends_with('/Point/draw') || n.fq...
    resolves_calls_and_imports-->>expect: doc .nodes .values() .find(|n| n.fqn.ends_with('/Point/draw') || n.fq...
    resolves_calls_and_imports-->>resolves_calls_and_imports: assert!( draw.edges.iter().any(|e| e.kind == EdgeKind::Calls), 'draw ...
    resolves_calls_and_imports-->>values: doc .nodes .values()
    resolves_calls_and_imports-->>as_str: n.kind.as_str()
    resolves_calls_and_imports-->>contains: n.fqn.contains('shapes')
    resolves_calls_and_imports-->>find: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::MODULE ...
    resolves_calls_and_imports-->>expect: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::MODULE ...
    resolves_calls_and_imports-->>values: doc .nodes .values()
    resolves_calls_and_imports-->>as_str: n.kind.as_str()
    resolves_calls_and_imports-->>as_ref: n.parent.as_ref()
    resolves_calls_and_imports-->>Some: Some(&shapes.id)
    resolves_calls_and_imports-->>contains: n.fqn.contains('import:./util')
    resolves_calls_and_imports-->>find: doc .nodes .values() .find(|n| { n.kind.as_str() == kind::code::IMPOR...
    resolves_calls_and_imports-->>expect: doc .nodes .values() .find(|n| { n.kind.as_str() == kind::code::IMPOR...
    resolves_calls_and_imports-->>resolves_calls_and_imports: assert!( import.edges.iter().any(|e| e.kind == EdgeKind::Imports), 'i...
    resolves_calls_and_imports-->>values: doc .nodes .values()
    resolves_calls_and_imports-->>as_str: n.kind.as_str()
    resolves_calls_and_imports-->>find: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::CLASS &...
    resolves_calls_and_imports-->>expect: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::CLASS &...
    resolves_calls_and_imports-->>resolves_calls_and_imports: assert!( point .edges .iter() .any(|e| e.kind == EdgeKind::Implements...
```

27 steps · 0 observed · 27 possible only

**Edges**

- calls → [`robby_extract::test_typescript::fixture`](fixture.md)
- calls → [`robby_extract::extract`](../extract.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)

## References

- calls → [`robby_extract`](../index.md) — extract
- calls → [`robby_extract::test_typescript`](index.md) — fixture
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_render::view::scene`](../../robby_render/view/scene/index.md) — contains

