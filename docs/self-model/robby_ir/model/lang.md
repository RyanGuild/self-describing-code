# `robby_ir::model::Lang`

[Self-model](../../README.md) · [model](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-ir/src/model.rs:126`](../../../../crates/robby-ir/src/model.rs#L126) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Lang {
    Rust,
    TypeScript,
    Unknown,
}
```

**Fields and variants**

- `Rust` — `crates/robby-ir/src/model.rs:129:5`
- `TypeScript` — `crates/robby-ir/src/model.rs:130:5`
- `Unknown` — `crates/robby-ir/src/model.rs:131:5`

