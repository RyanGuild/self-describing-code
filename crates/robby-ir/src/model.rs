//! The graph document: nodes, edges, provenance, diagnostics.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::kind::Kind;
use crate::process::Process;
use crate::span::Span;

/// Bumped only on breaking changes. Additive fields do not bump it.
pub const SCHEMA_VERSION: u32 = 1;

/// A stable, content-addressed node identifier.
///
/// Symbols use `lang://fqn`, e.g. `rust://robby_ir::model::Node`. Statements
/// use `parent#ordinal`. Ordinal-based ids are stable within a run but shift
/// when earlier siblings change; that is documented and accepted, because
/// byte-offset ids would shift on every edit above them instead.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn symbol(lang: &str, fqn: &str) -> Self {
        Self(format!("{lang}://{fqn}"))
    }

    pub fn child(parent: &NodeId, ordinal: usize) -> Self {
        Self(format!("{}#{}", parent.0, ordinal))
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Which producer contributed a fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    /// Parsed from source without annotations.
    Extractor,
    /// Contributed by an explicit macro annotation.
    Macro,
    /// Recorded while the code ran.
    Trace,
    /// Emitted by an agent describing its own process.
    Agent,
    /// Authored by hand (fixtures, bootstrap data).
    Hand,
}

impl Source {
    /// Lower numbers win when two producers disagree about the same fact.
    ///
    /// Runtime beats static because observation is ground truth; explicit
    /// annotation beats parsing because it carries intent.
    pub fn precedence(self) -> u8 {
        match self {
            Source::Trace => 0,
            Source::Agent => 1,
            Source::Macro => 2,
            Source::Extractor => 3,
            Source::Hand => 4,
        }
    }
}

/// Where a fact came from and how much to trust it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Provenance {
    pub source: Source,
    /// Unix milliseconds. `None` means the fact is timeless (static analysis).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<i64>,
    /// `0.0..=1.0`. Static resolution that could not disambiguate a path
    /// records its uncertainty here rather than dropping the edge.
    pub confidence: f32,
}

impl Provenance {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            observed_at: None,
            confidence: 1.0,
        }
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

impl Default for Provenance {
    fn default() -> Self {
        Self::new(Source::Hand)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Lang {
    Rust,
    TypeScript,
    Unknown,
}

/// A directed relationship between nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Edge {
    pub to: NodeId,
    pub kind: EdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "is_one")]
    pub confidence: f32,
    /// `false` when the target could not be resolved to a node in this graph.
    ///
    /// Unresolved edges are kept, never dropped. Losing edges silently is how
    /// a map stops being trustworthy.
    #[serde(default = "default_true")]
    pub resolved: bool,
}

fn is_one(v: &f32) -> bool {
    (*v - 1.0).abs() < f32::EPSILON
}

fn default_true() -> bool {
    true
}

