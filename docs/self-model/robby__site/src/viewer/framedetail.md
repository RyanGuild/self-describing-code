# `robby__site/src/viewer/FrameDetail`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:51`](../../../../../apps/site/src/viewer.ts#L51) |
| provenance | extractor · confidence 1.00 |

```rust
interface FrameDetail {
  kind: string;
  signature?: string;
  docs?: string;
  location?: string;
  provenance: string;
  process?: { step_count: number; observed_count: number };
  process_mermaid?: string;
  execution_mermaid?: string;
  source?: { file: string; text: string };
}
```

