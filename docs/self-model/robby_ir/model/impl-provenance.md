# `robby_ir::model::impl Provenance`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:105`](../../../../crates/robby-ir/src/model.rs#L105) |
| provenance | extractor · confidence 1.00 |

```rust
impl Provenance {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            observed_at: None,
            confidence: 1.0,
        }
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}
```

### `new`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn new(1)` |
| location | [`crates/robby-ir/src/model.rs:106`](../../../../crates/robby-ir/src/model.rs#L106) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn new(source: Source) -> Self {
        Self {
            source,
            observed_at: None,
            confidence: 1.0,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant new as new()
    new-->>new: Self { source, observed_at: None, confidence: 1.0, }
```

1 steps · 0 observed · 1 possible only

### `with_confidence`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn with_confidence(2)` |
| location | [`crates/robby-ir/src/model.rs:114`](../../../../crates/robby-ir/src/model.rs#L114) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant with_confidence as with_confidence()
    with_confidence-->>with_confidence: self.confidence = confidence.clamp(0.0, 1.0)
    with_confidence-->>with_confidence: self
```

2 steps · 0 observed · 2 possible only

