//! The graph document: nodes, edges, provenance, diagnostics.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::codepath::{CodePath, DEFAULT_MAX_DEPTH, build_code_paths};
use crate::kind::Kind;
use crate::process::Process;
use crate::run::LiveRunSource;
use crate::span::Span;

/// Bumped only on breaking changes. Additive fields do not bump it.
pub const SCHEMA_VERSION: u32 = 1;

/// A stable, content-addressed node identifier.
///
/// Symbols use `lang://fqn`, e.g. `rust://robby_ir::model::Node`. Statements
/// use `parent#ordinal`. Ordinal-based ids are stable within a run but shift
/// when earlier siblings change; that is documented and accepted, because
/// byte-offset ids would shift on every edit above them instead.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
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

    /// Serde name. Views display this rather than keeping a second match.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Extractor => "extractor",
            Self::Macro => "macro",
            Self::Trace => "trace",
            Self::Agent => "agent",
            Self::Hand => "hand",
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

// `skip_serializing_if` requires `fn(&T) -> bool`, so this stays a reference.
#[allow(clippy::trivially_copy_pass_by_ref)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
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

impl EdgeKind {
    /// Serde name (`references`, not the short diagram label).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Contains => "contains",
            Self::Calls => "calls",
            Self::Uses => "uses",
            Self::Imports => "imports",
            Self::Implements => "implements",
            Self::Extends => "extends",
            Self::Returns => "returns",
            Self::Field => "field",
            Self::References => "references",
        }
    }
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
    /// Parent in the containment tree. This is the link callers set.
    /// [`Document::merge`] maintains [`Self::children`] from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<NodeId>,
    /// Child ids in insertion order. Written only by [`Document::merge`].
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
        Self::with_lang(id, kind, fqn, Lang::Rust)
    }

    /// Like [`Node::new`], with an explicit language tag.
    pub fn with_lang(
        id: impl Into<NodeId>,
        kind: impl Into<Kind>,
        fqn: impl Into<String>,
        lang: Lang,
    ) -> Self {
        let fqn = fqn.into();
        let name = leaf_name(&fqn);
        Self {
            id: id.into(),
            kind: kind.into(),
            name,
            fqn,
            lang,
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

    pub fn set_lang(mut self, lang: Lang) -> Self {
        self.lang = lang;
        self
    }
}

/// Leaf segment of an FQN across Rust (`::`), path (`/`), and dotted TS forms.
pub fn leaf_name(fqn: &str) -> String {
    let after_colon = fqn.rsplit("::").next().unwrap_or(fqn);
    let after_slash = after_colon.rsplit('/').next().unwrap_or(after_colon);
    after_slash
        .rsplit('.')
        .next()
        .unwrap_or(after_slash)
        .to_owned()
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
    /// Every source file this document claims to describe.
    ///
    /// This is what makes "no file is invisible" checkable: the coverage gate
    /// reads this list, so a file that was never parsed is a failure rather
    /// than an absence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    /// Entrypoint-rooted call trees derived from resolved `Calls` edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub code_paths: Vec<CodePath>,
    /// Runtime-fetchable live run feeds (Cursor cloud dump, later Claude/…).
    ///
    /// Written to site `runs/sources.json` so Vite / the viewer can poll without
    /// re-extract. Empty in IR means use [`crate::run::default_live_run_sources`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub live_run_sources: Vec<LiveRunSource>,
    /// Children whose parent node is not in the document yet.
    ///
    /// Not part of the schema. Flushed when the parent is inserted so a child
    /// can be recorded before its parent without a second writer.
    #[serde(skip)]
    #[schemars(skip)]
    pending_children: BTreeMap<NodeId, Vec<NodeId>>,
    /// Parent declared for a child node that has not been inserted yet.
    #[serde(skip)]
    #[schemars(skip)]
    pending_parents: BTreeMap<NodeId, NodeId>,
}

impl Document {
    pub fn new(generator: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            generator: generator.into(),
            nodes: BTreeMap::new(),
            roots: Vec::new(),
            files: Vec::new(),
            code_paths: Vec::new(),
            live_run_sources: Vec::new(),
            pending_children: BTreeMap::new(),
            pending_parents: BTreeMap::new(),
        }
    }

    /// Rebuild [`Self::code_paths`] from current `Calls` edges and `main` entries.
    pub fn rebuild_code_paths(&mut self) {
        self.code_paths = build_code_paths(self, DEFAULT_MAX_DEPTH);
    }

    /// Record a source file as part of the map.
    pub fn declare_file(&mut self, path: impl Into<String>) {
        let path = path.into();
        if !self.files.contains(&path) {
            self.files.push(path);
            self.files.sort();
        }
    }

    pub fn insert(&mut self, node: Node) {
        self.merge(node);
    }

    /// Insert, or merge into an existing node with the same id.
    ///
    /// Merging matters for modules: a `mod foo;` declaration and the file
    /// `foo.rs` both describe `crate::foo`. The declaration carries the span,
    /// the file carries the contents, and they must not overwrite each other.
    ///
    /// Containment has one writer: this method. Set [`Node::parent`] on the
    /// child. A `children` list passed in is normalized into that same index,
    /// including when the child node arrives later.
    pub fn merge(&mut self, mut node: Node) {
        let id = node.id.clone();
        let declared_children = std::mem::take(&mut node.children);
        let incoming_parent = node.parent.clone();

        let adopted_parent = if let Some(existing) = self.nodes.get_mut(&id) {
            if existing.span.is_none() {
                existing.span = node.span;
            }
            let adopted = existing.parent.is_none() && incoming_parent.is_some();
            if adopted {
                existing.parent = incoming_parent;
            }
            for edge in node.edges {
                if !existing.edges.contains(&edge) {
                    existing.edges.push(edge);
                }
            }
            for (key, value) in node.annotations {
                existing.annotations.entry(key).or_insert(value);
            }
            for tag in node.tags {
                if !existing.tags.contains(&tag) {
                    existing.tags.push(tag);
                }
            }
            adopted
        } else {
            if node.parent.is_none()
                && let Some(parent) = self.pending_parents.remove(&id)
            {
                node.parent = Some(parent);
            }
            if node.parent.is_none() {
                self.roots.push(id.clone());
            }
            self.nodes.insert(id.clone(), node);
            if let Some(waiting) = self.pending_children.remove(&id) {
                for child in waiting {
                    self.record_child(&id, &child);
                }
            }
            false
        };

        if let Some(parent) = self.nodes.get(&id).and_then(|n| n.parent.clone()) {
            self.record_child(&parent, &id);
        }
        if adopted_parent {
            self.roots.retain(|root| root != &id);
        }
        for child in declared_children {
            self.record_child(&id, &child);
        }
        // A children list can name a node that is inserted later, with no parent
        // of its own. Point that node back so the two fields cannot diverge.
        self.adopt_parent(&id);
    }

    /// If some node already lists `id` and `id` has no parent, adopt that link.
    fn adopt_parent(&mut self, id: &NodeId) {
        if self.nodes.get(id).is_some_and(|node| node.parent.is_some()) {
            return;
        }
        let listed = self
            .nodes
            .iter()
            .find(|(parent_id, node)| *parent_id != id && node.children.contains(id))
            .map(|(parent_id, _)| parent_id.clone());
        if let Some(parent) = listed {
            self.record_child(&parent, id);
        }
    }

    /// Record `child` under `parent`, and point the child back if it has no parent yet.
    fn record_child(&mut self, parent: &NodeId, child: &NodeId) {
        if parent == child {
            return;
        }
        if let Some(parent_node) = self.nodes.get_mut(parent) {
            if !parent_node.children.contains(child) {
                parent_node.children.push(child.clone());
            }
        } else {
            let pending = self.pending_children.entry(parent.clone()).or_default();
            if !pending.contains(child) {
                pending.push(child.clone());
            }
        }
        if !self.nodes.contains_key(child) {
            self.pending_parents
                .entry(child.clone())
                .or_insert_with(|| parent.clone());
        }
        let needs_parent = self
            .nodes
            .get(child)
            .is_some_and(|node| node.parent.is_none());
        if needs_parent {
            if let Some(node) = self.nodes.get_mut(child) {
                node.parent = Some(parent.clone());
            }
            self.roots.retain(|root| root != child);
        }
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
        if let Some(parent) = &node.parent
            && !doc.nodes.contains_key(parent)
        {
            out.push(
                Diagnostic::error(
                    "node/parent-missing",
                    format!("parent {parent} is not in the document"),
                )
                .about(id.clone()),
            );
        }
        if let Some(parent) = &node.parent
            && let Some(parent_node) = doc.nodes.get(parent)
            && !parent_node.children.iter().any(|child| child == id)
        {
            out.push(
                Diagnostic::error(
                    "node/parent-link",
                    format!("parent {parent} does not list this node"),
                )
                .about(id.clone()),
            );
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
            } else if doc.nodes.get(child).and_then(|n| n.parent.as_ref()) != Some(id) {
                out.push(
                    Diagnostic::error(
                        "node/child-link",
                        format!("child {child} does not point at this parent"),
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

    for path in &doc.code_paths {
        if !doc.nodes.contains_key(&path.entry) {
            out.push(
                Diagnostic::error(
                    "codepath/entry-missing",
                    format!("code path entry {} is not in the document", path.entry),
                )
                .about(path.entry.clone()),
            );
        }
        for edge in &path.edges {
            if !doc.nodes.contains_key(&edge.from) {
                out.push(
                    Diagnostic::error(
                        "codepath/from-missing",
                        format!("code path edge from missing node {}", edge.from),
                    )
                    .about(edge.from.clone()),
                );
            }
            if !doc.nodes.contains_key(&edge.to) {
                out.push(
                    Diagnostic::error(
                        "codepath/to-missing",
                        format!("code path edge to missing node {}", edge.to),
                    )
                    .about(edge.to.clone()),
                );
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wire<T: serde::Serialize>(value: T) -> String {
        serde_json::to_value(value)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    }

    #[test]
    fn source_and_edge_names_match_serde() {
        for source in [
            Source::Extractor,
            Source::Macro,
            Source::Trace,
            Source::Agent,
            Source::Hand,
        ] {
            assert_eq!(wire(source), source.as_str());
        }
        for kind in [
            EdgeKind::Contains,
            EdgeKind::Calls,
            EdgeKind::Uses,
            EdgeKind::Imports,
            EdgeKind::Implements,
            EdgeKind::Extends,
            EdgeKind::Returns,
            EdgeKind::Field,
            EdgeKind::References,
        ] {
            assert_eq!(wire(kind), kind.as_str());
        }
    }

    #[test]
    fn parent_pointer_builds_the_child_index() {
        let mut doc = Document::new("test");
        doc.insert(Node::new("parent", "code.module", "parent"));

        let mut child = Node::new("child", "code.function", "child");
        child.parent = Some(NodeId::from("parent"));
        doc.insert(child);

        assert_eq!(
            doc.get(&NodeId::from("parent")).unwrap().children,
            vec![NodeId::from("child")]
        );
        assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error));
    }

    #[test]
    fn child_inserted_before_its_parent_is_linked() {
        let mut doc = Document::new("test");
        let mut child = Node::new("child", "code.function", "child");
        child.parent = Some(NodeId::from("parent"));
        doc.insert(child);
        doc.insert(Node::new("parent", "code.module", "parent"));

        assert_eq!(
            doc.get(&NodeId::from("parent")).unwrap().children,
            vec![NodeId::from("child")]
        );
        assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error));
    }

    #[test]
    fn declared_children_set_the_child_parent() {
        let mut doc = Document::new("test");
        let mut parent = Node::new("parent", "code.module", "parent");
        parent.children = vec![NodeId::from("child")];
        doc.insert(parent);
        doc.insert(Node::new("child", "code.function", "child"));

        let child = doc.get(&NodeId::from("child")).unwrap();
        assert_eq!(child.parent.as_ref().map(NodeId::as_str), Some("parent"));
        assert!(!doc.roots.iter().any(|id| id.as_str() == "child"));
        assert!(validate(&doc).iter().all(|d| d.severity != Severity::Error));
    }
}
