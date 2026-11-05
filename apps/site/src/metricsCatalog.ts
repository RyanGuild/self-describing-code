/**
 * IR improvement metric catalog for the Metrics org mode.
 *
 * Ids mirror `robby_ir::metrics::ids`, review_view scored products, and the
 * Project plan `docs/ir-improvement-metrics.md`. Scores are filled at runtime
 * from `runs/` (metric_test stats, review decisions, run `stats`).
 */

export type MetricDirection = "higher_better" | "lower_better" | "context";

/** One catalog entry — type is the family prefix (`doc`, `edge`, `ui`, …). */
export interface MetricCatalogEntry {
  id: string;
  /** Family / metric type used for "sort by type". */
  type: string;
  direction: MetricDirection;
  /** Optional default filesystem path or subject hint when no run is attached. */
  path?: string;
  /** Short label for the subject/module column when unscored. */
  subject?: string;
}

export interface MetricRow {
  id: string;
  type: string;
  score: string | null;
  direction: MetricDirection | "";
  /** Module / subject (FQN, node id, recipe subject, …). */
  subject: string;
  /** Filesystem path when known (`path:…`, `doc_path`, review artifact). */
  path: string;
  source: "catalog" | "metric_test" | "review" | "run";
  runId?: string | undefined;
}

/** Built-in catalog — first-slice computers + review scores + plan families. */
export const METRIC_CATALOG: readonly MetricCatalogEntry[] = [
  // robby_ir::metrics first slice
  {
    id: "doc.coverage_complete",
    type: "doc",
    direction: "higher_better",
    subject: "workspace",
  },
  {
    id: "edge.resolve_rate",
    type: "edge",
    direction: "higher_better",
    subject: "workspace",
  },
  // review_view Allow products (robby_render::review_metrics)
  {
    id: "ui.runrecord_shape_compliance",
    type: "ui",
    direction: "higher_better",
    subject: "runs/",
    path: "apps/site/public/runs",
  },
  {
    id: "agent.action_or_process_stage_rate",
    type: "agent",
    direction: "higher_better",
    subject: "robby.review_view",
  },
  // Broader catalog (computers land over time; still listed for improvement map)
  { id: "doc.code_ratio_mean", type: "doc", direction: "higher_better", subject: "workspace" },
  { id: "doc.unknown_gap_bytes", type: "doc", direction: "lower_better", subject: "workspace" },
  { id: "doc.validate_error_count", type: "doc", direction: "lower_better", subject: "workspace" },
  { id: "doc.known_kind_rate", type: "doc", direction: "higher_better", subject: "workspace" },
  { id: "file.code_ratio", type: "file", direction: "higher_better" },
  { id: "module.child_span_rate", type: "module", direction: "higher_better" },
  { id: "type.member_span_rate", type: "type", direction: "higher_better" },
  { id: "type.relation_resolve_rate", type: "type", direction: "higher_better" },
  { id: "fn.call_resolve_rate", type: "fn", direction: "higher_better" },
  { id: "fn.process_coverage", type: "fn", direction: "higher_better" },
  { id: "edge.confidence_mean", type: "edge", direction: "higher_better" },
  { id: "edge.unresolved_calls", type: "edge", direction: "lower_better" },
  { id: "process.observed_rate", type: "process", direction: "higher_better" },
  { id: "path.entry_coverage", type: "path", direction: "higher_better" },
  { id: "agent.decision_reason_rate", type: "agent", direction: "higher_better" },
  { id: "agent.run_terminal_rate", type: "agent", direction: "higher_better" },
  { id: "comment.decision_coverage", type: "comment", direction: "higher_better" },
  { id: "prov.confidence_mean", type: "prov", direction: "higher_better" },
];

const CATALOG_BY_ID = new Map(METRIC_CATALOG.map((e) => [e.id, e]));

export type MetricsSort = "type" | "path";

export function metricTypeOf(id: string): string {
  const known = CATALOG_BY_ID.get(id);
  if (known) {
    return known.type;
  }
  const dot = id.indexOf(".");
  return dot > 0 ? id.slice(0, dot) : id;
}

