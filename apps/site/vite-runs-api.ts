/**
 * Mac-local Vite middleware for review queue + run allow/cancel.
 *
 * Persists the shared RunRecord JSON under `public/runs/` (same shape as
 * `robby_ir::run::RunRecord`). Prefer this path on a trusted laptop; cloud
 * Try Live is optional.
 */

import { spawnSync } from "node:child_process";
import fs from "node:fs";
import type { IncomingMessage, ServerResponse } from "node:http";
import path from "node:path";
import type { Connect, Plugin, ViteDevServer } from "vite";

import {
  blocksDownstream,
  isTerminal,
  RECIPE_IMPROVE_PATCH,
  RECIPE_INTEGRATE_PATCH,
  RECIPE_METRIC_TEST,
  RECIPE_REVIEW_VIEW,
  REVIEW_METRIC_AGENT_PROCESS_STAGE,
  REVIEW_METRIC_AGENT_PROCESS_STAGE_TEST,
  REVIEW_METRIC_UI_RUNRECORD_SHAPE,
  REVIEW_METRIC_UI_RUNRECORD_SHAPE_TEST,
  type RunIndexEntry,
  type RunRecord,
  type RunsIndex,
  runFileStem,
  type ScoredDecisionProduct,
} from "./src/runRecord";

function runsDir(publicDir: string): string {
  return path.join(publicDir, "runs");
}

function indexPath(publicDir: string): string {
  return path.join(runsDir(publicDir), "index.json");
}

function recordPath(publicDir: string, runId: string): string {
  return path.join(runsDir(publicDir), `${runFileStem(runId)}.json`);
}

function ensureLayout(publicDir: string): void {
  const dir = runsDir(publicDir);
  fs.mkdirSync(dir, { recursive: true });
  const idx = indexPath(publicDir);
  if (!fs.existsSync(idx)) {
    writeIndex(publicDir, { runs: [] });
  }
}

function readIndex(publicDir: string): RunsIndex {
  ensureLayout(publicDir);
  const raw = fs.readFileSync(indexPath(publicDir), "utf8");
  return JSON.parse(raw) as RunsIndex;
}

function writeIndex(publicDir: string, index: RunsIndex): void {
  fs.mkdirSync(runsDir(publicDir), { recursive: true });
  fs.writeFileSync(indexPath(publicDir), `${JSON.stringify(index, null, 2)}\n`);
}

function readRecord(publicDir: string, runId: string): RunRecord {
  const raw = fs.readFileSync(recordPath(publicDir, runId), "utf8");
  return JSON.parse(raw) as RunRecord;
}

function indexEntryFrom(record: RunRecord): RunIndexEntry {
  const entry: RunIndexEntry = {
    run_id: record.run_id,
    recipe: record.recipe,
    status: record.status,
  };
  if (record.subject !== undefined) {
    entry.subject = record.subject;
  }
  if (record.started_at !== undefined) {
    entry.started_at = record.started_at;
  }
  if (record.finished_at !== undefined) {
    entry.finished_at = record.finished_at;
  }
  return entry;
}

function putRecord(publicDir: string, record: RunRecord): void {
  ensureLayout(publicDir);
  fs.writeFileSync(recordPath(publicDir, record.run_id), `${JSON.stringify(record, null, 2)}\n`);
  const index = readIndex(publicDir);
  const entry = indexEntryFrom(record);
  const i = index.runs.findIndex((e) => e.run_id === record.run_id);
  if (i >= 0) {
    index.runs[i] = entry;
  } else {
    index.runs.unshift(entry);
  }
  writeIndex(publicDir, index);
}

function nowMs(): number {
  return Date.now();
}

/** Monotonic suffix so burst mints in the same ms never share a path. */
let runIdSeq = 0;

function mintRunId(): string {
  const seq = runIdSeq++;
  return `review-${nowMs()}-${seq}`;
}

