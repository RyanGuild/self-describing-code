//! Prezi-style scene: nested frames with fixed geometry for the web camera.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use robby_ir::{Document, Node, NodeId, kind};

use crate::util::{edge_label, is_structural, short_kind, stable_id};
use crate::view::entrypoints::CallTree;
use crate::view::links::LinkGraph;
use crate::view::sibling::SiblingGraph;

/// Local canvas size for every frame.
pub const CANVAS_W: i64 = 1600;
pub const CANVAS_H: i64 = 1000;

const PAD: i64 = 80;
const HEADER: i64 = 96;
const PORTAL_STRIP: i64 = 140;
const CELL_MARGIN: i64 = 24;

/// Synthetic root id (not present in the IR).
pub const WORKSPACE_ID: &str = "workspace";

/// Whole zoomable map: one frame per structural node, plus a synthetic workspace.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Scene {
    pub root: String,
    pub canvas: CanvasSize,
    pub frames: BTreeMap<String, Frame>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CanvasSize {
    pub w: i64,
    pub h: i64,
}

/// Integer rect on a frame's local 1600×1000 canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Rect {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Self {
        Self { x, y, w, h }
    }
}

/// Absolute placement in workspace space. Floats so nested grids do not
/// collapse to 1×1 integer cells and stack on top of each other.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct WorldRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl WorldRect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, other: &WorldRect) -> bool {
        const EPS: f64 = 1e-6;
        other.x >= self.x - EPS
            && other.y >= self.y - EPS
            && other.x + other.w <= self.x + self.w + EPS
            && other.y + other.h <= self.y + self.h + EPS
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Frame {
    pub id: String,
    pub stable_id: String,
    pub name: String,
    pub fqn: String,
    pub kind_tag: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    pub children: Vec<String>,
    /// Absolute placement in the workspace world.
    pub world: WorldRect,
    /// Placement inside the parent's local canvas (absent on the root).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_in_parent: Option<Rect>,
    pub edges: Vec<SceneEdge>,
    pub portals: Vec<Portal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SceneEdge {
    pub from: String,
    pub to: String,
    pub label: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Portal {
    pub label: String,
    pub kind_label: String,
    /// Stable id of the destination frame when the edge resolves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    pub target_name: String,
}

impl Scene {
    /// Build the scene from a document. Layout is deterministic integer geometry.
    pub fn from_document(doc: &Document) -> Self {
        let root_stable = stable_id(WORKSPACE_ID);
        let mut frames = BTreeMap::new();

        let crates = crate_roots(doc);
        let child_ids: Vec<NodeId> = crates.iter().map(|n| n.id.clone()).collect();

        let mut root = Frame {
            id: WORKSPACE_ID.to_owned(),
            stable_id: root_stable.clone(),
            name: "workspace".to_owned(),
            fqn: "workspace".to_owned(),
            kind_tag: "workspace".to_owned(),
            parent: None,
            children: Vec::new(),
            world: WorldRect::new(0.0, 0.0, CANVAS_W as f64, CANVAS_H as f64),
            local_in_parent: None,
            edges: Vec::new(),
            portals: Vec::new(),
        };

        let placements = layout_grid(child_ids.len());
        for (i, node) in crates.iter().enumerate() {
            let local = placements[i];
            let world = map_local_to_world(&root.world, &local);
            let child_stable = build_frame(
                doc,
                node,
                Some(root_stable.clone()),
                local,
                world,
                &mut frames,
            );
            root.children.push(child_stable);
        }

        frames.insert(root_stable.clone(), root);

        Scene {
            root: root_stable,
            canvas: CanvasSize {
                w: CANVAS_W,
                h: CANVAS_H,
            },
            frames,
        }
    }

    /// Functional org: workspace → `main` entrypoints → Calls reachability tree.
    pub fn from_entrypoints(doc: &Document) -> Self {
        let tree = CallTree::from_document(doc);
        let root_stable = stable_id(WORKSPACE_ID);
        let mut frames = BTreeMap::new();

        let mut root = Frame {
            id: WORKSPACE_ID.to_owned(),
            stable_id: root_stable.clone(),
            name: "entrypoints".to_owned(),
            fqn: "entrypoints".to_owned(),
            kind_tag: "entrypoints".to_owned(),
            parent: None,
            children: Vec::new(),
            world: WorldRect::new(0.0, 0.0, CANVAS_W as f64, CANVAS_H as f64),
            local_in_parent: None,
            edges: Vec::new(),
            portals: Vec::new(),
        };

        let placements = layout_grid(tree.entries.len());
        for (i, entry_id) in tree.entries.iter().enumerate() {
            let Some(node) = doc.get(entry_id) else {
                continue;
            };
            let local = placements[i];
            let world = map_local_to_world(&root.world, &local);
            let child_stable = build_call_frame(
                doc,
                &tree,
                node,
                Some(root_stable.clone()),
                local,
                world,
                &mut frames,
            );
            root.children.push(child_stable);
        }

        // Portal strip: tree Calls + shared-callee cross edges.
        attach_call_portals(doc, &tree, &mut frames);

        frames.insert(root_stable.clone(), root);

        Scene {
            root: root_stable,
            canvas: CanvasSize {
                w: CANVAS_W,
                h: CANVAS_H,
            },
            frames,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scene serializes")
    }
}

fn crate_roots(doc: &Document) -> Vec<&Node> {
    let mut crates: Vec<&Node> = doc
        .nodes
        .values()
        .filter(|node| {
            matches!(node.kind.as_str(), kind::code::CRATE | kind::code::PACKAGE)
                && node.parent.is_none()
        })
        .collect();
    if crates.is_empty() {
        crates = doc
            .roots
            .iter()
            .filter_map(|id| doc.get(id))
            .filter(|node| is_structural(&node.kind))
            .collect();
    }
    crates.sort_by(|a, b| a.fqn.cmp(&b.fqn));
    crates
}

fn build_frame(
    doc: &Document,
    node: &Node,
    parent: Option<String>,
    local_in_parent: Rect,
    world: WorldRect,
    frames: &mut BTreeMap<String, Frame>,
) -> String {
    let sid = stable_id(node.id.as_str());

    let visible: Vec<&Node> = node
        .children
        .iter()
        .filter_map(|id| doc.get(id))
        .filter(|child| is_structural(&child.kind) && !is_scene_skipped(child.kind.as_str()))
        .collect();

    let sibling = SiblingGraph::from_root(doc, &node.id);
    let edges: Vec<SceneEdge> = sibling
        .edges
        .iter()
        .map(|e| SceneEdge {
            from: e.from.clone(),
            to: e.to.clone(),
            label: e.label.to_owned(),
            resolved: e.resolved,
        })
        .collect();

    let portals = portals_for(doc, node, &visible);

    let mut frame = Frame {
        id: node.id.as_str().to_owned(),
        stable_id: sid.clone(),
        name: node.name.clone(),
        fqn: node.fqn.clone(),
        kind_tag: short_kind(&node.kind).to_owned(),
        parent,
        children: Vec::new(),
        world,
        local_in_parent: Some(local_in_parent),
        edges,
        portals,
    };

    let placements = layout_grid(visible.len());
    for (i, child) in visible.iter().enumerate() {
        let local = placements[i];
        let child_world = map_local_to_world(&world, &local);
        let child_sid = build_frame(doc, child, Some(sid.clone()), local, child_world, frames);
        frame.children.push(child_sid);
    }

    frames.insert(sid.clone(), frame);
    sid
}

fn build_call_frame(
    doc: &Document,
    tree: &CallTree,
    node: &Node,
    parent: Option<String>,
    local_in_parent: Rect,
    world: WorldRect,
    frames: &mut BTreeMap<String, Frame>,
) -> String {
    let sid = stable_id(node.id.as_str());
    let child_ids = tree.children_of(&node.id);
    let visible: Vec<&Node> = child_ids.iter().filter_map(|id| doc.get(id)).collect();
    let child_id_set: BTreeSet<&NodeId> = visible.iter().map(|n| &n.id).collect();

    let edges: Vec<SceneEdge> = node
        .edges
        .iter()
        .filter(|e| e.kind == robby_ir::EdgeKind::Calls && e.resolved)
        .filter(|e| child_id_set.contains(&e.to))
        .filter_map(|e| {
            let target = doc.get(&e.to)?;
            Some(SceneEdge {
                from: node.name.clone(),
                to: target.name.clone(),
                label: edge_label(e.kind).to_owned(),
                resolved: e.resolved,
            })
        })
        .collect();

    let mut frame = Frame {
        id: node.id.as_str().to_owned(),
        stable_id: sid.clone(),
        // Entrypoints org: many leaves share names like `main` — title with path.
        name: node.fqn.clone(),
        fqn: node.fqn.clone(),
        kind_tag: short_kind(&node.kind).to_owned(),
        parent,
        children: Vec::new(),
        world,
        local_in_parent: Some(local_in_parent),
        edges,
        portals: Vec::new(),
    };

    let placements = layout_grid(visible.len());
    for (i, child) in visible.iter().enumerate() {
        let local = placements[i];
        let child_world = map_local_to_world(&world, &local);
        let child_sid = build_call_frame(
            doc,
            tree,
            child,
            Some(sid.clone()),
            local,
            child_world,
            frames,
        );
        frame.children.push(child_sid);
    }

    frames.insert(sid.clone(), frame);
    sid
}

/// Portals for every Calls hop among placed frames (tree children + cross edges).
fn attach_call_portals(doc: &Document, tree: &CallTree, frames: &mut BTreeMap<String, Frame>) {
    let mut pairs: Vec<(NodeId, NodeId)> = Vec::new();
    for (from_id, kids) in &tree.children {
        for to_id in kids {
            pairs.push((from_id.clone(), to_id.clone()));
        }
    }
    pairs.extend(tree.cross_edges.iter().cloned());
    pairs.sort_by(|a, b| {
        a.0.as_str()
            .cmp(b.0.as_str())
            .then_with(|| a.1.as_str().cmp(b.1.as_str()))
    });
    pairs.dedup();

    for (from_id, to_id) in &pairs {
        let from_sid = stable_id(from_id.as_str());
        let to_sid = stable_id(to_id.as_str());
        if !frames.contains_key(&from_sid) || !frames.contains_key(&to_sid) {
            continue;
        }
        let to_name = doc
            .get(to_id)
            .map_or_else(|| to_id.as_str().to_owned(), |n| n.fqn.clone());
        let from_name = doc
            .get(from_id)
            .map_or_else(|| from_id.as_str().to_owned(), |n| n.fqn.clone());

        if let Some(from_frame) = frames.get_mut(&from_sid) {
            from_frame.portals.push(Portal {
                label: to_name.clone(),
                kind_label: "calls".to_owned(),
                target: Some(to_sid.clone()),
                target_name: to_name,
            });
        }
        if let Some(to_frame) = frames.get_mut(&to_sid) {
            to_frame.portals.push(Portal {
                label: format!("from {from_name}"),
                kind_label: "called".to_owned(),
                target: Some(from_sid),
                target_name: from_name,
            });
        }
    }

    for frame in frames.values_mut() {
        frame.portals.sort_by(|a, b| {
            (
                a.kind_label.as_str(),
                a.target_name.as_str(),
                a.label.as_str(),
            )
                .cmp(&(
                    b.kind_label.as_str(),
                    b.target_name.as_str(),
                    b.label.as_str(),
                ))
        });
        frame.portals.dedup_by(|a, b| {
            a.label == b.label && a.kind_label == b.kind_label && a.target == b.target
        });
    }
}

fn portals_for(doc: &Document, node: &Node, siblings: &[&Node]) -> Vec<Portal> {
    let mut portals = Vec::new();
    let sibling_ids: BTreeSet<&NodeId> = siblings.iter().map(|n| &n.id).collect();

    let links = LinkGraph::for_node(doc, node);
    for outbound in &links.outbound {
        portals.push(Portal {
            label: outbound.module_fqn.clone(),
            kind_label: outbound.kind_label.to_owned(),
            target: Some(stable_id(outbound.module_id.as_str())),
            target_name: outbound.module_fqn.clone(),
        });
    }
    for inbound in &links.inbound {
        portals.push(Portal {
            label: format!("from {}", inbound.caller_fqn),
            kind_label: inbound.kinds.join(","),
            target: Some(stable_id(inbound.caller_id.as_str())),
            target_name: inbound.caller_fqn.clone(),
        });
    }

    // Unresolved edges leaving this frame: visible, no zoom target.
    let scope: BTreeSet<NodeId> = doc
        .subtree(&node.id)
        .into_iter()
        .map(|n| n.id.clone())
        .collect();

    let mut seen_unresolved = BTreeSet::new();
    for n in doc.subtree(&node.id) {
        for edge in &n.edges {
            if scope.contains(&edge.to) {
                continue;
            }
            if sibling_ids.contains(&edge.to) {
                continue;
            }
            let missing = doc.get(&edge.to).is_none();
            if edge.resolved && !missing {
                continue;
            }
            let key = (edge.to.as_str().to_owned(), edge.kind);
            if !seen_unresolved.insert(key) {
                continue;
            }
            portals.push(Portal {
                label: edge.to.as_str().to_owned(),
                kind_label: edge_label(edge.kind).to_owned(),
                target: None,
                target_name: edge.to.as_str().to_owned(),
            });
        }
    }

    portals.sort_by(|a, b| {
        (
            a.kind_label.as_str(),
            a.target_name.as_str(),
            a.label.as_str(),
        )
            .cmp(&(
                b.kind_label.as_str(),
                b.target_name.as_str(),
                b.label.as_str(),
            ))
    });
    portals.dedup_by(|a, b| {
        a.label == b.label && a.kind_label == b.kind_label && a.target == b.target
    });
    portals
}

/// Place `n` children in a grid on the local canvas (below the header, above portals).
fn layout_grid(n: usize) -> Vec<Rect> {
    if n == 0 {
        return Vec::new();
    }
    let cols = (n as f64).sqrt().ceil() as usize;
    let cols = cols.max(1);
    let rows = n.div_ceil(cols);

    let available_w = CANVAS_W - 2 * PAD;
    let available_h = CANVAS_H - HEADER - PORTAL_STRIP - PAD;
    let cell_w = available_w / cols as i64;
    let cell_h = available_h / rows as i64;

    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let col = (i % cols) as i64;
        let row = (i / cols) as i64;
        let fit_w = cell_w - 2 * CELL_MARGIN;
        let fit_h = cell_h - 2 * CELL_MARGIN;
        let scale_num = fit_w.min(fit_h);
        let scale_den = CANVAS_W.max(CANVAS_H);
        // Fit the child canvas into the cell, preserving aspect ratio.
        let child_w = (CANVAS_W * scale_num) / scale_den;
        let child_h = (CANVAS_H * scale_num) / scale_den;
        let x = PAD + col * cell_w + (cell_w - child_w) / 2;
        let y = HEADER + row * cell_h + (cell_h - child_h) / 2;
        out.push(Rect::new(x, y, child_w.max(1), child_h.max(1)));
    }
    out
}

/// Comments stay in the IR and markdown tree, but are not Prezi frames.
fn is_scene_skipped(kind_name: &str) -> bool {
    kind_name == kind::code::COMMENT
}

fn map_local_to_world(parent_world: &WorldRect, local: &Rect) -> WorldRect {
    let pw = parent_world.w / CANVAS_W as f64;
    let ph = parent_world.h / CANVAS_H as f64;
    WorldRect::new(
        parent_world.x + local.x as f64 * pw,
        parent_world.y + local.y as f64 * ph,
        local.w as f64 * pw,
        local.h as f64 * ph,
    )
}
