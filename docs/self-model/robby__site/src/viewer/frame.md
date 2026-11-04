# `robby__site/src/viewer/frame`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:521`](../../../../../apps/site/src/viewer.ts#L521) |
| provenance | extractor · confidence 1.00 |

```rust
function frame(id: string): SceneFrame {
  const f = scene!.frames[id];
  if (!f) {
    throw new Error(`missing frame ${id}`);
  }
  return f;
}
```

```mermaid
sequenceDiagram
    autonumber
    participant frame as frame()
    return-->>caller: 
```

1 steps · 0 observed · 1 possible only

## Referenced by

- calls ← [`robby__site/src/viewer/ancestors`](ancestors.md)
- calls ← [`robby__site/src/viewer/cameraForFocus`](cameraforfocus.md)
- calls ← [`robby__site/src/viewer/frameSafeParent`](framesafeparent.md)
- calls ← [`robby__site/src/viewer/loadDetail`](loaddetail.md)
- calls ← [`robby__site/src/viewer/makeFrameEl`](makeframeel.md)
- calls ← [`robby__site/src/viewer/placeAllFrames`](placeallframes.md)
- calls ← [`robby__site/src/viewer/remount`](remount.md)
- calls ← [`robby__site/src/viewer/screenRectFor`](screenrectfor.md)
- calls ← [`robby__site/src/viewer/zoomOut`](zoomout.md)