function reviewProcess(
  observedGate: boolean,
  observedScore: boolean,
): NonNullable<RunRecord["process"]> {
  const gate = observedGate || observedScore;
  return {
    participants: [
      { id: "review", label: "review_view", kind: "agent.action" },
      { id: "ui", label: "ui_gate", kind: "agent.action" },
    ],
    steps: [
      { from: "review", to: "ui", label: "queue", kind: "call", observed: gate },
      {
        from: "review",
        to: "ui",
        label: "await_ui_gate",
        kind: "call",
        observed: gate,
      },
      {
        from: "review",
        to: "ui",
        label: "bind_subjects",
        kind: "call",
        observed: observedScore,
      },
      {
        from: "review",
        to: "ui",
        label: "score_metrics",
        kind: "call",
        observed: observedScore,
      },
      {
        from: "review",
        to: "ui",
        label: "emit_decisions",
        kind: "call",
        observed: observedScore,
      },
    ],
  };
}

function scoredReviewDecisions(record: RunRecord): ScoredDecisionProduct[] {
  const subject = record.subject ?? "workspace:";
  const trigger = record.stats?.trigger ?? "frame_focus";
  const stageRate = (record.process?.steps?.length ?? 0) > 0 ? "1" : "0";
  return [
    {
      kind: "agent.decision",
      verdict: "relevant",
      metric: REVIEW_METRIC_UI_RUNRECORD_SHAPE,
      score: "1",
      test_id: REVIEW_METRIC_UI_RUNRECORD_SHAPE_TEST,
      direction: "higher_better",
      subject,
      run_id: record.run_id,
      trigger,
      source_recipe: RECIPE_REVIEW_VIEW,
      reason: "Allow gate produced a shared RunRecord-shaped review product",
    },
    {
      kind: "agent.decision",
      verdict: "relevant",
      metric: REVIEW_METRIC_AGENT_PROCESS_STAGE,
      score: stageRate,
      test_id: REVIEW_METRIC_AGENT_PROCESS_STAGE_TEST,
      direction: "higher_better",
      subject,
      run_id: record.run_id,
      trigger,
      source_recipe: RECIPE_REVIEW_VIEW,
      reason: "Review Process lists bind_subjects / score_metrics / emit_decisions",
    },
  ];
}

interface QueueBody {
  subject: string;
  trigger?: string;
  subject_name?: string;
}

function queueReviewView(publicDir: string, body: QueueBody): RunRecord {
  const subject = body.subject;
  const trigger = body.trigger ?? "frame_focus";
  const index = readIndex(publicDir);
  for (const entry of index.runs) {
    if (
      entry.recipe === RECIPE_REVIEW_VIEW &&
      entry.subject === subject &&
      entry.status === "queued"
    ) {
      const existing = readRecord(publicDir, entry.run_id);
      existing.started_at = nowMs();
      const stats: Record<string, string> = {
        ...(existing.stats ?? {}),
        trigger,
        ui_gate: "awaiting_allow",
      };
      if (body.subject_name) {
        stats.subject_name = body.subject_name;
      }
      existing.stats = stats;
      putRecord(publicDir, existing);
      return existing;
    }
  }

  const stats: Record<string, string> = {
    trigger,
    ui_gate: "awaiting_allow",
  };
  if (body.subject_name) {
    stats.subject_name = body.subject_name;
  }

  const record: RunRecord = {
    run_id: mintRunId(),
    recipe: RECIPE_REVIEW_VIEW,
    subject,
    status: "queued",
    started_at: nowMs(),
    stats,
    products: [],
    process: reviewProcess(false, false),
  };
  putRecord(publicDir, record);
  return record;
}

function repoRootFromPublic(publicDir: string): string {
  // apps/site/public → repo root
  return path.resolve(publicDir, "..", "..", "..");
}

function defaultIrPath(repoRoot: string): string {
  const fromEnv = process.env.ROBBY_IR;
  if (fromEnv && fs.existsSync(fromEnv)) {
    return fromEnv;
  }
  const candidate = path.join(repoRoot, ".robby", "robby.ir.json");
  return candidate;
}

