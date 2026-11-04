# `robby_ir::vocab::Highlight`

[Self-model](../../README.md) · [vocab](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/vocab.rs:66`](../../../../crates/robby-ir/src/vocab.rs#L66) |
| provenance | extractor · confidence 1.00 |

> Byte range into a code window. Offsets are UTF-8 bytes.

```rust
/// Byte range into a code window. Offsets are UTF-8 bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Highlight {
    pub start: u32,
    pub end: u32,
}
```

**Fields and variants**

- `start` — `crates/robby-ir/src/vocab.rs:69:5`
- `end` — `crates/robby-ir/src/vocab.rs:70:5`

