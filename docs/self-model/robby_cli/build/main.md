# `robby_cli::build::main`

[Self-model](../../README.md) · [build](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn main()` |
| location | [`crates/robby-cli/build.rs:19`](../../../../crates/robby-cli/build.rs#L19) |
| provenance | extractor · confidence 1.00 |

```rust
fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest.join("../..");

    println!("cargo:rerun-if-changed={}", manifest.join("build.rs").display());

    let document = robby_extract::extract(&root).unwrap_or_else(|err| {
        panic!("robby: extraction failed: {err:#}");
    });
    let loaded = pipeline::from_document(document, &root);

    // Any change to a mapped source file must regenerate the artifacts.
    for file in &loaded.document.files {
        println!("cargo:rerun-if-changed={}", root.join(file).display());
    }

    if !loaded.is_ok() {
        for diagnostic in loaded
            .diagnostics
            .iter()
            .filter(|d| d.severity == robby_ir::Severity::Error)
        {
            eprintln!("robby: [{}] {}", diagnostic.code, diagnostic.message);
        }
        for (_file, gap) in loaded.coverage.failures() {
            eprintln!(
                "robby: unclaimed code at {}  {:?}",
                gap.span.location(),
                gap.preview
            );
        }
        panic!("robby: coverage invariant violated — {}", loaded.summary());
    }

    pipeline::write_document(&loaded.document, &root.join(".robby/robby.ir.json"))
        .unwrap_or_else(|err| panic!("robby: cannot write IR: {err:#}"));
    pipeline::render_files(&loaded.document, &root.join(".robby/diagrams"))
        .unwrap_or_else(|err| panic!("robby: cannot render diagrams: {err:#}"));
    let pages = pipeline::write_report(&loaded, &root.join("docs/self-model"))
        .unwrap_or_else(|err| panic!("robby: cannot write report: {err:#}"));
    let _ = pages;
}
```

```mermaid
sequenceDiagram
    autonumber
    participant main as main()
    participant PathBuf__from as PathBuf::from
    participant join as join
    participant unwrap_or_else as unwrap_or_else
    participant pipeline__from_document as pipeline::from_document
    main-->>PathBuf__from: PathBuf::from(env::var('CARGO_MANIFEST_DIR').expect('CARGO_MANIFEST_D...
    main-->>join: manifest.join('../..')
    main-->>main: println!('cargo:rerun-if-changed={}', manifest.join('build.rs').displ...
    main-->>unwrap_or_else: robby_extract::extract(&root).unwrap_or_else(|err| { panic!('robby: e...
    main-->>pipeline__from_document: pipeline::from_document(document, &root)
    loop &loaded.document.files
    main-->>main: println!('cargo:rerun-if-changed={}', root.join(file).display())
    end
    alt !loaded.is_ok()
    loop loaded .diagnostics .iter() .filter(|d| d.severity == robby_ir::Sever...
    main-->>main: eprintln!('robby: [{}] {}', diagnostic.code, diagnostic.message)
    end
    loop loaded.coverage.failures()
    main-->>main: eprintln!( 'robby: unclaimed code at {} {:?}', gap.span.location(), g...
    end
    main-->>main: panic!('robby: coverage invariant violated — {}', loaded.summary())
    end
    main-->>unwrap_or_else: pipeline::write_document(&loaded.document, &root.join('.robby/robby.i...
    main-->>unwrap_or_else: pipeline::render_files(&loaded.document, &root.join('.robby/diagrams'...
    main-->>unwrap_or_else: pipeline::write_report(&loaded, &root.join('docs/self-model')) .unwra...
    main-->>main: pages
```

13 steps · 0 observed · 13 possible only

**Edges**

- calls → [`robby_ir::kind::impl From<&str> for Kind::from`](../../robby_ir/kind/index.md)
- calls → [`robby_render::pipeline::from_document`](../../robby_render/pipeline/from-document.md)

