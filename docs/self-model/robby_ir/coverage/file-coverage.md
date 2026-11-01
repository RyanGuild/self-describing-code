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
    participant as_bytes as as_bytes
    participant collect as collect
    participant sort_unstable as sort_unstable
    participant Vec__with_capacity as Vec::with_capacity
    participant push as push
    participant sum as sum
    participant Vec__new as Vec::new
    file_coverage-->>file_coverage: source.len() as u32
    file_coverage-->>as_bytes: source.as_bytes()
    file_coverage-->>collect: spans .iter() .filter(|s| s.file == file) .map(|s| { let (a, b) = s.b...
    file_coverage-->>sort_unstable: ranges.sort_unstable()
    file_coverage-->>Vec__with_capacity: Vec::with_capacity(ranges.len())
    loop ranges
    alt match merged.last_mut()
    file_coverage-->>file_coverage: last.1 = last.1.max(b)
    else _
    file_coverage-->>push: merged.push((a, b))
    end
    end
    file_coverage-->>sum: merged.iter().map(|(a, b)| b - a).sum()
    file_coverage-->>Vec__new: Vec::new()
    file_coverage-->>file_coverage: 0u32
    loop &merged
    alt *a > cursor
    file_coverage-->>push: gaps.push(make_gap(file, source, bytes, cursor, *a))
    end
    file_coverage-->>file_coverage: cursor = (*b).max(cursor)
    end
    alt cursor < len
    file_coverage-->>push: gaps.push(make_gap(file, source, bytes, cursor, len))
    end
    file_coverage-->>file_coverage: FileCoverage { file: file.to_owned(), total_bytes: len, covered_bytes...
```

14 steps · 0 observed · 14 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](../../robby_extract/index.md)

