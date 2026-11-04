//! Package structure vs functional coupling views, plus DSM over packages.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use robby_ir::{Document, EdgeKind, Node, NodeId, kind};

use crate::util::{is_container, short_kind, stable_id, text};

/// Bundle of the three dependency formalisms for one root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyViews {
    pub packages: PackageStructure,
    pub coupling: CouplingGraph,
}

impl DependencyViews {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        Self {
            packages: PackageStructure::from_root(doc, root),
            coupling: CouplingGraph::from_root(doc, root),
        }
    }

    pub fn workspace(doc: &Document) -> Self {
        Self {
            packages: PackageStructure::workspace(doc),
            coupling: CouplingGraph::workspace(doc),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.packages.is_empty() && self.coupling.is_empty()
    }
}

/// Nested source-structure packages + inter-package deps (and externals).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageStructure {
    pub root: PackageNode,
    pub externals: Vec<DepModule>,
    pub edges: Vec<WeightedEdge>,
    pub layers: Vec<Vec<String>>,
    pub cycles: Vec<Vec<String>>,
    /// Flat index (tree packages + externals) for DSM / lookups.
    pub modules: Vec<DepModule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageNode {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub children: Vec<PackageNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepModule {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub label: String,
    pub item_count: usize,
    pub external: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightedEdge {
    pub from: String,
    pub to: String,
    pub weight: usize,
    pub in_cycle: bool,
}

/// Internal call coupling: modules as nested subgraphs, functions as nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingGraph {
    pub root: CouplingModule,
    pub functions: Vec<CouplingFn>,
    pub edges: Vec<WeightedEdge>,
    /// Workspace mode: modules are leaf nodes (no functions).
    pub module_nodes: Vec<CouplingFn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingModule {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub children: Vec<CouplingModule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouplingFn {
    pub id: NodeId,
    pub stable_id: String,
    pub name: String,
    /// Owning module (or crate) stable_id.
    pub parent_stable_id: String,
}

impl PackageStructure {
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self::empty();
        };

        let packages: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| {
                is_container(node) && (node.id == root_node.id || is_under_fqn(root_node, node))
            })
            .collect();

        if packages.is_empty() {
            return Self::empty();
        }

        let tree = build_package_tree(root_node, &packages);
        let scope: BTreeSet<NodeId> = packages.iter().map(|p| p.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.subtree(root).into_iter(), false, root_node);

        Self {
            root: tree,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }

    pub fn workspace(doc: &Document) -> Self {
        let crates: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE))
            .collect();
        if crates.is_empty() {
            return Self::empty();
        }

        let mut children: Vec<PackageNode> = crates.iter().map(|c| package_leaf(c)).collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));

        let root = PackageNode {
            id: NodeId::from("workspace://root"),
            stable_id: stable_id("workspace://root"),
            name: "workspace".into(),
            fqn: "workspace".into(),
            children,
        };

        let scope: BTreeSet<NodeId> = crates.iter().map(|c| c.id.clone()).collect();
        let (edges, externals, modules, layers, cycles) =
            package_edges(doc, &scope, doc.nodes.values(), true, crates[0]);

        Self {
            root,
            externals,
            edges,
            layers,
            cycles,
            modules,
        }
    }

    pub fn empty() -> Self {
        Self {
            root: PackageNode {
                id: NodeId::from(""),
                stable_id: String::new(),
                name: String::new(),
                fqn: String::new(),
                children: Vec::new(),
            },
            externals: Vec::new(),
            edges: Vec::new(),
            layers: Vec::new(),
            cycles: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty() && self.modules.is_empty() && self.edges.is_empty()
    }

    pub fn module_by_stable<'a>(&'a self, id: &str) -> Option<&'a DepModule> {
        self.modules.iter().find(|m| m.stable_id == id)
    }
}

impl CouplingGraph {
    /// Function-level internal Calls under `root`.
    pub fn from_root(doc: &Document, root: &NodeId) -> Self {
        let Some(root_node) = doc.get(root) else {
            return Self::empty();
        };

        let packages: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| {
                is_container(node) && (node.id == root_node.id || is_under_fqn(root_node, node))
            })
            .collect();
        let package_scope: BTreeSet<NodeId> = packages.iter().map(|p| p.id.clone()).collect();