impl Edge {
    pub fn new(to: impl Into<NodeId>, kind: EdgeKind) -> Self {
        Self {
            to: to.into(),
            kind,
            span: None,
            confidence: 1.0,
            resolved: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Contains,
    Calls,
    Uses,
    Imports,
    Implements,
    Extends,
    Returns,
    Field,
    References,
}

/// A single described thing: a crate, a function, a statement, an observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Node {
    pub id: NodeId,
    pub kind: Kind,
    pub name: String,
    /// Fully-qualified, language-native path where one exists.
    pub fqn: String,
    pub lang: Lang,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    pub provenance: Provenance,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<NodeId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<Edge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<Process>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl Node {
    pub fn new(id: impl Into<NodeId>, kind: impl Into<Kind>, fqn: impl Into<String>) -> Self {
        let fqn = fqn.into();
        let name = fqn.rsplit("::").next().unwrap_or(&fqn).to_owned();
        Self {
            id: id.into(),
            kind: kind.into(),
            name,
            fqn,
            lang: Lang::Rust,
            span: None,
            signature: None,
            docs: None,
            provenance: Provenance::default(),
            parent: None,
            children: Vec::new(),
            edges: Vec::new(),
            process: None,
            annotations: BTreeMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_docs(mut self, docs: impl Into<String>) -> Self {
        self.docs = Some(docs.into());
        self
    }

    pub fn with_provenance(mut self, provenance: Provenance) -> Self {
        self.provenance = provenance;
        self
    }

    pub fn with_process(mut self, process: Process) -> Self {
        self.process = Some(process);
        self
    }

    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = Some(signature.into());
        self
    }
}

/// The whole graph for one workspace snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Document {
    pub schema_version: u32,
    /// Tool name and version that produced this document.
    pub generator: String,
    pub nodes: BTreeMap<NodeId, Node>,
    /// Node ids with no parent.
    pub roots: Vec<NodeId>,
}

impl Document {
    pub fn new(generator: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            generator: generator.into(),
            nodes: BTreeMap::new(),
            roots: Vec::new(),
        }
    }

    pub fn insert(&mut self, node: Node) {
        let id = node.id.clone();
        if node.parent.is_none() {
            self.roots.push(id.clone());
        } else if let Some(parent) = &node.parent {
            if let Some(p) = self.nodes.get_mut(parent) {
                if !p.children.contains(&id) {
                    p.children.push(id.clone());
                }
            }
        }
        self.nodes.insert(id, node);
    }

    pub fn get(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// All nodes under `root`, depth-first, deterministic order.
    pub fn subtree(&self, root: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        self.walk(root, &mut out);
        out
    }

    fn walk<'a>(&'a self, id: &NodeId, out: &mut Vec<&'a Node>) {
        if let Some(node) = self.nodes.get(id) {
            out.push(node);
            for child in &node.children {
                self.walk(child, out);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Diagnostic {
    pub severity: Severity,
    /// Short machine-readable code, e.g. `edge/unresolved`.
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeId>,
}

impl Diagnostic {
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }

    pub fn warning(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code: code.into(),
            message: message.into(),
            node: None,
        }
    }

    pub fn about(mut self, node: impl Into<NodeId>) -> Self {
        self.node = Some(node.into());
        self
    }
}

/// Structural checks that do not depend on source text.
///
/// Returns diagnostics rather than a `Result`: a graph with warnings is still
/// renderable, and hiding soft problems behind a hard error is how they rot.
pub fn validate(doc: &Document) -> Vec<Diagnostic> {
    let mut out = Vec::new();

    if doc.schema_version != SCHEMA_VERSION {
        out.push(Diagnostic::error(
            "schema/version",
            format!(
                "document schema_version {} does not match tool {}",
                doc.schema_version, SCHEMA_VERSION
            ),
        ));
    }

    for (id, node) in &doc.nodes {
        if let Some(parent) = &node.parent {
            if !doc.nodes.contains_key(parent) {
                out.push(
                    Diagnostic::error(
                        "node/parent-missing",
                        format!("parent {parent} is not in the document"),
                    )
                    .about(id.clone()),
                );
            }
        }
        for child in &node.children {
            if !doc.nodes.contains_key(child) {
                out.push(
                    Diagnostic::error(
                        "node/child-missing",
                        format!("child {child} is not in the document"),
                    )
                    .about(id.clone()),
                );
            }
        }
        for edge in &node.edges {
            if edge.resolved && !doc.nodes.contains_key(&edge.to) {
                out.push(
                    Diagnostic::error(
                        "edge/target-missing",
                        format!("resolved edge points at missing node {}", edge.to),
                    )
                    .about(id.clone()),
                );
            }
            if !edge.resolved {
                out.push(
                    Diagnostic::warning(
                        "edge/unresolved",
                        format!("edge to {} was never resolved", edge.to),
                    )
                    .about(id.clone()),
                );
            }
        }
        if !crate::kind::is_known(&node.kind) {
            out.push(
                Diagnostic::warning(
                    "kind/unknown-namespace",
                    format!("kind `{}` has an unknown namespace", node.kind),
                )
                .about(id.clone()),
            );
        }
    }

    out
}
