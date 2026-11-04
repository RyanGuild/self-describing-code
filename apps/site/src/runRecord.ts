/**
 * Shared RunRecord shape — mirrors `robby_ir::run::RunRecord`.
 * Site `runs/<id>.json` and IR `agent.run` use the same fields.
 */

export type RunStatus =
  | "queued"
  | "running"
  | "pass"
  | "fail"
  | "error"
  | "cancelled";

export const RECIPE_REVIEW_VIEW = "robby.review_view";
export const RECIPE_METRIC_TEST = "robby.metric_test";

export interface RunRecord {
  run_id: string;
  recipe: string;
  subject?: string;
  status: RunStatus;
  started_at?: number;
  finished_at?: number;
  exit_code?: number;
  stdout_tail?: string;
  stats?: Record<string, string>;
  products?: string[];
  process?: {
    participants: { id: string; label: string; kind: string }[];
    steps: {
      from: string;
      to: string;
      label: string;
      kind: string;
      observed: boolean;
    }[];
  };
}

export interface RunIndexEntry {
  run_id: string;
  recipe: string;
  status: string;
  subject?: string;
  started_at?: number;
}

export interface RunsIndex {
  runs: RunIndexEntry[];
}

export function isTerminal(status: RunStatus): boolean {
  return status === "pass" || status === "fail" || status === "error" || status === "cancelled";
}

export function blocksDownstream(record: RunRecord): boolean {
  if (record.status === "queued" || record.status === "cancelled" || record.status === "error") {
    return true;
  }
  const gate = record.stats?.ui_gate;
  return gate === "awaiting_allow" || gate === "paused" || gate === "cancelled";
}
