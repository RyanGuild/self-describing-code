//! Renderers: View → String. A [`Report`](crate::report::Report) assembles many
//! rendered views into one static artifact.

mod markdown;
mod mermaid;
pub mod tui;

pub use markdown::MarkdownRenderer;
pub use mermaid::MermaidRenderer;
pub use tui::{ShowOptions, TuiRenderer};

use crate::report::{Report, Section};
use crate::view::{
    CommentsView, CouplingGraph, DependencyViews, ExternalsView, HierarchyView, LinkGraph, NodeMeta,
    NodeViews, PackageStructure, ProcessView, SiblingGraph, SourceSlice, TypeModel,
};

/// Transform one view into text.
///
/// Callers that need a full artifact assemble many of these into a [`Report`].
pub trait Renderer {
    fn meta(&self, meta: &NodeMeta) -> String;
    fn process(&self, view: &ProcessView) -> String;
    fn types(&self, view: &TypeModel) -> String;
    /// Nested source-structure package digraph + externals.
    fn package_deps(&self, view: &PackageStructure) -> String;
    /// Internal call coupling (modules as subgraphs, functions as nodes).
    fn coupling_deps(&self, view: &CouplingGraph) -> String;
    /// Design / dependency structure matrix over packages.
    fn dependency_matrix(&self, view: &PackageStructure) -> String;
    fn hierarchy(&self, view: &HierarchyView) -> String;
    fn externals(&self, view: &ExternalsView) -> String;
    fn comments(&self, view: &CommentsView) -> String;
    fn links(&self, view: &LinkGraph) -> String;
    fn sibling_graph(&self, view: &SiblingGraph) -> String;
    fn source(&self, view: &SourceSlice) -> String;

    /// Dependency formalisms as report sections (skips empty bodies).
    fn dependency_sections(&self, views: &DependencyViews) -> Vec<Section> {
        let mut sections = Vec::new();
        if !views.packages.is_empty() {
            let body = self.package_deps(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Package diagram", body));
            }
        }
        if !views.coupling.is_empty() {
            let body = self.coupling_deps(&views.coupling);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Module coupling", body));
            }
        }
        if !views.packages.is_empty() {
            let body = self.dependency_matrix(&views.packages);
            if !body.trim().is_empty() {
                sections.push(Section::titled("Dependency matrix", body));
            }
        }
        sections
    }

    /// Build a report from the views selected for one node.
    fn report(&self, views: &NodeViews) -> Report {
        let mut report = Report::new();
        let meta = self.meta(&views.meta);
        if !meta.trim().is_empty() {
            report.push_body(meta);
        }
        if let Some(entry) = &views.entry {
            report.push(Section::titled("Entry", self.process(entry)));
        }
        if let Some(types) = &views.types {
            report.push(Section::titled("Types", self.types(types)));
        }
        if let Some(deps) = &views.dependencies {
            for section in self.dependency_sections(deps) {
                report.push(section);
            }
        }
        if let Some(externals) = &views.externals {
            let body = self.externals(externals);
            if !body.trim().is_empty() {
                report.push(Section::titled("Externals", body));
            }
        }
        if let Some(comments) = &views.comments {
            let body = self.comments(comments);
            if !body.trim().is_empty() {
                report.push(Section::titled("Comments", body));
            }
        }
        if let Some(process) = &views.process {
            report.push(Section::titled("Process", self.process(process)));
        }
        if let Some(hierarchy) = &views.hierarchy {
            report.push(Section::titled("Tree", self.hierarchy(hierarchy)));
        }
        if !views.links.is_empty() {
            report.push(Section::titled("Links", self.links(&views.links)));
        }
        if let Some(sibling) = &views.sibling {
            report.push(Section::titled("Graph", self.sibling_graph(sibling)));
        }
        if let Some(source) = &views.source {
            report.push(Section::titled("Source", self.source(source)));
        }
        report
    }
}
