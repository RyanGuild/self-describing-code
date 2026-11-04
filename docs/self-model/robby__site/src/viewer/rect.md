# `robby__site/src/viewer/Rect`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:20`](../../../../../apps/site/src/viewer.ts#L20) |
| provenance | extractor · confidence 1.00 |

> *
>  * Fields this file reads. The JSON is whatever `Scene` and `FrameDetail`
>  * in robby-render serialize; unused fields stay untyped.
>  * `process` here is `ProcessView` — only the counts are read.

```rust
interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}
```

