# `robby_extract::rust::ProcBuilder`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-extract/src/rust.rs:575`](../../../../crates/robby-extract/src/rust.rs#L575) |
| provenance | extractor · confidence 1.00 |

```rust
struct ProcBuilder<'a, 'b> {
    ctx: &'a Ctx<'b>,
    process: Process,
    blocks: Vec<Block>,
    me: String,
    me_id: String,
    pending: &'a mut Vec<PendingCall>,
}
```

**Fields and variants**

- `ctx` — `crates/robby-extract/src/rust.rs:576:5`
- `process` — `crates/robby-extract/src/rust.rs:577:5`
- `blocks` — `crates/robby-extract/src/rust.rs:578:5`
- `me` — `crates/robby-extract/src/rust.rs:579:5`
- `me_id` — `crates/robby-extract/src/rust.rs:580:5`
- `pending` — `crates/robby-extract/src/rust.rs:581:5`

