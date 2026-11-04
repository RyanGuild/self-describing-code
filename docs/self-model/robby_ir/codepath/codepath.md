# `robby_ir::codepath::CodePath`

[Self-model](../../README.md) · [codepath](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-ir/src/codepath.rs:40`](../../../../crates/robby-ir/src/codepath.rs#L40) |
| provenance | extractor · confidence 1.00 |

> Reachability tree rooted at one entrypoint function.

```rust
/// Reachability tree rooted at one entrypoint function.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CodePath {
    /// Entrypoint node (typically a `main`).
    pub entry: NodeId,
    /// BFS-ordered tree edges within this path. Shared callees may appear in
    /// multiple paths; consumers that need a single tree apply first-parent wins.
    pub edges: Vec<CodePathEdge>,
    pub max_depth: u32,
}
```

**Fields and variants**

- `entry` — `crates/robby-ir/src/codepath.rs:43:5`
- `edges` — `crates/robby-ir/src/codepath.rs:45:5`
- `max_depth` — `crates/robby-ir/src/codepath.rs:48:5`

