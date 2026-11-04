# `robby_ir::vocab::impl NoteKind`

[Self-model](../../README.md) · [vocab](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/vocab.rs:41`](../../../../crates/robby-ir/src/vocab.rs#L41) |
| provenance | extractor · confidence 1.00 |

```rust
impl NoteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageExample => "usage_example",
            Self::ImplementationNote => "implementation_note",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "usage_example" => Some(Self::UsageExample),
            "implementation_note" => Some(Self::ImplementationNote),
            _ => None,
        }
    }

    /// Prose label for Markdown and the terminal report.
    pub fn label(self) -> &'static str {
        match self {
            Self::UsageExample => "usage example",
            Self::ImplementationNote => "implementation note",
        }
    }
}
```

### `as_str`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn as_str(1)` |
| location | [`crates/robby-ir/src/vocab.rs:42`](../../../../crates/robby-ir/src/vocab.rs#L42) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageExample => "usage_example",
            Self::ImplementationNote => "implementation_note",
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant as_str as as_str()
```

0 steps · 0 observed · 0 possible only

### `parse`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn parse(1)` |
| location | [`crates/robby-ir/src/vocab.rs:49`](../../../../crates/robby-ir/src/vocab.rs#L49) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "usage_example" => Some(Self::UsageExample),
            "implementation_note" => Some(Self::ImplementationNote),
            _ => None,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant parse as parse()
    participant Some as Some
    alt match raw
    parse-->>Some: Some(Self::UsageExample)
    else 'implementation_note'
    parse-->>Some: Some(Self::ImplementationNote)
    end
```

2 steps · 0 observed · 2 possible only

### `label`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn label(1)` |
| location | [`crates/robby-ir/src/vocab.rs:57`](../../../../crates/robby-ir/src/vocab.rs#L57) |
| provenance | extractor · confidence 1.00 |

> Prose label for Markdown and the terminal report.

```rust
/// Prose label for Markdown and the terminal report.
    pub fn label(self) -> &'static str {
        match self {
            Self::UsageExample => "usage example",
            Self::ImplementationNote => "implementation note",
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant label as label()
```

0 steps · 0 observed · 0 possible only

