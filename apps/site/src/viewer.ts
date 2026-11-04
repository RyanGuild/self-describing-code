/**
 * Prezi-style camera over the Robby scene (flat world coordinates).
 *
 * Loads `scene.json` and lazy `frames/<id>.json` payloads written by
 * `robby web`. Click a card to zoom in; portals jump to related frames;
 * Escape / back zooms out.
 *
 * Frames stay laid out in the scene canvas. A single
 * screen-space transform per frame (camera × world placement) avoids the
 * downscale-then-upscale rasterization that keeps text pixelated after zoom.
 *
 * Soft-queues `robby.review_view` RunRecords on frame focus (Mac-local Vite
 * API → `public/runs/`). Runs/Reviews dock can Allow or Cancel.
 */

import {
  RECIPE_REVIEW_VIEW,
  type RunIndexEntry,
  type RunRecord,
  type RunsIndex,
} from "./runRecord";

const TWEEN_MS = 500;
/** Soft auto-queue on focus; skip workspace root and cancelled cooldown. */
const REVIEW_CANCEL_COOLDOWN_MS = 8_000;
const RUNS_POLL_MS = 1_500;

/**
 * Fields this file reads. The JSON is whatever `Scene` and `FrameDetail`
 * in robby-render serialize; unused fields stay untyped.
 * `process` here is `ProcessView` — only the counts are read.
 */
interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

interface Portal {
  label: string;
  kind_label: string;
  target?: string;
  target_name: string;
}

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

interface Scene {
  root: string;
  canvas: { w: number; h: number };
  frames: Record<string, SceneFrame>;
}

interface FrameDetail {
  kind: string;
  signature?: string;
  docs?: string;
  location?: string;
  provenance: string;
  process?: { step_count: number; observed_count: number };
  process_mermaid?: string;
  execution_mermaid?: string;
  source?: { file: string; text: string };
}

interface Camera {
  scale: number;
  tx: number;
  ty: number;
}

const ORG_MODES = {
  structure: {
    file: "scene.json",
    hint: "click a card to zoom · portals jump to related views",
  },
  entrypoints: {
    file: "scene-entrypoints.json",
    hint: "drill by call path · portals jump across shared callees",
  },
} as const;

type OrgMode = keyof typeof ORG_MODES;

const worldEl = requireElement("world");
const crumbEl = requireElement("crumb");
const hintEl = requireElement("hint");
const zoomOutBtn = requireButton("zoom-out");
const orgStructureBtn = requireButton("org-structure");
const orgEntrypointsBtn = requireButton("org-entrypoints");
const orgRunsBtn = requireButton("org-runs");
const runsDock = requireElement("runs-dock");
const runsPendingEl = requireElement("runs-pending");
const runsActiveEl = requireElement("runs-active");
const runsRecentEl = requireElement("runs-recent");
const runsDockCloseBtn = requireButton("runs-dock-close");

let scene: Scene | null = null;
let orgMode: OrgMode = "structure";
const sceneCache = new Map<OrgMode, Scene>();
let focusId = "";
const detailCache = new Map<string, FrameDetail>();
let flying = false;
let camera: Camera = { scale: 1, tx: 0, ty: 0 };
let cameraRaf = 0;
let runsDockOpen = false;
let runsFilter: "reviews" | "all" = "reviews";
let runsIndex: RunsIndex = { runs: [] };
const cancelCooldownUntil = new Map<string, number>();
let runsPollTimer = 0;

async function main() {
  try {
    const structure = await fetchJson<Scene>(ORG_MODES.structure.file);
    sceneCache.set("structure", structure);
    scene = structure;
  } catch (err) {
    worldEl.textContent = "Could not load scene.json. Run `robby web` then serve this app (Vite).";
    console.error(err);
    return;
  }

  applyScene(scene);
  void prefetchEntrypoints();
  initRunsDock();
  startRunsPoll();

  zoomOutBtn.addEventListener("click", () => zoomOut());
  orgStructureBtn.addEventListener("click", () => void setOrgMode("structure"));
  orgEntrypointsBtn.addEventListener("click", () => void setOrgMode("entrypoints"));
  orgRunsBtn.addEventListener("click", () => toggleRunsDock());
  window.addEventListener("resize", () => {
    camera = cameraForFocus(focusId);
    placeAllFrames();
  });
  window.addEventListener("keydown", (e) => {
    if (e.key === "Escape" || e.key === "Backspace") {
      e.preventDefault();
      zoomOut();
    }
  });
}

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

