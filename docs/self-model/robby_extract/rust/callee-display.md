# `robby_extract::rust::callee_display`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn callee_display(func: &Expr) -> String` |
| location | [`crates/robby-extract/src/rust.rs:818`](../../../../crates/robby-extract/src/rust.rs#L818) |
| provenance | extractor · confidence 1.00 |

> The readable callee, including its module path: `Document::new`.

```rust
/// The readable callee, including its module path: `Document::new`.
fn callee_display(func: &Expr) -> String {
    match func {
        Expr::Path(p) => path_text(&p.path),
        _ => callee_name(func),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant callee_display as callee_display()
    participant path_text as path_text
    participant callee_name as callee_name
    alt match func
    callee_display-->>path_text: path_text(&p.path)
    else _
    callee_display-->>callee_name: callee_name(func)
    end
```

2 steps · 0 observed · 2 possible only

**Edges**

- calls → [`robby_extract::rust::path_text`](path-text.md)
- calls → [`robby_extract::rust::callee_name`](callee-name.md)

## References

- calls → [`robby_extract::rust`](index.md) — callee_name, path_text

## Referenced by

- calls ← [`robby_extract::rust::walk_expr`](walk-expr.md)