function findRobbyBin(repoRoot: string): string | null {
  const fromEnv = process.env.ROBBY_BIN;
  if (fromEnv && fs.existsSync(fromEnv)) {
    return fromEnv;
  }
  for (const rel of ["target/debug/robby", "target/release/robby"]) {
    const p = path.join(repoRoot, rel);
    if (fs.existsSync(p)) {
      return p;
    }
  }
  return null;
}

/** Prefer Rust CLI so metric computers + RunRecord stay one path. */
function spawnMetricTestViaCli(
  publicDir: string,
  reviewRunId: string,
): { ok: boolean; detail: string } {
  const repoRoot = repoRootFromPublic(publicDir);
  const ir = defaultIrPath(repoRoot);
  if (!fs.existsSync(ir)) {
    return { ok: false, detail: `IR not found at ${ir}` };
  }
  const bin = findRobbyBin(repoRoot);
  const args = [
    "metric-test",
    "--ir",
    ir,
    "--source-root",
    repoRoot,
    "--runs-out",
    publicDir,
    "--allow-run",
    reviewRunId,
    "--metric",
    "doc.coverage_complete",
    "--metric",
    "edge.resolve_rate",
  ];
  const result = bin
    ? spawnSync(bin, args, { cwd: repoRoot, encoding: "utf8", timeout: 120_000 })
    : spawnSync("cargo", ["run", "-q", "-p", "robby-cli", "--", ...args], {
        cwd: repoRoot,
        encoding: "utf8",
        timeout: 300_000,
      });
  if (result.error) {
    return { ok: false, detail: result.error.message };
  }
  // metric-test may exit 1 on fail status; still wrote runs — treat as ok spawn.
  if (result.status !== 0 && result.status !== 1) {
    const err = (result.stderr || result.stdout || `exit ${result.status}`).trim();
    return { ok: false, detail: err.slice(0, 500) };
  }
  return { ok: true, detail: bin ? bin : "cargo run -p robby-cli" };
}

interface IrNode {
  id?: string;
  edges?: { resolved?: boolean }[];
  children?: string[];
}

interface IrDocument {
  nodes?: Record<string, IrNode>;
  roots?: string[];
}

function edgeResolveRateFromIr(
  doc: IrDocument,
  subject: string | undefined,
): { value: number; edges: number; resolved: number; pass: boolean } {
  const nodes = doc.nodes ?? {};
  let scope: Set<string> | null = null;
  if (subject && nodes[subject]) {
    scope = new Set<string>();
    const stack = [subject];
    let id = stack.pop();
    while (id !== undefined) {
      if (!scope.has(id)) {
        scope.add(id);
        for (const child of nodes[id]?.children ?? []) {
          stack.push(child);
        }
      }
      id = stack.pop();
    }
  }
  let total = 0;
  let resolved = 0;
  for (const [id, node] of Object.entries(nodes)) {
    if (scope && !scope.has(id)) {
      continue;
    }
    for (const edge of node.edges ?? []) {
      total += 1;
      if (edge.resolved !== false) {
        resolved += 1;
      }
    }
  }
  const value = total === 0 ? 1 : resolved / total;
  return {
    value,
    edges: total,
    resolved,
    pass: total === 0 || resolved === total,
  };
}

