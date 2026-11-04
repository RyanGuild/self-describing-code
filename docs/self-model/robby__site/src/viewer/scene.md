# `robby__site/src/viewer/Scene`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:45`](../../../../../apps/site/src/viewer.ts#L45) |
| provenance | extractor · confidence 1.00 |

```rust
interface Scene {
  root: string;
  canvas: { w: number; h: number };
  frames: Record<string, SceneFrame>;
}
```

