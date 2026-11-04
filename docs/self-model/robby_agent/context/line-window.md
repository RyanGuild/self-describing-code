# `robby_agent::context::line_window`

[Self-model](../../README.md) · [context](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn line_window(sources: &BTreeMap<String, String>, node: &Node) -> Option<String>` |
| location | [`crates/robby-agent/src/context.rs:184`](../../../../crates/robby-agent/src/context.rs#L184) |
| provenance | extractor · confidence 1.00 |

> ±20 lines around the node's span.

```rust
/// ±20 lines around the node's span.
fn line_window(sources: &BTreeMap<String, String>, node: &Node) -> Option<String> {
    let span = node.span.as_ref()?;
    let source = sources.get(&span.file)?;
    let start_line = span.start.line.saturating_sub(1) as usize;
    let end_line = span.end.line as usize;
    let from = start_line.saturating_sub(20);
    let to = (end_line + 20).min(source.lines().count().saturating_sub(1).max(end_line));
    let mut out = String::new();
    for (i, line) in source.lines().enumerate() {
        if i < from {
            continue;
        }
        if i > to {
            break;
        }
        out.push_str(line);
        out.push('\n');
    }
    if out.is_empty() { None } else { Some(out) }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant line_window as line_window()
    participant as_ref as as_ref
    participant get as get
    participant saturating_sub as saturating_sub
    participant lines as lines
    participant count as count
    participant max as max
    participant min as min
    participant String__new as String::new
    participant enumerate as enumerate
    participant push_str as push_str
    participant push as push
    participant is_empty as is_empty
    participant Some as Some
    line_window-->>as_ref: node.span.as_ref()
    line_window-->>line_window: ? (propagate)
    line_window-->>get: sources.get(&span.file)
    line_window-->>line_window: ? (propagate)
    line_window-->>saturating_sub: span.start.line.saturating_sub(1)
    line_window-->>saturating_sub: start_line.saturating_sub(20)
    line_window-->>lines: source.lines()
    line_window-->>count: source.lines().count()
    line_window-->>saturating_sub: source.lines().count().saturating_sub(1)
    line_window-->>max: source.lines().count().saturating_sub(1).max(end_line)
    line_window-->>min: (end_line + 20).min(source.lines().count().saturating_sub(1).max(end_...
    line_window-->>String__new: String::new()
    line_window-->>lines: source.lines()
    line_window-->>enumerate: source.lines().enumerate()
    loop source.lines().enumerate()
    line_window-->>push_str: out.push_str(line)
    line_window-->>push: out.push('\n')
    end
    line_window-->>is_empty: out.is_empty()
    alt out.is_empty()
    line_window-->>Some: Some(out)
    end
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby_ir::model::impl Document::get`](../../robby_ir/model/index.md)
- calls → [`robby_agent::enrich::impl FakeClient::new`](../enrich/tests/index.md)
- calls → [`robby_render::report::impl Report::push`](../../robby_render/report/index.md)
- calls → [`robby_ir::span::impl Span::is_empty`](../../robby_ir/span/index.md)

## References

- calls → [`robby_agent::enrich::tests`](../enrich/tests/index.md) — new
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — is_empty
- calls → [`robby_render::report`](../../robby_render/report/index.md) — push

## Referenced by

- calls ← [`robby_agent::context::build_for_docs_owner`](build-for-docs-owner.md)
- calls ← [`robby_agent::context::code_window_for`](code-window-for.md)

