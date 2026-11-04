# `robby_ir::contract::KindNamespace`

[Self-model](../../README.md) · [contract](index.md)

| | |
|---|---|
| kind | `code.trait` |
| location | [`crates/robby-ir/src/contract.rs:47`](../../../../crates/robby-ir/src/contract.rs#L47) |
| provenance | extractor · confidence 1.00 |

> Open-kind namespace queries. Unknown namespaces must still round-trip.

```rust
/// Open-kind namespace queries. Unknown namespaces must still round-trip.
pub trait KindNamespace {
    fn namespace(&self) -> &str;

    fn is_code(&self) -> bool {
        self.namespace() == "code"
    }

    fn is_runtime(&self) -> bool {
        self.namespace() == "runtime"
    }

    fn is_agent(&self) -> bool {
        self.namespace() == "agent"
    }

    /// `code` | `runtime` | `agent` — soft gap otherwise, never a hard error.
    fn is_known_namespace(&self) -> bool {
        matches!(self.namespace(), "code" | "runtime" | "agent")
    }
}
```

## Referenced by

- calls ← [`robby_ir::kind::is_known`](../kind/is-known.md)

### `namespace`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:49`](../../../../crates/robby-ir/src/contract.rs#L49) |
| provenance | extractor · confidence 1.00 |

```rust
fn namespace(&self) -> &str;
```

### `is_code`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:51`](../../../../crates/robby-ir/src/contract.rs#L51) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_code(&self) -> bool {
        self.namespace() == "code"
    }
```

### `is_runtime`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:55`](../../../../crates/robby-ir/src/contract.rs#L55) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_runtime(&self) -> bool {
        self.namespace() == "runtime"
    }
```

### `is_agent`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:59`](../../../../crates/robby-ir/src/contract.rs#L59) |
| provenance | extractor · confidence 1.00 |

```rust
fn is_agent(&self) -> bool {
        self.namespace() == "agent"
    }
```

### `is_known_namespace`

| | |
|---|---|
| kind | `code.method` |
| location | [`crates/robby-ir/src/contract.rs:63`](../../../../crates/robby-ir/src/contract.rs#L63) |
| provenance | extractor · confidence 1.00 |

```rust
/// `code` | `runtime` | `agent` — soft gap otherwise, never a hard error.
    fn is_known_namespace(&self) -> bool {
        matches!(self.namespace(), "code" | "runtime" | "agent")
    }
```

