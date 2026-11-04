# `robby__site/src/viewer/SceneFrame`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:34`](../../../../../apps/site/src/viewer.ts#L34) |
| provenance | extractor · confidence 1.00 |

```rust
interface SceneFrame {
  name: string;
  fqn: string;
  kind_tag: string;
  parent?: string;
  children: string[];
  world: Rect;
  local_in_parent?: Rect;
  portals: Portal[];
}
```

