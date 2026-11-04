# `robby__site/src/viewer/RunRecord`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:121`](../../../../../apps/site/src/viewer.ts#L121) |
| provenance | extractor · confidence 1.00 |

```rust
interface RunRecord {
  run_id: string;
  recipe: string;
  subject: string;
  status: string;
  started_at: number;
  finished_at: number;
  exit_code?: number;
  stdout_tail?: string;
  stats?: Record<string, unknown>;
  products?: string[];
  process?: {
    participants: { id: string; label: string }[];
    steps: RunStep[];
  };
}
```