        let mut weights: BTreeMap<(String, String), usize> = BTreeMap::new();
        let mut participants: BTreeMap<String, CouplingFn> = BTreeMap::new();

        for node in doc.subtree(root) {
            if !is_callable(node) {
                continue;
            }
            for edge in &node.edges {
                if edge.kind != EdgeKind::Calls {
                    continue;
                }
                let Some(target) = doc.get(&edge.to) else {
                    continue;
                };
                if !is_callable(target) || !under_root_fqn(root_node, target) {
                    continue;
                }

                let Some(from_mod) = owning_in_scope(doc, &node.id, &package_scope) else {
                    continue;
                };
                let Some(to_mod) = owning_in_scope(doc, &target.id, &package_scope) else {
                    continue;
                };

                let from_sid = stable_id(node.id.as_str());
                let to_sid = stable_id(target.id.as_str());
                participants
                    .entry(from_sid.clone())
                    .or_insert_with(|| CouplingFn {
                        id: node.id.clone(),
                        stable_id: from_sid.clone(),
                        name: node.name.clone(),
                        parent_stable_id: stable_id(from_mod.id.as_str()),
                    });
                participants
                    .entry(to_sid.clone())
                    .or_insert_with(|| CouplingFn {
                        id: target.id.clone(),
                        stable_id: to_sid.clone(),
                        name: target.name.clone(),
                        parent_stable_id: stable_id(to_mod.id.as_str()),
                    });
                if from_sid != to_sid {
                    *weights.entry((from_sid, to_sid)).or_default() += 1;
                }
            }
        }

        if participants.is_empty() {
            return Self::empty();
        }

        let owner_fqns: BTreeSet<String> = participants
            .values()
            .filter_map(|f| {
                packages
                    .iter()
                    .find(|m| stable_id(m.id.as_str()) == f.parent_stable_id)
                    .map(|m| m.fqn.clone())
            })
            .collect();

        // Modules that own a participant, plus every ancestor under root.
        let tree_packages: Vec<&Node> = packages
            .iter()
            .copied()
            .filter(|p| {
                owner_fqns
                    .iter()
                    .any(|fqn| fqn == &p.fqn || fqn.starts_with(&format!("{}::", p.fqn)))
            })
            .collect();

        let root_mod = build_coupling_tree(root_node, &tree_packages);
        let edges = finalize_edges(weights);
        let mut functions: Vec<CouplingFn> = participants.into_values().collect();
        functions.sort_by(|a, b| a.name.cmp(&b.name).then(a.stable_id.cmp(&b.stable_id)));