async function setOrgMode(next: OrgMode) {
  if (next === orgMode || flying) {
    return;
  }
  let cached = sceneCache.get(next);
  if (!cached) {
    if (next === "entrypoints") {
      await prefetchEntrypoints();
      cached = sceneCache.get(next);
    }
    if (!cached) {
      return;
    }
  }
  orgMode = next;
  orgStructureBtn.classList.toggle("is-active", next === "structure");
  orgEntrypointsBtn.classList.toggle("is-active", next === "entrypoints");
  orgRunsBtn.classList.toggle("is-active", false);
  hintEl.textContent = ORG_MODES[next].hint;
  applyScene(cached);
}

function applyScene(next: Scene) {
  scene = next;
  flying = false;
  window.cancelAnimationFrame(cameraRaf);
  focusId = next.root;
  camera = cameraForFocus(focusId);
  remount();
  void loadDetail(focusId);
  // Soft-queue only when the user focuses a non-root frame (not initial mount).
}

function currentScene(): Scene {
  if (!scene) {
    throw new Error("scene is not loaded");
  }
  return scene;
}

function frame(id: string) {
  const f = currentScene().frames[id];
  if (!f) {
    throw new Error(`missing frame ${id}`);
  }
  return f;
}

function ancestors(id: string) {
  const path: string[] = [];
  let cur: string | null = id;
  while (cur) {
    path.push(cur);
    cur = frame(cur).parent ?? null;
  }
  return path.reverse();
}

function cameraForFocus(id: string) {
  const f = frame(id);
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const pad = 0.92;
  const scale = Math.min(vw / f.world.w, vh / f.world.h) * pad;
  const cx = f.world.x + f.world.w / 2;
  const cy = f.world.y + f.world.h / 2;
  return {
    scale,
    tx: vw / 2 - cx * scale,
    ty: vh / 2 - cy * scale,
  };
}

/** Absolute screen rect for a frame from its world coordinates + camera. */
function screenRectFromWorld(f: SceneFrame, cam: Camera) {
  return {
    x: cam.tx + f.world.x * cam.scale,
    y: cam.ty + f.world.y * cam.scale,
    w: Math.max(f.world.w * cam.scale, 1),
    h: Math.max(f.world.h * cam.scale, 1),
  };
}

/**
 * Children of the focused frame are placed from `local_in_parent` into the
 * parent's screen box. Nested world integers collapse to 1×1 and stack; the
 * local grid on the 1600×1000 canvas does not.
 */
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

function placeFrame(el: HTMLElement, f: SceneFrame, cam: Camera) {
  const r = screenRectFor(f, cam);
  el.style.width = `${r.w}px`;
  el.style.height = `${r.h}px`;
  el.style.left = `${r.x}px`;
  el.style.top = `${r.y}px`;
  el.style.transform = "none";
}

function placeAllFrames() {
  // Only top-level world cards (the focus). Children live in the body's grid.
  for (const el of worldEl.querySelectorAll<HTMLElement>(":scope > .frame-wrap")) {
    const id = el.dataset.id;
    if (!id) {
      continue;
    }
    placeFrame(el, frame(id), camera);
  }
}

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

const PORTAL_PREVIEW = 8;

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

