//! Terminal renderer: ASCII sections assembled into a report artifact.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use ascii_dag::{Direction, Graph, RenderOptions};
use unicode_width::UnicodeWidthStr;

use crate::view::{
    ClassMember, ClassStereotype, CommentsView, CouplingGraph, CouplingModule, ExternalsView,
    HierarchyNode, HierarchyView, LinkGraph, NodeMeta, NodeViews, PackageNode, PackageStructure,
    ProcessEvent, ProcessView, SelectOpts, SiblingGraph, SourceSlice, TypeModel, collect,
};
use robby_ir::{Document, Node};

use super::Renderer;

/// Options for one `robby show` invocation.
#[derive(Debug, Clone)]
pub struct ShowOptions {
    pub depth: usize,
    pub width: usize,
    pub all: bool,
    pub tree: bool,
    pub graph: bool,
    pub process: bool,
    pub source: bool,
    pub entry: bool,
    pub types: bool,
    pub dependencies: bool,
    pub externals: bool,
    pub comments: bool,
    pub references: bool,
}

impl Default for ShowOptions {
    fn default() -> Self {
        Self {
            depth: 4,
            width: 96,
            all: false,
            tree: true,
            graph: false,
            process: true,
            source: true,
            entry: true,
            types: true,
            dependencies: true,
            externals: true,
            comments: true,
            references: true,
        }
    }
}

