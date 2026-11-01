# `robby_render::markdown::source_name`

[Self-model](../../README.md) · [markdown](index.md)

| | |
|---|---|
| kind | `code.function` |
| signature | `fn source_name(source: robby_ir::Source) -> &'static str` |
| location | [`crates/robby-render/src/markdown.rs:209`](../../../../crates/robby-render/src/markdown.rs#L209) |
| provenance | extractor · confidence 1.00 |

```rust
pub(crate) fn source_name(source: robby_ir::Source) -> &'static str {
    match source {
        robby_ir::Source::Extractor => "extractor",
        robby_ir::Source::Macro => "macro",
        robby_ir::Source::Trace => "trace",
        robby_ir::Source::Agent => "agent",
        robby_ir::Source::Hand => "hand",
    }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant source_name as source_name()
    alt match source
    source_name-->>source_name: 'extractor'
    else robby_ir::Source::Macro
    source_name-->>source_name: 'macro'
    else robby_ir::Source::Trace
    source_name-->>source_name: 'trace'
    else robby_ir::Source::Agent
    source_name-->>source_name: 'agent'
    else robby_ir::Source::Hand
    source_name-->>source_name: 'hand'
    end
```

5 steps · 0 observed · 5 possible only

