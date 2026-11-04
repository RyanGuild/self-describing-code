# `robby_extract::rust::callee_name`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn callee_name(func: &Expr) -> String` |
| location | [`crates/robby-extract/src/rust.rs:836`](../../../../crates/robby-extract/src/rust.rs#L836) |
| provenance | extractor · confidence 1.00 |

> The last segment, used to match against known items.

```rust
/// The last segment, used to match against known items.
fn callee_name(func: &Expr) -> String {
    match func {
        Expr::Path(p) => p
            .path
            .segments
            .last()
            .map_or_else(|| "call".to_owned(), |s| s.ident.to_string()),
        Expr::Field(f) => match &f.member {
            syn::Member::Named(n) => n.to_string(),
            syn::Member::Unnamed(i) => format!("field{}", i.index),
        },
        _ => "call".to_owned(),
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant callee_name as callee_name()
    participant last as last
    participant to_owned as to_owned
    participant to_string as to_string
    participant map_or_else as map_or_else
    alt match func
    callee_name-->>last: p .path .segments .last()
    callee_name-->>to_owned: 'call'.to_owned()
    callee_name-->>to_string: s.ident.to_string()
    callee_name-->>map_or_else: p .path .segments .last() .map_or_else(|| 'call'.to_owned(), |s| s.id...
    else Expr::Field(f)
    alt match &f.member
    callee_name-->>to_string: n.to_string()
    else syn::Member::Unnamed(i)
    callee_name-->>callee_name: format!('field{}', i.index)
    end
    else _
    callee_name-->>to_owned: 'call'.to_owned()
    end
```

7 steps · 0 observed · 7 possible only

## Referenced by

- calls ← [`robby_extract::rust::callee_display`](callee-display.md)
- calls ← [`robby_extract::rust::walk_expr`](walk-expr.md)

