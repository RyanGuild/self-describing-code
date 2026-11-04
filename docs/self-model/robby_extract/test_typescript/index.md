# `robby_extract::test_typescript`

[Self-model](../../README.md) · [robby_extract](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 10 |

## Package diagram

```mermaid
flowchart TB
    n_707c83973fcabf1f["test_typescript"]
    n_ea78dce2fbcbf64f(["robby_extract"])
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_707c83973fcabf1f --> n_356bf2c23da68961
    n_707c83973fcabf1f --> n_9c636b098ca4450e
    n_707c83973fcabf1f --> n_ea78dce2fbcbf64f
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_707c83973fcabf1f["test_typescript"]
        direction TB
        n_b8e37ee0e00d2102["bare_import_creates_external_npm_stub"]
        n_7eb75d53fd7f0121["emits_comment_nodes_and_attaches_docs"]
        n_be1b7fb4407a7ecb["extracts_typescript_package_with_lang_tag"]
        n_1b53af5a44fa5068["fixture"]
        n_1d410cca2cf6459b["resolves_calls_and_imports"]
        n_b0a6b64e43c0a4f9["typescript_fixture_is_fully_covered"]
    end
    n_1d410cca2cf6459b --> n_1b53af5a44fa5068
    n_7eb75d53fd7f0121 --> n_1b53af5a44fa5068
    n_b0a6b64e43c0a4f9 --> n_1b53af5a44fa5068
    n_b8e37ee0e00d2102 --> n_1b53af5a44fa5068
    n_be1b7fb4407a7ecb --> n_1b53af5a44fa5068
```

## Dependency matrix

| | test_typescript | robby_ir | robby_render | robby_extract |
|---|---:|---:|---:|---:|
| **test_typescript** | · | 11 | 5 | 6 |
| **robby_ir** | | · | | |
| **robby_render** | | | · | |
| **robby_extract** | | | | · |
## References

- calls → [`robby_extract`](../index.md) — extract
- calls → [`robby_extract::rust`](../rust/index.md) — new
- calls → [`robby_ir::coverage`](../../robby_ir/coverage/index.md) — document_coverage, failures
- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — as_str, from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — insert
- calls → [`robby_render::view::scene`](../../robby_render/view/scene/index.md) — contains
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`fixture`](fixture.md) | `code.function` | 3 | `crates/robby-extract/tests/typescript.rs:8:1` |
| [`extracts_typescript_package_with_lang_tag`](extracts-typescript-package-with-lang-tag.md) | `code.function` | 10 | `crates/robby-extract/tests/typescript.rs:12:1` |
| [`resolves_calls_and_imports`](resolves-calls-and-imports.md) | `code.function` | 27 | `crates/robby-extract/tests/typescript.rs:37:1` |
| [`bare_import_creates_external_npm_stub`](bare-import-creates-external-npm-stub.md) | `code.function` | 16 | `crates/robby-extract/tests/typescript.rs:86:1` |
| [`emits_comment_nodes_and_attaches_docs`](emits-comment-nodes-and-attaches-docs.md) | `code.function` | 15 | `crates/robby-extract/tests/typescript.rs:119:1` |
| [`typescript_fixture_is_fully_covered`](typescript-fixture-is-fully-covered.md) | `code.function` | 16 | `crates/robby-extract/tests/typescript.rs:162:1` |

## Imports

- `use std::collections::BTreeMap;`
- `use std::path::PathBuf;`
- [`use robby_ir::{EdgeKind, Lang, kind};`](../../robby_ir/index.md)

