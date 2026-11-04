//! Opportunity sampler for self-improvement runs.
//!
//! Picks a **seeded random subset** of IR subjects (default N = 8) so a metrics
//! loop does not score every function every tick. See Project
//! `docs/self-improvement-run-readiness.md`.
//!
//! Strategy for v1 is **random** only. A ranker model (predicted improvement on
//! `edge.resolve_rate` / `fn.call_resolve_rate` / …) is deliberately deferred
//! until a metric harness and labeled findings exist.

use std::collections::{BTreeMap, BTreeSet};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::kind;
use crate::model::{Document, EdgeKind, Node, NodeId};

/// Default subset size for the first self-improvement run.
pub const DEFAULT_SAMPLE_N: usize = 8;

/// RunRecord / observation stats key for the RNG seed.
pub const STAT_SAMPLE_SEED: &str = "sample_seed";
/// RunRecord / observation stats key for requested N.
pub const STAT_SAMPLE_N: &str = "sample_n";
/// RunRecord / observation stats key for strategy id (`random`).
pub const STAT_SAMPLE_STRATEGY: &str = "sample_strategy";
/// Candidate pool size before sampling (debug / observation).
pub const STAT_SAMPLE_CANDIDATES: &str = "sample_candidates";

/// v1 strategy id written into stats. Ranker is out of scope.
pub const STRATEGY_RANDOM: &str = "random";

/// Why a node is an improvement opportunity candidate.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum OpportunityReason {
    /// `code.function` / `code.method` that already carries a [`crate::Process`].
    FunctionWithProcess,
    /// `code.file` that owns (directly or via descendants) an unresolved `Calls` edge.
    FileUnresolvedCalls,
    /// `code.workspace` root — subject for document-level `doc.*` metrics.
    WorkspaceRoot,
}

impl OpportunityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FunctionWithProcess => "function_with_process",
            Self::FileUnresolvedCalls => "file_unresolved_calls",
            Self::WorkspaceRoot => "workspace_root",
        }
    }
}

/// One candidate (or selected) subject for a self-improvement slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Opportunity {
    pub subject: NodeId,
    pub reason: OpportunityReason,
    pub name: String,
}

/// Result of a seeded random sample over IR opportunities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OpportunitySample {
    pub seed: u64,
    /// Requested N (may exceed pool size; `subjects.len()` is what was drawn).
    pub n: usize,
    pub strategy: String,
    pub subjects: Vec<Opportunity>,
    pub candidate_count: usize,
}

impl OpportunitySample {
    /// Stats map for a parent review / metric_test [`crate::RunRecord`].
    pub fn stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(STAT_SAMPLE_SEED.to_owned(), self.seed.to_string());
        stats.insert(STAT_SAMPLE_N.to_owned(), self.n.to_string());
        stats.insert(STAT_SAMPLE_STRATEGY.to_owned(), self.strategy.clone());
        stats.insert(
            STAT_SAMPLE_CANDIDATES.to_owned(),
            self.candidate_count.to_string(),
        );
        stats.insert("sample_drawn".to_owned(), self.subjects.len().to_string());
        stats
    }

    /// Subject node ids in draw order (for `--subjects` / queue wiring).
    pub fn subject_ids(&self) -> Vec<&str> {
        self.subjects.iter().map(|o| o.subject.as_str()).collect()
    }
}

/// Synthetic document-level subject used when IR has no `code.workspace` node.
/// Matches site run subjects such as synthetic extract (`workspace:`).
pub const WORKSPACE_SUBJECT: &str = "workspace:";

