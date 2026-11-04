/**
 * Mac-local Vite middleware for review queue + run allow/cancel.
 *
 * Persists the shared RunRecord JSON under `public/runs/` (same shape as
 * `robby_ir::run::RunRecord`). Prefer this path on a trusted laptop; cloud
 * Try Live is optional.
 */

import fs from "node:fs";
import type { IncomingMessage, ServerResponse } from "node:http";
import path from "node:path";
import type { Connect, Plugin, ViteDevServer } from "vite";

import {
  isTerminal,
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

function mintRunId(): string {
  return `review-${nowMs()}`;
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

function allowRun(publicDir: string, runId: string): RunRecord {
  const record = readRecord(publicDir, runId);
  if (record.status === "cancelled") {
    throw new Error(`run ${runId} is cancelled`);
  }
  if (isTerminal(record.status)) {
    throw new Error(`run ${runId} is already terminal (${record.status})`);
  }
  const stats: Record<string, string> = {
    ...(record.stats ?? {}),
    ui_gate: "allowed",
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
  record.status = "pass";
  record.finished_at = nowMs();
  record.exit_code = 0;
  record.stdout_tail = `review_view emitted ${decisions.length} scored agent.decision product(s)`;
  putRecord(publicDir, record);
  return record;
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