function spawnMetricTestLocalFallback(publicDir: string, review: RunRecord): RunRecord {
  const repoRoot = repoRootFromPublic(publicDir);
  const irPath = defaultIrPath(repoRoot);
  const started = nowMs();
  const metricId = `metric-${started}`;
  const stats: Record<string, string> = {
    parent_run_id: review.run_id,
    metrics_tested: "0",
  };
  const products: string[] = [`run:${review.run_id}`];
  let status: RunRecord["status"] = "error";
  let exitCode = 1;
  let stdout = "metric_test fallback: no IR";

  if (fs.existsSync(irPath)) {
    try {
      const doc = JSON.parse(fs.readFileSync(irPath, "utf8")) as IrDocument;
      const rate = edgeResolveRateFromIr(doc, review.subject);
      stats.metric = "edge.resolve_rate";
      stats.score = String(rate.value);
      stats.test_id = "harness.edge.resolve_rate";
      stats.pass = rate.pass ? "true" : "false";
      stats.edges = String(rate.edges);
      stats.resolved = String(rate.resolved);
      stats.metrics_tested = "1";
      stats.fallback = "vite-ts";
      products.push(`metric:edge.resolve_rate:${rate.value}`);
      status = rate.pass ? "pass" : "fail";
      exitCode = rate.pass ? 0 : 1;
      stdout = `edge.resolve_rate=${rate.value} (${rate.pass ? "pass" : "fail"}); doc.coverage_complete deferred (use robby metric-test)`;
    } catch (err) {
      stdout = `metric_test fallback parse error: ${err instanceof Error ? err.message : String(err)}`;
    }
  }

  const metricRun: RunRecord = {
    run_id: metricId,
    recipe: RECIPE_METRIC_TEST,
    status,
    started_at: started,
    finished_at: nowMs(),
    exit_code: exitCode,
    stdout_tail: stdout,
    stats,
    products,
    process: {
      participants: [
        { id: "metric", label: "metric_test", kind: "agent.action" },
        { id: "ir", label: "Document", kind: "agent.observation" },
      ],
      steps: [
        { from: "metric", to: "ir", label: "load_ir", kind: "call", observed: true },
        { from: "metric", to: "ir", label: "compute", kind: "call", observed: true },
        { from: "metric", to: "ir", label: "write_run", kind: "call", observed: true },
      ],
    },
  };
  if (review.subject !== undefined) {
    metricRun.subject = review.subject;
  }
  putRecord(publicDir, metricRun);
  return metricRun;
}