/// Collect every opportunity candidate from `doc` (no sampling).
///
/// Buckets match the readiness plan mix: process-bearing callables, containers
/// with unresolved calls, and workspace / document root for `doc.*`. Each id
/// appears once; first insertion wins (`WorkspaceRoot` recorded before the
/// other buckets).
pub fn collect_opportunities(doc: &Document) -> Vec<Opportunity> {
    let mut by_id: BTreeMap<NodeId, Opportunity> = BTreeMap::new();

    let mut saw_workspace = false;
    for node in doc.nodes.values() {
        if node.kind.as_str() == kind::code::WORKSPACE {
            saw_workspace = true;
            insert_opportunity(&mut by_id, node, OpportunityReason::WorkspaceRoot);
        }
    }
    if !saw_workspace {
        // Document-level hook for `doc.*` metrics when extract has only crates.
        by_id
            .entry(NodeId::from(WORKSPACE_SUBJECT))
            .or_insert_with(|| Opportunity {
                subject: NodeId::from(WORKSPACE_SUBJECT),
                reason: OpportunityReason::WorkspaceRoot,
                name: "workspace".to_owned(),
            });
        for root_id in &doc.roots {
            if let Some(node) = doc.nodes.get(root_id) {
                let is_pkg = matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE);
                if is_pkg {
                    insert_opportunity(&mut by_id, node, OpportunityReason::WorkspaceRoot);
                }
            }
        }
    }

    for subject_id in containers_with_unresolved_calls(doc) {
        if let Some(node) = doc.nodes.get(&subject_id) {
            insert_opportunity(&mut by_id, node, OpportunityReason::FileUnresolvedCalls);
        } else {
            by_id
                .entry(subject_id.clone())
                .or_insert_with(|| Opportunity {
                    subject: subject_id.clone(),
                    reason: OpportunityReason::FileUnresolvedCalls,
                    name: leaf_name_of(&subject_id),
                });
        }
    }

    for node in doc.nodes.values() {
        let is_callable = matches!(
            node.kind.as_str(),
            kind::code::FUNCTION | kind::code::METHOD
        );
        if is_callable && node.process.as_ref().is_some_and(|p| !p.steps.is_empty()) {
            insert_opportunity(&mut by_id, node, OpportunityReason::FunctionWithProcess);
        }
    }

    by_id.into_values().collect()
}

fn insert_opportunity(
    by_id: &mut BTreeMap<NodeId, Opportunity>,
    node: &Node,
    reason: OpportunityReason,
) {
    by_id.entry(node.id.clone()).or_insert_with(|| Opportunity {
        subject: node.id.clone(),
        reason,
        name: node.name.clone(),
    });
}

fn leaf_name_of(id: &NodeId) -> String {
    crate::model::leaf_name(id.as_str())
}

/// Prefer `code.file`, else `code.module` / crate / package, else the call site.
fn containers_with_unresolved_calls(doc: &Document) -> BTreeSet<NodeId> {
    let mut subjects = BTreeSet::new();
    for node in doc.nodes.values() {
        let has_unresolved_call = node
            .edges
            .iter()
            .any(|e| e.kind == EdgeKind::Calls && !e.resolved);
        if !has_unresolved_call {
            continue;
        }
        subjects.insert(enclosing_call_container(doc, &node.id));
    }
    subjects
}

fn enclosing_call_container(doc: &Document, start: &NodeId) -> NodeId {
    let mut current = start.clone();
    let mut fallback = start.clone();
    for _ in 0..=doc.nodes.len() {
        let Some(node) = doc.nodes.get(&current) else {
            return fallback;
        };
        match node.kind.as_str() {
            kind::code::FILE => return current,
            kind::code::MODULE | kind::code::CRATE | kind::code::PACKAGE => {
                fallback = current.clone();
            }
            _ => {}
        }
        match node.parent.clone() {
            Some(parent) => current = parent,
            None => return fallback,
        }
    }
    fallback
}

/// Seeded random sample of up to `n` opportunities (default strategy: random).
///
/// Draws round-robin across reason buckets (after a per-bucket shuffle) so the
/// subset stays a mix when the pool allows it, then fills from leftovers.
/// Identical `(doc, seed, n)` always yields the same subjects in the same order.
pub fn sample_opportunities(doc: &Document, seed: u64, n: usize) -> OpportunitySample {
    let candidates = collect_opportunities(doc);
    let candidate_count = candidates.len();
    let drawn = draw_mixed(&candidates, seed, n);
    OpportunitySample {
        seed,
        n,
        strategy: STRATEGY_RANDOM.to_owned(),
        subjects: drawn,
        candidate_count,
    }
}

/// Same as [`sample_opportunities`] with [`DEFAULT_SAMPLE_N`].
pub fn sample_opportunities_default(doc: &Document, seed: u64) -> OpportunitySample {
    sample_opportunities(doc, seed, DEFAULT_SAMPLE_N)
}

