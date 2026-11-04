# `robby_render::web`

[Self-model](../../README.md) · [robby_render](../index.md)

| | |
|---|---|
| kind | `code.module` |
| declared at | [`crates/robby-render/src/lib.rs:18`](../../../../crates/robby-render/src/lib.rs#L18) |
| provenance | extractor · confidence 1.00 |
| nodes | 50 |

## Types

```mermaid
classDiagram
    class FrameDetail {
        +stable_id
        +id
        +fqn
        +kind
        +signature
        +docs
        +location
        +provenance
        +confidence
        +process
        +process_mermaid
        +execution_mermaid
        +source
        +for_workspace()
        +from_node()
        +from_node_in_doc()
        +to_json()
    }
    class ProcessDetail {
        +participants
        +events
        +step_count
        +observed_count
    }
    class ParticipantDetail {
        +id
        +label
    }
    class EventDetail {
        <<enumeration>>
        End
        Else
        Begin
        Interaction
    }
    class SourceDetail {
        +file
        +text
    }
```

## Package diagram

```mermaid
flowchart TB
    n_458e2543a821af84["web"]
    n_356bf2c23da68961(["robby_ir"])
    n_9c636b098ca4450e(["robby_render"])
    n_458e2543a821af84 --> n_356bf2c23da68961
    n_458e2543a821af84 --> n_9c636b098ca4450e
```

## Module coupling

```mermaid
flowchart TB
    subgraph n_458e2543a821af84["web"]
        direction TB
        n_12e10a94bd0b89a4["event_detail"]
        n_1cef7cd777a4897f["for_workspace"]
        n_87904cdb73eb4bef["from_node"]
        n_213e965c602f499e["from_node_in_doc"]
        n_156e84213d588a5f["provenance_name"]
        n_01028a6d08164775["step_kind_name"]
        n_4d3c3f485a9610d6["to_json"]
        n_035881844fb12b4b["write_site"]
    end
    n_035881844fb12b4b --> n_1cef7cd777a4897f
    n_035881844fb12b4b --> n_213e965c602f499e
    n_035881844fb12b4b -->|"4"| n_4d3c3f485a9610d6
    n_12e10a94bd0b89a4 --> n_01028a6d08164775
    n_213e965c602f499e --> n_156e84213d588a5f
    n_213e965c602f499e -.->|"1 circular"| n_87904cdb73eb4bef
    n_87904cdb73eb4bef -.->|"1 circular"| n_213e965c602f499e
```

## Dependency matrix

| | web | robby_ir | robby_render |
|---|---:|---:|---:|
| **web** | · | 5 | 17 |
| **robby_ir** | | · | |
| **robby_render** | | | · |

## Comments

- `crates/robby-render/src/web.rs:113:5` (docs, `robby_render::web::impl FrameDetail::from_node_in_doc`): "Like [`Self::from_node`], optionally using `doc` for path-level execution."
  ```
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
                  let detail = ProcessDetail {
                      participants: view
                          .participants
                          .into_iter()
                          .map(|p| ParticipantDetail {
                              id: p.id,
                              label: p.label,
                          })
                          .collect(),
                      events: view.events.into_iter().map(event_detail).collect(),
                      step_count: view.step_count,
                      observed_count: view.observed_count,
                  };
                  (Some(detail), Some(mermaid))
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
  
          let source = SourceSlice::from_node(node, sources).map(|s| SourceDetail {
              file: s.file,
              text: s.text,
          });
  
          Self {
              stable_id: stable_id(node.id.as_str()),
              id: node.id.as_str().to_owned(),
              fqn: node.fqn.clone(),
              kind: node.kind.as_str().to_owned(),
              signature: node.signature.clone(),
              docs: node.docs.clone(),
              location: node.span.as_ref().map(|s| s.location()),
              provenance: provenance_name(node.provenance.source),
              confidence: node.provenance.confidence,
              process,
              process_mermaid,
              execution_mermaid,
              source,
          }
      }
  ```
- `crates/robby-render/src/web.rs:224:1` (docs, `robby_render::web::write_site`): "Write scene + frame JSON under `out` (typically `apps/site/public`). Returns the number of frame payload files written.…"
  ```
  /// Write scene + frame JSON under `out` (typically `apps/site/public`).
  ///
  /// Returns the number of frame payload files written. The TypeScript viewer in
  /// `apps/site/src` loads these files; it is not copied here so the self-model
  /// keeps a single source of truth.
  pub fn write_site(
      doc: &Document,
      sources: &BTreeMap<String, String>,
      out: &Path,
  ) -> Result<usize> {
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
  
      Ok(count)
  }
  ```
- `crates/robby-render/src/web.rs:22:1` (docs, `robby_render::web::FrameDetail`): "Lazy detail for one frame (fetched after the camera settles)."
  ```
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
      pub process: Option<ProcessDetail>,
      /// Mermaid `sequenceDiagram` body for the process (same path as Markdown).
      #[serde(skip_serializing_if = "Option::is_none")]
      pub process_mermaid: Option<String>,
      /// Path-level execution sequence (Calls + owning types for methods).
      #[serde(skip_serializing_if = "Option::is_none")]
      pub execution_mermaid: Option<String>,
      #[serde(skip_serializing_if = "Option::is_none")]
      pub source: Option<SourceDetail>,
  }
  ```
## References

- calls → [`robby_ir::kind`](../../robby_ir/kind/index.md) — from
- calls → [`robby_ir::model`](../../robby_ir/model/index.md) — get, insert
- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — location
- calls → [`robby_render`](../index.md) — stable_id
- calls → [`robby_render::pipeline`](../pipeline/index.md) — from_document
- calls → [`robby_render::report`](../report/index.md) — new
- calls → [`robby_render::test_render`](../test_render/index.md) — process
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str
- calls → [`robby_render::view::execution`](../view/execution/index.md) — execution_process
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty
- calls → [`robby_render::view::process`](../view/process/index.md) — from_process
- calls → [`robby_render::view::scene`](../view/scene/index.md) — from_entrypoints
- calls → [`robby_render::view::select`](../view/select/index.md) — collect

## Referenced by

- calls ← [`robby_cli::web`](../../robby_cli/web.md)
- calls ← [`robby_render::test_web::entrypoints_scene_json_is_deterministic`](../test_web/entrypoints-scene-json-is-deterministic.md)
- calls ← [`robby_render::test_web::frame_detail_includes_execution_mermaid_for_call_path`](../test_web/frame-detail-includes-execution-mermaid-for-call-path.md)
- calls ← [`robby_render::test_web::leaf_payload_includes_source_and_process`](../test_web/leaf-payload-includes-source-and-process.md)
- calls ← [`robby_render::test_web::scene_json_is_byte_identical_across_runs`](../test_web/scene-json-is-byte-identical-across-runs.md)
- calls ← [`robby_render::test_web::write_site_emits_scene_and_frames`](../test_web/write-site-emits-scene-and-frames.md)
- calls ← [`robby_render::tui::tree`](../tui/tree.md)

## Items

| Item | Kind | Steps | Location |
|---|---|---:|---|
| [`FrameDetail`](framedetail.md) | `code.struct` | — | `crates/robby-render/src/web.rs:22:1` |
| [`ProcessDetail`](processdetail.md) | `code.struct` | — | `crates/robby-render/src/web.rs:49:1` |
| [`ParticipantDetail`](participantdetail.md) | `code.struct` | — | `crates/robby-render/src/web.rs:57:1` |
| [`EventDetail`](eventdetail.md) | `code.enum` | — | `crates/robby-render/src/web.rs:63:1` |
| [`SourceDetail`](sourcedetail.md) | `code.struct` | — | `crates/robby-render/src/web.rs:84:1` |
| [`impl FrameDetail`](impl-framedetail.md) | `code.impl` | — | `crates/robby-render/src/web.rs:90:1` |
| [`event_detail`](event-detail.md) | `code.function` | 3 | `crates/robby-render/src/web.rs:178:1` |
| [`step_kind_name`](step-kind-name.md) | `code.function` | 0 | `crates/robby-render/src/web.rs:204:1` |
| [`provenance_name`](provenance-name.md) | `code.function` | 0 | `crates/robby-render/src/web.rs:214:1` |
| [`write_site`](write-site.md) | `code.function` | 38 | `crates/robby-render/src/web.rs:224:1` |

## Imports

- `use std::collections::{BTreeMap, BTreeSet};`
- `use std::fs;`
- `use std::path::Path;`
- `use anyhow::Result;`
- `use serde::Serialize;`
- [`use robby_ir::{Document, Node, Source, StepKind};`](../../robby_ir/index.md)
- `use crate::render::MermaidRenderer;`
- `use crate::render::Renderer;`
- `use crate::util::stable_id;`
- `use crate::view::{`

