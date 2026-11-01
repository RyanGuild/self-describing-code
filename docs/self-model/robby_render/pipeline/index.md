# `robby_render::pipeline`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/lib.rs:9`](../../../../crates/robby-render/src/lib.rs#L9) |
| provenance | extractor · confidence 1.00 |
| nodes | 25 |

```mermaid
flowchart TD
    subgraph n_77294f3b6f21b79c["pipeline [module]"]
        direction TB
        n_42be26d5aaaf9be5["Loaded [struct]"]
        n_21de05adf7688d68["fn load_document(path: &Path) -> Result<Document> [function]"]
        n_55b8c37d0badb78d["fn load_sources(doc: &Document, root: &Path) -> BTreeMap<String, String> [function]"]
        n_470f85c178206b14["fn load(ir: &Path, root: &Path) -> Result<Loaded> [function]"]
        n_632a84e110a5dc84["fn from_document(document: Document, root: &Path) -> Loaded [function]"]
        n_8bacd3e44796f7f0["fn render_files(doc: &Document, out: &Path) -> std::io::Result<usize> [function]"]
        n_1a20b9c873c4d761["fn write_document(doc: &Document, out: &Path) -> Result<()> [function]"]
        n_273b2347595a4a92["fn write_report(loaded: &Loaded, out_dir: &Path) -> std::io::Result<usize> [function]"]
        n_f19279b0043fe2d4["fn remove_markdown(dir: &Path) -> std::io::Result<()> [function]"]
        n_2ee15b18fda893be["fn watched_files(loaded: &Loaded) -> Vec<PathBuf> [function]"]
        n_b3c04f794cd90823["fn slug(input: &str) -> String [function]"]
    end
    n_273b2347595a4a92 -->|calls| n_f19279b0043fe2d4
    n_470f85c178206b14 -->|calls| n_21de05adf7688d68
    n_632a84e110a5dc84 -->|calls| n_55b8c37d0badb78d
    n_f19279b0043fe2d4 -->|calls| n_f19279b0043fe2d4
```

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`Loaded`](loaded.md) | `code.struct` | — | `crates/robby-render/src/pipeline.rs:17:1` |
| [`impl Loaded`](impl-loaded.md) | `code.impl` | — | `crates/robby-render/src/pipeline.rs:27:1` |
| [`load_document`](load-document.md) | `code.function` | 3 | `crates/robby-render/src/pipeline.rs:49:1` |
| [`load_sources`](load-sources.md) | `code.function` | 6 | `crates/robby-render/src/pipeline.rs:55:1` |
| [`load`](load.md) | `code.function` | 3 | `crates/robby-render/src/pipeline.rs:79:1` |
| [`from_document`](from-document.md) | `code.function` | 4 | `crates/robby-render/src/pipeline.rs:85:1` |
| [`render_files`](render-files.md) | `code.function` | 11 | `crates/robby-render/src/pipeline.rs:99:1` |
| [`write_document`](write-document.md) | `code.function` | 5 | `crates/robby-render/src/pipeline.rs:117:1` |
| [`write_report`](write-report.md) | `code.function` | 12 | `crates/robby-render/src/pipeline.rs:126:1` |
| [`remove_markdown`](remove-markdown.md) | `code.function` | 9 | `crates/robby-render/src/pipeline.rs:158:1` |
| [`watched_files`](watched-files.md) | `code.function` | 1 | `crates/robby-render/src/pipeline.rs:176:1` |
| [`slug`](slug.md) | `code.function` | 9 | `crates/robby-render/src/pipeline.rs:185:1` |

## Imports

- `use std::collections::BTreeMap;`
- `use std::fs;`
- `use std::path::{Path, PathBuf};`
- `use anyhow::Result;`
- `use robby_ir::{CoverageReport, Diagnostic, Document, Severity, coverage, validate};`

