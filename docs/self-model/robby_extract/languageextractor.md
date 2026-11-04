# `robby_extract::LanguageExtractor`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-extract/src/lib.rs:18`](../../../crates/robby-extract/src/lib.rs#L18) |
| provenance | extractor · confidence 1.00 |

> One language's producer. Future crates (`robby-extract-rust`,
> `robby-extract-typescript`) should keep this signature.

```rust
/// One language's producer. Future crates (`robby-extract-rust`,
/// `robby-extract-typescript`) should keep this signature.
pub trait LanguageExtractor {
    fn language(&self) -> Lang;
    fn extract(&self, root: &Path) -> Result<Document>;
    fn source_file_count(&self, root: &Path) -> Result<usize>;
}
```

### `language`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-extract/src/lib.rs:21`](../../../crates/robby-extract/src/lib.rs#L21) |
| provenance | extractor · confidence 1.00 |

```rust
fn language(&self) -> Lang;
```

### `extract`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-extract/src/lib.rs:22`](../../../crates/robby-extract/src/lib.rs#L22) |
| provenance | extractor · confidence 1.00 |

```rust
fn extract(&self, root: &Path) -> Result<Document>;
```

### `source_file_count`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-extract/src/lib.rs:23`](../../../crates/robby-extract/src/lib.rs#L23) |
| provenance | extractor · confidence 1.00 |

```rust
fn source_file_count(&self, root: &Path) -> Result<usize>;
```

