# `robby_cli::build`

[Self-model](../../README.md) · [robby_cli](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 5 |

## Entry

```mermaid
sequenceDiagram
    autonumber
    participant main as main()
    participant env__var as env::var
    participant expect as expect
    participant PathBuf__from as PathBuf::from
    participant join as join
    participant robby_extract__extract as robby_extract::extract
    participant unwrap_or_else as unwrap_or_else
    participant pipeline__from_document as pipeline::from_document
    participant is_ok as is_ok
    participant iter as iter
    participant filter as filter
    participant failures as failures
    participant pipeline__write_document as pipeline::write_document
    participant pipeline__render_files as pipeline::render_files
    participant pipeline__write_report as pipeline::write_report
    main-->>env__var: env::var('CARGO_MANIFEST_DIR')
    main-->>expect: env::var('CARGO_MANIFEST_DIR').expect('CARGO_MANIFEST_DIR')
    main-->>PathBuf__from: PathBuf::from(env::var('CARGO_MANIFEST_DIR').expect('CARGO_MANIFEST_D...
    main-->>join: manifest.join('../..')
    main-->>main: println!( 'cargo:rerun-if-changed={}', manifest.join('build.rs').disp...
    main-->>robby_extract__extract: robby_extract::extract(&root)
    main-->>main: panic!('robby: extraction failed: {err: }')
    main-->>unwrap_or_else: robby_extract::extract(&root).unwrap_or_else(|err| { panic!('robby: e...
    main-->>pipeline__from_document: pipeline::from_document(document, &root)
    loop &loaded.document.files
    main-->>main: println!('cargo:rerun-if-changed={}', root.join(file).display())
    end
    main-->>is_ok: loaded.is_ok()
    alt !loaded.is_ok()
    main-->>iter: loaded .diagnostics .iter()
    main-->>filter: loaded .diagnostics .iter() .filter(|d| d.severity == robby_ir::Sever...
    loop loaded .diagnostics .iter() .filter(|d| d.severity == robby_ir::Sever...
    main-->>main: eprintln!('robby: [{}] {}', diagnostic.code, diagnostic.message)
    end
    main-->>failures: loaded.coverage.failures()
    loop loaded.coverage.failures()
    main-->>main: eprintln!( 'robby: unclaimed code at {} {:?}', gap.span.location(), g...
    end
    main-->>main: panic!('robby: coverage invariant violated — {}', loaded.summary())
    end
    main-->>join: root.join('.robby/robby.ir.json')
    main-->>pipeline__write_document: pipeline::write_document(&loaded.document, &root.join('.robby/robby.i...
    main-->>main: panic!('robby: cannot write IR: {err: }')
    main-->>unwrap_or_else: pipeline::write_document(&loaded.document, &root.join('.robby/robby.i...
    main-->>join: root.join('.robby/diagrams')
    main-->>pipeline__render_files: pipeline::render_files(&loaded.document, &root.join('.robby/diagrams'))
    main-->>main: panic!('robby: cannot render diagrams: {err: }')
    main-->>unwrap_or_else: pipeline::render_files(&loaded.document, &root.join('.robby/diagrams'...
    main-->>join: root.join('docs/self-model')
    main-->>pipeline__write_report: pipeline::write_report(&loaded, &root.join('docs/self-model'))
    main-->>main: panic!('robby: cannot write report: {err: }')
    main-->>unwrap_or_else: pipeline::write_report(&loaded, &root.join('docs/self-model')) .unwra...
```

## Package diagram

```mermaid
flowchart TB
    n_874b29800ed524d0["build"]
    n_a331a60ee1568354(["robby_cli"])
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_874b29800ed524d0 --> n_356bf2c23da68961
    n_874b29800ed524d0 --> n_9c636b098ca4450e
    n_874b29800ed524d0 --> n_a331a60ee1568354
```

## Dependency matrix

| | build | robby_ir | robby_render | robby_cli |
|---|---:|---:|---:|---:|
| **build** | · | 2 | 5 | 1 |
| **robby_ir** | | · | | |
| **robby_render** | | | · | |
| **robby_cli** | | | | · |
## References

- calls → [`robby_cli`](../index.md) — extract
- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — failures
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_render::pipeline`](../../robby_render/pipeline/index.md) — 5 items

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`main`](main.md) | `code.function` | 29 | `crates/robby-cli/build.rs:19:1` |

## Imports

- `use std::env;`
- `use std::path::PathBuf;`
- [`use robby_render::pipeline;`](../../robby_render/pipeline/index.md)

