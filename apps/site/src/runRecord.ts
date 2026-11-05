/**
 * Shared RunRecord shape — mirrors `robby_ir::run::RunRecord`.
 * Site `runs/<id>.json` and IR `agent.run` use the same fields.
 */

export type RunStatus = "queued" | "running" | "pass" | "fail" | "error" | "cancelled";

export const RECIPE_REVIEW_VIEW = "robby.review_view";
export const RECIPE_METRIC_TEST = "robby.metric_test";
export const RECIPE_IMPROVE_PATCH = "robby.improve_patch";
export const RECIPE_INTEGRATE_PATCH = "robby.integrate_patch";
export const RECIPE_CURSOR_CLOUD = "cursor.cloud";

/** Disk stem for `runs/<stem>.json` — mirrors Rust `run_file_stem` (`:` → `_`). */
export function runFileStem(runId: string): string {
  return runId.replaceAll(":", "_");
}

/** Catalog metrics scored on Allow for `robby.review_view` (mirrors robby_render::review_metrics). */
export const REVIEW_METRIC_UI_RUNRECORD_SHAPE = "ui.runrecord_shape_compliance";
export const REVIEW_METRIC_UI_RUNRECORD_SHAPE_TEST = "metric_test.ui.runrecord_shape_compliance";
export const REVIEW_METRIC_AGENT_PROCESS_STAGE = "agent.action_or_process_stage_rate";
export const REVIEW_METRIC_AGENT_PROCESS_STAGE_TEST =
  "metric_test.agent.action_or_process_stage_rate";

export interface ScoredDecisionProduct {
  kind: string;
  verdict: string;
  metric: string;
  score: string;
  test_id: string;
  direction: string;
  subject: string;
  run_id: string;
  trigger: string;
  source_recipe: string;
  reason: string;
}

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
  finished_at?: number;
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
