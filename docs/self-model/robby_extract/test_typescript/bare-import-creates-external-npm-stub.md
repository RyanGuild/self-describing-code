# `robby_extract::test_typescript::bare_import_creates_external_npm_stub`

[Self-model](../../README.md) · [test_typescript](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn bare_import_creates_external_npm_stub()` |
| location | [`crates/robby-extract/tests/typescript.rs:83`](../../../../crates/robby-extract/tests/typescript.rs#L83) |
| provenance | extractor · confidence 1.00 |

```rust
#[test]
fn bare_import_creates_external_npm_stub() {
    let doc = robby_extract::extract(&fixture()).expect("extract");

    let stub = doc
        .nodes
        .values()
        .find(|n| n.fqn == "npm/demo-ext")
        .expect("npm/demo-ext stub");
    assert_eq!(stub.kind.as_str(), kind::code::PACKAGE);
    assert_eq!(stub.lang, Lang::TypeScript);
    assert!(stub.tags.iter().any(|t| t == "external"));
    assert_eq!(
        stub.annotations
            .get("external")
            .map(std::string::String::as_str),
        Some("true")
    );

    let bare = doc
        .nodes
        .values()
        .find(|n| n.kind.as_str() == kind::code::IMPORT && n.fqn.contains("import:demo-ext"))
        .expect("bare import");
    assert!(
        bare.edges
            .iter()
            .any(|e| { e.kind == EdgeKind::Imports && e.to == stub.id }),
        "bare import should edge to stub: {:?}",
        bare.edges
    );
}
```

```mermaid
sequenceDiagram
    autonumber
    participant bare_import_creates_external_npm_stub as bare_import_creates_external_npm_stub()
    participant fixture as fixture
    participant robby_extract__extract as robby_extract::extract
    participant expect as expect
    participant values as values
    participant find as find
    participant as_str as as_str
    participant contains as contains
    bare_import_creates_external_npm_stub-->>fixture: fixture()
    bare_import_creates_external_npm_stub-->>robby_extract__extract: robby_extract::extract(&fixture())
    bare_import_creates_external_npm_stub-->>expect: robby_extract::extract(&fixture()).expect('extract')
    bare_import_creates_external_npm_stub-->>values: doc .nodes .values()
    bare_import_creates_external_npm_stub-->>find: doc .nodes .values() .find(|n| n.fqn == 'npm/demo-ext')
    bare_import_creates_external_npm_stub-->>expect: doc .nodes .values() .find(|n| n.fqn == 'npm/demo-ext') .expect('npm/...
    bare_import_creates_external_npm_stub-->>bare_import_creates_external_npm_stub: assert_eq!(stub.kind.as_str(), kind::code::PACKAGE)
    bare_import_creates_external_npm_stub-->>bare_import_creates_external_npm_stub: assert_eq!(stub.lang, Lang::TypeScript)
    bare_import_creates_external_npm_stub-->>bare_import_creates_external_npm_stub: assert!(stub.tags.iter().any(|t| t == 'external'))
    bare_import_creates_external_npm_stub-->>bare_import_creates_external_npm_stub: assert_eq!( stub.annotations .get('external') .map(std::string::Strin...
    bare_import_creates_external_npm_stub-->>values: doc .nodes .values()
    bare_import_creates_external_npm_stub-->>as_str: n.kind.as_str()
    bare_import_creates_external_npm_stub-->>contains: n.fqn.contains('import:demo-ext')
    bare_import_creates_external_npm_stub-->>find: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::IMPORT ...
    bare_import_creates_external_npm_stub-->>expect: doc .nodes .values() .find(|n| n.kind.as_str() == kind::code::IMPORT ...
    bare_import_creates_external_npm_stub-->>bare_import_creates_external_npm_stub: assert!( bare.edges .iter() .any(|e| { e.kind == EdgeKind::Imports &&...
```

16 steps · 0 observed · 16 possible only

**Edges**

- calls → [`robby_extract::test_typescript::fixture`](fixture.md)
- calls → [`robby_extract::extract`](../extract.md)
- calls → [`robby_ir::kind::impl Kind::as_str`](../../robby_ir/kind/index.md)
- calls → [`robby_render::view::scene::impl WorldRect::contains`](../../robby_render/view/scene/index.md)

## References

- calls → [`robby_extract`](../index.md) — extract
- calls → [`robby_extract::test_typescript`](index.md) — fixture
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str
- calls → [`robby_render::view::scene`](../../robby_render/view/scene/index.md) — contains