impl ShowOptions {
    pub fn select_opts(&self) -> SelectOpts {
        SelectOpts {
            hierarchy: self.tree,
            hierarchy_depth: self.depth,
            hierarchy_all: self.all,
            sibling: self.graph,
            source: self.source,
            types: self.types,
            dependencies: self.dependencies,
            externals: self.externals,
            comments: self.comments,
            links: self.references,
            entry: self.entry,
            process: self.process,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TuiRenderer {
    pub width: usize,
}

impl Default for TuiRenderer {
    fn default() -> Self {
        Self { width: 96 }
    }
}

impl TuiRenderer {
    pub fn new(width: usize) -> Self {
        Self { width }
    }

    /// Collect views for a node and render a terminal report artifact.
    pub fn show(
        &self,
        doc: &Document,
        node: &Node,
        sources: Option<&BTreeMap<String, String>>,
        opts: &ShowOptions,
    ) -> String {
        let views = collect(doc, node, sources, &opts.select_opts());
        self.render_report(&views)
    }

    pub fn render_report(&self, views: &NodeViews) -> String {
        self.report(views).to_terminal(self.width)
    }
}

impl Renderer for TuiRenderer {
    fn meta(&self, meta: &NodeMeta) -> String {
        let mut out = String::new();
        let rule = "─".repeat(self.width.clamp(24, 78));
        let _ = writeln!(out, "{}", meta.fqn);
        let _ = writeln!(out, "{rule}");
        let _ = writeln!(out, "kind        {}", meta.kind);
        if let Some(signature) = &meta.signature {
            let _ = writeln!(
                out,
                "signature   {}",
                clip(signature, self.width.saturating_sub(12))
            );
        }
        if let Some(location) = &meta.location {
            let _ = writeln!(out, "location    {location}");
        }
        let _ = writeln!(
            out,
            "provenance  {} · confidence {:.2}",
            meta.provenance_source, meta.confidence
        );
        let _ = writeln!(out, "nodes       {}", meta.node_count);
        if let Some(docs) = &meta.docs {
            out.push('\n');
            for line in docs.lines() {
                let _ = writeln!(out, "  {line}");
            }
        }
        out
    }

    fn process(&self, view: &ProcessView) -> String {
        if view.is_empty() {
            return String::from("(no steps)\n");
        }

        let labels: BTreeMap<&str, &str> = view
            .participants
            .iter()
            .map(|p| (p.id.as_str(), p.label.as_str()))
            .collect();
        let name = |id: &str| clip(labels.get(id).copied().unwrap_or(id), 28);

        let mut from_width = 0;
        let mut to_width = 0;
        let mut label_width = 0;
        let mut frame_depth = 0usize;
        let mut max_depth = 0usize;

        for event in &view.events {
            match event {
                ProcessEvent::Begin { .. } => {
                    frame_depth += 1;
                    max_depth = max_depth.max(frame_depth);
                }
                ProcessEvent::End => {
                    frame_depth = frame_depth.saturating_sub(1);
                }
                ProcessEvent::Interaction {
                    from, to, label, ..
                } => {
                    from_width = from_width.max(UnicodeWidthStr::width(name(from).as_str()));
                    to_width = to_width.max(UnicodeWidthStr::width(name(to).as_str()));
                    label_width = label_width.max(UnicodeWidthStr::width(clip(label, 52).as_str()));
                }
                ProcessEvent::Else { .. } => {}
            }
        }

        let number_width = view.step_count.to_string().len();
        let gutter = " ".repeat(number_width + 2);
        let mut out = String::new();

        out.push_str("lifelines  ");
        for (index, participant) in view.participants.iter().enumerate() {
            if index > 0 {
                out.push_str(" · ");
            }
            out.push_str(&clip(&participant.label, 32));
        }
        out.push('\n');
        let _ = writeln!(
            out,
            "{gutter}{} steps · {} observed · {} possible only",
            view.step_count,
            view.observed_count,
            view.step_count - view.observed_count
        );
        out.push('\n');

        let mut depth = 0usize;
        for event in &view.events {
            match event {
                ProcessEvent::End => {
                    depth = depth.saturating_sub(1);
                    let guide = "│ ".repeat(depth);
                    let _ = writeln!(out, "{gutter}{guide}└─ end");
                }
                ProcessEvent::Else { label } => {
                    let guide = "│ ".repeat(depth.saturating_sub(1));
                    let _ = writeln!(out, "{gutter}{guide}├─ else {}", clip(label, 60));
                }
                ProcessEvent::Begin { keyword, label } => {
                    let guide = "│ ".repeat(depth);
                    let _ = writeln!(out, "{gutter}{guide}┌─ {keyword} {}", clip(label, 60));
                    depth += 1;
                }
                ProcessEvent::Interaction {
                    index,
                    from,
                    to,
                    label,
                    observed,
                    kind,
                } => {
                    let guide = "│ ".repeat(depth);
                    let arrow = if *observed { "──▶" } else { "╌╌▶" };
                    let _ = writeln!(
                        out,
                        "{:>number_width$}  {guide}{} {arrow} {}  {}  {}",
                        index,
                        pad(&name(from), from_width),
                        pad(&name(to), to_width),
                        pad(&clip(label, 52), label_width),
                        kind.as_str(),
                    );
                }
            }
        }

        let _ = max_depth;
        out
    }

    fn types(&self, view: &TypeModel) -> String {
        let mut out = String::new();
        for class in &view.classes {
            let stereo = match class.stereotype {
                Some(ClassStereotype::Interface) => " «interface»",
                Some(ClassStereotype::Enumeration) => " «enumeration»",
                None => "",
            };
            let _ = writeln!(out, "{}{stereo}", class.name);
            for member in &class.members {
                match member {
                    ClassMember::Field(name) => {
                        let _ = writeln!(out, "  + {name}");
                    }
                    ClassMember::Method(name) => {
                        let _ = writeln!(out, "  + {name}()");
                    }
                    ClassMember::Variant(name) => {
                        let _ = writeln!(out, "  · {name}");
                    }
                }
            }
            out.push('\n');
        }
        for rel in &view.relations {
            let (arrow, label) = match rel.kind {
                crate::view::RelationKind::Implements => ("..|>", "implements"),
                crate::view::RelationKind::Extends => ("--|>", "extends"),
            };
            let _ = writeln!(out, "{} {arrow} {}  {label}", rel.from_id, rel.to_id);
        }
        out
    }

    fn package_deps(&self, view: &PackageStructure) -> String {
        ascii_package_structure(view)
    }

    fn coupling_deps(&self, view: &CouplingGraph) -> String {
        ascii_coupling(view)
    }

    fn dependency_matrix(&self, view: &PackageStructure) -> String {
        crate::render::mermaid::dsm_markdown_table(view)
    }

    fn externals(&self, view: &ExternalsView) -> String {
        let mut out = String::new();
        for entry in &view.entries {
            let importers = entry.importers.join(", ");
            let label = if entry.spec == entry.package {
                entry.package.clone()
            } else {
                format!("{} ({})", entry.package, entry.spec)
            };
            let _ = writeln!(out, "  {label} — imported by {importers}");
        }
        out
    }

    fn comments(&self, view: &CommentsView) -> String {
        use crate::view::apply_highlights;

        let mut out = String::new();
        for entry in &view.entries {
            let loc = entry.location.as_deref().unwrap_or("?");
            if let (Some(note_kind), Some(note)) = (entry.note_kind, entry.note.as_ref()) {
                let _ = writeln!(out, "  {loc} — {}", note_kind.label());
                for line in note.lines() {
                    let _ = writeln!(out, "    {line}");
                }
                if let Some(code) = &entry.code {
                    let marked = apply_highlights(code, &entry.highlights);
                    for line in marked.trim_end().lines() {
                        let _ = writeln!(out, "    │ {line}");
                    }
                }
                out.push('\n');
                continue;
            }
            let kind = entry.kind.as_str();
            let text = clip(&entry.text.replace('\n', " "), 80);
            if let Some(owner) = &entry.owner_fqn {
                let _ = writeln!(out, "  {loc} ({kind}, {owner}): {text}");
            } else {
                let _ = writeln!(out, "  {loc} ({kind}): {text}");
            }
            if let Some(code) = &entry.code {
                let marked = apply_highlights(code, &entry.highlights);
                for line in marked.trim_end().lines().take(8) {
                    let _ = writeln!(out, "    │ {line}");
                }
            }
        }
        out
    }

    fn hierarchy(&self, view: &HierarchyView) -> String {
        fn render(node: &HierarchyNode, prefix: &str, is_last: bool, out: &mut String) {
            let branch = if prefix.is_empty() {
                ""
            } else if is_last {
                "└── "
            } else {
                "├── "
            };
            if node.kind_tag.is_empty() {
                let _ = writeln!(out, "{prefix}{branch}{}", node.name);
            } else {
                let _ = writeln!(out, "{prefix}{branch}{} [{}]", node.name, node.kind_tag);
            }
            let child_prefix = if prefix.is_empty() {
                String::new()
            } else if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            let count = node.children.len();
            for (index, child) in node.children.iter().enumerate() {
                render(child, &child_prefix, index + 1 == count, out);
            }
        }

        let mut out = String::new();
        render(&view.root, "", true, &mut out);
        out
    }

    fn links(&self, view: &LinkGraph) -> String {
        let mut out = String::new();
        if !view.outbound.is_empty() {
            out.push_str("references\n");
            for link in &view.outbound {
                let detail = if link.items.len() <= 4 {
                    format!(" — {}", link.items.join(", "))
                } else {
                    format!(" — {} items", link.items.len())
                };
                let _ = writeln!(out, "  {} → {}{detail}", link.kind_label, link.module_fqn);
            }
            out.push('\n');
        }
        if !view.inbound.is_empty() {
            out.push_str("referenced by\n");
            for link in &view.inbound {
                let kinds = link.kinds.join(", ");
                let _ = writeln!(out, "  {kinds} ← {}", link.caller_fqn);
            }
            out.push('\n');
        }
        out
    }

    fn sibling_graph(&self, view: &SiblingGraph) -> String {
        if view.nodes.is_empty() {
            return String::from("(no items to lay out)\n");
        }

        let labels: Vec<String> = view
            .nodes
            .iter()
            .map(|node| format!("{} [{}]", clip(&node.name, 22), node.kind_tag))
            .collect();

        let mut graph = Graph::new();
        for (index, label) in labels.iter().enumerate() {
            graph.add_node(index, label.as_str());
        }

        let index_of: BTreeMap<&str, usize> = view
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.stable_id.as_str(), index))
            .collect();

        for edge in &view.edges {
            let Some(&from) = index_of.get(edge.from.as_str()) else {
                continue;
            };
            let Some(&to) = index_of.get(edge.to.as_str()) else {
                continue;
            };
            let _ = graph.add_edge(from, to, Some(edge.label));
        }

        graph.set_direction(Direction::TopDown);
        let mut out = graph
            .compute_layout()
            .render_string(&RenderOptions::plain());
        if !out.ends_with('\n') {
            out.push('\n');
        }
        out
    }

    fn source(&self, view: &SourceSlice) -> String {
        let mut out = String::new();
        for line in view.text.trim_end().lines() {
            let _ = writeln!(out, "  {line}");
        }
        out.push('\n');
        out
    }
}

fn ascii_package_structure(view: &PackageStructure) -> String {
    fn walk(node: &PackageNode, prefix: &str, is_last: bool, out: &mut String) {
        let branch = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        let _ = writeln!(out, "{prefix}{branch}{}", node.name);
        let child_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };
        let count = node.children.len();
        for (index, child) in node.children.iter().enumerate() {
            walk(child, &child_prefix, index + 1 == count, out);
        }
    }
    if view.is_empty() {
        return String::from("(no package structure)\n");
    }
    let mut out = String::new();
    walk(&view.root, "", true, &mut out);
    if !view.externals.is_empty() {
        out.push_str("externals\n");
        for ext in &view.externals {
            let _ = writeln!(out, "  · {}", ext.name);
        }
    }
    if !view.edges.is_empty() {
        out.push_str("depends\n");
        for edge in &view.edges {
            let mark = if edge.in_cycle { " ⇄" } else { "" };
            let from = view
                .module_by_stable(&edge.from)
                .map_or(edge.from.as_str(), |m| m.name.as_str());
            let to = view
                .module_by_stable(&edge.to)
                .map_or(edge.to.as_str(), |m| m.name.as_str());
            let _ = writeln!(out, "  {from} → {to}{mark}");
        }
    }
    out
}

