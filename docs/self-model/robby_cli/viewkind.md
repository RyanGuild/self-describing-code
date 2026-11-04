# `robby_cli::ViewKind`

[Self-model](../README.md) · [robby_cli](index.md)

| | |
|---|---|
| kind | `code.enum` |
| location | [`crates/robby-cli/src/main.rs:130`](../../../crates/robby-cli/src/main.rs#L130) |
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

- `All` — `crates/robby-cli/src/main.rs:132:5`
- `Entry` — `crates/robby-cli/src/main.rs:133:5`
- `Types` — `crates/robby-cli/src/main.rs:134:5`
- `Dependencies` — `crates/robby-cli/src/main.rs:135:5`
- `References` — `crates/robby-cli/src/main.rs:136:5`
- `Tree` — `crates/robby-cli/src/main.rs:137:5`
- `Graph` — `crates/robby-cli/src/main.rs:138:5`
- `Process` — `crates/robby-cli/src/main.rs:139:5`
- `Source` — `crates/robby-cli/src/main.rs:140:5`

