# `robby_extract::build_process`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn build_process(     f: &syn::ItemFn,     ctx: &Ctx<'_>,     fqn: &str,     pending: &mut Vec<PendingCall>, ) -> Process` |
| location | [`crates/robby-extract/src/lib.rs:551`](../../../crates/robby-extract/src/lib.rs#L551) |
| provenance | extractor · confidence 1.00 |

```rust
fn build_process(
    f: &syn::ItemFn,
    ctx: &Ctx<'_>,
    fqn: &str,
    pending: &mut Vec<PendingCall>,
) -> Process {
    build_process_sig(&f.sig, &f.block, ctx, fqn, pending)
}
```

```mermaid
sequenceDiagram
    autonumber
    participant build_process as build_process()
    participant build_process_sig as build_process_sig
    build_process-->>build_process_sig: build_process_sig(&f.sig, &f.block, ctx, fqn, pending)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_extract::build_process_sig`](build-process-sig.md)

