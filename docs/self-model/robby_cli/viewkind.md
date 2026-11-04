# `robby_cli::ViewKind`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-cli/src/main.rs:136`](../../../crates/robby-cli/src/main.rs#L136) |
| provenance | extractor · confidence 1.00 |

```rust
#[derive(Clone, Copy, Debug, ValueEnum)]
enum ViewKind {
    All,
    Entry,
    Types,
    Dependencies,
    References,
    Tree,
    Graph,
    Process,
    Source,
}
```

**Fields and variants**

- `All` — `crates/robby-cli/src/main.rs:138:5`
- `Entry` — `crates/robby-cli/src/main.rs:139:5`
- `Types` — `crates/robby-cli/src/main.rs:140:5`
- `Dependencies` — `crates/robby-cli/src/main.rs:141:5`
- `References` — `crates/robby-cli/src/main.rs:142:5`
- `Tree` — `crates/robby-cli/src/main.rs:143:5`
- `Graph` — `crates/robby-cli/src/main.rs:144:5`
- `Process` — `crates/robby-cli/src/main.rs:145:5`
- `Source` — `crates/robby-cli/src/main.rs:146:5`