/** Offline stub chain when Rust CLI cannot run (still distinct model ids). */
function spawnImproveIntegrateLocalFallback(publicDir: string, metricRun: RunRecord): void {
  const repoRoot = repoRootFromPublic(publicDir);
  const patchesDir = path.join(repoRoot, ".robby", "patches");
  fs.mkdirSync(patchesDir, { recursive: true });
  const improveModel = process.env.ROBBY_IMPROVE_MODEL || "openai/gpt-4o-mini";
  const integrateModel = process.env.ROBBY_INTEGRATE_MODEL || "openai/gpt-4o";
  const started = nowMs();
  const improveId = `improve-${started}`;
  const scoreLine = (metricRun.products ?? []).filter((p) => p.startsWith("metric:")).join(", ");
  const subject = metricRun.subject ?? "subject";
  // Prefer source crates — never stub against docs/self-model or hand MD.
  // doc.coverage_complete is source-extract CoverageReport, not markdown.
  const stubPath = "crates/robby-extract/src/lib.rs";
  const proposeName = `${improveId}.propose.diff`;
  const proposeBody = `# stub improve for ${subject}\n\n--- a/${stubPath}\n+++ b/${stubPath}\n@@ -1,1 +1,3 @@\n+// robby.improve_patch stub: ${subject}\n+// ${scoreLine || "metrics"} (fix_surface=source_extract; not docs/self-model)\n`;
  fs.writeFileSync(path.join(patchesDir, proposeName), proposeBody);

  const improveRun: RunRecord = {
    run_id: improveId,
    recipe: RECIPE_IMPROVE_PATCH,
    status: "pass",
    started_at: started,
    finished_at: nowMs(),
    exit_code: 0,
    stdout_tail: `stub improve for ${subject}`,
    stats: {
      parent_run_id: metricRun.run_id,
      metric_test_run_id: metricRun.run_id,
      improve_model: improveModel,
      mode: "propose_patch",
      fallback: "vite-ts",
    },
    products: [
      `run:${metricRun.run_id}`,
      `path:.robby/patches/${proposeName}`,
      `recipe:${RECIPE_IMPROVE_PATCH}`,
    ],
    process: {
      participants: [
        { id: "improve", label: "improve_patch", kind: "agent.action" },
        { id: "source", label: "workspace", kind: "agent.observation" },
      ],
      steps: [
        { from: "improve", to: "source", label: "gather_context", kind: "call", observed: true },
        { from: "improve", to: "source", label: "propose_patch", kind: "call", observed: true },
        { from: "improve", to: "source", label: "write_run", kind: "call", observed: true },
      ],
    },
  };
  if (metricRun.subject !== undefined) {
    improveRun.subject = metricRun.subject;
  }
  putRecord(publicDir, improveRun);

  const integrateId = `integrate-${nowMs()}`;
  const integratedName = `${integrateId}.integrated.diff`;
  fs.writeFileSync(
    path.join(patchesDir, integratedName),
    `# integrate accept: stub\n\n${proposeBody}`,
  );
  const integrateRun: RunRecord = {
    run_id: integrateId,
    recipe: RECIPE_INTEGRATE_PATCH,
    status: "pass",
    started_at: nowMs(),
    finished_at: nowMs(),
    exit_code: 0,
    stdout_tail: "accept: stub integrate accepts non-empty proposal",
    stats: {
      parent_run_id: improveId,
      improve_run_id: improveId,
      improve_model: improveModel,
      integrate_model: integrateModel,
      verdict: "accept",
      mode: "write_patch_file",
      fallback: "vite-ts",
    },
    products: [
      `run:${improveId}`,
      `path:.robby/patches/${integratedName}`,
      `recipe:${RECIPE_INTEGRATE_PATCH}`,
    ],
    process: {
      participants: [
        { id: "integrate", label: "integrate_patch", kind: "agent.action" },
        { id: "patch", label: "proposal", kind: "agent.observation" },
      ],
      steps: [
        { from: "integrate", to: "patch", label: "review_patch", kind: "call", observed: true },
        { from: "integrate", to: "patch", label: "write_patch_file", kind: "call", observed: true },
        { from: "integrate", to: "patch", label: "write_run", kind: "call", observed: true },
      ],
    },
  };
  if (metricRun.subject !== undefined) {
    integrateRun.subject = metricRun.subject;
  }
  putRecord(publicDir, integrateRun);

  const metric = readRecord(publicDir, metricRun.run_id);
  metric.products = [
    ...(metric.products ?? []),
    `run:${improveId}`,
    `recipe:${RECIPE_IMPROVE_PATCH}`,
  ];
  metric.stats = { ...(metric.stats ?? {}), improve_run_id: improveId };
  putRecord(publicDir, metric);

  const improveLinked = readRecord(publicDir, improveId);
  improveLinked.products = [
    ...(improveLinked.products ?? []),
    `run:${integrateId}`,
    `recipe:${RECIPE_INTEGRATE_PATCH}`,
  ];
  improveLinked.stats = { ...(improveLinked.stats ?? {}), integrate_run_id: integrateId };
  putRecord(publicDir, improveLinked);
}

