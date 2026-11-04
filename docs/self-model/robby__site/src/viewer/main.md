# `robby__site/src/viewer/main`

[Self-model](../../../README.md) · [robby__site/src/viewer](index.md)

| | |
|---|---|
| kind | `code.function` |
| location | [`apps/site/src/viewer.ts:180`](../../../../../apps/site/src/viewer.ts#L180) |
| provenance | extractor · confidence 1.00 |

```rust
async function main(): Promise<void> {
  try {
    const structure = await fetchJson<Scene>(ORG_MODES.structure.file);
    sceneCache.set("structure", structure);
    scene = structure;
  } catch (err) {
    worldEl.textContent =
      "Could not load scene.json. Run `robby web` then serve this app (Vite).";
    console.error(err);
    return;
  }

  applyScene(scene);
  void prefetchEntrypoints();
  void prefetchRuns();
  void startBuildWatch();

  zoomOutBtn.addEventListener("click", () => zoomOut());
  orgStructureBtn.addEventListener("click", () => void setOrgMode("structure"));
  orgEntrypointsBtn.addEventListener("click", () => void setOrgMode("entrypoints"));
  orgRunsBtn.addEventListener("click", () => void setOrgMode("runs"));
  window.addEventListener("resize", () => {
    if (orgMode === "runs" || !scene) {
      return;
    }
    camera = cameraForFocus(focusId);
    placeAllFrames();
  });
  window.addEventListener("keydown", (e) => {
    if (e.key === "Escape" || e.key === "Backspace") {
      e.preventDefault();
      if (orgMode === "runs") {
        return;
      }
      zoomOut();
    }
  });
}
```

```mermaid
sequenceDiagram
    autonumber
    participant main as main()
    participant fetchJson as fetchJson
    participant set as set
    participant error as error
    participant applyScene as applyScene
    participant addEventListener as addEventListener
    self-->>fetchJson: 
    self-->>await: 
    self-->>set: 
    alt catch
    self-->>error: 
    return-->>caller: 
    end
    self-->>applyScene: 
    self-->>addEventListener: 
    self-->>fn: 
    self-->>addEventListener: 
    self-->>fn: 
    self-->>addEventListener: 
    self-->>fn: 
    self-->>addEventListener: 
    self-->>fn: 
    self-->>addEventListener: 
    self-->>fn: 
    self-->>addEventListener: 
    self-->>fn: 
```

18 steps · 0 observed · 18 possible only

**Edges**

- calls → [`robby__site/src/viewer/fetchJson`](fetchjson.md)
- calls → [`robby__site/src/viewer/applyScene`](applyscene.md)

## References

- calls → [`robby__site/src/viewer`](index.md) — applyScene, fetchJson