        Self {
            root: root_mod,
            functions,
            edges,
            module_nodes: Vec::new(),
        }
    }

    /// Crates as subgraphs; direct modules as nodes; call-weighted module edges.
    pub fn workspace(doc: &Document) -> Self {
        let crates: Vec<&Node> = doc
            .nodes
            .values()
            .filter(|node| matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE))
            .collect();
        if crates.is_empty() {
            return Self::empty();
        }

        let mut module_nodes: BTreeMap<String, CouplingFn> = BTreeMap::new();
        let mut crate_children: BTreeMap<String, Vec<PackageNode>> = BTreeMap::new();

        for krate in &crates {
            let directs: Vec<&Node> = doc
                .nodes
                .values()
                .filter(|node| is_container(node) && is_direct_package(krate, node))
                .collect();
            let mut kids = Vec::new();
            for m in &directs {
                let sid = stable_id(m.id.as_str());
                module_nodes.insert(
                    sid.clone(),
                    CouplingFn {
                        id: m.id.clone(),
                        stable_id: sid.clone(),
                        name: m.name.clone(),
                        parent_stable_id: stable_id(krate.id.as_str()),
                    },
                );
                kids.push(package_leaf(m));
            }
            kids.sort_by(|a, b| a.name.cmp(&b.name));
            crate_children.insert(stable_id(krate.id.as_str()), kids);
        }

        // Scope = all direct modules across crates.
        let scope: BTreeSet<NodeId> = module_nodes.values().map(|m| m.id.clone()).collect();

        let mut weights: BTreeMap<(String, String), usize> = BTreeMap::new();
        for node in doc.nodes.values() {
            for edge in &node.edges {
                if edge.kind != EdgeKind::Calls {
                    continue;
                }
                let Some(from_mod) = owning_in_scope(doc, &node.id, &scope) else {
                    continue;
                };
                let Some(to_owner) = container_of(doc, &edge.to) else {
                    continue;
                };
                let Some(to_mod) = lift_to_scope(doc, to_owner, &scope) else {
                    continue;
                };
                if from_mod.id == to_mod.id {
                    continue;
                }
                let from_sid = stable_id(from_mod.id.as_str());
                let to_sid = stable_id(to_mod.id.as_str());
                *weights.entry((from_sid, to_sid)).or_default() += 1;
            }
        }

        // Only keep modules that participate in an edge.
        let active: BTreeSet<String> = weights
            .keys()
            .flat_map(|(a, b)| [a.clone(), b.clone()])
            .collect();
        module_nodes.retain(|sid, _| active.contains(sid));

        let mut children: Vec<CouplingModule> = crates
            .iter()
            .map(|c| {
                let sid = stable_id(c.id.as_str());
                let kids = crate_children
                    .remove(&sid)
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|k| active.contains(&k.stable_id))
                    .map(|k| CouplingModule {
                        id: k.id,
                        stable_id: k.stable_id,
                        name: k.name,
                        fqn: k.fqn,
                        children: Vec::new(),
                    })
                    .collect();
                CouplingModule {
                    id: c.id.clone(),
                    stable_id: sid,
                    name: c.name.clone(),
                    fqn: c.fqn.clone(),
                    children: kids,
                }
            })
            .filter(|c| !c.children.is_empty())
            .collect();
        children.sort_by(|a, b| a.name.cmp(&b.name));

        let root = CouplingModule {
            id: NodeId::from("workspace://root"),
            stable_id: stable_id("workspace://root"),
            name: "workspace".into(),
            fqn: "workspace".into(),
            children,
        };

        let edges = finalize_edges(weights);
        let mut module_nodes: Vec<CouplingFn> = module_nodes.into_values().collect();
        module_nodes.sort_by(|a, b| a.name.cmp(&b.name).then(a.stable_id.cmp(&b.stable_id)));

        if module_nodes.is_empty() {
            return Self::empty();
        }

        Self {
            root,
            functions: Vec::new(),
            edges,
            module_nodes,
        }
    }

    pub fn empty() -> Self {
        Self {
            root: CouplingModule {
                id: NodeId::from(""),
                stable_id: String::new(),
                name: String::new(),
                fqn: String::new(),
                children: Vec::new(),
            },
            functions: Vec::new(),
            edges: Vec::new(),
            module_nodes: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.functions.is_empty() && self.module_nodes.is_empty() && self.edges.is_empty()
    }

    pub fn leaf_nodes(&self) -> &[CouplingFn] {
        if self.functions.is_empty() {
            &self.module_nodes
        } else {
            &self.functions
        }
    }
}

