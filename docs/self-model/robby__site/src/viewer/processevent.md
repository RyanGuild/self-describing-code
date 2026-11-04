# `robby__site/src/viewer/ProcessEvent`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.interface` |
| location | [`apps/site/src/viewer.ts:51`](../../../../../apps/site/src/viewer.ts#L51) |
| provenance | extractor · confidence 1.00 |

```rust
interface ProcessEvent {
  type: "end" | "else" | "begin" | "interaction";
  label?: string;
  keyword?: string;
  index?: number;
  from?: string;
  to?: string;
  observed?: boolean;
  kind?: string;
}
```

