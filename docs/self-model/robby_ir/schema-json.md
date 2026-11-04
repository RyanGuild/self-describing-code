# `robby_ir::schema_json`

[Self-model](../README.md) · [robby_ir](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn schema_json() -> String` |
| location | [`crates/robby-ir/src/lib.rs:51`](../../../crates/robby-ir/src/lib.rs#L51) |
| provenance | extractor · confidence 1.00 |

> Pretty-printed JSON Schema.

```rust
/// Pretty-printed JSON Schema.
pub fn schema_json() -> String {
    serde_json::to_string_pretty(&schema()).expect("schema is always serializable")
}
```

```mermaid
sequenceDiagram
    autonumber
    participant schema_json as schema_json()
    participant schema as schema
    participant serde_json__to_string_pretty as serde_json::to_string_pretty
    participant expect as expect
    schema_json-->>schema: schema()
    schema_json-->>serde_json__to_string_pretty: serde_json::to_string_pretty(&schema())
    schema_json-->>expect: serde_json::to_string_pretty(&schema()).expect('schema is always seri...
```

3 steps · 0 observed · 3 possible only

**Edges**

- calls → [`robby_ir::schema`](schema.md)

## References

- calls → [`robby_ir`](index.md) — schema

## Referenced by

- calls ← [`robby_cli::run`](../robby_cli/run.md)

