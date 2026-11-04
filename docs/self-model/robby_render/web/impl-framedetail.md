# `robby_render::web::impl FrameDetail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.impl` |
| location | [`crates/robby-render/src/web.rs:48`](../../../../crates/robby-render/src/web.rs#L48) |
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
```

## References

- calls → [`robby_render`](../index.md) — stable_id
- calls → [`robby_render::test_render`](../test_render/index.md) — process
- calls → [`robby_render::view::comments`](../view/comments/index.md) — as_str
- calls → [`robby_render::view::execution`](../view/execution/index.md) — execution_process
- calls → [`robby_render::view::links`](../view/links/index.md) — is_empty
- calls → [`robby_render::view::process`](../view/process/index.md) — from_process

## Referenced by

- calls ← [`robby_render::test_web::entrypoints_scene_json_is_deterministic`](../test_web/entrypoints-scene-json-is-deterministic.md)
- calls ← [`robby_render::test_web::frame_detail_includes_execution_mermaid_for_call_path`](../test_web/frame-detail-includes-execution-mermaid-for-call-path.md)
- calls ← [`robby_render::test_web::leaf_payload_includes_source_and_process`](../test_web/leaf-payload-includes-source-and-process.md)
- calls ← [`robby_render::test_web::scene_json_is_byte_identical_across_runs`](../test_web/scene-json-is-byte-identical-across-runs.md)
- calls ← [`robby_render::tui::tree`](../tui/tree.md)
- calls ← [`robby_render::web::write_site`](write-site.md)

### `for_workspace`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn for_workspace(0)` |
| location | [`crates/robby-render/src/web.rs:49`](../../../../crates/robby-render/src/web.rs#L49) |
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
            provenance: Source::Extractor.as_str(),
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
    participant as_str as as_str
    for_workspace-->>stable_id: stable_id(WORKSPACE_ID)
    for_workspace-->>to_owned: WORKSPACE_ID.to_owned()
    for_workspace-->>to_owned: 'workspace'.to_owned()
    for_workspace-->>to_owned: 'code.workspace'.to_owned()
    for_workspace-->>to_owned: 'Synthetic root: crates and packages of this map.'.to_owned()
    for_workspace-->>Some: Some('Synthetic root: crates and packages of this map.'.to_owned())
    for_workspace-->>as_str: Source::Extractor.as_str()
```

7 steps · 0 observed · 7 possible only

**Edges**

- calls → [`robby_render::stable_id`](../stable-id.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)

### `from_node`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn from_node(2)` |
| location | [`crates/robby-render/src/web.rs:67`](../../../../crates/robby-render/src/web.rs#L67) |
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
| location | [`crates/robby-render/src/web.rs:71`](../../../../crates/robby-render/src/web.rs#L71) |
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
```

```mermaid
sequenceDiagram
    autonumber
    participant from_node_in_doc as from_node_in_doc()
    participant as_ref as as_ref
    participant ProcessView__from_process as ProcessView::from_process
    participant process as process
    participant Some as Some
    participant execution_process as execution_process
    participant is_empty as is_empty
    participant and_then as and_then
    participant SourceSlice__from_node as SourceSlice::from_node
    participant as_str as as_str
    participant stable_id as stable_id
    participant to_owned as to_owned
    participant clone as clone
    participant map as map
    from_node_in_doc-->>as_ref: node.process.as_ref()
    alt match node.process.as_ref()
    from_node_in_doc-->>ProcessView__from_process: ProcessView::from_process(p)
    from_node_in_doc-->>process: MermaidRenderer.process(&view)
    from_node_in_doc-->>Some: Some(view)
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
    from_node_in_doc-->>map: node.span.as_ref().map(robby_ir::Span::location)
    from_node_in_doc-->>as_str: node.provenance.source.as_str()
```

26 steps · 0 observed · 26 possible only

**Edges**

- calls → [`robby_render::view::process::impl ProcessView::from_process`](../view/process/index.md)
- calls → [`robby_render::test_render::process`](../test_render/process.md)
- calls → [`robby_render::view::execution::execution_process`](../view/execution/execution-process.md)
- calls → [`robby_render::view::links::impl LinkGraph::is_empty`](../view/links/index.md)
- calls → [`robby_render::view::process::impl ProcessView::from_process`](../view/process/index.md)
- calls → [`robby_render::test_render::process`](../test_render/process.md)
- calls → [`robby_render::web::impl FrameDetail::from_node`](index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::stable_id`](../stable-id.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)
- calls → [`robby_render::view::comments::impl CommentKind::as_str`](../view/comments/index.md)

### `to_json`

| | |
|---|---|
| kind | `code.method` |
| signature | `fn to_json(1)` |
| location | [`crates/robby-render/src/web.rs:115`](../../../../crates/robby-render/src/web.rs#L115) |
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

