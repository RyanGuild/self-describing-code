# `robby_extract::docs_of`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn docs_of<T: Spanned>(_t: &T, attrs: &[syn::Attribute]) -> Option<String>` |
| location | [`crates/robby-extract/src/lib.rs:398`](../../../crates/robby-extract/src/lib.rs#L398) |
| provenance | extractor · confidence 1.00 |

```rust
fn docs_of<T: Spanned>(_t: &T, attrs: &[syn::Attribute]) -> Option<String> {
    let mut lines = Vec::new();
    for attr in attrs {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    lines.push(s.value().trim().to_owned());
                }
            }
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
    participant push as push
    participant Some as Some
    docs_of-->>Vec__new: Vec::new()
    loop attrs
    alt attr.path().is_ident('doc')
    alt let syn::Meta::NameValue(nv) = &attr.meta
    alt let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv....
    docs_of-->>push: lines.push(s.value().trim().to_owned())
    end
    end
    end
    end
    alt lines.is_empty()
    docs_of-->>docs_of: None
    else else
    docs_of-->>Some: Some(lines.join('\n'))
    end
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_extract::impl LineIndex::new`](index.md)

