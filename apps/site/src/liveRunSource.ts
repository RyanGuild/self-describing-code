/**
 * Mirrors `robby_ir::run::LiveRunSource` — runtime-fetchable live run feeds.
 * Site `runs/sources.json` is the catalog the Vite middleware + viewer poll.
 */

import {
  RECIPE_CURSOR_CLOUD,
  type RunRecord,
  type RunStatus,
  runFileStem,
} from "./runRecord";

export type LiveRunTransport =
  | { kind: "file_poll"; path: string; interval_ms: number }
  | { kind: "http_poll"; url: string; interval_ms: number }
  | { kind: "file_watch"; path: string };

export interface LiveRunSource {
  id: string;
  recipe: string;
  label: string;
  transport: LiveRunTransport;
  adapter: string;
  enabled?: boolean;
}

export interface LiveRunSourcesFile {
  sources: LiveRunSource[];
}

export const DEFAULT_CURSOR_SOURCE: LiveRunSource = {
  id: RECIPE_CURSOR_CLOUD,
  recipe: RECIPE_CURSOR_CLOUD,
  label: "Cursor cloud / Project workers",
  transport: {
    kind: "file_poll",
    path: ".robby/external/cursor/agents.json",
    interval_ms: 5000,
  },
  adapter: "cursor_agents_v1",
  enabled: true,
};

export function defaultLiveRunSources(): LiveRunSource[] {
  return [DEFAULT_CURSOR_SOURCE];
}

/** Cursor MCP `list-cloud-agents` stub (camelCase). */
export interface CursorAgentStub {
  bcId: string;
  name?: string | null;
  status?: string | null;
  source?: string | null;
  createdAtMs?: number | null;
  updatedAtMs?: number | null;
  lastMessageActivityAtMs?: number | null;
  url?: string | null;
  branchName?: string | null;
  repoUrl?: string | null;
  originalModelName?: string | null;
  privateWorkerId?: string | null;
  usePrivateWorker?: boolean | null;
  role?: string | null;
  subject?: string | null;
}

export interface CursorIngestEnvelope {
  agents?: CursorAgentStub[];
  events?: unknown[];
  eventsByBcId?: Record<string, unknown[]>;
}

export function mapCursorStatus(raw: string | null | undefined): RunStatus {
  switch ((raw ?? "UNSPECIFIED").trim().toUpperCase()) {
    case "RUNNING":
    case "WAITING_FOR_BACKGROUND_WORK":
      return "running";
    case "NOT_YET_STARTED":
    case "UNSPECIFIED":
      return "queued";
    case "IDLE":
      return "pass";
    case "ERROR":
      return "error";
    case "ARCHIVED":
    case "EXPIRED":
      return "cancelled";
    default:
      return "error";
  }
}

export function cursorAgentToRunRecord(agent: CursorAgentStub): RunRecord {
  const status = mapCursorStatus(agent.status);
  const started = agent.createdAtMs ?? 0;
  const activity =
    agent.updatedAtMs ?? agent.lastMessageActivityAtMs ?? started;
  const name = agent.name ?? agent.bcId;
  const stats: Record<string, string> = {
    adapter: "cursor.cloud",
    bc_id: agent.bcId,
    event_count: "0",
  };
  if (agent.source) {
    stats["cursor.source"] = agent.source;
  }
  if (agent.originalModelName) {
    stats.model = agent.originalModelName;
  }
  if (agent.branchName) {
    stats.branch = agent.branchName;
  }
  if (agent.repoUrl) {
    stats.repo = agent.repoUrl;
  }
  if (agent.url) {
    stats.agent_url = agent.url;
  }
  if (agent.role) {
    stats.role = agent.role;
  }
  if (agent.usePrivateWorker) {
    stats.private_worker = "true";
  }
  if (agent.privateWorkerId) {
    stats.private_worker_id = agent.privateWorkerId;
  }

  const products: string[] = [];
  if (agent.url) {
    products.push(`url:${agent.url}`);
  }
  if (agent.branchName) {
    products.push(`branch:${agent.branchName}`);
  }

  return {
    run_id: `cursor:${agent.bcId}`,
    recipe: RECIPE_CURSOR_CLOUD,
    subject: agent.subject ?? "workspace:",
    status,
    started_at: started,
    finished_at: activity,
    stdout_tail: `cursor.cloud · ${name} · ${status}`,
    stats,
    products,
    process: {
      participants: [
        { id: "create", label: "create", kind: "agent.action" },
        { id: "run", label: "run", kind: "agent.action" },
      ],
      steps: [
        {
          from: "create",
          to: "run",
          label: "run",
          kind: "call",
          observed: true,
        },
      ],
    },
  };
}

export function parseCursorAgentsJson(raw: string): CursorAgentStub[] {
  const parsed = JSON.parse(raw) as CursorIngestEnvelope | CursorAgentStub[] | CursorAgentStub;
  if (Array.isArray(parsed)) {
    return parsed;
  }
  if (parsed && typeof parsed === "object" && "agents" in parsed && Array.isArray(parsed.agents)) {
    return parsed.agents;
  }
  if (parsed && typeof parsed === "object" && "bcId" in parsed) {
    return [parsed as CursorAgentStub];
  }
  return [];
}

export { runFileStem };
