//! `robby.integrate_patch` — second model reviews/integrates an improve proposal.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::improve::ProposedPatch;
use crate::model::{ChatClient, ChatMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrateVerdict {
    Accept,
    Reject,
    Revise,
}

impl IntegrateVerdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Revise => "revise",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "accept" => Some(Self::Accept),
            "reject" => Some(Self::Reject),
            "revise" => Some(Self::Revise),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrateDecision {
    pub verdict: IntegrateVerdict,
    #[serde(default)]
    pub reason: String,
    /// When `revise`, optional replacement unified diff text.
    #[serde(default)]
    pub unified_diff: Option<String>,
    #[serde(default)]
    pub model: String,
}

impl IntegrateDecision {
    /// Diff text to persist for accept/revise; `None` on reject.
    pub fn integrated_diff(&self, proposal: &ProposedPatch) -> Option<String> {
        match self.verdict {
            IntegrateVerdict::Reject => None,
            IntegrateVerdict::Accept => Some(proposal.unified_diff_text()),
            IntegrateVerdict::Revise => Some(
                self.unified_diff
                    .clone()
                    .filter(|d| !d.trim().is_empty())
                    .unwrap_or_else(|| proposal.unified_diff_text()),
            ),
        }
    }
}

pub struct IntegrateModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}

impl<'a, C: ChatClient + ?Sized> IntegrateModel<'a, C> {
    pub fn new(client: &'a C, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
        }
    }

    pub fn review(
        &self,
        proposal: &ProposedPatch,
        improve_model: &str,
        metrics: &[String],
    ) -> Result<IntegrateDecision> {
        let messages = [
            ChatMessage::system(
                "You are the integrate model for Robby. A different model already proposed a patch.\n\
                 Review it for safety and metric alignment. Reply with JSON only:\n\
                 {\"verdict\":\"accept\"|\"reject\"|\"revise\",\"reason\":\"...\",\"unified_diff\":null}\n\
                 - accept: write the proposal as-is.\n\
                 - reject: do not write a patch (explain why).\n\
                 - revise: supply an improved unified_diff string (source edits only).\n\
                 Never approve edits under .robby/, apps/site/public generated paths, or docs/self-model/.",
            ),
            ChatMessage::user(format!(
                "improve_model: {improve_model}\n\
                 integrate_model: {}\n\
                 metrics: {}\n\
                 summary: {}\n\
                 proposed_diff:\n{}",
                self.model,
                metrics.join(", "),
                proposal.summary,
                proposal.unified_diff_text()
            )),
        ];
        let raw = self.client.chat(&self.model, &messages, true)?;
        let mut decision = parse_integrate_decision(&raw)?;
        decision.model.clone_from(&self.model);
        Ok(decision)
    }
}

/// Offline integrate: accept non-empty stubs/proposals.
pub fn stub_integrate(proposal: &ProposedPatch) -> IntegrateDecision {
    if proposal.is_empty() {
        IntegrateDecision {
            verdict: IntegrateVerdict::Reject,
            reason: "empty proposal".into(),
            unified_diff: None,
            model: "stub".into(),
        }
    } else {
        IntegrateDecision {
            verdict: IntegrateVerdict::Accept,
            reason: "stub integrate accepts non-empty proposal (no OPENROUTER_API_KEY)".into(),
            unified_diff: None,
            model: "stub".into(),
        }
    }
}

fn parse_integrate_decision(raw: &str) -> Result<IntegrateDecision> {
    let value = extract_json(raw)?;
    let verdict = value
        .get("verdict")
        .and_then(|v| v.as_str())
        .and_then(IntegrateVerdict::parse)
        .with_context(|| format!("missing/invalid integrate verdict in {raw}"))?;
    let reason = value
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    let unified_diff = value
        .get("unified_diff")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .filter(|s| !s.trim().is_empty());
    Ok(IntegrateDecision {
        verdict,
        reason,
        unified_diff,
        model: String::new(),
    })
}

fn extract_json(raw: &str) -> Result<serde_json::Value> {
    let trimmed = raw.trim();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Ok(v);
    }
    let start = trimmed
        .find('{')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    let end = trimmed
        .rfind('}')
        .with_context(|| format!("no JSON object in response: {raw}"))?;
    serde_json::from_str(&trimmed[start..=end])
        .with_context(|| format!("invalid JSON in response: {raw}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::improve::{PatchFile, ProposedPatch};
    use crate::model::{ChatClient, ChatMessage};
    use std::sync::Mutex;

    struct FakeClient {
        responses: Mutex<Vec<String>>,
    }

    impl ChatClient for FakeClient {
        fn chat(
            &self,
            _model: &str,
            _messages: &[ChatMessage],
            _json_mode: bool,
        ) -> Result<String> {
            let mut q = self.responses.lock().unwrap();
            if q.is_empty() {
                anyhow::bail!("FakeClient exhausted");
            }
            Ok(q.remove(0))
        }
    }

    fn sample_proposal() -> ProposedPatch {
        ProposedPatch {
            summary: "tweak".into(),
            files: vec![PatchFile {
                path: "crates/robby-ir/src/lib.rs".into(),
                unified_diff: "--- a/crates/robby-ir/src/lib.rs\n+++ b/crates/robby-ir/src/lib.rs\n@@ -1 +1 @@\n-a\n+b\n"
                    .into(),
            }],
            metrics_targeted: vec!["edge.resolve_rate".into()],
            model: "openai/gpt-4o-mini".into(),
        }
    }

    #[test]
    fn accept_writes_proposal_diff() {
        let client = FakeClient {
            responses: Mutex::new(vec![
                r#"{"verdict":"accept","reason":"looks safe","unified_diff":null}"#.into(),
            ]),
        };
        let model = IntegrateModel::new(&client, "openai/gpt-4o");
        let proposal = sample_proposal();
        let decision = model
            .review(
                &proposal,
                "openai/gpt-4o-mini",
                &["edge.resolve_rate".into()],
            )
            .unwrap();
        assert_eq!(decision.verdict, IntegrateVerdict::Accept);
        assert_eq!(decision.model, "openai/gpt-4o");
        let diff = decision.integrated_diff(&proposal).unwrap();
        assert!(diff.contains("+++ b/crates/robby-ir/src/lib.rs"));
    }

    #[test]
    fn stub_accepts_nonempty() {
        let d = stub_integrate(&sample_proposal());
        assert_eq!(d.verdict, IntegrateVerdict::Accept);
    }
}
