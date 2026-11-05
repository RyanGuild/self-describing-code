//! Process interaction view: frame-diffed steps shared by Mermaid, TUI, and the site payload.

use serde::Serialize;

use robby_ir::{Block, Document, Process, StepKind};

use crate::util::stable_id;

/// Format-agnostic process diagram data.
///
/// This is also the JSON shape under `FrameDetail.process` / `execution`.
/// There is no second DTO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessView {
    pub participants: Vec<ParticipantView>,
    pub events: Vec<ProcessEvent>,
    pub step_count: usize,
    pub observed_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParticipantView {
    pub id: String,
    pub label: String,
    /// Stable frame id when this lifeline maps to a navigable node (execution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProcessEvent {
    End,
    Else {
        label: String,
    },
    Begin {
        keyword: &'static str,
        label: String,
    },
    Interaction {
        index: usize,
        from: String,
        to: String,
        label: String,
        observed: bool,
        kind: StepKind,
    },
}

impl ProcessView {
    pub fn from_process(process: &Process) -> Self {
        let participants = process
            .participants
            .iter()
            .map(|p| ParticipantView {
                id: p.id.clone(),
                label: p.label.clone(),
                target: None,
            })
            .collect();

        let observed_count = process.steps.iter().filter(|s| s.observed).count();
        let mut events = Vec::new();
        let mut stack: Vec<Block> = Vec::new();

        for (index, step) in process.steps.iter().enumerate() {
            let common = common_prefix(&stack, &step.blocks);

            for _ in common..stack.len() {
                events.push(ProcessEvent::End);
            }

            if common > 0
                && step.blocks.len() >= common
                && step.blocks[common - 1].branch != stack[common - 1].branch
            {
                let branch = step.blocks[common - 1].branch.clone().unwrap_or_default();
                events.push(ProcessEvent::Else { label: branch });
            }

            let mut opened = common;
            while opened < step.blocks.len() {
                let block = &step.blocks[opened];
                events.push(ProcessEvent::Begin {
                    keyword: block.kind.as_str(),
                    label: block.label.clone(),
                });
                opened += 1;
            }

            stack.clone_from(&step.blocks);

            events.push(ProcessEvent::Interaction {
                index: index + 1,
                from: step.from.clone(),
                to: step.to.clone(),
                label: step.label.clone(),
                observed: step.observed,
                kind: step.kind,
            });
        }

        for _ in 0..stack.len() {
            events.push(ProcessEvent::End);
        }

        Self {
            participants,
            events,
            step_count: process.steps.len(),
            observed_count,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.step_count == 0
    }

    /// Attach Prezi frame targets for participants whose label matches a node FQN.
    pub fn with_targets_from_doc(mut self, doc: &Document) -> Self {
        for participant in &mut self.participants {
            if participant.target.is_some() {
                continue;
            }
            if let Some(node) = doc.nodes.values().find(|n| n.fqn == participant.label) {
                participant.target = Some(stable_id(node.id.as_str()));
            }
        }
        self
    }
}

fn common_prefix(a: &[Block], b: &[Block]) -> usize {
    let mut i = 0;
    while i < a.len() && i < b.len() && a[i].kind == b[i].kind && a[i].label == b[i].label {
        i += 1;
    }
    i
}
