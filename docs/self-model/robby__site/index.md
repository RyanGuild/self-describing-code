# `robby__site`

[Self-model](../README.md)

| | |
|---|---|
| kind | `code.package` |
| provenance | extractor · confidence 1.00 |
| nodes | 51 |

## Package diagram

```mermaid
flowchart TB
    n_fc79e481bc8492ed["robby__site"]
```

## Dependency matrix

| | robby__site |
|---|---:|
| **robby__site** | · |

## Comments

- `apps/site/src/viewer.ts:15:1` (block, `robby__site/src/viewer/Rect`): "* * Fields this file reads. The JSON is whatever `Scene` and `FrameDetail` * in robby-render serialize; unused fields s…"
  ```
  interface Rect {
    x: number;
    y: number;
    w: number;
    h: number;
  }
  ```
- `apps/site/src/viewer.ts:1:1` (block, `robby__site/src/viewer/Rect`): "* * Prezi-style camera over the Robby scene (flat world coordinates). * * Loads `scene.json` and lazy `frames/<id>.json…"
  ```
  interface Rect {
    x: number;
    y: number;
    w: number;
    h: number;
  }
  ```
- `apps/site/src/viewer.ts:212:1` (block, `robby__site/src/viewer/screenRectFromWorld`): "* Absolute screen rect for a frame from its world coordinates + camera."
  ```
  function screenRectFromWorld(f: SceneFrame, cam: Camera) {
    return {
      x: cam.tx + f.world.x * cam.scale,
      y: cam.ty + f.world.y * cam.scale,
      w: Math.max(f.world.w * cam.scale, 1),
      h: Math.max(f.world.h * cam.scale, 1),
    };
  }
  ```
- `apps/site/src/viewer.ts:222:1` (block, `robby__site/src/viewer/screenRectFor`): "* * Children of the focused frame are placed from `local_in_parent` into the * parent's screen box. Nested world intege…"
  ```
  function screenRectFor(f: SceneFrame, cam: Camera) {
    if (f.parent === focusId && f.local_in_parent && scene) {
      const parent = screenRectFromWorld(frame(focusId), cam);
      const loc = f.local_in_parent;
      const { w, h } = scene.canvas;
      return {
        x: parent.x + (loc.x / w) * parent.w,
        y: parent.y + (loc.y / h) * parent.h,
        w: Math.max((loc.w / w) * parent.w, 1),
        h: Math.max((loc.h / h) * parent.h, 1),
      };
    }
    return screenRectFromWorld(f, cam);
  }
  ```
- `apps/site/src/viewer.ts:252:3` (line, `robby__site/src/viewer/remount`): "Only top-level world cards (the focus). Children live in the body's grid."
  ```
  function remount() {
    worldEl.innerHTML = "";
    const focus = frame(focusId);
    const focusEl = makeFrameEl(focusId, true, false);
    worldEl.appendChild(focusEl);
  
    // Contained nodes sit in a scrollable grid inside the focus body so they can
    // keep a readable minimum size instead of shrinking to fit.
    if (focus.children.length > 0) {
      const body = focusEl.querySelector(".frame-body");
      if (body) {
        const grid = document.createElement("div");
        grid.className = "child-grid";
        for (const childId of focus.children) {
          grid.appendChild(makeFrameEl(childId, false, false));
        }
        body.appendChild(grid);
      }
    }
  
    placeAllFrames();
    updateChrome();
  }
  ```
- `apps/site/src/viewer.ts:268:3` (line, `robby__site/src/viewer/portalButton`): "Contained nodes sit in a scrollable grid inside the focus body so they can"
  ```
  function portalButton(portal: Portal) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `portal${portal.target ? "" : " unresolved"}`;
    btn.innerHTML = `<span class="pk">${escapeHtml(portal.kind_label)}</span>${escapeHtml(portal.label)}`;
    btn.title = portal.target_name || portal.label;
    if (portal.target) {
      const target = portal.target;
      btn.addEventListener("click", (e) => {
        e.stopPropagation();
        flyTo(target);
      });
    }
    return btn;
  }
  ```
- `apps/site/src/viewer.ts:269:3` (line, `robby__site/src/viewer/portalButton`): "keep a readable minimum size instead of shrinking to fit."
  ```
  function portalButton(portal: Portal) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `portal${portal.target ? "" : " unresolved"}`;
    btn.innerHTML = `<span class="pk">${escapeHtml(portal.kind_label)}</span>${escapeHtml(portal.label)}`;
    btn.title = portal.target_name || portal.label;
    if (portal.target) {
      const target = portal.target;
      btn.addEventListener("click", (e) => {
        e.stopPropagation();
        flyTo(target);
      });
    }
    return btn;
  }
  ```
- `apps/site/src/viewer.ts:382:3` (line, `robby__site/src/viewer/flyTo`): "Portals only on the focused frame. Child previews stay as clean cards so"
  ```
  function flyTo(id: string) {
    if (!currentScene().frames[id] || id === focusId || flying) {
      return;
    }
    flying = true;
    focusId = id;
    remount();
    const dest = cameraForFocus(focusId);
    tweenCamera(dest, () => {
      // Remount at the settled camera so the focus card paints at native screen
      // resolution instead of keeping a texture from the start of the tween.
      camera = dest;
      remount();
      flying = false;
      void loadDetail(focusId);
    });
  }
  ```
- `apps/site/src/viewer.ts:383:3` (line, `robby__site/src/viewer/flyTo`): "dozens of "calls from …" chips don't paint over the sibling grid."
  ```
  function flyTo(id: string) {
    if (!currentScene().frames[id] || id === focusId || flying) {
      return;
    }
    flying = true;
    focusId = id;
    remount();
    const dest = cameraForFocus(focusId);
    tweenCamera(dest, () => {
      // Remount at the settled camera so the focus card paints at native screen
      // resolution instead of keeping a texture from the start of the tween.
      camera = dest;
      remount();
      flying = false;
      void loadDetail(focusId);
    });
  }
  ```
- `apps/site/src/viewer.ts:409:5` (line, `robby__site/src/viewer/zoomOut`): "Remount at the settled camera so the focus card paints at native screen"
  ```
  function zoomOut() {
    const parent = frame(focusId).parent;
    if (!parent) {
      return;
    }
    flyTo(parent);
  }
  ```
- `apps/site/src/viewer.ts:410:5` (line, `robby__site/src/viewer/zoomOut`): "resolution instead of keeping a texture from the start of the tween."
  ```
  function zoomOut() {
    const parent = frame(focusId).parent;
    if (!parent) {
      return;
    }
    flyTo(parent);
  }
  ```
- `apps/site/src/viewer.ts:481:3` (line, `robby__site/src/viewer/appendExecution`): "Containers keep the child grid — inject execution above it."
  ```
  function appendExecution(panel: HTMLElement, mermaidSrc: string, frameId: string) {
    const proc = document.createElement("div");
    proc.className = "process process-execution";
    const heading = document.createElement("h2");
    heading.textContent = "Execution";
    proc.appendChild(heading);
    const summary = document.createElement("div");
    summary.className = "meta";
    summary.textContent = "call path · functions and owning types";
    proc.appendChild(summary);
    const diagram = document.createElement("div");
    diagram.className = "process-mermaid";
    proc.appendChild(diagram);
    panel.appendChild(proc);
    void renderMermaid(diagram, mermaidSrc, `${frameId}-exec`);
  }
  ```
## Modules

- [`robby__site/src/viewer`](src/viewer/index.md) — 50 nodes

