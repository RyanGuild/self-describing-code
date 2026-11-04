# `robby_extract::rust::docs_of`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn docs_of<T: Spanned>(_t: &T, attrs: &[syn::Attribute]) -> Option<String>` |
| location | [`crates/robby-extract/src/rust.rs:390`](../../../../crates/robby-extract/src/rust.rs#L390) |
| provenance | extractor · confidence 1.00 |

```rust
fn docs_of<T: Spanned>(_t: &T, attrs: &[syn::Attribute]) -> Option<String> {
    let mut lines = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc")
            && let syn::Meta::NameValue(nv) = &attr.meta
            && let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
        {
            lines.push(s.value().trim().to_owned());
        }
    }
    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant docs_of as docs_of()
    participant Vec__new as Vec::new
    participant path as path
    participant is_ident as is_ident
    participant value as value
    participant trim as trim
    participant to_owned as to_owned
    participant push as push
    participant is_empty as is_empty
    participant join as join
    participant Some as Some
    docs_of-->>Vec__new: Vec::new()
    loop attrs
    docs_of-->>path: attr.path()
    docs_of-->>is_ident: attr.path().is_ident('doc')
    alt attr.path().is_ident('doc') && let syn::Meta::NameValue(nv) = &attr.m...
    docs_of-->>value: s.value()
    docs_of-->>trim: s.value().trim()
    docs_of-->>to_owned: s.value().trim().to_owned()
    docs_of-->>push: lines.push(s.value().trim().to_owned())
    end
    end
    docs_of-->>is_empty: lines.is_empty()
    alt lines.is_empty()
    docs_of-->>join: lines.join('\n')
    docs_of-->>Some: Some(lines.join('\n'))
    end
```

10 steps · 0 observed · 10 possible only

**Edges**

- calls → [`robby_extract::rust::impl LineIndex::new`](index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)

## References

- calls → [`robby_extract::rust`](index.md) — new
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_extract::rust::walk_items`](walk-items.md)

