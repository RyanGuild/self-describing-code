//! Source text slice for a node span.

use std::collections::BTreeMap;

use robby_ir::Node;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceSlice {
    pub file: String,
    pub text: String,
}

impl SourceSlice {
    pub fn from_node(node: &Node, sources: &BTreeMap<String, String>) -> Option<Self> {
        let span = node.span.as_ref()?;
        let source = sources.get(&span.file)?;
        let text = source
            .get(span.start.byte as usize..span.end.byte as usize)?
            .to_owned();
        Some(Self {
            file: span.file.clone(),
            text,
        })
    }
}
