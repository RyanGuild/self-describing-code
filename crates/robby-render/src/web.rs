//! Prezi-style site data: scene geometry + lazy frame payloads.
//!
//! The TypeScript viewer lives in `apps/site`. It reads this JSON structurally
//! and does not keep a second copy of the schema. `Scene` and [`FrameDetail`]
//! (whose `process` is [`crate::view::ProcessView`]) are the payload.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Serialize;

use robby_ir::{Document, Node, Source};

use crate::render::MermaidRenderer;
use crate::render::Renderer;
use crate::util::stable_id;
use crate::view::{ProcessView, Scene, SourceSlice, WORKSPACE_ID, execution_process};

/// Lazy detail for one frame (fetched after the camera settles).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FrameDetail {
    pub stable_id: String,
    pub id: String,
    pub fqn: String,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub provenance: &'static str,
    pub confidence: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<ProcessView>,
    /// Mermaid `sequenceDiagram` body for the process (same path as Markdown).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_mermaid: Option<String>,
    /// Path-level execution sequence (Calls + owning types for methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mermaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceSlice>,
}

impl FrameDetail {
    pub fn for_workspace() -> Self {
        Self {
            stable_id: stable_id(WORKSPACE_ID),
            id: WORKSPACE_ID.to_owned(),
            fqn: "workspace".to_owned(),
            kind: "code.workspace".to_owned(),
            signature: None,
            docs: Some("Synthetic root: crates and packages of this map.".to_owned()),
            location: None,
            provenance: Source::Extractor.as_str(),
            confidence: 1.0,
            process: None,
            process_mermaid: None,
            execution_mermaid: None,
            source: None,
        }
    }

    pub fn from_node(node: &Node, sources: &BTreeMap<String, String>) -> Self {
        Self::from_node_in_doc(node, sources, None)
    }

    /// Like [`Self::from_node`], optionally using `doc` for path-level execution.
    pub fn from_node_in_doc(
        node: &Node,
        sources: &BTreeMap<String, String>,
        doc: Option<&Document>,
    ) -> Self {
        let (process, process_mermaid) = match node.process.as_ref() {
            Some(p) => {
                let view = ProcessView::from_process(p);
                let mermaid = MermaidRenderer.process(&view);
                (Some(view), Some(mermaid))
            }
            None => (None, None),
        };

        let execution_mermaid = doc.and_then(|d| {
            let exec = execution_process(d, &node.id)?;
            // Prefer path diagram when it adds call hops beyond the local body.
            if exec.steps.is_empty() {
                return None;
            }
            let view = ProcessView::from_process(&exec);
            Some(MermaidRenderer.process(&view))
        });

        let source = SourceSlice::from_node(node, sources);

        Self {
            stable_id: stable_id(node.id.as_str()),
            id: node.id.as_str().to_owned(),
            fqn: node.fqn.clone(),
            kind: node.kind.as_str().to_owned(),
            signature: node.signature.clone(),
            docs: node.docs.clone(),
            location: node.span.as_ref().map(robby_ir::Span::location),
            provenance: node.provenance.source.as_str(),
            confidence: node.provenance.confidence,
            process,
            process_mermaid,
            execution_mermaid,
            source,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("frame detail serializes")
    }
}

/// Write scene + frame JSON under `out` (typically `apps/site/public`).
///
/// Returns the number of frame payload files written. The TypeScript viewer in
/// `apps/site/src` loads these files; it is not copied here so the self-model
/// keeps a single source of truth.
pub fn write_site(doc: &Document, sources: &BTreeMap<String, String>, out: &Path) -> Result<usize> {
    fs::create_dir_all(out)?;
    let frames_dir = out.join("frames");
    fs::create_dir_all(&frames_dir)?;

    let structure = Scene::from_document(doc);
    let entrypoints = Scene::from_entrypoints(doc);
    fs::write(out.join("scene.json"), structure.to_json())?;
    fs::write(out.join("scene-entrypoints.json"), entrypoints.to_json())?;

    let mut count = 0usize;

    let workspace = FrameDetail::for_workspace();
    fs::write(
        frames_dir.join(format!("{}.json", workspace.stable_id)),
        workspace.to_json(),
    )?;
    count += 1;

    let mut written: BTreeSet<String> = BTreeSet::new();
    written.insert(workspace.stable_id.clone());

    for scene in [&structure, &entrypoints] {
        for frame in scene.frames.values() {
            if frame.id == WORKSPACE_ID {
                continue;
            }
            if !written.insert(frame.stable_id.clone()) {
                continue;
            }
            let Some(node) = doc.get(&robby_ir::NodeId::from(frame.id.as_str())) else {
                continue;
            };
            let detail = FrameDetail::from_node_in_doc(node, sources, Some(doc));
            fs::write(
                frames_dir.join(format!("{}.json", detail.stable_id)),
                detail.to_json(),
            )?;
            count += 1;
        }
    }

    // Stamp for `robby web --watch` / viewer poll: changes when the site is rewritten.
    let rebuilt_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let build = serde_json::json!({
        "rebuilt_at": rebuilt_at,
        "frames": count,
        "nodes": doc.nodes.len(),
        "files": doc.files.len(),
        "generator": doc.generator,
    });
    fs::write(out.join("build.json"), serde_json::to_string_pretty(&build)?)?;

    // Completed pipeline history for Runs mode (synthetic extract + this web build).
    crate::runs::write_site_runs(doc, out, count, rebuilt_at)?;

    Ok(count)
}
