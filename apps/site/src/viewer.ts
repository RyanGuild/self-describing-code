/**
 * Prezi-style camera over the Robby scene (flat world coordinates).
 *
 * Loads `scene.json` and lazy `frames/<id>.json` payloads written by
 * `robby web`. Click a card to zoom in; portals jump to related frames;
 * Escape / back zooms out. Focus (and org mode) live in the URL hash as
 * `#frame=<stable_id>&org=…` so reload and watch remount keep the camera.
 *
 * Frames stay laid out at full canvas resolution (1600×1000). A single
 * screen-space transform per frame (camera × world placement) avoids the
 * downscale-then-upscale rasterization that keeps text pixelated after zoom.
 *
 * Soft-queues `robby.review_view` RunRecords on frame focus (Mac-local Vite
 * API → `public/runs/`). Runs mode can Allow or Cancel pending reviews.
 */

import { RECIPE_CURSOR_CLOUD, RECIPE_REVIEW_VIEW, runFileStem } from "./runRecord";

const CANVAS_W = 1600;
const CANVAS_H = 1000;
const TWEEN_MS = 500;
const REVIEW_CANCEL_COOLDOWN_MS = 8_000;

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

interface Scene {
  root: string;
  canvas: { w: number; h: number };
  frames: Record<string, SceneFrame>;
}

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

