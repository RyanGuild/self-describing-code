# `robby_ir::run::impl RecipeCatalog for [Recipe]`

[Self-model](../../README.md) · [run](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-ir/src/run.rs:180`](../../../../crates/robby-ir/src/run.rs#L180) |
| provenance | extractor · confidence 1.00 |

```rust
impl RecipeCatalog for [Recipe] {
    fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        self.iter().find(|r| &r.id == id)
    }
}
```

### `get`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn get(2)` |
| location | [`crates/robby-ir/src/run.rs:181`](../../../../crates/robby-ir/src/run.rs#L181) |
| provenance | extractor · confidence 1.00 |

```rust
fn get(&self, id: &RecipeId) -> Option<&Recipe> {
        self.iter().find(|r| &r.id == id)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant get as get()
    participant iter as iter
    participant find as find
    get-->>iter: self.iter()
    get-->>find: self.iter().find(|r| &r.id == id)
```

2 steps · 0 observed · 2 possible only

