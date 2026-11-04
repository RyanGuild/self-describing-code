# `robby__site/src/viewer/prefetchEntrypoints`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:127`](../../../../../apps/site/src/viewer.ts#L127) |
| provenance | extractor · confidence 1.00 |

```rust
async function prefetchEntrypoints() {
  if (sceneCache.has("entrypoints")) {
    return;
  }
  try {
    const entrypoints = await fetchJson<Scene>(ORG_MODES.entrypoints.file);
    sceneCache.set("entrypoints", entrypoints);
  } catch (err) {
    console.error(err);
    orgEntrypointsBtn.disabled = true;
    orgEntrypointsBtn.title = `${ORG_MODES.entrypoints.file} missing — re-run robby web`;
  }
}
```

```mermaid
sequenceDiagram
    autonumber
    participant prefetchEntrypoints as prefetchEntrypoints()
    participant fetchJson as fetchJson
    participant set as set
    participant error as error
    alt if
    return-->>caller: 
    end
    self-->>fetchJson: 
    self-->>await: 
    self-->>set: 
    alt catch
    self-->>error: 
    end
```

5 steps · 0 observed · 5 possible only

**Edges**

- calls → [`robby__site/src/viewer/fetchJson`](fetchjson.md)

## References

- calls → [`robby__site/src/viewer`](index.md) — fetchJson

## Referenced by

- calls ← [`robby__site/src/viewer/setOrgMode`](setorgmode.md)

