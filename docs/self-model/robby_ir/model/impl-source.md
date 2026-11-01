# `robby_ir::model::impl Source`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:77`](../../../../crates/robby-ir/src/model.rs#L77) |
| provenance | extractor · confidence 1.00 |

```rust
impl Source {
    /// Lower numbers win when two producers disagree about the same fact.
    ///
    /// Runtime beats static because observation is ground truth; explicit
    /// annotation beats parsing because it carries intent.
    pub fn precedence(self) -> u8 {
        match self {
            Source::Trace => 0,
            Source::Agent => 1,
            Source::Macro => 2,
            Source::Extractor => 3,
            Source::Hand => 4,
        }
    }
}
```

### `precedence`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn precedence(1)` |
| location | [`crates/robby-ir/src/model.rs:78`](../../../../crates/robby-ir/src/model.rs#L78) |
| provenance | extractor · confidence 1.00 |

> Lower numbers win when two producers disagree about the same fact.
> 
> Runtime beats static because observation is ground truth; explicit
> annotation beats parsing because it carries intent.

```rust
/// Lower numbers win when two producers disagree about the same fact.
    ///
    /// Runtime beats static because observation is ground truth; explicit
    /// annotation beats parsing because it carries intent.
    pub fn precedence(self) -> u8 {
        match self {
            Source::Trace => 0,
            Source::Agent => 1,
            Source::Macro => 2,
            Source::Extractor => 3,
            Source::Hand => 4,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant precedence as precedence()
    alt match self
    precedence-->>precedence: 0
    else Source::Agent
    precedence-->>precedence: 1
    else Source::Macro
    precedence-->>precedence: 2
    else Source::Extractor
    precedence-->>precedence: 3
    else Source::Hand
    precedence-->>precedence: 4
    end
```

5 steps · 0 observed · 5 possible only

