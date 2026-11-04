# `robby_extract::rust::LineIndex`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-extract/src/rust.rs:461`](../../../../crates/robby-extract/src/rust.rs#L461) |
| provenance | extractor · confidence 1.00 |

> Converts proc-macro2 line/column positions into byte offsets.
> 
> `proc-macro2`'s `span-locations` reports columns as character offsets within
> the line, so the conversion walks chars rather than bytes. This is what makes
> coverage exact on files containing non-ASCII identifiers or comments.

```rust
/// Converts proc-macro2 line/column positions into byte offsets.
///
/// `proc-macro2`'s `span-locations` reports columns as character offsets within
/// the line, so the conversion walks chars rather than bytes. This is what makes
/// coverage exact on files containing non-ASCII identifiers or comments.
pub struct LineIndex {
    starts: Vec<u32>,
    source: String,
}
```

**Fields and variants**

- `starts` — `crates/robby-extract/src/rust.rs:467:5`
- `source` — `crates/robby-extract/src/rust.rs:468:5`

