# `robby_extract::rust::ProcBuilder`

[Self-model](../../README.md) · [rust](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-extract/src/rust.rs:572`](../../../../crates/robby-extract/src/rust.rs#L572) |
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

- `ctx` — `crates/robby-extract/src/rust.rs:573:5`
- `process` — `crates/robby-extract/src/rust.rs:574:5`
- `blocks` — `crates/robby-extract/src/rust.rs:575:5`
- `me` — `crates/robby-extract/src/rust.rs:576:5`
- `me_id` — `crates/robby-extract/src/rust.rs:577:5`
- `pending` — `crates/robby-extract/src/rust.rs:578:5`

