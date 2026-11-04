# `robby_render::web::impl FrameDetail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/web.rs:90`](../../../../crates/robby-render/src/web.rs#L90) |
| provenance | extractor · confidence 1.00 |

```rust
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
            provenance: "extractor",
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

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("frame detail serializes")
    }
}
```

## References

- calls → [`robby_ir::span`](../../robby_ir/span/index.md) — location
- calls → [`robby_render`](../index.md) — stable_id
- calls → [`robby_render::test_render`](../test_render/index.md) — process
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str
- calls → [`robby_render::view::execution`](../view/execution/index.md) — execution_process
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty
- calls → [`robby_render::view::process`](../view/process/index.md) — from_process
- calls → [`robby_render::view::select`](../view/select/index.md) — collect
- calls → [`robby_render::web`](index.md) — provenance_name

## Referenced by

- calls ← [`robby_render::test_web::frame_detail_includes_execution_mermaid_for_call_path`](../test_web/frame-detail-includes-execution-mermaid-for-call-path.md)
- calls ← [`robby_render::test_web::leaf_payload_includes_source_and_process`](../test_web/leaf-payload-includes-source-and-process.md)
- calls ← [`robby_render::tui::tree`](../tui/tree.md)
- calls ← [`robby_render::web::write_site`](write-site.md)

