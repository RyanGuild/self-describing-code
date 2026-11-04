//! OpenRouter harness for enriching Robby IR with agent decisions.
//!
//! Offline only: call [`enrich_comments`] from the CLI, then render from the
//! cached document. Renderers never hit the network.

mod context;
mod enrich;
mod model;
mod openrouter;

pub use context::{CommentContext, collect_comment_contexts};
pub use enrich::{EnrichOpts, EnrichStats, enrich_comments};
pub use model::{
    ChatClient, ChatMessage, DEFAULT_DECISION_MODEL, DEFAULT_NOTE_MODEL, Decision, DecisionModel,
    Highlight, Note, NoteKind, NoteModel, Role, Verdict,
};
pub use openrouter::OpenRouterClient;
