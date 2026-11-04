# `robby_ir::coverage::file_coverage`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn file_coverage(file: &str, source: &str, spans: &[Span]) -> FileCoverage` |
| location | [`crates/robby-ir/src/coverage.rs:145`](../../../../crates/robby-ir/src/coverage.rs#L145) |
| provenance | extractor · confidence 1.00 |

> Compute coverage for one file given the spans that claim it.

```rust
/// Compute coverage for one file given the spans that claim it.
pub fn file_coverage(file: &str, source: &str, spans: &[Span]) -> FileCoverage {
    let len = source.len() as u32;
    let bytes = source.as_bytes();

    let mut ranges: Vec<(u32, u32)> = spans
        .iter()
        .filter(|s| s.file == file)
        .map(|s| {
            let (a, b) = s.byte_range();
            (a.min(len), b.min(len))
        })
        .filter(|(a, b)| a < b)
        .collect();
    ranges.sort_unstable();

    let mut merged: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for (a, b) in ranges {
        match merged.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => merged.push((a, b)),
        }
    }

    let covered_bytes: u32 = merged.iter().map(|(a, b)| b - a).sum();

    let mut gaps = Vec::new();
    let mut cursor = 0u32;
    for (a, b) in &merged {
        if *a > cursor {
            gaps.push(make_gap(file, source, bytes, cursor, *a));
        }
        cursor = (*b).max(cursor);
    }
    if cursor < len {
        gaps.push(make_gap(file, source, bytes, cursor, len));
    }

    FileCoverage {
        file: file.to_owned(),
        total_bytes: len,
        covered_bytes,
        gaps,
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant file_coverage as file_coverage()
    participant len as len
    participant as_bytes as as_bytes
    participant iter as iter
    participant filter as filter
    participant byte_range as byte_range
    participant min as min
    participant map as map
    participant collect as collect
    participant sort_unstable as sort_unstable
    participant Vec__with_capacity as Vec::with_capacity
    participant last_mut as last_mut
    participant max as max
    participant push as push
    participant sum as sum
    participant Vec__new as Vec::new
    participant make_gap as make_gap
    participant to_owned as to_owned
    file_coverage-->>len: source.len()
    file_coverage-->>as_bytes: source.as_bytes()
    file_coverage-->>iter: spans .iter()
    file_coverage-->>filter: spans .iter() .filter(|s| s.file == file)
    file_coverage-->>byte_range: s.byte_range()
    file_coverage-->>min: a.min(len)
    file_coverage-->>min: b.min(len)
    file_coverage-->>map: spans .iter() .filter(|s| s.file == file) .map(|s| { let (a, b) = s.b...
    file_coverage-->>filter: spans .iter() .filter(|s| s.file == file) .map(|s| { let (a, b) = s.b...
    file_coverage-->>collect: spans .iter() .filter(|s| s.file == file) .map(|s| { let (a, b) = s.b...
    file_coverage-->>sort_unstable: ranges.sort_unstable()
    file_coverage-->>len: ranges.len()
    file_coverage-->>Vec__with_capacity: Vec::with_capacity(ranges.len())
    loop ranges
    file_coverage-->>last_mut: merged.last_mut()
    alt match merged.last_mut()
    file_coverage-->>max: last.1.max(b)
    else _
    file_coverage-->>push: merged.push((a, b))
    end
    end
    file_coverage-->>iter: merged.iter()
    file_coverage-->>map: merged.iter().map(|(a, b)| b - a)
    file_coverage-->>sum: merged.iter().map(|(a, b)| b - a).sum()
    file_coverage-->>Vec__new: Vec::new()
    loop &merged
    alt *a > cursor
    file_coverage-->>make_gap: make_gap(file, source, bytes, cursor, *a)
    file_coverage-->>push: gaps.push(make_gap(file, source, bytes, cursor, *a))
    end
    file_coverage-->>max: (*b).max(cursor)
    end
    alt cursor < len
    file_coverage-->>make_gap: make_gap(file, source, bytes, cursor, len)
    file_coverage-->>push: gaps.push(make_gap(file, source, bytes, cursor, len))
    end
    file_coverage-->>to_owned: file.to_owned()
```

26 steps · 0 observed · 26 possible only

**Edges**

- calls → [`robby_ir::span::impl Span::byte_range`](../span/index.md)
- calls → [`robby_render::view::select::collect`](../../robby_render/view/select/collect.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)
- calls → [`robby_ir::coverage::make_gap`](make-gap.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::coverage::make_gap`](make-gap.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)

## References

- calls → [`robby_ir::coverage`](index.md) — make_gap
- calls → [`robby_ir::kind`](../kind/index.md) — new
- calls → [`robby_ir::span`](../span/index.md) — byte_range
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push
- calls → [`robby_render::view::select`](../../robby_render/view/select/index.md) — collect

## Referenced by

- calls ← [`robby_ir::coverage::document_coverage`](document-coverage.md)
- calls ← [`robby_ir::test_coverage::attributes_are_classified`](../test_coverage/attributes-are-classified.md)
- calls ← [`robby_ir::test_coverage::empty_file_is_trivially_complete`](../test_coverage/empty-file-is-trivially-complete.md)
- calls ← [`robby_ir::test_coverage::fully_covered_file_has_no_gaps`](../test_coverage/fully-covered-file-has-no-gaps.md)
- calls ← [`robby_ir::test_coverage::gap_preview_is_whitespace_collapsed`](../test_coverage/gap-preview-is-whitespace-collapsed.md)
- calls ← [`robby_ir::test_coverage::nested_spans_are_unioned_not_double_counted`](../test_coverage/nested-spans-are-unioned-not-double-counted.md)
- calls ← [`robby_ir::test_coverage::spans_beyond_eof_are_clamped_not_panicking`](../test_coverage/spans-beyond-eof-are-clamped-not-panicking.md)
- calls ← [`robby_ir::test_coverage::spans_from_other_files_are_ignored`](../test_coverage/spans-from-other-files-are-ignored.md)
- calls ← [`robby_ir::test_coverage::typescript_comments_and_shebang_are_trivia`](../test_coverage/typescript-comments-and-shebang-are-trivia.md)
- calls ← [`robby_ir::test_coverage::typescript_hash_is_not_a_rust_attribute`](../test_coverage/typescript-hash-is-not-a-rust-attribute.md)
- calls ← [`robby_ir::test_coverage::unclaimed_code_fails_with_the_exact_byte`](../test_coverage/unclaimed-code-fails-with-the-exact-byte.md)
- calls ← [`robby_ir::test_coverage::whitespace_and_comments_are_classified_not_failed`](../test_coverage/whitespace-and-comments-are-classified-not-failed.md)