### `for_workspace`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn for_workspace(0)` |
| location | [`crates/robby-render/src/web.rs:91`](../../../../crates/robby-render/src/web.rs#L91) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn for_workspace() -> Self {
        Self {
            stable_id: stable_id(WORKSPACE_ID),
            id: WORKSPACE_ID.to_owned(),
            fqn: "workspace".to_owned(),
            kind: "code.workspace".to_owned(),
            signature: None,
            docs: Some("Synthetic root: crates and packages of this map.".to_owned()),
            location: None,
            provenance: "extractor",
            confidence: 1.0,
            process: None,
            process_mermaid: None,
            execution_mermaid: None,
            source: None,
        }
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant for_workspace as for_workspace()
    participant stable_id as stable_id
    participant to_owned as to_owned
    participant Some as Some
    for_workspace-->>stable_id: stable_id(WORKSPACE_ID)
    for_workspace-->>to_owned: WORKSPACE_ID.to_owned()
    for_workspace-->>to_owned: 'workspace'.to_owned()
    for_workspace-->>to_owned: 'code.workspace'.to_owned()
    for_workspace-->>to_owned: 'Synthetic root: crates and packages of this map.'.to_owned()
    for_workspace-->>Some: Some('Synthetic root: crates and packages of this map.'.to_owned())
```

6 steps · 0 observed · 6 possible only

**Edges**

- calls → [`robby_render::stable_id`](../stable-id.md)

### `from_node`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_node(2)` |
| location | [`crates/robby-render/src/web.rs:109`](../../../../crates/robby-render/src/web.rs#L109) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn from_node(node: &Node, sources: &BTreeMap<String, String>) -> Self {
        Self::from_node_in_doc(node, sources, None)
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant from_node as from_node()
    participant Self__from_node_in_doc as Self::from_node_in_doc
    from_node-->>Self__from_node_in_doc: Self::from_node_in_doc(node, sources, None)
```

1 steps · 0 observed · 1 possible only

**Edges**

- calls → [`robby_render::web::impl FrameDetail::from_node_in_doc`](index.md)

### `from_node_in_doc`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_node_in_doc(3)` |
| location | [`crates/robby-render/src/web.rs:113`](../../../../crates/robby-render/src/web.rs#L113) |
| provenance | extractor · confidence 1.00 |

> Like [`Self::from_node`], optionally using `doc` for path-level execution.

```rust
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

```mermaid
sequenceDiagram
    autonumber
    participant from_node_in_doc as from_node_in_doc()
    participant as_ref as as_ref
    participant ProcessView__from_process as ProcessView::from_process
    participant process as process
    participant into_iter as into_iter
    participant map as map
    participant collect as collect
    participant Some as Some
    participant execution_process as execution_process
    participant is_empty as is_empty
    participant and_then as and_then
    participant SourceSlice__from_node as SourceSlice::from_node
    participant as_str as as_str
    participant stable_id as stable_id
    participant to_owned as to_owned
    participant clone as clone
    participant location as location
    participant provenance_name as provenance_name
    from_node_in_doc-->>as_ref: node.process.as_ref()
    alt match node.process.as_ref()
    from_node_in_doc-->>ProcessView__from_process: ProcessView::from_process(p)
    from_node_in_doc-->>process: MermaidRenderer.process(&view)
    from_node_in_doc-->>into_iter: view .participants .into_iter()
    from_node_in_doc-->>map: view .participants .into_iter() .map(|p| ParticipantDetail { id: p.id...
    from_node_in_doc-->>collect: view .participants .into_iter() .map(|p| ParticipantDetail { id: p.id...
    from_node_in_doc-->>into_iter: view.events.into_iter()
    from_node_in_doc-->>map: view.events.into_iter().map(event_detail)
    from_node_in_doc-->>collect: view.events.into_iter().map(event_detail).collect()
    from_node_in_doc-->>Some: Some(detail)
    from_node_in_doc-->>Some: Some(mermaid)
    end
    from_node_in_doc-->>execution_process: execution_process(d, &node.id)
    from_node_in_doc-->>from_node_in_doc: ? (propagate)
    from_node_in_doc-->>is_empty: exec.steps.is_empty()
    alt exec.steps.is_empty()
    from_node_in_doc-->>from_node_in_doc: return
    end
    from_node_in_doc-->>ProcessView__from_process: ProcessView::from_process(&exec)
    from_node_in_doc-->>process: MermaidRenderer.process(&view)
    from_node_in_doc-->>Some: Some(MermaidRenderer.process(&view))
    from_node_in_doc-->>and_then: doc.and_then(|d| { let exec = execution_process(d, &node.id)? // Pre...
    from_node_in_doc-->>SourceSlice__from_node: SourceSlice::from_node(node, sources)
    from_node_in_doc-->>map: SourceSlice::from_node(node, sources).map(|s| SourceDetail { file: s....
    from_node_in_doc-->>as_str: node.id.as_str()
    from_node_in_doc-->>stable_id: stable_id(node.id.as_str())
    from_node_in_doc-->>as_str: node.id.as_str()
    from_node_in_doc-->>to_owned: node.id.as_str().to_owned()
    from_node_in_doc-->>clone: node.fqn.clone()
    from_node_in_doc-->>as_str: node.kind.as_str()
    from_node_in_doc-->>to_owned: node.kind.as_str().to_owned()
    from_node_in_doc-->>clone: node.signature.clone()
    from_node_in_doc-->>clone: node.docs.clone()
    from_node_in_doc-->>as_ref: node.span.as_ref()
    from_node_in_doc-->>location: s.location()
    from_node_in_doc-->>map: node.span.as_ref().map(|s| s.location())
    from_node_in_doc-->>provenance_name: provenance_name(node.provenance.source)
```

34 steps · 0 observed · 34 possible only

**Edges**

- calls → [`robby_render::view::process::impl ProcessView::from_process`](../view/process/index.md)
- calls → [`robby_render::test_render::process`](../test_render/process.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::view::select::collect`](../view/select/collect.md)
- calls → [`robby_render::view::execution::execution_process`](../view/execution/execution-process.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::view::process::impl ProcessView::from_process`](../view/process/index.md)
- calls → [`robby_render::test_render::process`](../test_render/process.md)
- calls → [`robby_render::web::impl FrameDetail::from_node`](index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::stable_id`](../stable-id.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::view::comments::impl NoteKind::as_str`](../view/comments/index.md)
- calls → [`robby_ir::span::impl Span::location`](../../robby_ir/span/index.md)
- calls → [`robby_render::web::provenance_name`](provenance-name.md)

### `to_json`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn to_json(1)` |
| location | [`crates/robby-render/src/web.rs:173`](../../../../crates/robby-render/src/web.rs#L173) |
| provenance | extractor · confidence 1.00 |

```rust
pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("frame detail serializes")
    }
```

```mermaid
sequenceDiagram
    autonumber
    participant to_json as to_json()
    participant serde_json__to_string_pretty as serde_json::to_string_pretty
    participant expect as expect
    to_json-->>serde_json__to_string_pretty: serde_json::to_string_pretty(self)
    to_json-->>expect: serde_json::to_string_pretty(self).expect('frame detail serializes')
```

2 steps · 0 observed · 2 possible only

