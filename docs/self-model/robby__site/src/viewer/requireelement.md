# `robby__site/src/viewer/requireElement`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:999`](../../../../../apps/site/src/viewer.ts#L999) |
| provenance | extractor · confidence 1.00 |

```rust
function requireElement(id: string): HTMLElement {
  const el = document.getElementById(id);
  if (!(el instanceof HTMLElement)) {
    throw new Error(`missing #${id}`);
  }
  return el;
}
```

```mermaid
sequenceDiagram
    autonumber
    participant requireElement as requireElement()
    participant getElementById as getElementById
    self-->>getElementById: 
    return-->>caller: 
```

2 steps · 0 observed · 2 possible only

