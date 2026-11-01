# `robby_extract::ProcBuilder`

[Self-model](../README.md) · [robby_extract](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-extract/src/lib.rs:586`](../../../crates/robby-extract/src/lib.rs#L586) |
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

- `ctx` — `crates/robby-extract/src/lib.rs:587:5`
- `process` — `crates/robby-extract/src/lib.rs:588:5`
- `blocks` — `crates/robby-extract/src/lib.rs:589:5`
- `me` — `crates/robby-extract/src/lib.rs:590:5`
- `me_id` — `crates/robby-extract/src/lib.rs:591:5`
- `pending` — `crates/robby-extract/src/lib.rs:592:5`

