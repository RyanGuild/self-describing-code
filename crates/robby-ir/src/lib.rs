//! Robby's intermediate representation.
//!
//! One schema, three producers (extractor, macros, trace recorder), one view.
//! Everything that renders Robby's self-model reads this crate and nothing else.
//!
//! # Invariants
//!
//! 1. **Total representation.** Every byte of source is owned by a node or
//!    classified by [`coverage`]. Unclassified bytes fail the gate.
//! 2. **Diagrams are views.** [`process::Process`] and [`codepath::CodePath`]
//!    are the stored forms; Mermaid / Prezi layouts are derived. Nothing stores
//!    markup.
//! 3. **Open kinds.** [`kind::Kind`] is a namespaced string, so `agent.*`
//!    producers can be added without a schema break.
//! 4. **Edges are never dropped.** Unresolved edges survive with
//!    `resolved: false` and a confidence score.

pub mod codepath;
pub mod contract;
pub mod coverage;
pub mod kind;
pub mod metrics;
pub mod model;
pub mod process;
pub mod run;
pub mod span;
pub mod vocab;

pub use codepath::{CodePath, CodePathEdge, DEFAULT_MAX_DEPTH, build_code_paths, find_entrypoints};
pub use contract::{Absorb, CoverageGate, KindNamespace};
pub use coverage::{
    CoverageReport, FileCoverage, Gap, GapReason, document_coverage, file_coverage,
};
pub use kind::Kind;
pub use metrics::{
    Direction, MetricMeasurement, UnknownMetric, doc_coverage_complete, edge_resolve_rate,
    edge_resolve_rate_at, stamp_first_slice, write_into_stats,
};
pub use model::{
    Diagnostic, Document, Edge, EdgeKind, Lang, Node, NodeId, Provenance, SCHEMA_VERSION, Severity,
    Source, leaf_name, validate,
};
pub use process::{Block, BlockKind, Participant, Process, Step, StepKind};
pub use run::{
    Recipe, RecipeCatalog, RecipeId, RunRecord, RunStatus, builtin_recipes, pipelines_json,
};
pub use span::{Position, Span};
pub use vocab::{Highlight, NoteKind, Verdict};

/// The JSON Schema for [`Document`], derived from the Rust types.
///
/// This is the single source of truth for the contract. TypeScript types are
/// generated from this later rather than maintained by hand.
pub fn schema() -> schemars::Schema {
    schemars::schema_for!(Document)
}

/// Pretty-printed JSON Schema.
pub fn schema_json() -> String {
    serde_json::to_string_pretty(&schema()).expect("schema is always serializable")
}
