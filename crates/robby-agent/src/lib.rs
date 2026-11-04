//! OpenRouter harness for enriching Robby IR with agent decisions.
//!
//! Offline only: call [`enrich_comments`] from the CLI, then render from the
//! cached document. Renderers never hit the network.
//!
//! Also hosts the headless [`review_doc`] producer (`robby.review_doc`).

mod context;
mod enrich;
mod model;
mod openrouter;
mod review_doc;

pub use context::{CommentContext, collect_comment_contexts};
pub use enrich::{EnrichOpts, EnrichStats, enrich_comments};
pub use model::{
    ChatClient, ChatMessage, DEFAULT_DECISION_MODEL, DEFAULT_NOTE_MODEL, Decision, DecisionModel,
    Highlight, Note, NoteKind, NoteModel, Role, Verdict,
};
pub use openrouter::OpenRouterClient;
pub use review_doc::{
    METRIC_DOC_COVERAGE_COMPLETE, METRIC_EDGE_RESOLVE_RATE, ReviewDocOpts, ReviewDocResult,
    ReviewDocStats, ReviewFinding, review_artifact_product, review_artifact_relpath, review_doc,
};