export function normalizePathHint(raw: string | undefined | null): string {
  if (!raw) {
    return "";
  }
  let s = raw.trim();
  if (s.startsWith("path:")) {
    s = s.slice("path:".length);
  }
  if (s.startsWith("file:")) {
    s = s.slice("file:".length);
  }
  // Node / workspace subjects are not filesystem paths.
  if (s.startsWith("workspace:") || s.startsWith("rust://") || s.startsWith("n_")) {
    return "";
  }
  return s;
}

function directionOf(id: string, raw?: string): MetricDirection | "" {
  if (raw === "higher_better" || raw === "lower_better" || raw === "context") {
    return raw;
  }
  return CATALOG_BY_ID.get(id)?.direction ?? "";
}

/** Seed rows from the static catalog (unscored placeholders). */
export function catalogSeedRows(): MetricRow[] {
  return METRIC_CATALOG.map((e) => ({
    id: e.id,
    type: e.type,
    score: null,
    direction: e.direction,
    subject: e.subject ?? "",
    path: e.path ?? "",
    source: "catalog" as const,
  }));
}

export interface MetricObservation {
  id: string;
  score: string;
  direction?: string | undefined;
  subject?: string | undefined;
  path?: string | undefined;
  source: MetricRow["source"];
  runId?: string | undefined;
}

/** Merge catalog seeds with live observations (latest score wins per id+subject+path). */
export function mergeMetricRows(
  seeds: MetricRow[],
  observations: MetricObservation[],
): MetricRow[] {
  const keyOf = (id: string, subject: string, path: string) => `${id}\0${subject}\0${path}`;
  const map = new Map<string, MetricRow>();
  for (const row of seeds) {
    map.set(keyOf(row.id, row.subject, row.path), { ...row });
  }
  for (const obs of observations) {
    const id = obs.id;
    const subject = obs.subject ?? CATALOG_BY_ID.get(id)?.subject ?? "";
    const path = normalizePathHint(obs.path) || CATALOG_BY_ID.get(id)?.path || "";
    const key = keyOf(id, subject, path);
    const prev = map.get(key);
    map.set(key, {
      id,
      type: metricTypeOf(id),
      score: obs.score,
      direction: directionOf(id, obs.direction) || prev?.direction || "",
      subject,
      path,
      source: obs.source,
      runId: obs.runId,
    });
  }
  return [...map.values()];
}

export function sortMetricRows(rows: MetricRow[], sort: MetricsSort): MetricRow[] {
  const copy = [...rows];
  if (sort === "path") {
    copy.sort((a, b) => {
      const ap = a.path || "\uffff";
      const bp = b.path || "\uffff";
      return ap.localeCompare(bp) || a.id.localeCompare(b.id) || a.subject.localeCompare(b.subject);
    });
  } else {
    copy.sort(
      (a, b) =>
        a.type.localeCompare(b.type) ||
        a.id.localeCompare(b.id) ||
        a.subject.localeCompare(b.subject),
    );
  }
  return copy;
}

/** Known primary catalog ids + `metric.<id>` / `*.direction` companions. */
export function isPrimaryMetricStatKey(key: string): boolean {
  if (key.endsWith(".direction") || key.endsWith(".test_id")) {
    return false;
  }
  if (key.startsWith("metric.")) {
    return true;
  }
  if (CATALOG_BY_ID.has(key)) {
    return true;
  }
  // Open family.name ids (one dot, no nested detail counters).
  return /^[a-z]+\.[a-z0-9_]+$/i.test(key) && !key.startsWith("cursor.");
}

export function primaryMetricIdFromStatKey(key: string): string {
  return key.startsWith("metric.") ? key.slice("metric.".length) : key;
}

/** Parse `metric:<id>:<score>` products from metric_test. */
export function parseMetricProduct(product: string): { id: string; score: string } | null {
  if (!product.startsWith("metric:")) {
    return null;
  }
  const rest = product.slice("metric:".length);
  const colon = rest.lastIndexOf(":");
  if (colon <= 0) {
    return null;
  }
  const id = rest.slice(0, colon);
  const score = rest.slice(colon + 1);
  if (!id || !score) {
    return null;
  }
  return { id, score };
}
