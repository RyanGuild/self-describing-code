# `robby__site/src/viewer/escapeHtml`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:636`](../../../../../apps/site/src/viewer.ts#L636) |
| provenance | extractor · confidence 1.00 |

```rust
function escapeHtml(s: string) {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
```

```mermaid
sequenceDiagram
    autonumber
    participant escapeHtml as escapeHtml()
    participant replace as replace
    self-->>replace: 
    return-->>caller: 
```

2 steps · 0 observed · 2 possible only

