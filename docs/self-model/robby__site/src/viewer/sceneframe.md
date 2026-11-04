# `robby__site/src/viewer/SceneFrame`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:31`](../../../../../apps/site/src/viewer.ts#L31) |
| provenance | extractor · confidence 1.00 |

```rust
interface SceneFrame {
  id: string;
  stable_id: string;
  name: string;
  fqn: string;
  kind_tag: string;
  parent?: string;
  children: string[];
  world: Rect;
  local_in_parent?: Rect;
  edges: { from: string; to: string; label: string; resolved: boolean }[];
  portals: Portal[];
}
```

