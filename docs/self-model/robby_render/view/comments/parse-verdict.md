# `robby_render::view::comments::parse_verdict`

[Self-model](../../../README.md) · [view::comments](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn parse_verdict(decision: &Node) -> Option<Verdict>` |
| location | [`crates/robby-render/src/view/comments.rs:151`](../../../../../crates/robby-render/src/view/comments.rs#L151) |
| provenance | extractor · confidence 1.00 |

```rust
fn parse_verdict(decision: &Node) -> Option<Verdict> {
    Verdict::parse(decision.annotations.get("verdict")?.as_str())
}
```

```mermaid
sequenceDiagram
    autonumber
    participant parse_verdict as parse_verdict()
    participant get as get
    participant as_str as as_str
    participant Verdict__parse as Verdict::parse
    parse_verdict-->>get: decision.annotations.get('verdict')
    parse_verdict-->>parse_verdict: ? (propagate)
    parse_verdict-->>as_str: decision.annotations.get('verdict')?.as_str()
    parse_verdict-->>Verdict__parse: Verdict::parse(decision.annotations.get('verdict')?.as_str())
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../../robby_ir/model/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](index.md)
- calls → [`robby_ir::vocab::impl Verdict::parse`](../../../robby_ir/vocab/index.md)

## References

- calls → [`robby_ir::model`](../../../robby_ir/model/index.md) — get
- calls → [`robby_ir::vocab`](../../../robby_ir/vocab/index.md) — parse
- calls → [`robby_render::view::comments`](index.md) — as_str

