/**
 * Mac-local Vite middleware for review queue + run allow/cancel + live sources.
 *
 * Persists the shared RunRecord JSON under `public/runs/` (same shape as
 * `robby_ir::run::RunRecord`). Live feeds are declared in `runs/sources.json`
 * (`LiveRunSource` IR); Cursor agents land via file poll / POST push.
 */

import fs from "node:fs";
import type { IncomingMessage, ServerResponse } from "node:http";
import path from "node:path";
import type { Connect, Plugin, ViteDevServer } from "vite";

import {
  isTerminal,
  RECIPE_REVIEW_VIEW,
  type RunIndexEntry,
  type RunRecord,
  type RunsIndex,
  runFileStem,
} from "./src/runRecord";
import {
  type LiveRunSource,
  type LiveRunSourcesFile,
  cursorAgentToRunRecord,
  defaultLiveRunSources,
  parseCursorAgentsJson,
} from "./src/liveRunSource";

function runsDir(publicDir: string): string {
  return path.join(publicDir, "runs");
}

function indexPath(publicDir: string): string {
  return path.join(runsDir(publicDir), "index.json");
}

function recordPath(publicDir: string, runId: string): string {
  return path.join(runsDir(publicDir), `${runFileStem(runId)}.json`);
}

function sourcesPath(publicDir: string): string {
  return path.join(runsDir(publicDir), "sources.json");
}

function ensureLayout(publicDir: string): void {
  const dir = runsDir(publicDir);
  fs.mkdirSync(dir, { recursive: true });
  const idx = indexPath(publicDir);
  if (!fs.existsSync(idx)) {
    writeIndex(publicDir, { runs: [] });
  }
  if (!fs.existsSync(sourcesPath(publicDir))) {
    writeSources(publicDir, { sources: defaultLiveRunSources() });
  }
}

function readSources(publicDir: string): LiveRunSourcesFile {
  ensureLayout(publicDir);
  const raw = fs.readFileSync(sourcesPath(publicDir), "utf8");
  return JSON.parse(raw) as LiveRunSourcesFile;
}

function writeSources(publicDir: string, file: LiveRunSourcesFile): void {
  fs.mkdirSync(runsDir(publicDir), { recursive: true });
  fs.writeFileSync(sourcesPath(publicDir), `${JSON.stringify(file, null, 2)}\n`);
}

/** Repo root is three levels above `apps/site/public`. */
function repoRootFromPublic(publicDir: string): string {
  return path.resolve(publicDir, "..", "..", "..");
}

function resolveSourcePath(publicDir: string, relOrAbs: string): string {
  if (path.isAbsolute(relOrAbs)) {
    return relOrAbs;
  }
  return path.resolve(repoRootFromPublic(publicDir), relOrAbs);
}

function upsertCursorAgents(publicDir: string, raw: string): RunRecord[] {
  const agents = parseCursorAgentsJson(raw);
  const records = agents.map(cursorAgentToRunRecord);
  for (const record of records) {
    putRecord(publicDir, record);
  }
  return records;
}

function refreshFilePollSources(publicDir: string): {
  refreshed: number;
  records: number;
  paths: string[];
} {
  const file = readSources(publicDir);
  let refreshed = 0;
  let records = 0;
  const paths: string[] = [];
  for (const source of file.sources) {
    if (source.enabled === false) {
      continue;
    }
    if (source.transport.kind !== "file_poll" && source.transport.kind !== "file_watch") {
      continue;
    }
    if (source.adapter !== "cursor_agents_v1") {
      continue;
    }
    const abs = resolveSourcePath(publicDir, source.transport.path);
    paths.push(abs);
    if (!fs.existsSync(abs)) {
      continue;
    }
    const raw = fs.readFileSync(abs, "utf8");
    const written = upsertCursorAgents(publicDir, raw);
    refreshed += 1;
    records += written.length;
  }
  return { refreshed, records, paths };
}

function watchLiveSources(publicDir: string): void {
  const seen = new Map<string, number>();
  const tick = () => {
    try {
      const file = readSources(publicDir);
      let minInterval = 5_000;
      for (const source of file.sources) {
        if (source.enabled === false) {
          continue;
        }
        if (source.transport.kind === "file_poll") {
          minInterval = Math.min(minInterval, source.transport.interval_ms || 5_000);
          const abs = resolveSourcePath(publicDir, source.transport.path);
          if (!fs.existsSync(abs)) {
            continue;
          }
          const mtime = fs.statSync(abs).mtimeMs;
          const prev = seen.get(abs);
          if (prev === undefined || mtime !== prev) {
            seen.set(abs, mtime);
            if (source.adapter === "cursor_agents_v1") {
              const raw = fs.readFileSync(abs, "utf8");
              upsertCursorAgents(publicDir, raw);
            }
          }
        }
      }
      // Re-arm with the tightest configured interval.
      setTimeout(tick, minInterval);
    } catch (err) {
      console.error("[robby-runs-api] live source watch failed", err);
      setTimeout(tick, 5_000);
    }
  };
  setTimeout(tick, 1_000);
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

function reviewProcess(observedGate: boolean): NonNullable<RunRecord["process"]> {
  return {
    participants: [
      { id: "review", label: "review_view", kind: "agent.action" },
      { id: "ui", label: "ui_gate", kind: "agent.action" },
    ],
    steps: [
      { from: "review", to: "ui", label: "queue", kind: "call", observed: false },
      {
        from: "review",
        to: "ui",
        label: "await_ui_gate",
        kind: "call",
        observed: observedGate,
      },
      { from: "review", to: "ui", label: "bind_subjects", kind: "call", observed: false },
      { from: "review", to: "ui", label: "score_metrics", kind: "call", observed: false },
      { from: "review", to: "ui", label: "emit_decisions", kind: "call", observed: false },
    ],
  };
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
    process: reviewProcess(false),
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
  record.status = "running";
  record.stats = { ...(record.stats ?? {}), ui_gate: "allowed" };
  record.process = reviewProcess(true);
  // Slice 1 stub: no LLM / metric_test products.
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

        if (
          method === "GET" &&
          (url === "/api/runs/sources" || url.startsWith("/api/runs/sources?"))
        ) {
          sendJson(res, 200, readSources(publicDir));
          return;
        }

        if (
          method === "POST" &&
          (url === "/api/runs/live/refresh" || url.startsWith("/api/runs/live/refresh?"))
        ) {
          sendJson(res, 200, refreshFilePollSources(publicDir));
          return;
        }

        if (
          method === "POST" &&
          (url === "/api/runs/live/push" || url.startsWith("/api/runs/live/push?"))
        ) {
          const raw = await readBody(req);
          const records = upsertCursorAgents(publicDir, raw || "{}");
          // Keep the dump file in sync for file_poll sources / CLI watch.
          const dumpRel = ".robby/external/cursor/agents.json";
          const dumpAbs = resolveSourcePath(publicDir, dumpRel);
          fs.mkdirSync(path.dirname(dumpAbs), { recursive: true });
          fs.writeFileSync(dumpAbs, raw || "{}");
          sendJson(res, 200, { upserted: records.length, run_ids: records.map((r) => r.run_id) });
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
      // Ensure default LiveRunSource catalog exists even before `robby web`.
      try {
        readSources(publicDir);
      } catch {
        writeSources(publicDir, { sources: defaultLiveRunSources() });
      }
      attachApi(server, publicDir);
      watchLiveSources(publicDir);
    },
  };
}