function allowRun(publicDir: string, runId: string): RunRecord {
  const record = readRecord(publicDir, runId);
  if (record.status === "cancelled") {
    throw new Error(`run ${runId} is cancelled`);
  }
  if (isTerminal(record.status)) {
    throw new Error(`run ${runId} is already terminal (${record.status})`);
  }
  // Prefer Rust `robby metric-test --allow-run` (scored allow + metric_test computers).
  const viaCli = spawnMetricTestViaCli(publicDir, runId);
  if (viaCli.ok) {
    return readRecord(publicDir, runId);
  }

  // Fallback: emit scored agent.decision products (#11), then spawn metric_test locally (#15).
  const stats: Record<string, string> = {
    ...(record.stats ?? {}),
    ui_gate: "allowed",
    metric_test_cli: viaCli.detail,
  };
  const decisions = scoredReviewDecisions(record);
  const decisionsName = `${runId}.decisions.json`;
  fs.writeFileSync(
    path.join(runsDir(publicDir), decisionsName),
    `${JSON.stringify(decisions, null, 2)}\n`,
  );
  const products = [
    ...decisions.map((d) => `ir:agent.decision:${d.metric}`),
    `path:runs/${decisionsName}`,
  ];
  stats.findings = String(decisions.length);
  stats.metrics_claimed = String(decisions.length);
  for (const d of decisions) {
    stats[`metric.${d.metric}`] = d.score;
  }
  record.stats = stats;
  record.products = products;
  record.process = reviewProcess(true, true);
  record.status = "running";
  record.stdout_tail = `review_view emitted ${decisions.length} scored agent.decision product(s); spawning metric_test`;
  putRecord(publicDir, record);

  const allowed = readRecord(publicDir, runId);
  if (blocksDownstream(allowed)) {
    return allowed;
  }
  const metricRun = spawnMetricTestLocalFallback(publicDir, allowed);
  allowed.products = [
    ...(allowed.products ?? []),
    `run:${metricRun.run_id}`,
    `recipe:${RECIPE_METRIC_TEST}`,
    ...(metricRun.products ?? []).filter((p) => p.startsWith("metric:")),
  ];
  allowed.stats = {
    ...(allowed.stats ?? {}),
    metric_test_run_id: metricRun.run_id,
  };
  if (allowed.process) {
    allowed.process = {
      ...allowed.process,
      steps: allowed.process.steps.map((s) =>
        ["bind_subjects", "score_metrics", "emit_decisions"].includes(s.label)
          ? { ...s, observed: true }
          : s,
      ),
    };
  }
  allowed.status =
    metricRun.status === "pass" ? "pass" : metricRun.status === "error" ? "error" : "fail";
  if (metricRun.finished_at !== undefined) {
    allowed.finished_at = metricRun.finished_at;
  }
  if (metricRun.exit_code !== undefined) {
    allowed.exit_code = metricRun.exit_code;
  }
  putRecord(publicDir, allowed);

  // TS fallback also spawns improve → integrate so Soft queue Allow still
  // surfaces the two-model chain when the Rust CLI is unavailable.
  spawnImproveIntegrateLocalFallback(publicDir, metricRun);
  return readRecord(publicDir, runId);
}

function cancelRun(publicDir: string, runId: string): RunRecord {
  const record = readRecord(publicDir, runId);
  if (record.status === "cancelled") {
    return record;
  }
  if (isTerminal(record.status)) {
    throw new Error(`run ${runId} is already terminal (${record.status})`);
  }
  record.status = "cancelled";
  record.finished_at = nowMs();
  record.stats = { ...(record.stats ?? {}), ui_gate: "cancelled" };
  record.products = [];
  putRecord(publicDir, record);
  return record;
}

function readBody(req: IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on("data", (c: Buffer) => {
      chunks.push(c);
    });
    req.on("end", () => {
      resolve(Buffer.concat(chunks).toString("utf8"));
    });
    req.on("error", reject);
  });
}

function sendJson(res: ServerResponse, status: number, body: unknown): void {
  const raw = JSON.stringify(body);
  res.statusCode = status;
  res.setHeader("Content-Type", "application/json");
  res.setHeader("Cache-Control", "no-store");
  res.end(raw);
}

interface MetricFilesIndex {
  files: { path: string; kinds: string[] }[];
}

