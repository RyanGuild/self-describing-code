//! Structured process: the ordered interaction between participants.
//!
//! This is the single most important design decision in the IR. Diagrams are
//! **views over `Process`**, never stored as markup. That means a static
//! control-flow walk, a runtime trace, and (later) an agent's own action log
//! all produce the same structure and all render through the same code path.
//!
//! Storing Mermaid directly would make merging static and observed behaviour
//! impossible, and would let a producer emit a diagram that no longer matches
//! anything the graph knows about.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::kind::Kind;
use crate::span::Span;

/// One lifeline in a sequence diagram.
///
/// Today a participant is a module, scope, or type. The field set is
/// deliberately actor-neutral so that `agent.*` participants can be added
/// later without touching the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Participant {
    /// Local identifier, referenced by [`Step::from`] and [`Step::to`].
    pub id: String,
    /// Human-facing label, e.g. `io::load` or `planner`.
    pub label: String,
    pub kind: Kind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StepKind {
    Call,
    Return,
    Signal,
    Decision,
    Note,
}

/// A control-flow frame enclosing a step.
///
/// Frames form a stack per step. The renderer diffs consecutive stacks and
/// emits `alt` / `else` / `opt` / `loop` / `end` accordingly, which keeps the
/// IR flat and diff-friendly while still expressing nesting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Block {
    pub kind: BlockKind,
    pub label: String,
    /// For `alt`: which branch this step belongs to. Rendered as `else`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BlockKind {
    Alt,
    Opt,
    Loop,
    Par,
    Break,
}

/// A single ordered interaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Step {
    pub from: String,
    pub to: String,
    pub label: String,
    pub kind: StepKind,
    /// `true` when a runtime trace observed this step.
    ///
    /// `false` means "possible per static analysis". The renderer draws
    /// observed steps solid and possible steps dashed, which is how the
    /// static/runtime merge is expressed visually.
    pub observed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,
}

/// The process a node performs, or the process a trace recorded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Process {
    pub participants: Vec<Participant>,
    pub steps: Vec<Step>,
}

impl Process {
    pub fn new() -> Self {
        Self {
            participants: Vec::new(),
            steps: Vec::new(),
        }
    }

    /// Add a participant, returning its index. Duplicate ids are ignored.
    pub fn participant(&mut self, id: impl Into<String>, label: impl Into<String>, kind: Kind) -> usize {
        let id = id.into();
        if let Some(idx) = self.participants.iter().position(|p| p.id == id) {
            return idx;
        }
        self.participants.push(Participant {
            id,
            label: label.into(),
            kind,
        });
        self.participants.len() - 1
    }

    pub fn step(&mut self, step: Step) {
        self.steps.push(step);
    }

    /// Merge an observed process into this (static) one.
    ///
    /// Matching is by `(from, to, label)`. Steps seen at runtime are marked
    /// observed; steps known statically but never seen stay dashed. Observed
    /// steps not present statically are appended, because runtime is the
    /// ground truth for what actually happened.
    pub fn absorb(&mut self, observed: &Process) {
        for p in &observed.participants {
            if !self.participants.iter().any(|q| q.id == p.id) {
                self.participants.push(p.clone());
            }
        }

        let mut matched = vec![false; observed.steps.len()];
        for step in &mut self.steps {
            if let Some(idx) = observed.steps.iter().position(|o| {
                o.from == step.from && o.to == step.to && o.label == step.label
            }) {
                step.observed = true;
                matched[idx] = true;
            }
        }
        for (idx, step) in observed.steps.iter().enumerate() {
            if !matched[idx] {
                self.steps.push(step.clone());
            }
        }
    }
}

impl Default for Process {
    fn default() -> Self {
        Self::new()
    }
}
