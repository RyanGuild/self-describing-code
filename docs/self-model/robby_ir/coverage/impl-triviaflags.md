# `robby_ir::coverage::impl TriviaFlags`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/coverage.rs:311`](../../../../crates/robby-ir/src/coverage.rs#L311) |
| provenance | extractor · confidence 1.00 |

```rust
impl TriviaFlags {
    fn classify(self) -> GapReason {
        match (self.attribute, self.comment) {
            (true, false) => GapReason::Attribute,
            (false, true) => GapReason::Comment,
            (false, false) => GapReason::Whitespace,
            (true, true) => GapReason::Trivia,
        }
    }
}
```

## Referenced by

- calls ← [`robby_ir::coverage::make_gap`](make-gap.md)

### `classify`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn classify(1)` |
| location | [`crates/robby-ir/src/coverage.rs:312`](../../../../crates/robby-ir/src/coverage.rs#L312) |
| provenance | extractor · confidence 1.00 |

```rust
fn classify(self) -> GapReason {
        match (self.attribute, self.comment) {
            (true, false) => GapReason::Attribute,
            (false, true) => GapReason::Comment,
            (false, false) => GapReason::Whitespace,
            (true, true) => GapReason::Trivia,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant classify as classify()
```

0 steps · 0 observed · 0 possible only