fn package_edges<'a>(
    doc: &Document,
    scope: &BTreeSet<NodeId>,
    nodes: impl Iterator<Item = &'a Node>,
    both_in_scope: bool,
    _root_hint: &Node,
) -> (
    Vec<WeightedEdge>,
    Vec<DepModule>,
    Vec<DepModule>,
    Vec<Vec<String>>,
    Vec<Vec<String>>,
) {
    let mut weights: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut modules: BTreeMap<String, DepModule> = BTreeMap::new();

    for id in scope {
        if let Some(node) = doc.get(id) {
            insert_module(&mut modules, node, false, doc);
        }
    }

    for node in nodes {
        for edge in &node.edges {
            let Some(from_mod) = owning_in_scope(doc, &node.id, scope) else {
                continue;
            };
            let Some(to_owner) = container_of(doc, &edge.to) else {
                continue;
            };
            let Some(to_mod) = lift_to_scope(doc, to_owner, scope).or_else(|| {
                if both_in_scope {
                    None
                } else {
                    Some(crate_of(doc, to_owner).unwrap_or(to_owner))
                }
            }) else {
                continue;
            };
            if from_mod.id == to_mod.id {
                continue;
            }
            let from_sid = stable_id(from_mod.id.as_str());
            let to_sid = stable_id(to_mod.id.as_str());
            insert_module(&mut modules, from_mod, false, doc);
            insert_module(&mut modules, to_mod, !scope.contains(&to_mod.id), doc);
            *weights.entry((from_sid, to_sid)).or_default() += 1;
        }
    }

    let adjacency: BTreeMap<String, BTreeSet<String>> = {
        let mut adj: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for sid in modules.keys() {
            adj.entry(sid.clone()).or_default();
        }
        for (from, to) in weights.keys() {
            adj.entry(from.clone()).or_default().insert(to.clone());
        }
        adj
    };

    let sccs = tarjan(&adjacency);
    let cyclic: BTreeSet<String> = sccs
        .iter()
        .filter(|scc| scc.len() > 1)
        .flat_map(|scc| scc.iter().cloned())
        .collect();

    let mut edge_in_cycle: BTreeSet<(String, String)> = BTreeSet::new();
    for scc in &sccs {
        if scc.len() < 2 {
            continue;
        }
        let set: BTreeSet<_> = scc.iter().cloned().collect();
        for (from, to) in weights.keys() {
            if set.contains(from) && set.contains(to) {
                edge_in_cycle.insert((from.clone(), to.clone()));
            }
        }
    }

    let mut edges: Vec<WeightedEdge> = weights
        .into_iter()
        .map(|((from, to), weight)| WeightedEdge {
            in_cycle: edge_in_cycle.contains(&(from.clone(), to.clone())),
            from,
            to,
            weight,
        })
        .collect();
    edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));

    let cycles: Vec<Vec<String>> = sccs
        .into_iter()
        .filter(|scc| scc.len() > 1)
        .map(|mut scc| {
            scc.sort();
            scc
        })
        .collect();

    let layers = topological_layers(&adjacency, &cyclic);

    let mut module_list: Vec<DepModule> = modules.into_values().collect();
    module_list.sort_by(|a, b| a.name.cmp(&b.name));
    let externals: Vec<DepModule> = module_list.iter().filter(|m| m.external).cloned().collect();

    (edges, externals, module_list, layers, cycles)
}

fn finalize_edges(weights: BTreeMap<(String, String), usize>) -> Vec<WeightedEdge> {
    let adjacency: BTreeMap<String, BTreeSet<String>> = {
        let mut adj: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (from, to) in weights.keys() {
            adj.entry(from.clone()).or_default().insert(to.clone());
            adj.entry(to.clone()).or_default();
        }
        adj
    };
    let sccs = tarjan(&adjacency);
    let mut edge_in_cycle: BTreeSet<(String, String)> = BTreeSet::new();
    for scc in &sccs {
        if scc.len() < 2 {
            continue;
        }
        let set: BTreeSet<_> = scc.iter().cloned().collect();
        for (from, to) in weights.keys() {
            if set.contains(from) && set.contains(to) {
                edge_in_cycle.insert((from.clone(), to.clone()));
            }
        }
    }
    let mut edges: Vec<WeightedEdge> = weights
        .into_iter()
        .map(|((from, to), weight)| WeightedEdge {
            in_cycle: edge_in_cycle.contains(&(from.clone(), to.clone())),
            from,
            to,
            weight,
        })
        .collect();
    edges.sort_by(|a, b| (&a.from, &a.to).cmp(&(&b.from, &b.to)));
    edges
}

fn build_package_tree(root: &Node, packages: &[&Node]) -> PackageNode {
    fn children_of(parent_fqn: &str, by_fqn: &BTreeMap<&str, &Node>) -> Vec<PackageNode> {
        let prefix = format!("{parent_fqn}::");
        let mut kids = Vec::new();
        for (fqn, node) in by_fqn {
            let Some(rest) = fqn.strip_prefix(&prefix) else {
                continue;
            };
            if rest.is_empty() || rest.contains("::") {
                continue;
            }
            let mut child = package_leaf(node);
            child.children = children_of(fqn, by_fqn);
            kids.push(child);
        }
        kids.sort_by(|a, b| a.name.cmp(&b.name));
        kids
    }
    let by_fqn: BTreeMap<&str, &Node> = packages.iter().map(|p| (p.fqn.as_str(), *p)).collect();
    let mut root_node = package_leaf(root);
    root_node.children = children_of(&root.fqn, &by_fqn);
    root_node
}

