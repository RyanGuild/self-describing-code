//! Format-agnostic projections of the IR.
//!
//! Views hold graph/process/type data. Renderers turn them into Mermaid, Markdown,
//! or terminal text. Selection policy lives in [`collect`].

mod comments;
mod deps;
mod entrypoints;
mod execution;
mod externals;
mod hierarchy;
mod links;
mod process;
mod scene;
mod select;
mod sibling;
mod source;
mod types;

pub use comments::{
    CommentEntry, CommentKind, CommentsView, NoteKind, Verdict, apply_highlights,
};
pub use deps::{
    CouplingFn, CouplingGraph, CouplingModule, DepModule, DependencyViews, PackageNode,
    PackageStructure, WeightedEdge,
};
pub use entrypoints::{CallTree, MAX_CALL_DEPTH};
pub use execution::{execution_process, from_code_path as execution_from_code_path};
pub use robby_ir::find_entrypoints;
pub use externals::{ExternalEntry, ExternalsView};
pub use hierarchy::{HierarchyNode, HierarchyView};
pub use links::{InboundRef, LinkGraph, OutboundRef};
pub use process::{ParticipantView, ProcessEvent, ProcessView};
pub use scene::{
    CANVAS_H, CANVAS_W, CanvasSize, Frame, Portal, Rect, Scene, SceneEdge, WORKSPACE_ID, WorldRect,
};
pub use select::{NodeMeta, NodeViews, SelectOpts, collect, is_binary_module, main_process};
pub use sibling::{SiblingEdge, SiblingGraph, SiblingNode};
pub use source::SourceSlice;
pub use types::{
    ClassMember, ClassStereotype, ClassView, RelationKind, TypeModel, TypeRelation,
};