fn ascii_coupling(view: &CouplingGraph) -> String {
    fn walk(
        node: &CouplingModule,
        by_parent: &BTreeMap<&str, Vec<&str>>,
        prefix: &str,
        is_last: bool,
        out: &mut String,
    ) {
        let branch = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        let _ = writeln!(out, "{prefix}{branch}{}", node.name);
        let child_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };
        let fns = by_parent
            .get(node.stable_id.as_str())
            .cloned()
            .unwrap_or_default();
        let total = node.children.len() + fns.len();
        for (index, child) in node.children.iter().enumerate() {
            walk(child, by_parent, &child_prefix, index + 1 == total, out);
        }
        for (index, name) in fns.iter().enumerate() {
            let last = node.children.len() + index + 1 == total;
            let b = if last { "└── " } else { "├── " };
            let _ = writeln!(out, "{child_prefix}{b}{name}()");
        }
    }

    if view.is_empty() {
        return String::from("(no internal coupling)\n");
    }
    let leaves = view.leaf_nodes();
    let by_parent: BTreeMap<&str, Vec<&str>> = {
        let mut map: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for leaf in leaves {
            map.entry(leaf.parent_stable_id.as_str())
                .or_default()
                .push(leaf.name.as_str());
        }
        map
    };
    let mut out = String::new();
    walk(&view.root, &by_parent, "", true, &mut out);
    if !view.edges.is_empty() {
        out.push_str("calls\n");
        let name_of = |sid: &str| -> String {
            leaves
                .iter()
                .find(|f| f.stable_id == sid)
                .map_or_else(|| sid.to_owned(), |f| f.name.clone())
        };
        for edge in &view.edges {
            let mark = if edge.in_cycle { " ⇄" } else { "" };
            let weight = if edge.weight > 1 {
                format!(" ×{}", edge.weight)
            } else {
                String::new()
            };
            let _ = writeln!(
                out,
                "  {} → {}{weight}{mark}",
                name_of(&edge.from),
                name_of(&edge.to)
            );
        }
    }
    out
}

fn clip(text: &str, max: usize) -> String {
    if UnicodeWidthStr::width(text) <= max {
        return text.to_owned();
    }
    if max <= 1 {
        return "…".to_owned();
    }
    let mut out = String::new();
    let mut width = 0;
    for ch in text.chars() {
        let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if width + ch_width + 1 > max {
            break;
        }
        out.push(ch);
        width += ch_width;
    }
    out.push('…');
    out
}

fn pad(text: &str, width: usize) -> String {
    let current = UnicodeWidthStr::width(text);
    if current >= width {
        text.to_owned()
    } else {
        format!("{text}{}", " ".repeat(width - current))
    }
}