fn build_coupling_tree(root: &Node, packages: &[&Node]) -> CouplingModule {
    fn convert(node: PackageNode) -> CouplingModule {
        CouplingModule {
            id: node.id,
            stable_id: node.stable_id,
            name: node.name,
            fqn: node.fqn,
            children: node.children.into_iter().map(convert).collect(),
        }
    }
    let tree = build_package_tree(root, packages);
    convert(tree)
}

fn package_leaf(node: &Node) -> PackageNode {
    PackageNode {
        id: node.id.clone(),
        stable_id: stable_id(node.id.as_str()),
        name: node.name.clone(),
        fqn: node.fqn.clone(),
        children: Vec::new(),
    }
}

fn insert_module(
    modules: &mut BTreeMap<String, DepModule>,
    node: &Node,
    external: bool,
    doc: &Document,
) {
    let sid = stable_id(node.id.as_str());
    modules.entry(sid.clone()).or_insert_with(|| DepModule {
        id: node.id.clone(),
        stable_id: sid,
        name: node.name.clone(),
        label: text(&format!("{} [{}]", node.name, short_kind(&node.kind))),
        item_count: doc.subtree(&node.id).len(),
        external,
    });
}

fn is_callable(node: &Node) -> bool {
    matches!(
        node.kind.as_str(),
        kind::code::FUNCTION | kind::code::METHOD
    )
}

fn is_under_fqn(root: &Node, candidate: &Node) -> bool {
    candidate.fqn.starts_with(&format!("{}::", root.fqn))
}

fn under_root_fqn(root: &Node, candidate: &Node) -> bool {
    candidate.fqn == root.fqn || is_under_fqn(root, candidate)
}

fn owning_in_scope<'a>(
    doc: &'a Document,
    id: &NodeId,
    scope: &BTreeSet<NodeId>,
) -> Option<&'a Node> {
    let node = doc.get(id)?;
    if let Some(m) = best_scope_prefix(doc, node, scope) {
        return Some(m);
    }
    let mut current = node;
    loop {
        if scope.contains(&current.id) {
            return Some(current);
        }
        let parent = current.parent.as_ref().and_then(|p| doc.get(p))?;
        current = parent;
    }
}

fn container_of<'a>(doc: &'a Document, id: &NodeId) -> Option<&'a Node> {
    let mut current = doc.get(id)?;
    if is_container(current) {
        return Some(current);
    }
    while let Some(parent_id) = &current.parent {
        let parent = doc.get(parent_id)?;
        if is_container(parent) {
            return Some(parent);
        }
        current = parent;
    }
    None
}

fn crate_of<'a>(doc: &'a Document, node: &'a Node) -> Option<&'a Node> {
    let mut current = Some(node);
    while let Some(n) = current {
        if matches!(n.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE) {
            return Some(n);
        }
        current = n.parent.as_ref().and_then(|id| doc.get(id));
    }
    None
}

fn lift_to_scope<'a>(
    doc: &'a Document,
    node: &'a Node,
    scope: &BTreeSet<NodeId>,
) -> Option<&'a Node> {
    if let Some(m) = best_scope_prefix(doc, node, scope) {
        return Some(m);
    }
    let mut current = Some(node);
    while let Some(n) = current {
        if scope.contains(&n.id) {
            return Some(n);
        }
        current = n.parent.as_ref().and_then(|id| doc.get(id));
    }
    None
}

fn best_scope_prefix<'a>(
    doc: &'a Document,
    node: &Node,
    scope: &BTreeSet<NodeId>,
) -> Option<&'a Node> {
    let mut best: Option<&Node> = None;
    let mut best_len = 0usize;
    for id in scope {
        let Some(candidate) = doc.get(id) else {
            continue;
        };
        if node.id == candidate.id || node.fqn == candidate.fqn {
            return Some(candidate);
        }
        let prefix = format!("{}::", candidate.fqn);
        if node.fqn.starts_with(&prefix) && candidate.fqn.len() > best_len {
            best_len = candidate.fqn.len();
            best = Some(candidate);
        }
    }
    best
}

