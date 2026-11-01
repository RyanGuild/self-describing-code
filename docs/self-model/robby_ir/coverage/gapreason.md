# `robby_ir::coverage::GapReason`

[Self-model](../../README.md) · [coverage](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/coverage.rs:24`](../../../../crates/robby-ir/src/coverage.rs#L24) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GapReason {
    /// Nothing but whitespace.
    Whitespace,
    /// Nothing but comments. Rendered, but not counted as a failure: comments
    /// are the raw material for the `why` question.
    Comment,
    /// Nothing but attributes, e.g. `#[derive(Debug)]`.
    Attribute,
    /// A mixture of the above.
    Trivia,
    /// Something is here that no node claims. This fails the gate.
    Unknown { at: Position },
}
```

**Fields and variants**

- `Whitespace` — `crates/robby-ir/src/coverage.rs:27:5`
- `Comment` — `crates/robby-ir/src/coverage.rs:29:5`
- `Attribute` — `crates/robby-ir/src/coverage.rs:32:5`
- `Trivia` — `crates/robby-ir/src/coverage.rs:34:5`
- `Unknown` — `crates/robby-ir/src/coverage.rs:36:5`

