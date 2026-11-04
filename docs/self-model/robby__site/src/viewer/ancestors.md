# `robby__site/src/viewer/ancestors`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:195`](../../../../../apps/site/src/viewer.ts#L195) |
| provenance | extractor · confidence 1.00 |

```rust
function ancestors(id: string): string[] {
  const path: string[] = [];
  let cur: string | null = id;
  while (cur) {
    path.push(cur);
    cur = frame(cur).parent ?? null;
  }
  return path.reverse();
}
```

```mermaid
sequenceDiagram
    autonumber
    participant ancestors as ancestors()
    participant push as push
    participant frame as frame
    participant reverse as reverse
    loop while
    self-->>push: 
    self-->>frame: 
    end
    self-->>reverse: 
    return-->>caller: 
```

4 steps · 0 observed · 4 possible only

**Edges**

- calls → [`robby__site/src/viewer/frame`](frame.md)

## References

- calls → [`robby__site/src/viewer`](index.md) — frame