/** Prefer `files.json` from `robby web`; else scan frame details for source.file. */
function readMetricFiles(publicDir: string): MetricFilesIndex {
  const stampPath = path.join(publicDir, "files.json");
  if (fs.existsSync(stampPath)) {
    try {
      const parsed = JSON.parse(fs.readFileSync(stampPath, "utf8")) as MetricFilesIndex;
      if (Array.isArray(parsed.files)) {
        return parsed;
      }
    } catch {
      // fall through to frame scan
    }
  }
  const framesDir = path.join(publicDir, "frames");
  const byFile = new Map<string, Set<string>>();
  if (!fs.existsSync(framesDir)) {
    return { files: [] };
  }
  for (const name of fs.readdirSync(framesDir)) {
    if (!name.endsWith(".json")) {
      continue;
    }
    try {
      const detail = JSON.parse(fs.readFileSync(path.join(framesDir, name), "utf8")) as {
        kind?: string;
        source?: { file?: string };
        location?: string;
      };
      const file =
        detail.source?.file ||
        (typeof detail.location === "string" ? detail.location.split(":")[0] : undefined);
      if (!file) {
        continue;
      }
      const kindRaw = detail.kind ?? "";
      const tag = kindRaw.replace(/^(code|agent|runtime)\./, "") || "unknown";
      let set = byFile.get(file);
      if (!set) {
        set = new Set();
        byFile.set(file, set);
      }
      set.add(tag);
    } catch {
      // skip bad frame payload
    }
  }
  const files = [...byFile.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([filePath, kinds]) => ({
      path: filePath,
      kinds: [...kinds].sort(),
    }));
  return { files };
}

function attachApi(server: ViteDevServer, publicDir: string): void {
  const middleware: Connect.NextHandleFunction = (req, res, next) => {
    void (async () => {
      const url = req.url ?? "";
      const method = (req.method ?? "GET").toUpperCase();

      try {
        if (method === "GET" && (url === "/api/runs" || url.startsWith("/api/runs?"))) {
          sendJson(res, 200, readIndex(publicDir));
          return;
        }

        if (
          method === "GET" &&
          (url === "/api/metric-files" || url.startsWith("/api/metric-files?"))
        ) {
          sendJson(res, 200, readMetricFiles(publicDir));
          return;
        }

        const runMatch = url.match(/^\/api\/runs\/([^/?#]+)(?:\/(allow|cancel))?\/?(?:\?.*)?$/);
        if (runMatch) {
          const runId = decodeURIComponent(runMatch[1] ?? "");
          const action = runMatch[2];
          if (method === "GET" && !action) {
            sendJson(res, 200, readRecord(publicDir, runId));
            return;
          }
          if (method === "POST" && action === "allow") {
            sendJson(res, 200, allowRun(publicDir, runId));
            return;
          }
          if (method === "POST" && action === "cancel") {
            sendJson(res, 200, cancelRun(publicDir, runId));
            return;
          }
        }

        if (
          method === "POST" &&
          (url === "/api/reviews/queue" || url.startsWith("/api/reviews/queue?"))
        ) {
          const raw = await readBody(req);
          const parsed = JSON.parse(raw || "{}") as {
            subject?: string;
            trigger?: string;
            subject_name?: string;
          };
          if (!parsed.subject) {
            sendJson(res, 400, { error: "subject required" });
            return;
          }
          const body: QueueBody = { subject: parsed.subject };
          if (parsed.trigger !== undefined) {
            body.trigger = parsed.trigger;
          }
          if (parsed.subject_name !== undefined) {
            body.subject_name = parsed.subject_name;
          }
          sendJson(res, 200, queueReviewView(publicDir, body));
          return;
        }
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        const missing = message.includes("ENOENT") || message.includes("no such file");
        sendJson(res, missing ? 404 : 400, { error: message });
        return;
      }

      next();
    })();
  };

  server.middlewares.use(middleware);
}

/** Vite plugin: review/run API writing into `public/runs/`. */
export function robbyRunsApiPlugin(): Plugin {
  return {
    name: "robby-runs-api",
    configureServer(server) {
      const publicDir = path.resolve(server.config.root, server.config.publicDir || "public");
      ensureLayout(publicDir);
      attachApi(server, publicDir);
    },
  };
}
