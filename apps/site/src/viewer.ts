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
 * API → `public/runs/`) without leaving the map. Open Runs (dock) to Allow
 * or Cancel pending reviews.
 */

import {
  catalogSeedRows,
  companionStatKeys,
  expandFilesystemRows,
  fsScopeFor,
  isPrimaryMetricStatKey,
  type MetricObservation,
  type MetricRow,
  type MetricsSort,
  mergeMetricRows,
  metricCalculationDetail,
  metricRowKey,
  normalizePathHint,
  parseMetricProduct,
  primaryMetricIdFromStatKey,
  type SourceFileInventory,
  sortMetricRows,
} from "./metricsCatalog";
import {
  RECIPE_CURSOR_CLOUD,
  RECIPE_IMPROVE_PATCH,
  RECIPE_INTEGRATE_PATCH,
  RECIPE_METRIC_TEST,
  RECIPE_REVIEW_VIEW,
  runFileStem,
  type ScoredDecisionProduct,
} from "./runRecord";
import {
  modelFromMermaid,
  modelFromProcess,
  mountSequenceDiagram,
  type SeqModel,
  type SeqParticipant,
} from "./sequenceDiagram";

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

interface ProcessPayload {
  participants: { id: string; label: string; target?: string }[];
  events: ProcessEvent[];
  step_count: number;
  observed_count: number;
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
  process?: ProcessPayload;
  /** Mermaid sequenceDiagram for the definition Process (legacy / Markdown). */
  process_mermaid?: string;
  /** Call-path or observed-runtime sequence (structured; preferred by the site canvas). */
  execution?: ProcessPayload;
  /** Same sequence as Mermaid — distinct from process_mermaid (legacy payloads). */
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
type OrgMode = SceneOrgMode | "runs" | "metrics";

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
const HINT_METRICS =
  "improvement metrics · by filesystem shows which metrics apply to which source files";
const WATCH_POLL_MS = 1500;
const RUNS_POLL_MS = 2000;

type RunsFilter = "all" | "live" | "cursor" | "reviews";

const worldEl = requireElement("world");
const viewportEl = requireElement("viewport");
const chromeEl = requireElement("chrome");
const crumbEl = requireElement("crumb");
const hintEl = requireElement("hint");
const watchBadgeEl = requireElement("watch-badge");
const zoomOutBtn = requireButton("zoom-out");
const orgStructureBtn = requireButton("org-structure");
const orgEntrypointsBtn = requireButton("org-entrypoints");
const orgRunsBtn = requireButton("org-runs");
const orgMetricsBtn = requireButton("org-metrics");
const runsPanelEl = requireElement("runs-panel");
const runsListEl = requireElement("runs-list");
const runsDetailEl = requireElement("runs-detail");
const metricsPanelEl = requireElement("metrics-panel");
const metricsTbodyEl = requireElement("metrics-tbody");
const metricsSummaryEl = requireElement("metrics-summary");
const metricsDetailEl = requireElement("metrics-detail");

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
let runsFilter: RunsFilter = "live";
let lastRunsIndexJson = "";
let metricsSort: MetricsSort = "type";
let metricsRows: MetricRow[] = [];
let metricsDisplayRows: MetricRow[] = [];
let metricsFileInventory: SourceFileInventory[] = [];
let metricsLoaded = false;
let selectedMetricKey: string | null = null;
const cancelCooldownUntil = new Map<string, number>();

async function main(): Promise<void> {
  syncChromeSafeTop();
  try {
    const structure = await fetchJson<Scene>(ORG_MODES.structure.file);
    sceneCache.set("structure", structure);
    scene = structure;
  } catch (err) {
    worldEl.textContent = "Could not load scene.json. Run `robby web` then serve this app (Vite).";
    console.error(err);
    syncChromeSafeTop();
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
  } else if (initialOrg === "metrics") {
    await prefetchRuns();
    void prefetchEntrypoints();
    orgMode = "metrics";
    syncOrgChrome("metrics");
    await showMetricsPanel();
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
  initMetricsSort();
  syncRunsFilterButtons();
  syncMetricsSortButtons();

  zoomOutBtn.addEventListener("click", () => zoomOut());
  orgStructureBtn.addEventListener("click", () => void setOrgMode("structure"));
  orgEntrypointsBtn.addEventListener("click", () => void setOrgMode("entrypoints"));
  orgRunsBtn.addEventListener("click", () => void setOrgMode("runs"));
  orgMetricsBtn.addEventListener("click", () => void setOrgMode("metrics"));
  syncChromeSafeTop();
  window.addEventListener("resize", () => {
    syncChromeSafeTop();
    if (orgMode === "runs" || orgMode === "metrics" || !scene) {
      return;
    }
    camera = cameraForFocus(focusId);
    placeAllFrames();
  });
  window.addEventListener("keydown", (e) => {
    if (e.key === "Escape" || e.key === "Backspace") {
      e.preventDefault();
      if (orgMode === "runs" || orgMode === "metrics") {
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
    } else if (keepOrg === "metrics") {
      metricsLoaded = false;
      await showMetricsPanel();
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
  syncChromeSafeTop();
  if (orgMode !== "runs" && orgMode !== "metrics" && scene) {
    camera = cameraForFocus(focusId);
    placeAllFrames();
  }
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
  } else if (orgMode === "metrics") {
    metricsLoaded = false;
    await showMetricsPanel();
  }
  return true;
}

/** Ask Vite to re-read LiveRunSource file dumps (Cursor agents.json). */
async function refreshLiveSources(): Promise<void> {
  try {
    await fetch("/api/runs/live/refresh", { method: "POST" });
  } catch {
    // Static serve / older Vite without live middleware — ignore.
  }
}

/** Poll runs + live sources so cursor.cloud rows stay current. */
async function startRunsWatch(): Promise<void> {
  window.setInterval(() => {
    void (async () => {
      await refreshLiveSources();
      const changed = await prefetchRuns(true);
      if (changed && orgMode === "runs") {
        runDetailCache.clear();
        showRunsPanel();
      } else if (changed && orgMode === "metrics") {
        // prefetchRuns already rebuilds the metrics table when org is metrics
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
  orgMetricsBtn.classList.toggle("is-active", mode === "metrics");
  if (mode === "runs") {
    hintEl.textContent = HINT_RUNS;
  } else if (mode === "metrics") {
    hintEl.textContent = HINT_METRICS;
  } else {
    hintEl.textContent = ORG_MODES[mode].hint;
  }
  viewportEl.classList.toggle("runs-open", mode === "runs");
  viewportEl.classList.toggle("metrics-open", mode === "metrics");
  runsPanelEl.hidden = mode !== "runs";
  metricsPanelEl.hidden = mode !== "metrics";
  zoomOutBtn.disabled = mode === "runs" || mode === "metrics" || !scene || !frameSafeParent();
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
  if (next === "metrics") {
    await prefetchRuns();
    orgMode = "metrics";
    syncOrgChrome("metrics");
    await showMetricsPanel();
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
 * URL hash: `#frame=<stable_id>` and optional `&org=structure|entrypoints|runs|metrics`.
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
    orgRaw === "entrypoints" || orgRaw === "structure" || orgRaw === "runs" || orgRaw === "metrics"
      ? orgRaw
      : undefined;
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
  // Dock modes (runs / metrics) may open before a frame focus is set; still
  // persist `org=` so reload restores the panel.
  const focus = focusId || scene?.root || "";
  if (!focus && orgMode === "structure") {
    return;
  }
  const next = focus ? locationHashFor(focus, orgMode) : `#org=${orgMode}`;
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
    if (wantOrg === "metrics") {
      await prefetchRuns();
      orgMode = "metrics";
      syncOrgChrome("metrics");
      await showMetricsPanel();
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

async function showMetricsPanel(): Promise<void> {
  syncOrgChrome("metrics");
  crumbEl.textContent = "Metrics";
  if (!metricsLoaded) {
    const [rows, files] = await Promise.all([collectMetricRows(), loadMetricFileInventory()]);
    metricsRows = rows;
    metricsFileInventory = files;
    metricsLoaded = true;
  }
  renderMetricsTable();
  if (selectedMetricKey) {
    const still = metricsDisplayRows.find((r) => metricRowKey(r) === selectedMetricKey);
    if (still) {
      void selectMetricRow(still);
    } else {
      selectedMetricKey = null;
      renderMetricDetailPlaceholder();
    }
  } else {
    renderMetricDetailPlaceholder();
  }
}

function initMetricsSort(): void {
  for (const btn of metricsPanelEl.querySelectorAll<HTMLButtonElement>(".metrics-sort-btn")) {
    btn.addEventListener("click", () => {
      const sort = btn.dataset.sort;
      if (sort !== "type" && sort !== "path") {
        return;
      }
      metricsSort = sort;
      selectedMetricKey = null;
      syncMetricsSortButtons();
      renderMetricsTable();
      renderMetricDetailPlaceholder();
    });
  }
}

function syncMetricsSortButtons(): void {
  for (const btn of metricsPanelEl.querySelectorAll<HTMLButtonElement>(".metrics-sort-btn")) {
    btn.classList.toggle("is-active", btn.dataset.sort === metricsSort);
  }
}

async function loadMetricFileInventory(): Promise<SourceFileInventory[]> {
  try {
    const fromApi = await fetchJson<{ files: SourceFileInventory[] }>(
      `/api/metric-files?t=${Date.now()}`,
    );
    if (Array.isArray(fromApi.files) && fromApi.files.length > 0) {
      return fromApi.files;
    }
  } catch {
    // static files.json from robby web
  }
  try {
    const fromSite = await fetchJson<{ files: SourceFileInventory[] }>(
      `files.json?t=${Date.now()}`,
    );
    if (Array.isArray(fromSite.files)) {
      return fromSite.files;
    }
  } catch {
    // empty inventory — filesystem view still shows workspace/runs buckets
  }
  return [];
}

function renderMetricsTable(): void {
  metricsDisplayRows =
    metricsSort === "path"
      ? expandFilesystemRows(metricsRows, metricsFileInventory)
      : sortMetricRows(metricsRows, "type");
  const sorted = metricsDisplayRows;
  const scored = sorted.filter((r) => r.score !== null).length;
  const fileCount = metricsFileInventory.length;
  metricsSummaryEl.textContent =
    metricsSort === "path"
      ? `${sorted.length} metric×file rows · ${fileCount} source files · click for calculation`
      : `${sorted.length} metrics · ${scored} scored · by type · click for calculation`;
  metricsTbodyEl.replaceChildren();
  if (sorted.length === 0) {
    const tr = document.createElement("tr");
    tr.className = "metrics-empty";
    const td = document.createElement("td");
    td.colSpan = 6;
    td.textContent =
      metricsSort === "path"
        ? "No source-file inventory yet — run robby web or open Metrics with the Vite API."
        : "No metrics in catalog or runs yet.";
    tr.appendChild(td);
    metricsTbodyEl.appendChild(tr);
    return;
  }
  let lastPath = "\0";
  for (const row of sorted) {
    if (metricsSort === "path" && row.path !== lastPath) {
      lastPath = row.path;
      const group = document.createElement("tr");
      group.className = "metrics-path-group";
      const td = document.createElement("td");
      td.colSpan = 6;
      td.textContent = row.path || "(no path)";
      group.appendChild(td);
      metricsTbodyEl.appendChild(group);
    }
    const key = metricRowKey(row);
    const tr = document.createElement("tr");
    tr.tabIndex = 0;
    tr.dataset.metricKey = key;
    tr.setAttribute("role", "button");
    tr.setAttribute(
      "aria-label",
      metricsSort === "path"
        ? `${row.id} on ${row.path || "workspace"}`
        : `Show calculation for ${row.id}`,
    );
    if (key === selectedMetricKey) {
      tr.classList.add("is-selected");
    }
    tr.append(
      cell(row.id, "metric-id", row.id),
      cell(row.type),
      cell(row.score ?? "—", "metric-score"),
      cell(row.subject || "—", "metric-subject", row.subject),
      cell(row.path || "—", "metric-path", row.path),
      cell(row.source),
    );
    tr.addEventListener("click", () => {
      void selectMetricRow(row);
    });
    tr.addEventListener("keydown", (e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        void selectMetricRow(row);
      }
    });
    metricsTbodyEl.appendChild(tr);
  }
}

function cell(text: string, className?: string, title?: string): HTMLTableCellElement {
  const td = document.createElement("td");
  if (className) {
    td.className = className;
  }
  td.textContent = text;
  if (title) {
    td.title = title;
  }
  return td;
}

function renderMetricDetailPlaceholder(): void {
  metricsDetailEl.hidden = false;
  metricsDetailEl.replaceChildren();
  const empty = document.createElement("div");
  empty.className = "metrics-detail-empty";
  empty.textContent = "Select a metric to see how it is calculated.";
  metricsDetailEl.appendChild(empty);
}

async function selectMetricRow(row: MetricRow): Promise<void> {
  selectedMetricKey = metricRowKey(row);
  for (const tr of metricsTbodyEl.querySelectorAll("tr[data-metric-key]")) {
    tr.classList.toggle("is-selected", tr.getAttribute("data-metric-key") === selectedMetricKey);
  }
  crumbEl.textContent = `Metrics · ${row.id}`;
  await renderMetricDetail(row);
}

async function renderMetricDetail(row: MetricRow): Promise<void> {
  const calc = metricCalculationDetail(row.id);
  const direction = row.direction || calc.direction || "—";
  metricsDetailEl.hidden = false;
  metricsDetailEl.replaceChildren();

  const title = document.createElement("h2");
  title.textContent = row.id;
  const lead = document.createElement("p");
  lead.className = "metrics-detail-lead";
  lead.textContent = calc.definition;

  const kv = document.createElement("dl");
  kv.className = "run-kv";
  const pairs: [string, string][] = [
    ["Type", row.type],
    ["Direction", direction],
    ["Score", row.score ?? "— (not measured yet)"],
    ["Subject", row.subject || "—"],
    ["Path", row.path || "—"],
    ["Source", row.source],
    ["Attachment", calc.attachment],
    ["Computer", calc.computer],
  ];
  if (calc.testId) {
    pairs.push(["Test id", calc.testId]);
  }
  if (calc.appearsOn) {
    pairs.push(["Appears on", calc.appearsOn]);
  }
  if (row.runId) {
    pairs.push(["Run", row.runId]);
  }
  const scope = fsScopeFor(row.id);
  const applies: string[] = [];
  if (scope.workspace) {
    applies.push("workspace");
  }
  if (scope.anyFile) {
    applies.push("any source file");
  }
  if (scope.kinds?.length) {
    applies.push(`files with ${scope.kinds.join(" / ")}`);
  }
  if (scope.runs) {
    applies.push("runs / reviews");
  }
  if (scope.productPaths?.length) {
    applies.push(...scope.productPaths);
  }
  if (applies.length) {
    pairs.push(["Applies to", applies.join(" · ")]);
  }
  if (row.path && row.path !== "(workspace)" && row.path !== "(runs)") {
    pairs.push(["This row", row.path]);
  }
  for (const [k, v] of pairs) {
    const dt = document.createElement("dt");
    dt.textContent = k;
    const dd = document.createElement("dd");
    dd.textContent = v;
    kv.append(dt, dd);
  }

  const formulaLabel = document.createElement("h2");
  formulaLabel.textContent = "Formula";
  formulaLabel.style.fontSize = "14px";
  formulaLabel.style.color = "var(--muted)";
  formulaLabel.style.fontFamily = "inherit";
  const formula = document.createElement("pre");
  formula.className = "metrics-formula";
  formula.textContent = calc.formula;

  metricsDetailEl.append(title, lead, kv, formulaLabel, formula);

  const companions = companionStatKeys(row.id);
  if (row.runId && companions.length > 0) {
    let record = runDetailCache.get(row.runId);
    if (!record) {
      try {
        const stem = runFileStem(row.runId);
        record = await fetchJson<RunRecord>(
          `runs/${encodeURIComponent(stem)}.json?t=${Date.now()}`,
        );
        runDetailCache.set(row.runId, record);
      } catch {
        record = undefined;
      }
    }
    if (record?.stats) {
      const measured: [string, string][] = [];
      for (const key of companions) {
        const val = record.stats[key];
        if (val !== undefined) {
          measured.push([key, val]);
        }
      }
      const dirKey = `${row.id}.direction`;
      if (record.stats[dirKey]) {
        measured.push([dirKey, record.stats[dirKey]]);
      }
      const testKey = `${row.id}.test_id`;
      if (record.stats[testKey]) {
        measured.push([testKey, record.stats[testKey]]);
      }
      if (measured.length > 0) {
        const h = document.createElement("h2");
        h.textContent = "Measured details";
        h.style.fontSize = "14px";
        h.style.color = "var(--muted)";
        h.style.fontFamily = "inherit";
        const ul = document.createElement("ul");
        ul.className = "metrics-companions";
        for (const [k, v] of measured) {
          const li = document.createElement("li");
          li.textContent = `${k} = ${v}`;
          ul.appendChild(li);
        }
        metricsDetailEl.append(h, ul);
      }
    }
  }
}

/** Pull metric scores from catalog + runs (metric_test / review decisions / stats). */
async function collectMetricRows(): Promise<MetricRow[]> {
  const observations: MetricObservation[] = [];
  const entries = runsIndex?.runs ?? [];
  // Prefer recipes that actually carry metric claims; cap fan-out.
  const interesting = entries
    .filter(
      (e) =>
        e.recipe === RECIPE_METRIC_TEST ||
        e.recipe === RECIPE_REVIEW_VIEW ||
        e.recipe === "robby.review_doc" ||
        e.recipe === RECIPE_IMPROVE_PATCH ||
        e.recipe === RECIPE_INTEGRATE_PATCH,
    )
    .slice(0, 40);
  // Also scan a few recent runs for `metric.*` / catalog stats.
  const recent = entries.slice(0, 24);
  const seen = new Set<string>();
  const toLoad: RunIndexEntry[] = [];
  for (const e of [...interesting, ...recent]) {
    if (seen.has(e.run_id)) {
      continue;
    }
    seen.add(e.run_id);
    toLoad.push(e);
  }

  await Promise.all(
    toLoad.map(async (entry) => {
      let record = runDetailCache.get(entry.run_id);
      if (!record) {
        try {
          const stem = runFileStem(entry.run_id);
          record = await fetchJson<RunRecord>(
            `runs/${encodeURIComponent(stem)}.json?t=${Date.now()}`,
          );
          runDetailCache.set(entry.run_id, record);
        } catch {
          return;
        }
      }
      observations.push(...observationsFromRun(record));
      for (const product of record.products ?? []) {
        if (!product.startsWith("path:runs/") || !product.endsWith(".decisions.json")) {
          continue;
        }
        const rel = product.slice("path:".length);
        try {
          const decisions = await fetchJson<ScoredDecisionProduct[]>(`${rel}?t=${Date.now()}`);
          for (const d of decisions) {
            observations.push({
              id: d.metric,
              score: d.score,
              direction: d.direction,
              subject: d.subject || record.stats?.subject_name || "",
              path: normalizePathHint(d.subject) || normalizePathHint(record.stats?.doc_path),
              source: "review",
              runId: d.run_id || record.run_id,
            });
          }
        } catch {
          // decisions file may be absent on older runs
        }
      }
    }),
  );

  return mergeMetricRows(catalogSeedRows(), observations);
}

function observationsFromRun(record: RunRecord): MetricObservation[] {
  const out: MetricObservation[] = [];
  const stats = record.stats ?? {};
  const subject =
    stats.subject_name ||
    record.subject ||
    (record.recipe === RECIPE_METRIC_TEST ? "workspace" : "");
  const path =
    normalizePathHint(stats.doc_path) ||
    normalizePathHint(record.subject) ||
    normalizePathHint((record.products ?? []).find((p) => p.startsWith("path:")));

  const source: MetricObservation["source"] =
    record.recipe === RECIPE_METRIC_TEST
      ? "metric_test"
      : record.recipe === RECIPE_REVIEW_VIEW || record.recipe === "robby.review_doc"
        ? "review"
        : "run";

  for (const [key, value] of Object.entries(stats)) {
    if (!isPrimaryMetricStatKey(key)) {
      continue;
    }
    const id = primaryMetricIdFromStatKey(key);
    out.push({
      id,
      score: value,
      direction: stats[`${id}.direction`] || stats[`metric.${id}.direction`],
      subject,
      path,
      source,
      runId: record.run_id,
    });
  }

  for (const product of record.products ?? []) {
    const parsed = parseMetricProduct(product);
    if (!parsed) {
      continue;
    }
    out.push({
      id: parsed.id,
      score: parsed.score,
      direction: stats[`${parsed.id}.direction`],
      subject,
      path,
      source: record.recipe === RECIPE_METRIC_TEST ? "metric_test" : source,
      runId: record.run_id,
    });
  }
  return out;
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
  // Soft-queue reviews plus the Allow chain (metric → improve → integrate).
  return runs.filter(
    (r) =>
      r.recipe === RECIPE_REVIEW_VIEW ||
      r.recipe === RECIPE_METRIC_TEST ||
      r.recipe === RECIPE_IMPROVE_PATCH ||
      r.recipe === RECIPE_INTEGRATE_PATCH,
  );
}

function runsFilterLabel(filter: RunsFilter): string {
  if (filter === "live") {
    return "Live";
  }
  if (filter === "cursor") {
    return "cursor.*";
  }
  if (filter === "reviews") {
    return "Soft / Reviews";
  }
  return "All";
}

function formatRunsListPlain(runs: RunIndexEntry[], filter: RunsFilter): string {
  const soft = runs.filter((r) => r.status === "queued").length;
  const live = runs.filter((r) => r.status === "running" || r.status === "queued").length;
  const lines = [
    `filter: ${runsFilterLabel(filter)}`,
    `count: ${runs.length}`,
    `soft_queued: ${soft}`,
    `live: ${live}`,
    "",
  ];
  for (const entry of runs) {
    lines.push([entry.status, entry.recipe, entry.run_id, shortRunLabel(entry)].join("\t"));
  }
  return lines.join("\n");
}

function renderRunsList(): void {
  runsListEl.innerHTML = "";
  const all = runsIndex?.runs ?? [];
  const runs = filteredRunEntries();

  const summary = document.createElement("div");
  summary.className = "runs-list-summary";
  const summaryLabel = document.createElement("span");
  summaryLabel.className = "runs-list-summary-label";
  const soft = runs.filter((r) => r.status === "queued").length;
  const live = runs.filter((r) => r.status === "running" || r.status === "queued").length;
  summaryLabel.textContent =
    `${runsFilterLabel(runsFilter)} · ${runs.length}` +
    (runs.length ? ` · soft ${soft} · live ${live}` : "");
  summary.appendChild(summaryLabel);
  if (runs.length > 0) {
    summary.appendChild(makeCopyButton(() => formatRunsListPlain(runs, runsFilter)));
  }
  runsListEl.appendChild(summary);

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
    const isLive = entry.status === "running" || entry.status === "queued";
    const timeLabel = isLive
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

function formatRunOverviewPlain(detail: RunRecord, duration: string): string {
  const lines = [
    `run_id: ${detail.run_id}`,
    `recipe: ${detail.recipe}`,
    `status: ${detail.status}`,
    `subject: ${detail.subject ?? "—"}`,
    `duration: ${duration}`,
    `started: ${detail.started_at !== undefined ? formatAbsolute(detail.started_at) : "—"}`,
    `finished: ${detail.finished_at !== undefined ? formatAbsolute(detail.finished_at) : "—"}`,
  ];
  if (detail.exit_code !== undefined) {
    lines.push(`exit_code: ${detail.exit_code}`);
  }
  if (detail.stats && Object.keys(detail.stats).length > 0) {
    lines.push("", "stats:");
    for (const [key, value] of Object.entries(detail.stats)) {
      lines.push(`  ${key}: ${value}`);
    }
  }
  return lines.join("\n");
}

function formatRunStagesPlain(detail: RunRecord): string {
  const steps = detail.process?.steps ?? [];
  const lines = [`run_id: ${detail.run_id}`, `stages: ${steps.length}`, ""];
  for (const step of steps) {
    const label = step.label && step.label !== step.to ? ` · ${step.label}` : "";
    const planned = step.observed ? "" : " (planned)";
    lines.push(`${step.from} → ${step.to}${label}${planned}`);
  }
  return lines.join("\n");
}

function formatRunProductsPlain(detail: RunRecord): string {
  const products = detail.products ?? [];
  const lines = [`run_id: ${detail.run_id}`, `products: ${products.length}`, ""];
  for (const product of products) {
    lines.push(product);
  }
  return lines.join("\n");
}

function appendCopyBlockHead(host: HTMLElement, title: string, getText: () => string): void {
  const head = document.createElement("div");
  head.className = "copy-block-head";
  const heading = document.createElement("h2");
  heading.textContent = title;
  head.appendChild(heading);
  head.appendChild(makeCopyButton(getText));
  host.appendChild(head);
}

function makeCopyButton(getText: () => string): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "copy-btn";
  btn.textContent = "Copy";
  btn.title = "Copy as plain text";
  btn.addEventListener("click", (event) => {
    event.preventDefault();
    event.stopPropagation();
    void copyPlainText(getText(), btn);
  });
  return btn;
}

async function copyPlainText(text: string, btn: HTMLButtonElement): Promise<void> {
  const previous = btn.textContent ?? "Copy";
  try {
    if (!navigator.clipboard?.writeText) {
      throw new Error("clipboard unavailable");
    }
    await navigator.clipboard.writeText(text);
    btn.textContent = "Copied";
    btn.classList.add("is-copied");
  } catch {
    btn.textContent = "Copy failed";
    btn.classList.remove("is-copied");
  }
  window.setTimeout(() => {
    btn.textContent = previous;
    btn.classList.remove("is-copied");
  }, 1200);
}

function renderRunDetail(detail: RunRecord): void {
  runsDetailEl.hidden = false;
  runsDetailEl.replaceChildren();

  const started = detail.started_at ?? 0;
  const finished = detail.finished_at ?? detail.started_at ?? 0;
  const live = detail.status === "running" || detail.status === "queued";
  const duration = live ? "live" : formatDuration(started, finished);
  const canGate =
    detail.recipe === RECIPE_REVIEW_VIEW &&
    (detail.status === "running" || detail.status === "queued");

  const overview = document.createElement("section");
  overview.className = "run-block";
  appendCopyBlockHead(overview, detail.recipe, () => formatRunOverviewPlain(detail, duration));

  const meta = document.createElement("div");
  meta.className = "meta";
  meta.innerHTML =
    `<span class="run-status ${statusClass(detail.status)}">${escapeHtml(detail.status)}</span>` +
    ` · ${escapeHtml(detail.run_id)} · ${escapeHtml(duration)}`;
  overview.appendChild(meta);

  if (canGate) {
    const actions = document.createElement("div");
    actions.className = "run-actions";
    actions.dataset.runActions = detail.run_id;
    if (detail.status === "queued") {
      const allow = document.createElement("button");
      allow.type = "button";
      allow.textContent = "Allow / Continue";
      allow.title =
        "Score decisions, spawn robby.metric_test, then improve_patch → integrate_patch";
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
    overview.appendChild(actions);
  }

  const kv = document.createElement("dl");
  kv.className = "run-kv";
  const kvRows: Array<[string, string]> = [
    ["subject", detail.subject ?? "—"],
    ["started", formatAbsolute(started)],
    [
      live ? "updated" : "finished",
      detail.finished_at !== undefined ? formatAbsolute(detail.finished_at) : "—",
    ],
  ];
  if (detail.exit_code !== undefined) {
    kvRows.push(["exit_code", String(detail.exit_code)]);
  }
  if (detail.stats) {
    for (const [key, value] of Object.entries(detail.stats)) {
      kvRows.push([key, String(value)]);
    }
  }
  for (const [key, value] of kvRows) {
    const dt = document.createElement("dt");
    dt.textContent = key;
    const dd = document.createElement("dd");
    dd.textContent = value;
    kv.appendChild(dt);
    kv.appendChild(dd);
  }
  overview.appendChild(kv);
  runsDetailEl.appendChild(overview);

  if (detail.stdout_tail) {
    const block = document.createElement("section");
    block.className = "run-block";
    appendCopyBlockHead(block, "Stdout", () => detail.stdout_tail ?? "");
    const stdout = document.createElement("div");
    stdout.className = "stdout";
    stdout.textContent = detail.stdout_tail;
    block.appendChild(stdout);
    runsDetailEl.appendChild(block);
  }

  const products = detail.products ?? [];
  if (products.length > 0) {
    const block = document.createElement("section");
    block.className = "run-block stages";
    appendCopyBlockHead(block, "Products / patches", () => formatRunProductsPlain(detail));
    const list = document.createElement("ol");
    for (const product of products) {
      const li = document.createElement("li");
      if (product.startsWith("url:")) {
        const href = product.slice(4);
        const link = document.createElement("a");
        link.href = href;
        link.target = "_blank";
        link.rel = "noreferrer";
        link.textContent = href;
        li.appendChild(link);
      } else {
        li.textContent = product;
      }
      list.appendChild(li);
    }
    block.appendChild(list);
    runsDetailEl.appendChild(block);
  }

  const steps = detail.process?.steps ?? [];
  if (steps.length > 0) {
    const block = document.createElement("section");
    block.className = "run-block stages";
    const participants = (detail.process?.participants ?? [])
      .map((p) => `${p.id}${p.label && p.label !== p.id ? ` (${p.label})` : ""}`)
      .join(", ");
    appendCopyBlockHead(block, "Stages", () => {
      const base = formatRunStagesPlain(detail);
      return participants ? `${base}\n\nparticipants: ${participants}` : base;
    });
    if (participants) {
      const summary = document.createElement("div");
      summary.className = "meta run-chart-summary";
      summary.textContent = `${steps.length} steps · ${participants}`;
      block.appendChild(summary);
    }
    const list = document.createElement("ol");
    for (const step of steps) {
      const li = document.createElement("li");
      const label = step.label && step.label !== step.to ? ` · ${step.label}` : "";
      const planned = step.observed ? "" : " (planned)";
      li.textContent = `${step.from} → ${step.to}${label}${planned}`;
      list.appendChild(li);
    }
    block.appendChild(list);
    runsDetailEl.appendChild(block);
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

/** Usable map rect under the fixed chrome (and side padding on narrow widths). */
function usableViewport(): { vw: number; vh: number; ox: number; oy: number } {
  const chromeH = chromeEl.getBoundingClientRect().height || 52;
  const narrow = window.innerWidth < 820;
  const side = narrow ? 10 : 16;
  const bottom = narrow ? 10 : 16;
  const top = Math.ceil(chromeH + 8);
  return {
    vw: Math.max(window.innerWidth - side * 2, 120),
    vh: Math.max(window.innerHeight - top - bottom, 120),
    ox: side,
    oy: top,
  };
}

/** Keep dock insets in sync with wrapped chrome height. */
function syncChromeSafeTop(): void {
  const chromeH = chromeEl.getBoundingClientRect().height || 52;
  document.documentElement.style.setProperty("--chrome-safe-top", `${Math.ceil(chromeH + 8)}px`);
}

function cameraForFocus(id: string): Camera {
  const f = frame(id);
  const { vw, vh, ox, oy } = usableViewport();
  const pad = 0.96;
  const scale = Math.min(vw / f.world.w, vh / f.world.h) * pad;
  const cx = f.world.x + f.world.w / 2;
  const cy = f.world.y + f.world.h / 2;
  return {
    scale,
    tx: ox + vw / 2 - cx * scale,
    ty: oy + vh / 2 - cy * scale,
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
  let { x, y, w, h } = r;
  // Narrow/tall viewports: grow the focus card past world aspect so the child
  // grid / sequence panel have room (children live in CSS layout, not world).
  if (el.classList.contains("is-focus")) {
    const { vw, vh, oy } = usableViewport();
    if (vh > vw * 1.15) {
      const minH = Math.min(vh * 0.78, vh);
      if (h < minH) {
        h = minH;
        y = oy + Math.max(0, (vh - h) / 2);
      }
    }
  }
  el.style.width = `${w}px`;
  el.style.height = `${h}px`;
  el.style.left = `${x}px`;
  el.style.top = `${y}px`;
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

/** Hard ceiling so a very wide strip still collapses early. */
const PORTAL_PREVIEW_MAX = 6;
const PORTAL_GAP = 8;

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

function portalMoreButton(label: string, title: string, onClick: () => void): HTMLButtonElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "portal portal-more";
  btn.textContent = label;
  btn.title = title;
  btn.addEventListener("click", (e) => {
    e.stopPropagation();
    onClick();
  });
  return btn;
}

/** Measure an off-layout probe inside the strip so CSS portal styles apply. */
function measurePortalWidth(strip: HTMLElement, el: HTMLElement): number {
  el.style.position = "absolute";
  el.style.visibility = "hidden";
  el.style.pointerEvents = "none";
  strip.appendChild(el);
  const width = el.offsetWidth;
  el.remove();
  el.style.position = "";
  el.style.visibility = "";
  el.style.pointerEvents = "";
  return width;
}

/**
 * How many portals fit in the collapsed strip alongside a "+N more" control
 * (or all of them when everything fits under the hard max).
 */
function fitVisiblePortalCount(strip: HTMLElement, portals: Portal[]): number {
  const total = portals.length;
  if (total === 0) {
    return 0;
  }

  const avail = strip.clientWidth;
  if (avail <= 0) {
    return Math.min(PORTAL_PREVIEW_MAX, total);
  }

  const widths = portals.map((portal) => measurePortalWidth(strip, portalButton(portal)));

  const rowWidth = (count: number, withMore: number | null): number => {
    let used = 0;
    for (let i = 0; i < count; i++) {
      const width = widths[i] ?? 0;
      used += width + (i > 0 ? PORTAL_GAP : 0);
    }
    if (withMore != null) {
      const more = portalMoreButton(`+${withMore} more`, "", () => {});
      const moreW = measurePortalWidth(strip, more);
      used += (count > 0 ? PORTAL_GAP : 0) + moreW;
    }
    return used;
  };

  if (total <= PORTAL_PREVIEW_MAX && rowWidth(total, null) <= avail) {
    return total;
  }

  let count = Math.min(PORTAL_PREVIEW_MAX, total - 1);
  while (count >= 0) {
    const hidden = total - count;
    if (rowWidth(count, hidden) <= avail) {
      return count;
    }
    count--;
  }
  return 0;
}

function buildPortalStrip(f: SceneFrame): HTMLElement {
  const strip = document.createElement("div");
  strip.className = "portal-strip";
  let expanded = false;
  let visibleCount = Math.min(PORTAL_PREVIEW_MAX, f.portals.length);
  let fitting = false;

  const syncFocusClass = (): void => {
    const focusWrap = strip.closest(".frame-wrap");
    focusWrap?.classList.toggle("portals-expanded", expanded);
  };

  const render = (): void => {
    strip.innerHTML = "";
    strip.classList.toggle("is-expanded", expanded);
    syncFocusClass();

    if (expanded) {
      for (const portal of f.portals) {
        strip.appendChild(portalButton(portal));
      }
      if (f.portals.length > visibleCount) {
        strip.appendChild(
          portalMoreButton("Show less", "Collapse related views", () => {
            expanded = false;
            render();
          }),
        );
      }
      return;
    }

    const list = f.portals.slice(0, visibleCount);
    for (const portal of list) {
      strip.appendChild(portalButton(portal));
    }
    const hidden = f.portals.length - visibleCount;
    if (hidden > 0) {
      strip.appendChild(
        portalMoreButton(`+${hidden} more`, "Show all related views", () => {
          expanded = true;
          render();
        }),
      );
    }
  };

  const refit = (): void => {
    if (!strip.isConnected || expanded || fitting) {
      return;
    }
    fitting = true;
    try {
      // Clear children so measurement probes are the only strip content.
      strip.innerHTML = "";
      const next = fitVisiblePortalCount(strip, f.portals);
      if (next !== visibleCount) {
        visibleCount = next;
      }
      render();
    } finally {
      fitting = false;
    }
  };

  render();

  const ro = new ResizeObserver(() => {
    if (!strip.isConnected) {
      ro.disconnect();
      return;
    }
    refit();
  });
  // Observe after the strip is in the layout tree (next frame).
  requestAnimationFrame(() => {
    if (!strip.isConnected) {
      return;
    }
    ro.observe(strip);
    refit();
  });

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
  syncChromeSafeTop();
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
    const execModel = resolveSequenceModel(detail, "execution");
    if (execModel) {
      const panel = document.createElement("div");
      panel.className = "detail detail-execution";
      appendSequenceSection(
        panel,
        "Execution",
        "call path or runtime observations · not the definition process",
        execModel,
        id,
      );
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

  const execModel = resolveSequenceModel(detail, "execution");
  if (execModel) {
    appendSequenceSection(
      panel,
      "Execution",
      "call path or runtime observations · not the definition process",
      execModel,
      id,
    );
  }

  const processModel = resolveSequenceModel(detail, "process");
  if (processModel) {
    appendSequenceSection(
      panel,
      "Process",
      `${processModel.stepCount} steps · ${processModel.observedCount} observed`,
      processModel,
      id,
    );
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

  if (!execModel && !processModel && !detail.source && !detail.docs) {
    const empty = document.createElement("div");
    empty.className = "empty-leaf";
    empty.textContent = "No process or source on this node.";
    panel.appendChild(empty);
  }

  host.appendChild(panel);
}

function resolveSequenceModel(detail: FrameDetail, kind: "execution" | "process"): SeqModel | null {
  if (kind === "execution") {
    if (detail.execution && detail.execution.participants.length > 0) {
      return modelFromProcess(detail.execution);
    }
    if (detail.execution_mermaid) {
      return modelFromMermaid(detail.execution_mermaid);
    }
    return null;
  }
  if (detail.process && detail.process.participants.length > 0) {
    return modelFromProcess(detail.process);
  }
  if (detail.process_mermaid) {
    return modelFromMermaid(detail.process_mermaid);
  }
  return null;
}

function appendSequenceSection(
  panel: HTMLElement,
  title: string,
  summaryText: string,
  model: SeqModel,
  frameId: string,
): void {
  const proc = document.createElement("div");
  proc.className = title === "Execution" ? "process process-execution" : "process";
  const heading = document.createElement("h2");
  heading.textContent = title;
  proc.appendChild(heading);
  const summary = document.createElement("div");
  summary.className = "meta";
  summary.textContent = summaryText;
  proc.appendChild(summary);
  const diagram = document.createElement("div");
  proc.appendChild(diagram);
  panel.appendChild(proc);
  mountSequenceDiagram(
    diagram,
    model,
    (participant) => resolveParticipantTarget(participant, frameId),
    (target) => flyTo(target),
  );
}

/** Map a lifeline header to a portal / child / scene frame (same as portal clicks). */
function resolveParticipantTarget(
  participant: SeqParticipant,
  frameId: string,
): string | undefined {
  if (participant.target && scene?.frames[participant.target]) {
    return participant.target;
  }
  const f = frame(frameId);
  const label = participant.label;
  const norm = (s: string): string =>
    s
      .replace(/::impl\s+/g, "::")
      .replace(/\s+/g, "")
      .toLowerCase();
  const want = norm(label);

  if (norm(f.fqn) === want || norm(f.name) === want) {
    return frameId;
  }

  for (const portal of f.portals) {
    if (!portal.target) {
      continue;
    }
    if (
      portal.target_name === label ||
      portal.label === label ||
      norm(portal.target_name) === want ||
      norm(portal.label) === want
    ) {
      return portal.target;
    }
  }

  for (const childId of f.children) {
    const child = frame(childId);
    if (norm(child.fqn) === want || norm(child.name) === want) {
      return childId;
    }
  }

  // Type lifelines (e.g. CoverageReport) → first portal/child mentioning the type.
  for (const portal of f.portals) {
    if (!portal.target) {
      continue;
    }
    const tn = portal.target_name;
    if (
      tn.endsWith(`::${label}`) ||
      tn.includes(`::${label}::`) ||
      tn.includes(`${label}::`) ||
      tn.endsWith(label)
    ) {
      return portal.target;
    }
  }

  if (scene) {
    for (const fr of Object.values(scene.frames)) {
      if (norm(fr.fqn) === want || norm(fr.name) === want) {
        return fr.stable_id;
      }
    }
  }
  return undefined;
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
    // Refresh the Runs index/badge only — stay on the structure map so a
    // crate/frame click keeps flyTo focus instead of hijacking into Runs.
    await prefetchRuns(true);
  } catch (err) {
    console.error(err);
  }
}

void main();
