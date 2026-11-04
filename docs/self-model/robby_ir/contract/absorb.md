# `robby_ir::contract::Absorb`

[Self-model](../../README.md) · [contract](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-ir/src/contract.rs:11`](../../../../crates/robby-ir/src/contract.rs#L11) |
| provenance | extractor · confidence 1.00 |

> Merge an observed value into a static (or earlier) one without dropping prior
> structure.
> 
> For [`Process`], matching is by `(from, to, label)`: observed steps mark
> matches; unmatched observed steps append; static-only steps stay dashed.

```rust
/// Merge an observed value into a static (or earlier) one without dropping prior
/// structure.
///
/// For [`Process`], matching is by `(from, to, label)`: observed steps mark
/// matches; unmatched observed steps append; static-only steps stay dashed.
pub trait Absorb {
    fn absorb(&mut self, observed: &Self);
}
```

## Referenced by

- calls ← [`robby_ir::contract::absorb_trait_marks_matching_steps_observed`](tests/absorb-trait-marks-matching-steps-observed.md)

### `absorb`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:17`](../../../../crates/robby-ir/src/contract.rs#L17) |
| provenance | extractor · confidence 1.00 |

```rust
fn absorb(&mut self, observed: &Self);
```

