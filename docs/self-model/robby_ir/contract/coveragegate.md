# `robby_ir::contract::CoverageGate`

[Self-model](../../README.md) · [contract](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-ir/src/contract.rs:75`](../../../../crates/robby-ir/src/contract.rs#L75) |
| provenance | extractor · confidence 1.00 |

> Coverage completeness gate: unknown gaps fail; trivia does not.

```rust
/// Coverage completeness gate: unknown gaps fail; trivia does not.
pub trait CoverageGate {
    fn passes(&self) -> bool;

    fn failing_gaps(&self) -> Vec<(&FileCoverage, &Gap)>;
}
```

## Referenced by

- calls ← [`robby_render::pipeline::impl Loaded::is_ok`](../../robby_render/pipeline/index.md)

### `passes`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:77`](../../../../crates/robby-ir/src/contract.rs#L77) |
| provenance | extractor · confidence 1.00 |

```rust
fn passes(&self) -> bool;
```

### `failing_gaps`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:79`](../../../../crates/robby-ir/src/contract.rs#L79) |
| provenance | extractor · confidence 1.00 |

```rust
fn failing_gaps(&self) -> Vec<(&FileCoverage, &Gap)>;
```

