# `robby__site/src/viewer/FrameDetail`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:62`](../../../../../apps/site/src/viewer.ts#L62) |
| provenance | extractor · confidence 1.00 |

```rust
interface FrameDetail {
  stable_id: string;
  id: string;
  fqn: string;
  kind: string;
  signature?: string;
  docs?: string;
  location?: string;
  provenance: string;
  confidence: number;
  process?: {
    participants: { id: string; label: string }[];
    events: ProcessEvent[];
    step_count: number;
    observed_count: number;
  };
  /** Mermaid sequenceDiagram from robby-render (same as Markdown). */
  process_mermaid?: string;
  /** Path-level Calls sequence (functions + owning types). */
  execution_mermaid?: string;
  source?: { file: string; text: string };
}
```