function buildPortalStrip(f: SceneFrame) {
  const strip = document.createElement("div");
  strip.className = "portal-strip";
  let expanded = false;

  const syncFocusClass = () => {
    const focusWrap = strip.closest(".frame-wrap");
    focusWrap?.classList.toggle("portals-expanded", expanded);
  };

  const render = () => {
    strip.innerHTML = "";
    strip.classList.toggle("is-expanded", expanded);
    syncFocusClass();
    const list = expanded ? f.portals : f.portals.slice(0, PORTAL_PREVIEW);
    for (const portal of list) {
      strip.appendChild(portalButton(portal));
    }
    if (!expanded && f.portals.length > PORTAL_PREVIEW) {
      const more = document.createElement("button");
      more.type = "button";
      more.className = "portal portal-more";
      more.textContent = `+${f.portals.length - PORTAL_PREVIEW} more`;
      more.title = "Show all related views";
      more.addEventListener("click", (e) => {
        e.stopPropagation();
        expanded = true;
        render();
      });
      strip.appendChild(more);
    } else if (expanded && f.portals.length > PORTAL_PREVIEW) {
      const fewer = document.createElement("button");
      fewer.type = "button";
      fewer.className = "portal portal-more";
      fewer.textContent = "Show less";
      fewer.title = "Collapse related views";
      fewer.addEventListener("click", (e) => {
        e.stopPropagation();
        expanded = false;
        render();
      });
      strip.appendChild(fewer);
    }
  };

  render();
  return strip;
}

function makeFrameEl(id: string, isFocus: boolean, isAncestor: boolean) {
  const f = frame(id);
  const wrap = document.createElement("div");
  wrap.className = `frame-wrap${isFocus ? " is-focus" : ""}${isAncestor ? " is-ancestor" : ""}`;
  wrap.dataset.id = id;

  const inner = document.createElement("div");
  inner.className = `frame${isFocus ? " is-focus" : ""}`;

  const header = document.createElement("div");
  header.className = "frame-header";
  const showFqn = f.fqn && f.fqn !== f.name;
  header.innerHTML = `<span class="kind-chip">${escapeHtml(f.kind_tag)}</span><span class="frame-title">${escapeHtml(f.name)}</span>${showFqn ? `<span class="frame-fqn">${escapeHtml(f.fqn)}</span>` : ""}`;
  if (isFocus && f.parent) {
    const parent = f.parent;
    header.style.cursor = "pointer";
    header.title = "Zoom out";
    header.addEventListener("click", (e) => {
      e.stopPropagation();
      flyTo(parent);
    });
  }
  inner.appendChild(header);

  const body = document.createElement("div");
  body.className = "frame-body";
  body.dataset.detailHost = id;
  inner.appendChild(body);

  // Portals only on the focused frame. Child previews stay as clean cards so
  // dozens of "calls from …" chips don't paint over the sibling grid.
  if (isFocus && f.portals.length > 0) {
    inner.appendChild(buildPortalStrip(f));
  }
  wrap.appendChild(inner);

  if (!isFocus && !isAncestor) {
    wrap.style.cursor = "pointer";
    wrap.addEventListener("click", (e) => {
      e.stopPropagation();
      flyTo(id);
    });
  }

  return wrap;
}

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
    void maybeQueueReviewForFocus(focusId);
  });
}

function zoomOut() {
  const parent = frame(focusId).parent;
  if (!parent) {
    return;
  }
  flyTo(parent);
}

function easeOutCubic(t: number) {
  return 1 - (1 - t) ** 3;
}

function tweenCamera(next: Camera, onDone?: () => void) {
  window.cancelAnimationFrame(cameraRaf);
  const from = { ...camera };
  const start = performance.now();

  const tick = (now: number) => {
    const t = Math.min(1, (now - start) / TWEEN_MS);
    const e = easeOutCubic(t);
    camera = {
      scale: from.scale + (next.scale - from.scale) * e,
      tx: from.tx + (next.tx - from.tx) * e,
      ty: from.ty + (next.ty - from.ty) * e,
    };
    placeAllFrames();
    if (t < 1) {
      cameraRaf = window.requestAnimationFrame(tick);
    } else {
      camera = next;
      placeAllFrames();
      onDone?.();
    }
  };
  cameraRaf = window.requestAnimationFrame(tick);
}