fn draw_mixed(candidates: &[Opportunity], seed: u64, n: usize) -> Vec<Opportunity> {
    if n == 0 || candidates.is_empty() {
        return Vec::new();
    }

    let mut buckets: BTreeMap<OpportunityReason, Vec<Opportunity>> = BTreeMap::new();
    for opp in candidates {
        buckets.entry(opp.reason).or_default().push(opp.clone());
    }

    let mut rng = SplitMix64::new(seed);
    for bucket in buckets.values_mut() {
        fisher_yates(bucket, &mut rng);
    }

    let order = [
        OpportunityReason::WorkspaceRoot,
        OpportunityReason::FileUnresolvedCalls,
        OpportunityReason::FunctionWithProcess,
    ];
    let mut indices: BTreeMap<OpportunityReason, usize> =
        order.iter().map(|r| (*r, 0usize)).collect();
    let mut drawn: Vec<Opportunity> = Vec::with_capacity(n.min(candidates.len()));
    let mut seen: BTreeSet<NodeId> = BTreeSet::new();

    // Round-robin one from each non-exhausted bucket.
    loop {
        if drawn.len() >= n {
            break;
        }
        let mut progressed = false;
        for reason in order {
            if drawn.len() >= n {
                break;
            }
            let Some(bucket) = buckets.get(&reason) else {
                continue;
            };
            let idx = indices.get(&reason).copied().unwrap_or(0);
            if idx >= bucket.len() {
                continue;
            }
            let opp = &bucket[idx];
            indices.insert(reason, idx + 1);
            if seen.insert(opp.subject.clone()) {
                drawn.push(opp.clone());
                progressed = true;
            }
        }
        if !progressed {
            break;
        }
    }

    drawn
}

fn fisher_yates<T>(items: &mut [T], rng: &mut SplitMix64) {
    if items.len() < 2 {
        return;
    }
    for i in (1..items.len()).rev() {
        let j = rng.gen_range(i + 1);
        items.swap(i, j);
    }
}

