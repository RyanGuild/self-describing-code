/**
 * IR improvement metric catalog for the Metrics org mode.
 *
 * Ids mirror `robby_ir::metrics::ids`, review_view scored products, and the
 * Project plan `docs/ir-improvement-metrics.md`. Scores are filled at runtime
 * from `runs/` (metric_test stats, review decisions, run `stats`).
 */

export type MetricDirection = "higher_better" | "lower_better" | "context";

export type MetricComputer =
  | "robby_ir::metrics"
  | "robby.review_view"
  | "robby.metric_test"
  | "planned";

/** One catalog entry — type is the family prefix (`doc`, `edge`, `ui`, …). */
export interface MetricCatalogEntry {
  id: string;
  /** Family / metric type used for "sort by type". */
  type: string;
  direction: MetricDirection;
  /** How the score is computed (human-readable). */
  definition: string;
  /** Compact formula / expression. */
  formula: string;
  /** What the metric attaches to in IR / products. */
  attachment: string;
  /** Where the computer lives today (or planned). */
  computer: MetricComputer;
  /** Empirical harness id when known. */
  testId?: string | undefined;
  /** Where the score appears in the product. */
  appearsOn?: string | undefined;
  /** Optional default filesystem path or subject hint when no run is attached. */
  path?: string | undefined;
  /** Short label for the subject/module column when unscored. */
  subject?: string | undefined;
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

function entry(
  partial: Omit<MetricCatalogEntry, "definition" | "formula" | "attachment" | "computer"> & {
    definition: string;
    formula: string;
    attachment: string;
    computer: MetricComputer;
  },
): MetricCatalogEntry {
  return partial;
}

/** Built-in catalog — first-slice computers + review scores + plan families. */
export const METRIC_CATALOG: readonly MetricCatalogEntry[] = [
  entry({
    id: "doc.coverage_complete",
    type: "doc",
    direction: "higher_better",
    subject: "workspace",
    definition:
      "Coverage gate for the workspace IR: 1 when CoverageReport::is_complete() (no unknown gaps), else 0.",
    formula: "is_complete(CoverageReport) → 1 | 0",
    attachment: "document / CoverageReport",
    computer: "robby_ir::metrics",
    testId: "metric_test.doc.coverage_complete",
    appearsOn: "RunRecord.stats; robby check; metric_test",
  }),
  entry({
    id: "edge.resolve_rate",
    type: "edge",
    direction: "higher_better",
    subject: "workspace",
    definition:
      "Share of IR edges with resolved=true. Document-wide by default; can scope to one node's outgoing edges. Empty edge sets score 1.0 (vacuous).",
    formula: "resolved_edges / total_edges  (0 edges → 1.0)",
    attachment: "document or node edges",
    computer: "robby_ir::metrics",
    testId: "metric_test.edge.resolve_rate",
    appearsOn: "RunRecord.stats (+ edge.resolved / edge.total details)",
  }),
  entry({
    id: "ui.runrecord_shape_compliance",
    type: "ui",
    direction: "higher_better",
    subject: "runs/",
    path: "apps/site/public/runs",
    definition:
      "Scored on Allow for robby.review_view: 1 when the gate produced a shared RunRecord-shaped review product under runs/.",
    formula: "Allow → RunRecord JSON parseable as shared shape → 1",
    attachment: "run / product",
    computer: "robby.review_view",
    testId: "metric_test.ui.runrecord_shape_compliance",
    appearsOn: "review *.decisions.json; RunRecord.stats metric.*",
  }),
  entry({
    id: "agent.action_or_process_stage_rate",
    type: "agent",
    direction: "higher_better",
    subject: "robby.review_view",
    definition:
      "1 when the review RunRecord carries a non-empty Process (bind_subjects / score_metrics / emit_decisions stages), else 0.",
    formula: "process.steps.length > 0 → 1 | 0",
    attachment: "run Process",
    computer: "robby.review_view",
    testId: "metric_test.agent.action_or_process_stage_rate",
    appearsOn: "review decisions; Pipeline HUD",
  }),
  entry({
    id: "doc.code_ratio_mean",
    type: "doc",
    direction: "higher_better",
    subject: "workspace",
    definition: "Mean of FileCoverage::code_ratio across all files in the coverage report.",
    formula: "mean(file.code_ratio for file in CoverageReport.files)",
    attachment: "document / CoverageReport",
    computer: "planned",
    appearsOn: "Report tables; workspace HUD",
  }),
  entry({
    id: "doc.unknown_gap_bytes",
    type: "doc",
    direction: "lower_better",
    subject: "workspace",
    definition: "Sum of unknown-gap byte lengths reported by coverage failures.",
    formula: "sum(unknown_gap_bytes)",
    attachment: "document / CoverageReport",
    computer: "planned",
    appearsOn: "Coverage failures; immersive heat",
  }),
  entry({
    id: "doc.validate_error_count",
    type: "doc",
    direction: "lower_better",
    subject: "workspace",
    definition: "Count of validate() diagnostics with Severity::Error.",
    formula: "count(d in validate(doc) where d.severity == Error)",
    attachment: "document",
    computer: "planned",
    appearsOn: "robby check; run observation",
  }),
  entry({
    id: "doc.known_kind_rate",
    type: "doc",
    direction: "higher_better",
    subject: "workspace",
    definition: "Share of nodes whose kind passes kind::is_known.",
    formula: "known_kind_nodes / total_nodes",
    attachment: "document nodes",
    computer: "planned",
    appearsOn: "Soft gap list; Structure filter",
  }),
  entry({
    id: "file.code_ratio",
    type: "file",
    direction: "higher_better",
    definition: "Per-file covered code bytes over non-trivia bytes (FileCoverage::code_ratio).",
    formula: "code_bytes / non_trivia_bytes",
    attachment: "code.file",
    computer: "planned",
    appearsOn: "FileCoverage; frame tint",
  }),
  entry({
    id: "module.child_span_rate",
    type: "module",
    direction: "higher_better",
    definition: "Among a module's children, share that carry a source span.",
    formula: "children_with_span / children",
    attachment: "code.module / crate / package",
    computer: "planned",
    appearsOn: "Module cards; extract completeness",
  }),
  entry({
    id: "type.member_span_rate",
    type: "type",
    direction: "higher_better",
    definition: "Fields, variants, or methods with spans over total members.",
    formula: "members_with_span / members",
    attachment: "type-like nodes",
    computer: "planned",
    appearsOn: "Type frames; extract",
  }),
  entry({
    id: "type.relation_resolve_rate",
    type: "type",
    direction: "higher_better",
    definition: "Implements / Extends / Uses edges with resolved=true over those relation edges.",
    formula: "resolved(Implements|Extends|Uses) / relation_edges",
    attachment: "type relation edges",
    computer: "planned",
    appearsOn: "Portals; type diagrams",
  }),
  entry({
    id: "fn.call_resolve_rate",
    type: "fn",
    direction: "higher_better",
    definition: "Outgoing Calls edges with resolved=true over outgoing Calls.",
    formula: "resolved(Calls) / Calls",
    attachment: "code.function / method / macro",
    computer: "planned",
    appearsOn: "Entrypoints; unresolved portals",
  }),
  entry({
    id: "fn.process_coverage",
    type: "fn",
    direction: "higher_better",
    definition: "Process steps that cite a source span over total steps (scrubber ↔ source sync).",
    formula: "steps_with_span / process.steps",
    attachment: "callable Process",
    computer: "planned",
    appearsOn: "Scrubber; Process Mermaid",
  }),
  entry({
    id: "edge.confidence_mean",
    type: "edge",
    direction: "higher_better",
    definition: "Mean confidence across edges in scope.",
    formula: "mean(edge.confidence)",
    attachment: "edges",
    computer: "planned",
    appearsOn: "Frame meta; edge overlay",
  }),
  entry({
    id: "edge.unresolved_calls",
    type: "edge",
    direction: "lower_better",
    definition: "Count of Calls edges with resolved=false.",
    formula: "count(Calls where !resolved)",
    attachment: "Calls edges",
    computer: "planned",
    appearsOn: "Entrypoints org",
  }),
  entry({
    id: "process.observed_rate",
    type: "process",
    direction: "higher_better",
    definition: "Observed Process steps after absorb over total steps.",
    formula: "observed_steps / steps",
    attachment: "Process on any node",
    computer: "planned",
    appearsOn: "Mermaid solid/dashed",
  }),
  entry({
    id: "path.entry_coverage",
    type: "path",
    direction: "higher_better",
    definition:
      "Tagged mains/tests (entrypoints) that have a non-empty code path over those entrypoints.",
    formula: "entrypoints_with_path / tagged_entrypoints",
    attachment: "Document.code_paths",
    computer: "planned",
    appearsOn: "Entrypoints scene",
  }),
  entry({
    id: "agent.decision_reason_rate",
    type: "agent",
    direction: "higher_better",
    definition: "agent.decision nodes with a non-empty reason annotation over decisions.",
    formula: "decisions_with_reason / decisions",
    attachment: "agent.decision",
    computer: "planned",
    appearsOn: "Enrich quality",
  }),
  entry({
    id: "agent.run_terminal_rate",
    type: "agent",
    direction: "higher_better",
    definition: "Runs that reached a terminal status over started runs.",
    formula: "terminal_runs / started_runs",
    attachment: "RunRecord",
    computer: "planned",
    appearsOn: "Runs mode UI",
  }),
  entry({
    id: "comment.decision_coverage",
    type: "comment",
    direction: "higher_better",
    definition: "Enrichable comments that have an agent.decision child.",
    formula: "comments_with_decision / enrichable_comments",
    attachment: "code.comment",
    computer: "planned",
    appearsOn: "Enrich theater; EnrichStats",
  }),
  entry({
    id: "prov.confidence_mean",
    type: "prov",
    direction: "higher_better",
    definition: "Mean provenance.confidence for extractor nodes in scope.",
    formula: "mean(node.provenance.confidence)",
    attachment: "nodes (cross-cutting)",
    computer: "planned",
    appearsOn: "Frame detail",
  }),
];

const CATALOG_BY_ID = new Map(METRIC_CATALOG.map((e) => [e.id, e]));

export type MetricsSort = "type" | "path";

export function catalogEntry(id: string): MetricCatalogEntry | undefined {
  return CATALOG_BY_ID.get(id);
}

/** Fallback detail text for ids discovered only from runs. */
export function metricCalculationDetail(id: string): {
  definition: string;
  formula: string;
  attachment: string;
  computer: MetricComputer | "unknown";
  testId?: string | undefined;
  appearsOn?: string | undefined;
  direction: MetricDirection | "";
} {
  const known = CATALOG_BY_ID.get(id);
  if (known) {
    return {
      definition: known.definition,
      formula: known.formula,
      attachment: known.attachment,
      computer: known.computer,
      testId: known.testId,
      appearsOn: known.appearsOn,
      direction: known.direction,
    };
  }
  return {
    definition:
      "Open catalog id observed in run stats or products. No built-in computer description is registered in the site catalog yet.",
    formula: id,
    attachment: "unknown",
    computer: "unknown",
    direction: "",
  };
}

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

/** Stable key for a scored/catalog row instance. */
export function metricRowKey(row: Pick<MetricRow, "id" | "subject" | "path">): string {
  return `${row.id}\0${row.subject}\0${row.path}`;
}

/** Merge catalog seeds with live observations (latest score wins per id+subject+path). */
export function mergeMetricRows(
  seeds: MetricRow[],
  observations: MetricObservation[],
): MetricRow[] {
  const map = new Map<string, MetricRow>();
  for (const row of seeds) {
    map.set(metricRowKey(row), { ...row });
  }
  for (const obs of observations) {
    const id = obs.id;
    const subject = obs.subject ?? CATALOG_BY_ID.get(id)?.subject ?? "";
    const path = normalizePathHint(obs.path) || CATALOG_BY_ID.get(id)?.path || "";
    const key = metricRowKey({ id, subject, path });
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

/** Companion detail keys often stamped beside a primary score in run stats. */
export function companionStatKeys(metricId: string): string[] {
  switch (metricId) {
    case "doc.coverage_complete":
      return ["doc.coverage_files", "doc.unknown_gap_count"];
    case "edge.resolve_rate":
      return ["edge.resolved", "edge.total"];
    default:
      return [];
  }
}
