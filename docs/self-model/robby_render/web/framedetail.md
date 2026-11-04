# `robby_render::web::FrameDetail`

[Self-model](../../README.md) · [web](index.md)

| | |
|---|---|
| kind | `code.struct` |
| location | [`crates/robby-render/src/web.rs:22`](../../../../crates/robby-render/src/web.rs#L22) |
| provenance | extractor · confidence 1.00 |

> Lazy detail for one frame (fetched after the camera settles).

```rust
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

**Fields and variants**

- `stable_id` — `crates/robby-render/src/web.rs:25:5`
- `id` — `crates/robby-render/src/web.rs:26:5`
- `fqn` — `crates/robby-render/src/web.rs:27:5`
- `kind` — `crates/robby-render/src/web.rs:28:5`
- `signature` — `crates/robby-render/src/web.rs:29:5`
- `docs` — `crates/robby-render/src/web.rs:31:5`
- `location` — `crates/robby-render/src/web.rs:33:5`
- `provenance` — `crates/robby-render/src/web.rs:35:5`
- `confidence` — `crates/robby-render/src/web.rs:36:5`
- `process` — `crates/robby-render/src/web.rs:37:5`
- `process_mermaid` — `crates/robby-render/src/web.rs:39:5`
- `execution_mermaid` — `crates/robby-render/src/web.rs:42:5`
- `source` — `crates/robby-render/src/web.rs:45:5`

