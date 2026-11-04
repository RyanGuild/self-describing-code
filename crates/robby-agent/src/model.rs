//! Decision and note model roles over a shared chat client.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::context::CommentContext;

pub use robby_ir::{Highlight, NoteKind, Verdict};

pub const DEFAULT_DECISION_MODEL: &str = "openai/gpt-4o-mini";
pub const DEFAULT_NOTE_MODEL: &str = "openai/gpt-4o";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }
}

/// Pluggable chat backend (OpenRouter or fakes in tests).
pub trait ChatClient {
    fn chat(&self, model: &str, messages: &[ChatMessage], json_mode: bool) -> Result<String>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decision {
    pub verdict: Verdict,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub kind: NoteKind,
    pub body: String,
    #[serde(default)]
    pub highlights: Vec<Highlight>,
}

pub struct DecisionModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}

impl<'a, C: ChatClient + ?Sized> DecisionModel<'a, C> {
    pub fn new(client: &'a C, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
        }
    }

    pub fn decide(&self, ctx: &CommentContext) -> Result<Decision> {
        let messages = [
            ChatMessage::system(
                "You judge whether a source comment is useful for a codebase map.\n\
                 Reply with JSON only: {\"verdict\":\"relevant\"|\"noise\",\"reason\":\"...\"}.\n\
                 Mark as noise: TODOs, license headers, commented-out code, noisy separators.\n\
                 Mark as relevant: API intent, invariants, usage hints, non-obvious behavior.",
            ),
            ChatMessage::user(format_context(ctx)),
        ];
        let raw = self.client.chat(&self.model, &messages, true)?;
        parse_decision(&raw)
    }
}

pub struct NoteModel<'a, C: ChatClient + ?Sized> {
    pub client: &'a C,
    pub model: String,
}

impl<'a, C: ChatClient + ?Sized> NoteModel<'a, C> {
    pub fn new(client: &'a C, model: impl Into<String>) -> Self {
        Self {
            client,
            model: model.into(),
        }
    }

    pub fn write_note(&self, ctx: &CommentContext) -> Result<Note> {
        let code_len = ctx.code_window.len();
        let messages = [
            ChatMessage::system(format!(
                "You write a short note for a relevant source comment.\n\
                 Reply with JSON only:\n\
                 {{\"kind\":\"usage_example\"|\"implementation_note\",\"body\":\"...\",\
                 \"highlights\":[{{\"start\":0,\"end\":0}}]}}\n\
                 - usage_example: how a caller should use the code.\n\
                 - implementation_note: non-obvious implementation detail.\n\
                 highlights are UTF-8 byte offsets into the code window (length {code_len}).\n\
                 Prefer 0–3 tight highlights; empty array is ok."
            )),
            ChatMessage::user(format_context(ctx)),
        ];
        let raw = self.client.chat(&self.model, &messages, true)?;
        let mut note = parse_note(&raw)?;
        clamp_highlights(&mut note.highlights, code_len as u32);
        Ok(note)
    }
}

fn format_context(ctx: &CommentContext) -> String {
    let owner = ctx.owner_fqn.as_deref().unwrap_or("(none)");
    let location = ctx.location.as_deref().unwrap_or("(unknown)");
    format!(
        "file: {}\nlocation: {}\nowner: {}\n\ncomment:\n{}\n\ncode:\n{}",
        ctx.file, location, owner, ctx.comment_text, ctx.code_window
    )
}

fn parse_decision(raw: &str) -> Result<Decision> {
    let value = extract_json(raw)?;
    let verdict = value
        .get("verdict")
        .and_then(|v| v.as_str())
        .and_then(Verdict::parse)
        .with_context(|| format!("missing/invalid verdict in {raw}"))?;
    let reason = value
        .get("reason")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    Ok(Decision { verdict, reason })
}

fn parse_note(raw: &str) -> Result<Note> {
    let value = extract_json(raw)?;
    let kind = value
        .get("kind")
        .and_then(|v| v.as_str())
        .and_then(NoteKind::parse)
        .with_context(|| format!("missing/invalid note kind in {raw}"))?;
    let body = value
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_owned();
    if body.is_empty() {
        bail!("empty note body in {raw}");
    }
    let highlights = value
        .get("highlights")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|h| {
                    let start = h.get("start")?.as_u64()? as u32;
                    let end = h.get("end")?.as_u64()? as u32;
                    Some(Highlight { start, end })
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(Note {
        kind,
        body,
        highlights,
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

fn clamp_highlights(highlights: &mut Vec<Highlight>, len: u32) {
    highlights.retain_mut(|h| {
        if h.end > len {
            h.end = len;
        }
        if h.start > h.end {
            return false;
        }
        h.start < h.end
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_decision_and_note_json() {
        let d = parse_decision(r#"{"verdict":"relevant","reason":"api hint"}"#).unwrap();
        assert_eq!(d.verdict, Verdict::Relevant);
        let n = parse_note(
            r#"{"kind":"usage_example","body":"Call draw().","highlights":[{"start":0,"end":4}]}"#,
        )
        .unwrap();
        assert_eq!(n.kind, NoteKind::UsageExample);
        assert_eq!(n.highlights[0].end, 4);
    }
}
