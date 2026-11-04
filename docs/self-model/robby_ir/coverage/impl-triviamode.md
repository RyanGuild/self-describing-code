# `robby_ir::coverage::impl TriviaMode`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/coverage.rs:277`](../../../../crates/robby-ir/src/coverage.rs#L277) |
| provenance | extractor · confidence 1.00 |

```rust
impl TriviaMode {
    fn from_path(path: &str) -> Self {
        const SCRIPT: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];
        let script = std::path::Path::new(path)
            .extension()
            .is_some_and(|ext| SCRIPT.iter().any(|cand| ext.eq_ignore_ascii_case(cand)));
        if script { Self::TypeScript } else { Self::Rust }
    }
}
```

## References

- calls → [`robby_ir::kind`](../kind/index.md) — new

## Referenced by

- calls ← [`robby_extract::typescript::extract_file`](../../robby_extract/typescript/extract-file.md)
- calls ← [`robby_ir::coverage::make_gap`](make-gap.md)

### `from_path`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_path(1)` |
| location | [`crates/robby-ir/src/coverage.rs:278`](../../../../crates/robby-ir/src/coverage.rs#L278) |
| provenance | extractor · confidence 1.00 |

```rust
fn from_path(path: &str) -> Self {
        const SCRIPT: &[&str] = &["ts", "tsx", "mts", "cts", "js", "jsx", "mjs", "cjs"];
        let script = std::path::Path::new(path)
            .extension()
            .is_some_and(|ext| SCRIPT.iter().any(|cand| ext.eq_ignore_ascii_case(cand)));
        if script { Self::TypeScript } else { Self::Rust }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from_path as from_path()
    participant std__path__Path__new as std::path::Path::new
    participant extension as extension
    participant iter as iter
    participant eq_ignore_ascii_case as eq_ignore_ascii_case
    participant any as any
    participant is_some_and as is_some_and
    from_path-->>from_path: const SCRIPT: &[&str] = &['ts', 'tsx', 'mts', 'cts', 'js', 'jsx', 'mj...
    from_path-->>std__path__Path__new: std::path::Path::new(path)
    from_path-->>extension: std::path::Path::new(path) .extension()
    from_path-->>iter: SCRIPT.iter()
    from_path-->>eq_ignore_ascii_case: ext.eq_ignore_ascii_case(cand)
    from_path-->>any: SCRIPT.iter().any(|cand| ext.eq_ignore_ascii_case(cand))
    from_path-->>is_some_and: std::path::Path::new(path) .extension() .is_some_and(|ext| SCRIPT.ite...
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_ir::kind::impl Kind::new`](../kind/index.md)