function updateChrome() {
  crumbEl.textContent = ancestors(focusId)
    .map((id) => frame(id).name)
    .join(" / ");
  zoomOutBtn.disabled = !frame(focusId).parent;
}

async function loadDetail(id: string) {
  const f = frame(id);
  const host = worldEl.querySelector(`[data-detail-host="${cssEscape(id)}"]`);
  if (!host) {
    return;
  }

  let detail = detailCache.get(id);
  if (!detail) {
    try {
      detail = await fetchJson<FrameDetail>(`frames/${id}.json`);
      detailCache.set(id, detail);
    } catch (err) {
      console.error(err);
      return;
    }
  }

  const isContainer = f.children.length > 0;
  // Containers keep the child grid — inject execution above it.
  if (isContainer) {
    for (const el of host.querySelectorAll(":scope > .detail")) {
      el.remove();
    }
    if (detail.execution_mermaid) {
      const panel = document.createElement("div");
      panel.className = "detail detail-execution";
      appendExecution(panel, detail.execution_mermaid, id);
      const grid = host.querySelector(".child-grid");
      if (grid) {
        host.insertBefore(panel, grid);
      } else {
        host.prepend(panel);
      }
    }
    return;
  }

  host.innerHTML = "";

  const panel = document.createElement("div");
  panel.className = "detail";

  const meta = document.createElement("div");
  meta.className = "meta";
  meta.textContent = [detail.kind, detail.signature, detail.location, detail.provenance]
    .filter(Boolean)
    .join(" · ");
  panel.appendChild(meta);

  if (detail.docs) {
    const docs = document.createElement("div");
    docs.className = "docs";
    docs.textContent = detail.docs;
    panel.appendChild(docs);
  }

  if (detail.execution_mermaid) {
    appendExecution(panel, detail.execution_mermaid, id);
  }

  if (detail.process_mermaid) {
    const proc = document.createElement("div");
    proc.className = "process";
    const heading = document.createElement("h2");
    heading.textContent = "Process";
    proc.appendChild(heading);
    if (detail.process) {
      const summary = document.createElement("div");
      summary.className = "meta";
      summary.textContent = `${detail.process.step_count} steps · ${detail.process.observed_count} observed`;
      proc.appendChild(summary);
    }
    const diagram = document.createElement("div");
    diagram.className = "process-mermaid";
    proc.appendChild(diagram);
    panel.appendChild(proc);
    void renderMermaid(diagram, detail.process_mermaid, `${id}-process`);
  }

  if (detail.source) {
    const src = document.createElement("div");
    src.className = "source";
    src.innerHTML = `<div class="file">${escapeHtml(detail.source.file)}</div>`;
    const pre = document.createElement("pre");
    pre.textContent = detail.source.text;
    src.appendChild(pre);
    panel.appendChild(src);
  }

  if (!detail.execution_mermaid && !detail.process_mermaid && !detail.source && !detail.docs) {
    const empty = document.createElement("div");
    empty.className = "empty-leaf";
    empty.textContent = "No process or source on this node.";
    panel.appendChild(empty);
  }

  host.appendChild(panel);
}

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

let mermaidSeq = 0;
let mermaidReady: Promise<typeof import("mermaid")> | null = null;

function loadMermaid(): Promise<typeof import("mermaid")> {
  if (!mermaidReady) {
    mermaidReady = import("mermaid").then((mod) => {
      mod.default.initialize({
        startOnLoad: false,
        theme: "dark",
        securityLevel: "strict",
        flowchart: { htmlLabels: false },
      });
      return mod;
    });
  }
  return mermaidReady;
}

async function renderMermaid(host: HTMLElement, definition: string, frameId: string) {
  try {
    const { default: mermaid } = await loadMermaid();
    const renderId = `mmd-${frameId.replace(/[^a-zA-Z0-9_-]/g, "_")}-${++mermaidSeq}`;
    const { svg } = await mermaid.render(renderId, definition);
    host.innerHTML = svg;
  } catch (err) {
    console.error(err);
    const pre = document.createElement("pre");
    pre.className = "process-mermaid-fallback";
    pre.textContent = definition;
    host.replaceChildren(pre);
  }
}