fn is_direct_package(root: &Node, candidate: &Node) -> bool {
    let prefix = format!("{}::", root.fqn);
    let Some(rest) = candidate.fqn.strip_prefix(&prefix) else {
        return false;
    };
    !rest.is_empty() && !rest.contains("::")
}

fn tarjan(adj: &BTreeMap<String, BTreeSet<String>>) -> Vec<Vec<String>> {
    fn strongconnect(
        v: &str,
        adj: &BTreeMap<String, BTreeSet<String>>,
        index: &mut usize,
        stack: &mut Vec<String>,
        on_stack: &mut BTreeSet<String>,
        indices: &mut BTreeMap<String, usize>,
        lowlink: &mut BTreeMap<String, usize>,
        sccs: &mut Vec<Vec<String>>,
    ) {
        indices.insert(v.to_owned(), *index);
        lowlink.insert(v.to_owned(), *index);
        *index += 1;
        stack.push(v.to_owned());
        on_stack.insert(v.to_owned());

        if let Some(neighbors) = adj.get(v) {
            for w in neighbors {
                if !indices.contains_key(w) {
                    strongconnect(w, adj, index, stack, on_stack, indices, lowlink, sccs);
                    let lw = lowlink[w];
                    let lv = lowlink[v];
                    lowlink.insert(v.to_owned(), lv.min(lw));
                } else if on_stack.contains(w) {
                    let iw = indices[w];
                    let lv = lowlink[v];
                    lowlink.insert(v.to_owned(), lv.min(iw));
                }
            }
        }

        if lowlink[v] == indices[v] {
            let mut scc = Vec::new();
            loop {
                let w = stack.pop().expect("scc stack");
                on_stack.remove(&w);
                scc.push(w.clone());
                if w == v {
                    break;
                }
            }
            sccs.push(scc);
        }
    }

    let mut index = 0usize;
    let mut stack: Vec<String> = Vec::new();
    let mut on_stack: BTreeSet<String> = BTreeSet::new();
    let mut indices: BTreeMap<String, usize> = BTreeMap::new();
    let mut lowlink: BTreeMap<String, usize> = BTreeMap::new();
    let mut sccs: Vec<Vec<String>> = Vec::new();

    for v in adj.keys() {
        if !indices.contains_key(v) {
            strongconnect(
                v,
                adj,
                &mut index,
                &mut stack,
                &mut on_stack,
                &mut indices,
                &mut lowlink,
                &mut sccs,
            );
        }
    }

    sccs
}

fn topological_layers(
    adj: &BTreeMap<String, BTreeSet<String>>,
    cyclic: &BTreeSet<String>,
) -> Vec<Vec<String>> {
    let mut indegree: BTreeMap<String, usize> = adj.keys().map(|k| (k.clone(), 0)).collect();
    let mut forward: BTreeMap<String, BTreeSet<String>> =
        adj.keys().map(|k| (k.clone(), BTreeSet::new())).collect();

    for (from, tos) in adj {
        for to in tos {
            if cyclic.contains(from) && cyclic.contains(to) {
                continue;
            }
            forward.get_mut(from).unwrap().insert(to.clone());
            *indegree.get_mut(to).unwrap() += 1;
        }
    }

    let mut queue: VecDeque<String> = indegree
        .iter()
        .filter(|(_, d)| **d == 0)
        .map(|(k, _)| k.clone())
        .collect();
    let mut layers = Vec::new();

    while !queue.is_empty() {
        let mut layer: Vec<String> = queue.drain(..).collect();
        layer.sort();
        let mut next = VecDeque::new();
        for v in &layer {
            for w in &forward[v] {
                let d = indegree.get_mut(w).unwrap();
                *d -= 1;
                if *d == 0 {
                    next.push_back(w.clone());
                }
            }
        }
        layers.push(layer);
        queue = next;
    }

    let placed: BTreeSet<_> = layers.iter().flatten().cloned().collect();
    let mut rest: Vec<String> = adj
        .keys()
        .filter(|k| !placed.contains(*k))
        .cloned()
        .collect();
    if !rest.is_empty() {
        rest.sort();
        layers.push(rest);
    }

    layers
}
