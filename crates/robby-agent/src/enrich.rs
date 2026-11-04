//! Two-stage comment enrichment into `agent.decision` nodes.

use std::collections::BTreeMap;

use anyhow::Result;
use robby_ir::{Document, Lang, Node, NodeId, Provenance, Source, kind};

use crate::context::{CommentContext, collect_comment_contexts};
use crate::model::{
    ChatClient, DEFAULT_DECISION_MODEL, DEFAULT_NOTE_MODEL, DecisionModel, NoteModel, Verdict,
};

#[derive(Debug, Clone)]
pub struct EnrichOpts {
    pub decision_model: String,
    pub note_model: String,
    /// Cap how many comments to process this run (`None` = all).
    pub limit: Option<usize>,
}

impl Default for EnrichOpts {
    fn default() -> Self {
        Self {
            decision_model: std::env::var("ROBBY_DECISION_MODEL")
                .unwrap_or_else(|_| DEFAULT_DECISION_MODEL.into()),
            note_model: std::env::var("ROBBY_NOTE_MODEL")
                .unwrap_or_else(|_| DEFAULT_NOTE_MODEL.into()),
            limit: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnrichStats {
    pub considered: usize,
    pub decided: usize,
    pub relevant: usize,
    pub noise: usize,
    pub noted: usize,
    pub skipped_existing: usize,
}

/// Run two-stage enrich for comments lacking an `agent.decision` child.
pub fn enrich_comments(
    doc: &mut Document,
    sources: &BTreeMap<String, String>,
    client: &dyn ChatClient,
    opts: &EnrichOpts,
) -> Result<EnrichStats> {
    let mut contexts = collect_comment_contexts(doc, sources);
    let total = contexts.len();
    if let Some(limit) = opts.limit {
        contexts.truncate(limit);
    }

    let decision = DecisionModel::new(client, &opts.decision_model);
    let notes = NoteModel::new(client, &opts.note_model);

    let mut stats = EnrichStats {
        considered: contexts.len(),
        skipped_existing: total.saturating_sub(contexts.len()),
        ..Default::default()
    };

    // Collect contexts first so we don't hold borrows while mutating.
    for ctx in contexts {
        let d = decision.decide(&ctx)?;
        stats.decided += 1;
        match d.verdict {
            Verdict::Noise => {
                stats.noise += 1;
                insert_decision(doc, &ctx, &d, None, &opts.decision_model, None);
            }
            Verdict::Relevant => {
                stats.relevant += 1;
                let note = notes.write_note(&ctx)?;
                stats.noted += 1;
                insert_decision(
                    doc,
                    &ctx,
                    &d,
                    Some(&note),
                    &opts.decision_model,
                    Some(&opts.note_model),
                );
            }
        }
    }

    Ok(stats)
}

fn insert_decision(
    doc: &mut Document,
    ctx: &CommentContext,
    decision: &crate::model::Decision,
    note: Option<&crate::model::Note>,
    decision_model: &str,
    note_model: Option<&str>,
) {
    let Some(parent) = doc.get(&ctx.parent_id) else {
        return;
    };
    let fqn = format!("{}/decision", parent.fqn);
    let lang = match parent.id.as_str().split("://").next() {
        Some("typescript") => "typescript",
        Some("rust") => "rust",
        other => other.unwrap_or("agent"),
    };
    let parent_lang = parent.lang;
    let parent_id = parent.id.clone();

    let mut node = Node::with_lang(
        NodeId::symbol(lang, &fqn),
        kind::agent::DECISION,
        fqn,
        match parent_lang {
            Lang::TypeScript => Lang::TypeScript,
            Lang::Rust => Lang::Rust,
            Lang::Unknown => Lang::Unknown,
        },
    );
    node.parent = Some(parent_id);
    node.provenance = Provenance::new(Source::Agent);
    node.annotations
        .insert("verdict".into(), decision.verdict.as_str().into());
    if !decision.reason.is_empty() {
        node.annotations
            .insert("reason".into(), decision.reason.clone());
    }
    node.annotations
        .insert("decision_model".into(), decision_model.into());

    if let Some(note) = note {
        node.docs = Some(note.body.clone());
        node.annotations
            .insert("note_kind".into(), note.kind.as_str().into());
        if let Some(model) = note_model {
            node.annotations.insert("note_model".into(), model.into());
        }
        if let Ok(json) = serde_json::to_string(&note.highlights) {
            node.annotations.insert("highlights".into(), json);
        }
    }

    doc.merge(node);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ChatClient, ChatMessage};
    use robby_ir::{Position, Span};
    use std::sync::Mutex;

    struct FakeClient {
        responses: Mutex<Vec<String>>,
    }

    impl FakeClient {
        fn new(responses: Vec<String>) -> Self {
            Self {
                responses: Mutex::new(responses),
            }
        }
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

    fn mini_doc() -> (Document, BTreeMap<String, String>) {
        let mut doc = Document::new("test");
        let module = Node::with_lang(
            NodeId::symbol("typescript", "demo/shapes"),
            kind::code::MODULE,
            "demo/shapes",
            Lang::TypeScript,
        );
        doc.insert(module);

        let mut comment = Node::with_lang(
            NodeId::symbol("typescript", "demo/shapes/comment@0"),
            kind::code::COMMENT,
            "demo/shapes/comment@0",
            Lang::TypeScript,
        );
        comment.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
        comment.docs = Some("Draws a point.".into());
        comment
            .annotations
            .insert("comment_kind".into(), "block".into());
        comment.span = Some(Span {
            file: "shapes.ts".into(),
            start: Position::new(1, 1, 0),
            end: Position::new(1, 20, 19),
        });
        doc.insert(comment);

        let mut iface = Node::with_lang(
            NodeId::symbol("typescript", "demo/shapes/Drawable"),
            kind::code::INTERFACE,
            "demo/shapes/Drawable",
            Lang::TypeScript,
        );
        iface.parent = Some(NodeId::symbol("typescript", "demo/shapes"));
        iface.span = Some(Span {
            file: "shapes.ts".into(),
            start: Position::new(2, 1, 20),
            end: Position::new(4, 2, 60),
        });
        doc.insert(iface);

        let mut sources = BTreeMap::new();
        sources.insert(
            "shapes.ts".into(),
            "/** Draws a point. */\nexport interface Drawable {\n  draw(): void;\n}\n".into(),
        );
        (doc, sources)
    }

    #[test]
    fn noise_short_circuits_without_note_call() {
        let (mut doc, sources) = mini_doc();
        let client = FakeClient::new(vec![r#"{"verdict":"noise","reason":"filler"}"#.into()]);
        let stats = enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default()).unwrap();
        assert_eq!(stats.noise, 1);
        assert_eq!(stats.noted, 0);
        let decision = doc
            .nodes
            .values()
            .find(|n| n.kind.as_str() == kind::agent::DECISION)
            .expect("decision");
        assert_eq!(
            decision
                .annotations
                .get("verdict")
                .map(std::string::String::as_str),
            Some("noise")
        );
        assert!(decision.docs.is_none());
    }

    #[test]
    fn relevant_writes_note_and_highlights() {
        let (mut doc, sources) = mini_doc();
        let client = FakeClient::new(vec![
            r#"{"verdict":"relevant","reason":"api"}"#.into(),
            r#"{"kind":"usage_example","body":"Implement draw().","highlights":[{"start":0,"end":6}]}"#.into(),
        ]);
        let stats = enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default()).unwrap();
        assert_eq!(stats.relevant, 1);
        assert_eq!(stats.noted, 1);
        let decision = doc
            .nodes
            .values()
            .find(|n| n.kind.as_str() == kind::agent::DECISION)
            .expect("decision");
        assert_eq!(decision.docs.as_deref(), Some("Implement draw()."));
        assert_eq!(
            decision
                .annotations
                .get("note_kind")
                .map(std::string::String::as_str),
            Some("usage_example")
        );
        assert!(
            decision
                .annotations
                .get("highlights")
                .is_some_and(|h| h.contains("\"start\":0"))
        );
    }

    #[test]
    fn idempotent_skip_when_decision_exists() {
        let (mut doc, sources) = mini_doc();
        let client = FakeClient::new(vec![r#"{"verdict":"noise","reason":"x"}"#.into()]);
        enrich_comments(&mut doc, &sources, &client, &EnrichOpts::default()).unwrap();
        let client2 = FakeClient::new(vec![]);
        let stats = enrich_comments(&mut doc, &sources, &client2, &EnrichOpts::default()).unwrap();
        assert_eq!(stats.considered, 0);
        assert_eq!(stats.decided, 0);
    }
}