async function fetchJson<T>(path: string): Promise<T> {
  const res = await fetch(path);
  if (!res.ok) {
    throw new Error(`${path}: ${res.status}`);
  }
  return (await res.json()) as T;
}

function requireElement(id: string) {
  const el = document.getElementById(id);
  if (!(el instanceof HTMLElement)) {
    throw new Error(`missing #${id}`);
  }
  return el;
}

function requireButton(id: string) {
  const el = document.getElementById(id);
  if (!(el instanceof HTMLButtonElement)) {
    throw new Error(`missing button #${id}`);
  }
  return el;
}

function escapeHtml(s: string) {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function cssEscape(s: string) {
  if (window.CSS && typeof CSS.escape === "function") {
    return CSS.escape(s);
  }
  return s.replace(/"/g, '\\"');
}

/* ── Runs / Reviews dock (shared RunRecord) ─────────────────────────── */

function initRunsDock() {
  runsDockCloseBtn.addEventListener("click", () => setRunsDockOpen(false));
  for (const btn of runsDock.querySelectorAll<HTMLButtonElement>(".runs-filter")) {
    btn.addEventListener("click", () => {
      const filter = btn.dataset.filter;
      if (filter !== "reviews" && filter !== "all") {
        return;
      }
      runsFilter = filter;
      for (const other of runsDock.querySelectorAll(".runs-filter")) {
        other.classList.toggle("is-active", other === btn);
      }
      renderRunsDock();
    });
  }
}

function toggleRunsDock() {
  setRunsDockOpen(!runsDockOpen);
}

function setRunsDockOpen(open: boolean) {
  runsDockOpen = open;
  runsDock.hidden = !open;
  orgRunsBtn.classList.toggle("is-active", open);
  if (open) {
    void refreshRunsIndex();
  }
}

function startRunsPoll() {
  window.clearInterval(runsPollTimer);
  runsPollTimer = window.setInterval(() => {
    void refreshRunsIndex();
  }, RUNS_POLL_MS);
  void refreshRunsIndex();
}

async function refreshRunsIndex() {
  try {
    runsIndex = await fetchJson<RunsIndex>("/api/runs");
  } catch {
    try {
      runsIndex = await fetchJson<RunsIndex>("runs/index.json");
    } catch (err) {
      console.error(err);
      return;
    }
  }
  if (runsDockOpen) {
    renderRunsDock();
  }
}

function filteredRuns(): RunIndexEntry[] {
  if (runsFilter === "all") {
    return runsIndex.runs;
  }
  return runsIndex.runs.filter((r) => r.recipe === RECIPE_REVIEW_VIEW);
}

function renderRunsDock() {
  const rows = filteredRuns();
  const pending = rows.filter((r) => r.status === "queued");
  const active = rows.filter((r) => r.status === "running");
  const recent = rows.filter(
    (r) => r.status !== "queued" && r.status !== "running",
  ).slice(0, 12);
  fillRunsList(runsPendingEl, pending, true);
  fillRunsList(runsActiveEl, active, true);
  fillRunsList(runsRecentEl, recent, false);
}

function fillRunsList(host: HTMLElement, entries: RunIndexEntry[], actionable: boolean) {
  host.innerHTML = "";
  for (const entry of entries) {
    host.appendChild(makeRunsRow(entry, actionable));
  }
}

function makeRunsRow(entry: RunIndexEntry, actionable: boolean): HTMLLIElement {
  const li = document.createElement("li");
  li.className = "runs-row";
  li.dataset.runId = entry.run_id;

  const subjectFrame = entry.subject ? scene?.frames[entry.subject] : undefined;
  const name = subjectFrame?.name ?? entry.subject ?? entry.run_id;
  const main = document.createElement("div");
  main.className = "runs-row-main";
  main.innerHTML =
    `<div class="runs-row-title">${escapeHtml(name)}</div>` +
    `<div class="runs-row-meta">` +
    `<span class="status-chip ${escapeHtml(entry.status)}">${escapeHtml(entry.status)}</span>` +
    ` · ${escapeHtml(shortRecipe(entry.recipe))}` +
    (entry.started_at ? ` · ${relativeTime(entry.started_at)}` : "") +
    `</div>`;
  li.appendChild(main);

  if (actionable && entry.recipe === RECIPE_REVIEW_VIEW) {
    const actions = document.createElement("div");
    actions.className = "runs-row-actions";
    if (entry.status === "queued") {
      const allow = document.createElement("button");
      allow.type = "button";
      allow.textContent = "Allow";
      allow.title = "Continue review (stub in this slice)";
      allow.addEventListener("click", (e) => {
        e.stopPropagation();
        void postRunAction(entry.run_id, "allow");
      });
      actions.appendChild(allow);

      const cancel = document.createElement("button");
      cancel.type = "button";
      cancel.className = "danger";
      cancel.textContent = "Cancel";
      cancel.addEventListener("click", (e) => {
        e.stopPropagation();
        void postRunAction(entry.run_id, "cancel");
      });
      actions.appendChild(cancel);
    } else if (entry.status === "running") {
      const cancel = document.createElement("button");
      cancel.type = "button";
      cancel.className = "danger";
      cancel.textContent = "Cancel";
      cancel.addEventListener("click", (e) => {
        e.stopPropagation();
        void postRunAction(entry.run_id, "cancel");
      });
      actions.appendChild(cancel);
    }
    li.appendChild(actions);
  }

  if (entry.subject && scene?.frames[entry.subject]) {
    li.style.cursor = "pointer";
    li.title = "Fly to subject";
    li.addEventListener("click", () => {
      flyTo(entry.subject!);
    });
  }

  return li;
}

function shortRecipe(recipe: string): string {
  if (recipe === RECIPE_REVIEW_VIEW) {
    return "review_view";
  }
  const parts = recipe.split(".");
  return parts[parts.length - 1] ?? recipe;
}

function relativeTime(ms: number): string {
  const delta = Math.max(0, Date.now() - ms);
  if (delta < 60_000) {
    return `${Math.round(delta / 1000)}s ago`;
  }
  if (delta < 3_600_000) {
    return `${Math.round(delta / 60_000)}m ago`;
  }
  return `${Math.round(delta / 3_600_000)}h ago`;
}

async function postRunAction(runId: string, action: "allow" | "cancel") {
  try {
    const res = await fetch(`/api/runs/${encodeURIComponent(runId)}/${action}`, {
      method: "POST",
    });
    if (!res.ok) {
      const body = (await res.json().catch(() => null)) as { error?: string } | null;
      throw new Error(body?.error ?? `${action} failed: ${res.status}`);
    }
    const record = (await res.json()) as RunRecord;
    if (action === "cancel" && record.subject) {
      cancelCooldownUntil.set(record.subject, Date.now() + REVIEW_CANCEL_COOLDOWN_MS);
    }
    await refreshRunsIndex();
  } catch (err) {
    console.error(err);
  }
}

async function maybeQueueReviewForFocus(id: string) {
  if (!scene || !scene.frames[id]) {
    return;
  }
  if (id === scene.root) {
    return;
  }
  const until = cancelCooldownUntil.get(id) ?? 0;
  if (Date.now() < until) {
    return;
  }
  const f = frame(id);
  try {
    const res = await fetch("/api/reviews/queue", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        subject: id,
        trigger: "frame_focus",
        subject_name: f.name,
      }),
    });
    if (!res.ok) {
      const body = (await res.json().catch(() => null)) as { error?: string } | null;
      throw new Error(body?.error ?? `queue failed: ${res.status}`);
    }
    await refreshRunsIndex();
    if (!runsDockOpen) {
      // Nudge: open dock so the pending review is visible.
      setRunsDockOpen(true);
    }
  } catch (err) {
    console.error(err);
  }
}

void main();
