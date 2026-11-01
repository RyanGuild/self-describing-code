# `robby_ir::model::impl Diagnostic`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/model.rs:389`](../../../../crates/robby-ir/src/model.rs#L389) |
| provenance | extractor · confidence 1.00 |

```rust
impl Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }

    pub fn about(mut self, node: impl Into<NodeId>) -> Self {
        self.node = Some(node.into());
        self
    }
}
```

### `error`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn error(2)` |
| location | [`crates/robby-ir/src/model.rs:390`](../../../../crates/robby-ir/src/model.rs#L390) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant error as error()
    error-->>error: Self { severity: Severity::Error, code: code.into(), message: message...
```

1 steps · 0 observed · 1 possible only

### `warning`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn warning(2)` |
| location | [`crates/robby-ir/src/model.rs:399`](../../../../crates/robby-ir/src/model.rs#L399) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant warning as warning()
    warning-->>warning: Self { severity: Severity::Warning, code: code.into(), message: messa...
```

1 steps · 0 observed · 1 possible only

### `about`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn about(2)` |
| location | [`crates/robby-ir/src/model.rs:408`](../../../../crates/robby-ir/src/model.rs#L408) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn about(mut self, node: impl Into<NodeId>) -> Self {
        self.node = Some(node.into());
        self
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant about as about()
    about-->>about: self.node = Some(node.into())
    about-->>about: self
```

2 steps · 0 observed · 2 possible only

