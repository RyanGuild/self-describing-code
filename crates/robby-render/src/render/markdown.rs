//! Markdown renderer: fenced Mermaid diagrams + DSM table + link lists.
//!
//! A full docs page is a [`Report`](crate::report::Report) built by the page
//! assembler (`tree`) from these rendered views plus prose/tables.

use std::fmt::Write as _;

use crate::view::{
    CommentsView, CouplingGraph, ExternalsView, HierarchyView, LinkGraph, NodeMeta,
    PackageStructure, ProcessView, SiblingGraph, SourceSlice, TypeModel,
};

use super::Renderer;
use super::mermaid::{self, MermaidRenderer};

#[derive(Debug, Default, Clone, Copy)]
pub struct MarkdownRenderer {
    mermaid: MermaidRenderer,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    fn fence(lang: &str, body: &str) -> String {
        if body.trim().is_empty() {
            return String::new();
        }
        let mut out = String::from("```");
        out.push_str(lang);
        out.push('\n');
        out.push_str(body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
        out.push_str("```\n");
        out
    }
}

impl Renderer for MarkdownRenderer {
    fn meta(&self, meta: &NodeMeta) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "| | |");
        let _ = writeln!(out, "|---|---|");
        let _ = writeln!(out, "| kind | `{}` |", meta.kind);
        if let Some(sig) = &meta.signature {
            let _ = writeln!(out, "| signature | `{}` |", cell(sig));
        }
        if let Some(location) = &meta.location {
            let _ = writeln!(out, "| location | `{location}` |");
        }
        let _ = writeln!(
            out,
            "| provenance | {} · confidence {:.2} |",
            meta.provenance_source, meta.confidence
        );
        let _ = writeln!(out, "| nodes | {} |", meta.node_count);
        out.push('\n');
        if let Some(docs) = &meta.docs {
            for line in docs.lines() {
                let _ = writeln!(out, "> {line}");
            }
            out.push('\n');
        }
        out
    }

    fn process(&self, view: &ProcessView) -> String {
        Self::fence("mermaid", &self.mermaid.process(view))
    }

    fn types(&self, view: &TypeModel) -> String {
        Self::fence("mermaid", &self.mermaid.types(view))
    }

    fn package_deps(&self, view: &PackageStructure) -> String {
        Self::fence("mermaid", &self.mermaid.package_deps(view))
    }

    fn coupling_deps(&self, view: &CouplingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.coupling_deps(view))
    }

    fn dependency_matrix(&self, view: &PackageStructure) -> String {
        mermaid::dsm_markdown_table(view)
    }

    fn hierarchy(&self, _view: &HierarchyView) -> String {
        String::new()
    }

    fn externals(&self, view: &ExternalsView) -> String {
        let mut out = String::new();
        for entry in &view.entries {
            let importers = entry.importers.join(", ");
            let label = if entry.spec == entry.package {
                format!("`{}`", entry.package)
            } else {
                format!("`{}` (`{}`)", entry.package, entry.spec)
            };
            let _ = writeln!(out, "- {label} — imported by `{importers}`");
        }
        out
    }

    fn comments(&self, view: &CommentsView) -> String {
        use crate::view::apply_highlights;

        let mut out = String::new();
        for entry in &view.entries {
            let loc = entry.location.as_deref().unwrap_or("?");
            if let (Some(note_kind), Some(note)) = (entry.note_kind, entry.note.as_ref()) {
                let _ = writeln!(out, "- `{loc}` — **{}**", note_kind.label());
                for line in note.lines() {
                    let _ = writeln!(out, "  > {line}");
                }
                if let Some(code) = &entry.code {
                    let marked = apply_highlights(code, &entry.highlights);
                    out.push_str("  ```\n");
                    for line in marked.trim_end().lines() {
                        let _ = writeln!(out, "  {line}");
                    }
                    out.push_str("  ```\n");
                }
                out.push('\n');
                continue;
            }
            let kind = entry.kind.as_str();
            let text = trim_comment(&entry.text, 120);
            if let Some(owner) = &entry.owner_fqn {
                let _ = writeln!(out, "- `{loc}` ({kind}, `{owner}`): {text}");
            } else {
                let _ = writeln!(out, "- `{loc}` ({kind}): {text}");
            }
            if let Some(code) = &entry.code {
                let marked = apply_highlights(code, &entry.highlights);
                out.push_str("  ```\n");
                for line in marked.trim_end().lines() {
                    let _ = writeln!(out, "  {line}");
                }
                out.push_str("  ```\n");
            }
        }
        out
    }

    fn links(&self, view: &LinkGraph) -> String {
        let mut out = String::new();
        if !view.outbound.is_empty() {
            out.push_str("## References\n\n");
            for link in &view.outbound {
                let detail = if link.items.len() <= 4 {
                    format!(" — {}", link.items.join(", "))
                } else {
                    format!(" — {} items", link.items.len())
                };
                let _ = writeln!(out, "- {} → `{}`{detail}", link.kind_label, link.module_fqn);
            }
            out.push('\n');
        }
        if !view.inbound.is_empty() {
            out.push_str("## Referenced by\n\n");
            for link in &view.inbound {
                let kinds = link.kinds.join(", ");
                let _ = writeln!(out, "- {kinds} ← `{}`", link.caller_fqn);
            }
            out.push('\n');
        }
        out
    }

    fn sibling_graph(&self, view: &SiblingGraph) -> String {
        Self::fence("mermaid", &self.mermaid.sibling_graph(view))
    }

    fn source(&self, view: &SourceSlice) -> String {
        let mut out = String::from("```rust\n");
        out.push_str(view.text.trim_end());
        out.push_str("\n```\n");
        out
    }
}

fn cell(input: &str) -> String {
    input.replace('|', "\\|").replace('\n', " ")
}

fn trim_comment(text: &str, max: usize) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max {
        return format!("\"{flat}\"");
    }
    let truncated: String = flat.chars().take(max.saturating_sub(1)).collect();
    format!("\"{truncated}…\"")
}