interface FrameDetail {
  stable_id: string;
  id: string;
  fqn: string;
  kind: string;
  signature?: string;
  docs?: string;
  location?: string;
  provenance: string;
  confidence: number;
  process?: {
    participants: { id: string; label: string }[];
    events: ProcessEvent[];
    step_count: number;
    observed_count: number;
  };
  /** Mermaid sequenceDiagram from robby-render (same as Markdown). */
  process_mermaid?: string;
  /** Path-level Calls sequence (functions + owning types). */
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

type SceneOrgMode = keyof typeof ORG_MODES;
type OrgMode = SceneOrgMode | "runs";

interface BuildStamp {
  rebuilt_at: number;
  frames: number;
  nodes?: number;
  files?: number;
  generator?: string;
}

interface RunIndexEntry {
  run_id: string;
  recipe: string;
  status: string;
  subject?: string;
  started_at?: number;
  finished_at?: number;
}

interface RunsIndex {
  runs: RunIndexEntry[];
}

interface RunStep {
  from: string;
  to: string;
  label: string;
  observed?: boolean;
}

/** Mirrors `robby_ir::run::RunRecord` (site JSON). */
interface RunRecord {
  run_id: string;
  recipe: string;
  subject?: string;
  status: string;
  started_at?: number;
  finished_at?: number;
  exit_code?: number;
  stdout_tail?: string;
  stats?: Record<string, string>;
  products?: string[];
  process?: {
    participants: { id: string; label: string; kind?: string }[];
    steps: RunStep[];
  };
}

const HINT_RUNS = "pipeline + cursor.cloud + reviews · Allow/Cancel review_view · live rows poll";
const WATCH_POLL_MS = 1500;
const RUNS_POLL_MS = 2000;

type RunsFilter = "all" | "live" | "cursor" | "reviews";

const worldEl = requireElement("world");
const viewportEl = requireElement("viewport");
const crumbEl = requireElement("crumb");
const hintEl = requireElement("hint");
const watchBadgeEl = requireElement("watch-badge");
const zoomOutBtn = requireButton("zoom-out");
const orgStructureBtn = requireButton("org-structure");
const orgEntrypointsBtn = requireButton("org-entrypoints");
const orgRunsBtn = requireButton("org-runs");
const runsPanelEl = requireElement("runs-panel");
const runsListEl = requireElement("runs-list");
const runsDetailEl = requireElement("runs-detail");

let scene: Scene | null = null;
let orgMode: OrgMode = "structure";
const sceneCache = new Map<SceneOrgMode, Scene>();
let focusId = "";
const detailCache = new Map<string, FrameDetail>();
let flying = false;
let camera: Camera = { scale: 1, tx: 0, ty: 0 };
let cameraRaf = 0;
let lastBuildAt: number | null = null;
let reloading = false;
let runsIndex: RunsIndex | null = null;
let selectedRunId: string | null = null;
const runDetailCache = new Map<string, RunRecord>();
let runsFilter: RunsFilter = "all";
let lastRunsIndexJson = "";
const cancelCooldownUntil = new Map<string, number>();

async function main(): Promise<void> {
  try {
    const structure = await fetchJson<Scene>(ORG_MODES.structure.file);
    sceneCache.set("structure", structure);
    scene = structure;
  } catch (err) {
    worldEl.textContent = "Could not load scene.json. Run `robby web` then serve this app (Vite).";
    console.error(err);
    return;
  }

  const saved = readLocationHash();
  let initialOrg: OrgMode = saved.org ?? "structure";
  if (initialOrg === "runs") {
    await prefetchRuns();
    void prefetchEntrypoints();
    orgMode = "runs";
    syncOrgChrome("runs");
    showRunsPanel();
    syncLocationHash();
  } else if (initialOrg === "entrypoints") {
    await prefetchEntrypoints();
    if (!sceneCache.has("entrypoints")) {
      initialOrg = "structure";
    }
    void prefetchRuns();
    orgMode = initialOrg;
    syncOrgChrome(initialOrg);
    const initialScene = sceneCache.get(initialOrg) ?? scene;
    applyScene(initialScene, resolveFocusId(initialScene, saved.frame));
  } else {
    void prefetchEntrypoints();
    void prefetchRuns();
    orgMode = "structure";
    syncOrgChrome("structure");
    applyScene(scene, resolveFocusId(scene, saved.frame));
  }
  void startBuildWatch();
  void startRunsWatch();
  initRunsFilters();
  syncRunsFilterButtons();

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
  window.addEventListener("hashchange", () => {
    void applyLocationFromHash();
  });
}

/** Poll `build.json` written by `robby web` / `--watch`; remount when IR site rebuilds. */
async function startBuildWatch(): Promise<void> {
  const tick = async () => {
    try {
      const stamp = await fetchJson<BuildStamp>(`build.json?t=${Date.now()}`);
      if (lastBuildAt === null) {
        lastBuildAt = stamp.rebuilt_at;
        showWatchBadge(stamp, false);
      } else if (stamp.rebuilt_at !== lastBuildAt) {
        lastBuildAt = stamp.rebuilt_at;
        await reloadFromDisk(stamp);
      }
    } catch {
      // build.json optional until first `robby web` with stamp support
    }
  };
  await tick();
  window.setInterval(() => {
    void tick();
  }, WATCH_POLL_MS);
}

async function reloadFromDisk(stamp: BuildStamp): Promise<void> {
  if (reloading || flying) {
    return;
  }
  reloading = true;
  const keepFocus = focusId;
  const keepOrg = orgMode;
  const hash = readLocationHash();
  // Prefer in-memory focus when it still matches the hash (watch remount);
  // otherwise honor the hash so a mid-reload deep link is not lost.
  const preferred = keepFocus && (!hash.frame || hash.frame === keepFocus) ? keepFocus : hash.frame;
  try {
    detailCache.clear();
    sceneCache.clear();
    runDetailCache.clear();
    const structure = await fetchJson<Scene>(`${ORG_MODES.structure.file}?t=${Date.now()}`);
    sceneCache.set("structure", structure);
    try {
      const entrypoints = await fetchJson<Scene>(`${ORG_MODES.entrypoints.file}?t=${Date.now()}`);
      sceneCache.set("entrypoints", entrypoints);
    } catch (err) {
      console.error(err);
    }
    await prefetchRuns(true);
    orgMode = keepOrg;
    syncOrgChrome(keepOrg);
    if (keepOrg === "runs") {
      showRunsPanel();
      syncLocationHash();
    } else {
      const next = sceneCache.get(keepOrg) ?? structure;
      applyScene(next, resolveFocusId(next, preferred));
    }
    showWatchBadge(stamp, true);
  } catch (err) {
    console.error(err);
  } finally {
    reloading = false;
  }
}

function showWatchBadge(stamp: BuildStamp, justRebuilt: boolean): void {
  const nodes = stamp.nodes ?? "?";
  const label = justRebuilt
    ? `IR rebuilt · ${stamp.frames} frames · ${nodes} nodes`
    : `live · ${stamp.frames} frames · ${nodes} nodes`;
  watchBadgeEl.textContent = label;
  watchBadgeEl.hidden = false;
}

async function prefetchEntrypoints(): Promise<void> {
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

async function prefetchRuns(force = false): Promise<boolean> {
  if (runsIndex && !force) {
    return false;
  }
  try {
    let raw: string;
    try {
      raw = await fetchText(`/api/runs?t=${Date.now()}`);
    } catch {
      raw = await fetchText(`runs/index.json?t=${Date.now()}`);
    }
    const changed = raw !== lastRunsIndexJson;
    if (!force && !changed) {
      return false;
    }
    lastRunsIndexJson = raw;
    runsIndex = JSON.parse(raw) as RunsIndex;
    orgRunsBtn.disabled = false;
    orgRunsBtn.title = "";
    updateRunsLiveChrome();
  } catch (err) {
    console.error(err);
    runsIndex = { runs: [] };
    lastRunsIndexJson = "";
    orgRunsBtn.title = "runs/index.json missing — re-run robby web";
    return false;
  }
  if (orgMode === "runs") {
    renderRunsList();
  }
  return true;
}

/** Poll runs so cursor.cloud live rows refresh without an IR rebuild. */
async function startRunsWatch(): Promise<void> {
  window.setInterval(() => {
    void (async () => {
      const changed = await prefetchRuns(true);
      if (changed && orgMode === "runs") {
        runDetailCache.clear();
        showRunsPanel();
      } else {
        updateRunsLiveChrome();
      }
    })();
  }, RUNS_POLL_MS);
}

function updateRunsLiveChrome(): void {
  const live = (runsIndex?.runs ?? []).filter(
    (r) => r.status === "running" || r.status === "queued",
  ).length;
  if (live > 0) {
    orgRunsBtn.dataset.live = String(live);
    orgRunsBtn.title = `${live} live run${live === 1 ? "" : "s"}`;
  } else {
    delete orgRunsBtn.dataset.live;
  }
}

async function fetchText(path: string): Promise<string> {
  const res = await fetch(path);
  if (!res.ok) {
    throw new Error(`${path}: ${res.status}`);
  }
  return res.text();
}

function syncRunsFilterButtons(): void {
  for (const btn of runsPanelEl.querySelectorAll<HTMLButtonElement>(".runs-filter")) {
    btn.classList.toggle("is-active", btn.dataset.filter === runsFilter);
  }
}

function syncOrgChrome(mode: OrgMode): void {
  orgStructureBtn.classList.toggle("is-active", mode === "structure");
  orgEntrypointsBtn.classList.toggle("is-active", mode === "entrypoints");
  orgRunsBtn.classList.toggle("is-active", mode === "runs");
  if (mode === "runs") {
    hintEl.textContent = HINT_RUNS;
  } else {
    hintEl.textContent = ORG_MODES[mode].hint;
  }
  viewportEl.classList.toggle("runs-open", mode === "runs");
  runsPanelEl.hidden = mode !== "runs";
  zoomOutBtn.disabled = mode === "runs" || !scene || !frameSafeParent();
}

function frameSafeParent(): boolean {
  try {
    return Boolean(scene && frame(focusId).parent);
  } catch {
    return false;
  }
}

async function setOrgMode(next: OrgMode): Promise<void> {
  if (next === orgMode || flying) {
    return;
  }
  if (next === "runs") {
    await prefetchRuns();
    orgMode = "runs";
    syncOrgChrome("runs");
    showRunsPanel();
    syncLocationHash();
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
  syncOrgChrome(next);
  applyScene(cached);
}

/**
 * URL hash: `#frame=<stable_id>` and optional `&org=structure|entrypoints|runs`.
 * Updated with replaceState so navigation does not spam history.
 */
interface LocationHash {
  frame?: string;
  org?: OrgMode;
}

function readLocationHash(): LocationHash {
  const raw = window.location.hash.replace(/^#/, "").trim();
  if (!raw) {
    return {};
  }
  const params = new URLSearchParams(raw.includes("=") ? raw : `frame=${raw}`);
  const frame = params.get("frame") || params.get("location") || undefined;
  const orgRaw = params.get("org");
  const org: OrgMode | undefined =
    orgRaw === "entrypoints" || orgRaw === "structure" || orgRaw === "runs" ? orgRaw : undefined;
  const out: LocationHash = {};
  if (frame) {
    out.frame = frame;
  }
  if (org) {
    out.org = org;
  }
  return out;
}

function locationHashFor(focus: string, org: OrgMode): string {
  const params = new URLSearchParams();
  params.set("frame", focus);
  if (org !== "structure") {
    params.set("org", org);
  }
  return `#${params.toString()}`;
}

function syncLocationHash(): void {
  if (!focusId) {
    return;
  }
  const next = locationHashFor(focusId, orgMode);
  if (window.location.hash === next) {
    return;
  }
  const url = `${window.location.pathname}${window.location.search}${next}`;
  history.replaceState(null, "", url);
}

function resolveFocusId(next: Scene, preferred?: string | null): string {
  if (preferred && next.frames[preferred]) {
    return preferred;
  }
  return next.root;
}

/** Apply hash when the user edits the URL (back/forward on hash alone). */
async function applyLocationFromHash(): Promise<void> {
  if (!scene || flying || reloading) {
    return;
  }
  const saved = readLocationHash();
  const wantOrg = saved.org ?? "structure";
  if (wantOrg !== orgMode) {
    if (wantOrg === "runs") {
      await prefetchRuns();
      orgMode = "runs";
      syncOrgChrome("runs");
      showRunsPanel();
      syncLocationHash();
      return;
    }
    if (wantOrg === "entrypoints" && !sceneCache.has("entrypoints")) {
      await prefetchEntrypoints();
    }
    const cached = sceneCache.get(wantOrg);
    if (!cached) {
      return;
    }
    orgMode = wantOrg;
    syncOrgChrome(wantOrg);
    applyScene(cached, resolveFocusId(cached, saved.frame));
    return;
  }
  const target = resolveFocusId(scene, saved.frame);
  if (target === focusId) {
    syncLocationHash();
    return;
  }
  focusId = target;
  camera = cameraForFocus(focusId);
  remount();
  void loadDetail(focusId);
  syncLocationHash();
}

function applyScene(next: Scene, preferredFocus?: string | null): void {
  scene = next;
  flying = false;
  window.cancelAnimationFrame(cameraRaf);
  focusId = resolveFocusId(next, preferredFocus);
  camera = cameraForFocus(focusId);
  remount();
  void loadDetail(focusId);
  syncLocationHash();
}

function showRunsPanel(): void {
  syncOrgChrome("runs");
  crumbEl.textContent = "Runs";
  renderRunsList();
  if (selectedRunId) {
    void selectRun(selectedRunId);
  } else {
    runsDetailEl.hidden = true;
    runsDetailEl.innerHTML = "";
  }
}

function filteredRunEntries(): RunIndexEntry[] {
  const runs = runsIndex?.runs ?? [];
  if (runsFilter === "all") {
    return runs;
  }
  if (runsFilter === "live") {
    return runs.filter((r) => r.status === "running" || r.status === "queued");
  }
  if (runsFilter === "cursor") {
    return runs.filter((r) => r.recipe === RECIPE_CURSOR_CLOUD || r.recipe.startsWith("cursor."));
  }
  return runs.filter((r) => r.recipe === RECIPE_REVIEW_VIEW);
}

function renderRunsList(): void {
  runsListEl.innerHTML = "";
  const all = runsIndex?.runs ?? [];
  const runs = filteredRunEntries();
  if (all.length === 0) {
    const empty = document.createElement("div");
    empty.className = "runs-empty";
    empty.textContent =
      "No RunRecords yet. Run `robby web` or `robby ingest cursor --from agents.json`.";
    runsListEl.appendChild(empty);
    return;
  }
  if (runs.length === 0) {
    const empty = document.createElement("div");
    empty.className = "runs-empty";
    empty.textContent =
      runsFilter === "reviews"
        ? "No reviews yet. Focus a map frame to soft-queue robby.review_view."
        : "No runs match this filter.";
    runsListEl.appendChild(empty);
    return;
  }
  for (const entry of runs) {
    const btn = document.createElement("button");
    btn.type = "button";
    btn.className = `run-row${entry.run_id === selectedRunId ? " is-selected" : ""}`;
    btn.setAttribute("role", "listitem");
    const subject = shortRunLabel(entry);
    const started = entry.started_at ?? 0;
    const finished = entry.finished_at ?? entry.started_at ?? 0;
    const live = entry.status === "running" || entry.status === "queued";
    const timeLabel = live
      ? "live"
      : `${formatDuration(started, finished)} · ${formatRelative(finished || started)}`;
    btn.innerHTML =
      `<span class="run-status ${statusClass(entry.status)}">${escapeHtml(entry.status)}</span>` +
      `<span class="run-main">` +
      `<span class="run-recipe">${escapeHtml(entry.recipe)}</span>` +
      `<span class="run-subject">${escapeHtml(subject)}</span>` +
      `</span>` +
      `<span class="run-time">${escapeHtml(timeLabel)}</span>`;
    btn.addEventListener("click", () => {
      void selectRun(entry.run_id);
    });
    runsListEl.appendChild(btn);
  }
}

function shortRunLabel(entry: RunIndexEntry): string {
  if (entry.run_id.startsWith("cursor:")) {
    return entry.run_id.slice("cursor:".length);
  }
  return entry.subject ?? "—";
}

async function selectRun(runId: string): Promise<void> {
  selectedRunId = runId;
  renderRunsList();
  let detail = runDetailCache.get(runId);
  if (!detail) {
    try {
      const stem = runFileStem(runId);
      detail = await fetchJson<RunRecord>(`runs/${encodeURIComponent(stem)}.json?t=${Date.now()}`);
      runDetailCache.set(runId, detail);
    } catch (err) {
      console.error(err);
      runsDetailEl.hidden = false;
      runsDetailEl.innerHTML = `<div class="runs-empty">Could not load runs/${escapeHtml(runFileStem(runId))}.json</div>`;
      return;
    }
  }
  renderRunDetail(detail);
}

function renderRunDetail(detail: RunRecord): void {
  runsDetailEl.hidden = false;
  const started = detail.started_at ?? 0;
  const finished = detail.finished_at ?? detail.started_at ?? 0;
  const live = detail.status === "running" || detail.status === "queued";
  const duration = live ? "live" : formatDuration(started, finished);
  const stats = detail.stats
    ? Object.entries(detail.stats)
        .map(([k, v]) => `<dt>${escapeHtml(k)}</dt><dd>${escapeHtml(String(v))}</dd>`)
        .join("")
    : "";
  const products = (detail.products ?? [])
    .map((p) => {
      if (p.startsWith("url:")) {
        const href = p.slice(4);
        return `<li><a href="${escapeHtml(href)}" target="_blank" rel="noreferrer">${escapeHtml(href)}</a></li>`;
      }
      return `<li>${escapeHtml(p)}</li>`;
    })
    .join("");
  const stages = (detail.process?.steps ?? [])
    .map(
      (s) =>
        `<li>${escapeHtml(s.from)} → ${escapeHtml(s.to)}` +
        (s.label && s.label !== s.to ? ` · ${escapeHtml(s.label)}` : "") +
        (s.observed ? "" : " (planned)") +
        `</li>`,
    )
    .join("");
  const canGate =
    detail.recipe === RECIPE_REVIEW_VIEW &&
    (detail.status === "running" || detail.status === "queued");
  runsDetailEl.innerHTML =
    `<h2>${escapeHtml(detail.recipe)}</h2>` +
    `<div class="meta"><span class="run-status ${statusClass(detail.status)}">${escapeHtml(detail.status)}</span>` +
    ` · ${escapeHtml(detail.run_id)} · ${escapeHtml(duration)}</div>` +
    (canGate
      ? `<div class="run-actions" data-run-actions="${escapeHtml(detail.run_id)}"></div>`
      : "") +
    `<dl class="run-kv">` +
    `<dt>subject</dt><dd>${escapeHtml(detail.subject ?? "—")}</dd>` +
    `<dt>started</dt><dd>${escapeHtml(formatAbsolute(started))}</dd>` +
    `<dt>${live ? "updated" : "finished"}</dt><dd>${escapeHtml(detail.finished_at !== undefined ? formatAbsolute(detail.finished_at) : "—")}</dd>` +
    (detail.exit_code !== undefined
      ? `<dt>exit_code</dt><dd>${escapeHtml(String(detail.exit_code))}</dd>`
      : "") +
    stats +
    `</dl>` +
    (detail.stdout_tail ? `<div class="stdout">${escapeHtml(detail.stdout_tail)}</div>` : "") +
    (products ? `<div class="stages"><h2>Products</h2><ol>${products}</ol></div>` : "") +
    (stages ? `<div class="stages"><h2>Stages</h2><ol>${stages}</ol></div>` : "");

  const actions = runsDetailEl.querySelector("[data-run-actions]");
  if (actions && canGate) {
    if (detail.status === "queued") {
      const allow = document.createElement("button");
      allow.type = "button";
      allow.textContent = "Allow / Continue";
      allow.title = "Start review (LLM stubbed in this slice)";
      allow.addEventListener("click", () => {
        void postRunAction(detail.run_id, "allow");
      });
      actions.appendChild(allow);
    }
    const cancel = document.createElement("button");
    cancel.type = "button";
    cancel.className = "danger";
    cancel.textContent = "Cancel";
    cancel.addEventListener("click", () => {
      void postRunAction(detail.run_id, "cancel");
    });
    actions.appendChild(cancel);
  }
}

function statusClass(status: string): string {
  if (status === "fail" || status === "error") {
    return `is-${status}`;
  }
  if (status === "running" || status === "queued") {
    return `is-${status}`;
  }
  return "is-pass";
}

function formatDuration(startedAt: number, finishedAt: number): string {
  const ms = Math.max(0, finishedAt - startedAt);
  if (ms < 1000) {
    return `${ms}ms`;
  }
  if (ms < 60_000) {
    return `${(ms / 1000).toFixed(1)}s`;
  }
  return `${Math.round(ms / 60_000)}m`;
}

function formatRelative(ts: number): string {
  const delta = Date.now() - ts;
  if (delta < 60_000) {
    return "just now";
  }
  if (delta < 3_600_000) {
    return `${Math.round(delta / 60_000)}m ago`;
  }
  if (delta < 86_400_000) {
    return `${Math.round(delta / 3_600_000)}h ago`;
  }
  return `${Math.round(delta / 86_400_000)}d ago`;
}

function formatAbsolute(ts: number): string {
  try {
    return new Date(ts).toISOString();
  } catch {
    return String(ts);
  }
}

function currentScene(): Scene {
  if (!scene) {
    throw new Error("scene is not loaded");
  }
  return scene;
}

function frame(id: string): SceneFrame {
  const f = currentScene().frames[id];
  if (!f) {
    throw new Error(`missing frame ${id}`);
  }
  return f;
}

function ancestors(id: string): string[] {
  const path: string[] = [];
  let cur: string | null = id;
  while (cur) {
    path.push(cur);
    cur = frame(cur).parent ?? null;
  }
  return path.reverse();
}

function cameraForFocus(id: string): Camera {
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

interface ScreenRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

/** Absolute screen rect for a frame from its world coordinates + camera. */
function screenRectFromWorld(f: SceneFrame, cam: Camera): ScreenRect {
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
function screenRectFor(f: SceneFrame, cam: Camera): ScreenRect {
  if (f.parent === focusId && f.local_in_parent) {
    const parent = screenRectFromWorld(frame(focusId), cam);
    const loc = f.local_in_parent;
    return {
      x: parent.x + (loc.x / CANVAS_W) * parent.w,
      y: parent.y + (loc.y / CANVAS_H) * parent.h,
      w: Math.max((loc.w / CANVAS_W) * parent.w, 1),
      h: Math.max((loc.h / CANVAS_H) * parent.h, 1),
    };
  }
  return screenRectFromWorld(f, cam);
}

function placeFrame(el: HTMLElement, f: SceneFrame, cam: Camera): void {
  const r = screenRectFor(f, cam);
  el.style.width = `${r.w}px`;
  el.style.height = `${r.h}px`;
  el.style.left = `${r.x}px`;
  el.style.top = `${r.y}px`;
  el.style.transform = "none";
}

function placeAllFrames(): void {
  // Only top-level world cards (the focus). Children live in the body's grid.
  for (const el of worldEl.querySelectorAll<HTMLElement>(":scope > .frame-wrap")) {
    const id = el.dataset.id;
    if (!id) {
      continue;
    }
    placeFrame(el, frame(id), camera);
  }
}

function remount(): void {
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

function portalButton(portal: Portal): HTMLButtonElement {
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

function buildPortalStrip(f: SceneFrame): HTMLElement {
  const strip = document.createElement("div");
  strip.className = "portal-strip";
  let expanded = false;

  const syncFocusClass = (): void => {
    const focusWrap = strip.closest(".frame-wrap");
    focusWrap?.classList.toggle("portals-expanded", expanded);
  };

  const render = (): void => {
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

function makeFrameEl(id: string, isFocus: boolean, isAncestor: boolean): HTMLElement {
  const f = frame(id);
  const wrap = document.createElement("div");
  wrap.className = `frame-wrap${isFocus ? " is-focus" : ""}${isAncestor ? " is-ancestor" : ""}`;
  wrap.dataset.id = id;

  const inner = document.createElement("div");
  inner.className = `frame${isFocus ? " is-focus" : ""}`;

  const header = document.createElement("div");
  header.className = "frame-header";
  const showFqn = f.fqn && f.fqn !== f.name;
  header.innerHTML =
    `<span class="kind-chip">${escapeHtml(f.kind_tag)}</span>` +
    `<span class="frame-title">${escapeHtml(f.name)}</span>` +
    (showFqn ? `<span class="frame-fqn">${escapeHtml(f.fqn)}</span>` : "");
  if (isFocus && f.parent) {
    header.style.cursor = "pointer";
    header.title = "Zoom out";
    header.addEventListener("click", (e) => {
      e.stopPropagation();
      if (f.parent) {
        flyTo(f.parent);
      }
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

function flyTo(id: string): void {
  if (!scene?.frames[id] || id === focusId || flying) {
    return;
  }
  flying = true;
  focusId = id;
  syncLocationHash();
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
    syncLocationHash();
  });
}

function zoomOut(): void {
  const parent = frame(focusId).parent;
  if (!parent) {
    return;
  }
  flyTo(parent);
}

function easeOutCubic(t: number): number {
  return 1 - (1 - t) ** 3;
}

function tweenCamera(next: Camera, onDone?: () => void): void {
  window.cancelAnimationFrame(cameraRaf);
  const from = { ...camera };
  const start = performance.now();

  const tick = (now: number): void => {
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

function updateChrome(): void {
  crumbEl.textContent = ancestors(focusId)
    .map((id) => frame(id).name)
    .join(" / ");
  zoomOutBtn.disabled = !frame(focusId).parent;
}

async function loadDetail(id: string): Promise<void> {
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

function appendExecution(panel: HTMLElement, mermaidSrc: string, frameId: string): void {
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

async function renderMermaid(
  host: HTMLElement,
  definition: string,
  frameId: string,
): Promise<void> {
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

function requireElement(id: string): HTMLElement {
  const el = document.getElementById(id);
  if (!(el instanceof HTMLElement)) {
    throw new Error(`missing #${id}`);
  }
  return el;
}

function requireButton(id: string): HTMLButtonElement {
  const el = document.getElementById(id);
  if (!(el instanceof HTMLButtonElement)) {
    throw new Error(`missing button #${id}`);
  }
  return el;
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function cssEscape(s: string): string {
  if (window.CSS && typeof CSS.escape === "function") {
    return CSS.escape(s);
  }
  return s.replace(/"/g, '\\"');
}

function initRunsFilters(): void {
  for (const btn of runsPanelEl.querySelectorAll<HTMLButtonElement>(".runs-filter")) {
    btn.addEventListener("click", () => {
      const filter = btn.dataset.filter;
      if (filter !== "reviews" && filter !== "all" && filter !== "live" && filter !== "cursor") {
        return;
      }
      runsFilter = filter;
      syncRunsFilterButtons();
      renderRunsList();
    });
  }
}

async function postRunAction(runId: string, action: "allow" | "cancel"): Promise<void> {
  try {
    const res = await fetch(`/api/runs/${encodeURIComponent(runId)}/${action}`, {
      method: "POST",
    });
    if (!res.ok) {
      const body = (await res.json().catch(() => null)) as { error?: string } | null;
      throw new Error(body?.error ?? `${action} failed: ${res.status}`);
    }
    const record = (await res.json()) as RunRecord;
    runDetailCache.set(runId, record);
    if (action === "cancel" && record.subject) {
      cancelCooldownUntil.set(record.subject, Date.now() + REVIEW_CANCEL_COOLDOWN_MS);
    }
    await prefetchRuns(true);
    if (selectedRunId === runId) {
      renderRunDetail(record);
    }
  } catch (err) {
    console.error(err);
  }
}

async function maybeQueueReviewForFocus(id: string): Promise<void> {
  if (!scene?.frames[id] || id === scene.root) {
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
    const record = (await res.json()) as RunRecord;
    runDetailCache.set(record.run_id, record);
    await prefetchRuns(true);
    // Surface the pending review in Runs mode.
    if (orgMode !== "runs") {
      await setOrgMode("runs");
    }
    runsFilter = "reviews";
    for (const btn of runsPanelEl.querySelectorAll(".runs-filter")) {
      btn.classList.toggle("is-active", (btn as HTMLElement).dataset.filter === "reviews");
    }
    selectedRunId = record.run_id;
    showRunsPanel();
  } catch (err) {
    console.error(err);
  }
}

void main();