/// Tiny deterministic PRNG (SplitMix64). No `rand` dependency.
#[derive(Debug, Clone)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        // Avoid the all-zero fixed point looking "stuck" for seed 0 callers.
        Self {
            state: seed.wrapping_add(0x9E37_79B9_7F4A_7C15),
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in `0..upper` (upper > 0).
    fn gen_range(&mut self, upper: usize) -> usize {
        debug_assert!(upper > 0);
        (self.next_u64() % upper as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Edge, Lang, Node};
    use crate::process::{Process, Step, StepKind};

    fn doc_with_mix() -> Document {
        let mut doc = Document::new("test");

        let ws = Node::new("rust://ws", kind::code::WORKSPACE, "ws");
        let file_a = Node::new("rust://ws/a.rs", kind::code::FILE, "a.rs");
        let file_b = Node::new("rust://ws/b.rs", kind::code::FILE, "b.rs");

        let mut proc = Process::new();
        proc.participant("a", "a", kind::Kind::from(kind::code::FUNCTION));
        proc.participant("b", "b", kind::Kind::from(kind::code::FUNCTION));
        proc.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "call".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: vec![],
        });

        let mut fn_ok =
            Node::new("rust://ws/a.rs#f1", kind::code::FUNCTION, "f1").with_process(proc);
        fn_ok.parent = Some(NodeId::from("rust://ws/a.rs"));

        let mut fn_plain = Node::new("rust://ws/a.rs#f2", kind::code::FUNCTION, "f2");
        fn_plain.parent = Some(NodeId::from("rust://ws/a.rs"));

        let mut fn_unresolved = Node::new("rust://ws/b.rs#g", kind::code::FUNCTION, "g");
        fn_unresolved.parent = Some(NodeId::from("rust://ws/b.rs"));
        fn_unresolved.edges.push(Edge {
            to: NodeId::from("rust://missing"),
            kind: EdgeKind::Calls,
            span: None,
            confidence: 0.5,
            resolved: false,
        });

        // Extra process fns so N=8 can draw more than the three bucket heads.
        let mut extras = Vec::new();
        for i in 0..10 {
            let id = format!("rust://ws/a.rs#extra{i}");
            let mut p = Process::new();
            p.participant("x", "x", kind::Kind::from(kind::code::FUNCTION));
            p.step(Step {
                from: "x".into(),
                to: "x".into(),
                label: format!("step{i}"),
                kind: StepKind::Call,
                observed: false,
                span: None,
                blocks: vec![],
            });
            let mut node =
                Node::new(id.clone(), kind::code::FUNCTION, format!("extra{i}")).with_process(p);
            node.parent = Some(NodeId::from("rust://ws/a.rs"));
            node.lang = Lang::Rust;
            extras.push(node);
        }

        let mut file_a = file_a;
        file_a.parent = Some(NodeId::from("rust://ws"));
        let mut file_b = file_b;
        file_b.parent = Some(NodeId::from("rust://ws"));

        doc.insert(ws);
        doc.insert(file_a);
        doc.insert(file_b);
        doc.insert(fn_ok);
        doc.insert(fn_plain);
        doc.insert(fn_unresolved);
        for node in extras {
            doc.insert(node);
        }
        doc
    }

    #[test]
    fn collects_workspace_process_fn_and_unresolved_file() {
        let doc = doc_with_mix();
        let opps = collect_opportunities(&doc);
        let reasons: BTreeSet<_> = opps.iter().map(|o| o.reason).collect();
        assert!(reasons.contains(&OpportunityReason::WorkspaceRoot));
        assert!(reasons.contains(&OpportunityReason::FunctionWithProcess));
        assert!(reasons.contains(&OpportunityReason::FileUnresolvedCalls));
        // Plain function without process is not a candidate.
        assert!(
            opps.iter()
                .all(|o| o.subject.as_str() != "rust://ws/a.rs#f2")
        );
        // Unresolved call contributes the *file*, not only the function.
        assert!(opps.iter().any(|o| {
            o.reason == OpportunityReason::FileUnresolvedCalls
                && o.subject.as_str() == "rust://ws/b.rs"
        }));
    }

    #[test]
    fn default_n_is_eight_and_not_every_candidate() {
        let doc = doc_with_mix();
        let sample = sample_opportunities_default(&doc, 42);
        assert_eq!(sample.n, DEFAULT_SAMPLE_N);
        assert_eq!(sample.strategy, STRATEGY_RANDOM);
        assert_eq!(sample.subjects.len(), DEFAULT_SAMPLE_N);
        assert!(sample.candidate_count > DEFAULT_SAMPLE_N);
    }

    #[test]
    fn same_seed_is_deterministic() {
        let doc = doc_with_mix();
        let a = sample_opportunities(&doc, 7, 5);
        let b = sample_opportunities(&doc, 7, 5);
        assert_eq!(a.subjects, b.subjects);
        let c = sample_opportunities(&doc, 8, 5);
        assert_ne!(a.subject_ids(), c.subject_ids());
    }

    #[test]
    fn mix_prefers_all_buckets_when_possible() {
        let doc = doc_with_mix();
        let sample = sample_opportunities(&doc, 1, 3);
        let reasons: BTreeSet<_> = sample.subjects.iter().map(|o| o.reason).collect();
        assert_eq!(reasons.len(), 3);
    }

    #[test]
    fn stats_carry_seed_and_n() {
        let doc = doc_with_mix();
        let sample = sample_opportunities(&doc, 99, 4);
        let stats = sample.stats();
        assert_eq!(stats.get(STAT_SAMPLE_SEED).map(String::as_str), Some("99"));
        assert_eq!(stats.get(STAT_SAMPLE_N).map(String::as_str), Some("4"));
        assert_eq!(
            stats.get(STAT_SAMPLE_STRATEGY).map(String::as_str),
            Some(STRATEGY_RANDOM)
        );
    }

    #[test]
    fn synthetic_workspace_when_extract_has_only_crates() {
        let mut doc = Document::new("test");
        doc.insert(Node::new("rust://demo", kind::code::CRATE, "demo"));
        let mut process = Process::new();
        process.participant("a", "a", kind::Kind::from(kind::code::FUNCTION));
        process.step(Step {
            from: "a".into(),
            to: "a".into(),
            label: "noop".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: vec![],
        });
        let mut func =
            Node::new("rust://demo::main", kind::code::FUNCTION, "main").with_process(process);
        func.parent = Some(NodeId::from("rust://demo"));
        doc.insert(func);

        let opps = collect_opportunities(&doc);
        assert!(opps.iter().any(|o| {
            o.reason == OpportunityReason::WorkspaceRoot && o.subject.as_str() == WORKSPACE_SUBJECT
        }));
        assert!(opps.iter().any(|o| {
            o.reason == OpportunityReason::WorkspaceRoot && o.subject.as_str() == "rust://demo"
        }));
        let sample = sample_opportunities(&doc, 3, 2);
        let reasons: BTreeSet<_> = sample.subjects.iter().map(|o| o.reason).collect();
        assert!(reasons.contains(&OpportunityReason::WorkspaceRoot));
        assert!(reasons.contains(&OpportunityReason::FunctionWithProcess));
    }
}
