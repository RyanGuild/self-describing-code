# `robby_ir::test_coverage`

[Self-model](../../README.md) · [robby_ir](../index.md)

| | |
|---|---|
| kind | `code.module` |
| provenance | extractor · confidence 1.00 |
| nodes | 21 |

```mermaid
flowchart TD
    subgraph n_46b0c6f667b1fb40["test_coverage [module]"]
        direction TB
        n_148f2b948cc3894a["FILE [const]"]
        n_514c7e46850aa6a4["fn span(source: &str, start: u32, end: u32) -> Span [function]"]
        n_4065d207f8671e25["fn fully_covered_file_has_no_gaps() [function]"]
        n_cd050a460ec66c34["fn nested_spans_are_unioned_not_double_counted() [function]"]
        n_1e530833babc36c3["fn whitespace_and_comments_are_classified_not_failed() [function]"]
        n_0b193972a1983a06["fn attributes_are_classified() [function]"]
        n_80b1b25edb807166["fn unclaimed_code_fails_with_the_exact_byte() [function]"]
        n_4d2a47f96135b80e["fn empty_file_is_trivially_complete() [function]"]
        n_7235be4ea824b149["fn spans_beyond_eof_are_clamped_not_panicking() [function]"]
        n_ddb2a12ff2308547["fn spans_from_other_files_are_ignored() [function]"]
        n_d248e92311a4cff7["fn unreferenced_files_appear_fully_uncovered() [function]"]
        n_f916c261571f423f["fn report_orders_worst_files_first() [function]"]
        n_c83f7c789f6b13f6["fn gap_preview_is_whitespace_collapsed() [function]"]
        n_6e5287b66b2ff142["fn frontier_spans_exclude_parents_that_have_span_children() [function]"]
        n_eee4ab9af74a2168["fn document_coverage_is_union_at_any_depth() [function]"]
    end
```

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`FILE`](file.md) | `code.const` | — | `crates/robby-ir/tests/coverage.rs:14:1` |
| [`span`](span.md) | `code.function` | 1 | `crates/robby-ir/tests/coverage.rs:16:1` |
| [`fully_covered_file_has_no_gaps`](fully-covered-file-has-no-gaps.md) | `code.function` | 6 | `crates/robby-ir/tests/coverage.rs:20:1` |
| [`nested_spans_are_unioned_not_double_counted`](nested-spans-are-unioned-not-double-counted.md) | `code.function` | 5 | `crates/robby-ir/tests/coverage.rs:31:1` |
| [`whitespace_and_comments_are_classified_not_failed`](whitespace-and-comments-are-classified-not-failed.md) | `code.function` | 6 | `crates/robby-ir/tests/coverage.rs:45:1` |
| [`attributes_are_classified`](attributes-are-classified.md) | `code.function` | 6 | `crates/robby-ir/tests/coverage.rs:56:1` |
| [`unclaimed_code_fails_with_the_exact_byte`](unclaimed-code-fails-with-the-exact-byte.md) | `code.function` | 10 | `crates/robby-ir/tests/coverage.rs:67:1` |
| [`empty_file_is_trivially_complete`](empty-file-is-trivially-complete.md) | `code.function` | 3 | `crates/robby-ir/tests/coverage.rs:92:1` |
| [`spans_beyond_eof_are_clamped_not_panicking`](spans-beyond-eof-are-clamped-not-panicking.md) | `code.function` | 4 | `crates/robby-ir/tests/coverage.rs:99:1` |
| [`spans_from_other_files_are_ignored`](spans-from-other-files-are-ignored.md) | `code.function` | 5 | `crates/robby-ir/tests/coverage.rs:107:1` |
| [`unreferenced_files_appear_fully_uncovered`](unreferenced-files-appear-fully-uncovered.md) | `code.function` | 13 | `crates/robby-ir/tests/coverage.rs:117:1` |
| [`report_orders_worst_files_first`](report-orders-worst-files-first.md) | `code.function` | 10 | `crates/robby-ir/tests/coverage.rs:142:1` |
| [`gap_preview_is_whitespace_collapsed`](gap-preview-is-whitespace-collapsed.md) | `code.function` | 5 | `crates/robby-ir/tests/coverage.rs:158:1` |
| [`frontier_spans_exclude_parents_that_have_span_children`](frontier-spans-exclude-parents-that-have-span-children.md) | `code.function` | 13 | `crates/robby-ir/tests/coverage.rs:168:1` |
| [`document_coverage_is_union_at_any_depth`](document-coverage-is-union-at-any-depth.md) | `code.function` | 9 | `crates/robby-ir/tests/coverage.rs:187:1` |

## Imports

- `use std::collections::BTreeMap;`
- `use robby_ir::coverage::{document_coverage, file_coverage};`
- `use robby_ir::model::{Document, Node, NodeId};`
- `use robby_ir::span::Span;`
- `use robby_ir::{GapReason, kind};`

