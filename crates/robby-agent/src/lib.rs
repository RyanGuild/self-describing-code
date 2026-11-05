//! OpenRouter harness for enriching Robby IR with agent decisions.
//!
//! Offline only: call [`enrich_comments`] from the CLI, then render from the
//! cached document. Renderers never hit the network.
//!
//! Also hosts the headless [`review_doc`] producer (`robby.review_doc`) and the
//! two-model [`improve`] / [`integrate`] patch loop (`robby.improve_patch` /
//! `robby.integrate_patch`).

mod context;
mod edit_paths;
mod enrich;
mod improve;
mod integrate;
mod model;
mod openrouter;
mod review_doc;

pub use context::{CommentContext, collect_comment_contexts};
pub use edit_paths::{
    filter_forbidden_files, is_forbidden_edit_path, is_generated_or_markdown_subject,
    is_preferred_source_path, normalize_repo_path, patch_path_rules_prompt,
};
pub use enrich::{EnrichOpts, EnrichStats, enrich_comments};
pub use improve::{
    ImproveContext, ImproveModel, PatchFile, ProposedPatch, default_improve_model,
    default_integrate_model, ensure_distinct_models, stub_propose,
};
pub use integrate::{IntegrateDecision, IntegrateModel, IntegrateVerdict, stub_integrate};
pub use model::{
    ChatClient, ChatMessage, DEFAULT_DECISION_MODEL, DEFAULT_IMPROVE_MODEL,
    DEFAULT_INTEGRATE_MODEL, DEFAULT_NOTE_MODEL, Decision, DecisionModel, Highlight, Note,
    NoteKind, NoteModel, Role, Verdict,
};
pub use openrouter::OpenRouterClient;
pub use review_doc::{
    METRIC_DOC_COVERAGE_COMPLETE, METRIC_EDGE_RESOLVE_RATE, ReviewDocOpts, ReviewDocResult,
    ReviewDocStats, ReviewFinding, review_artifact_product, review_artifact_relpath, review_doc,
};
