//! Shared pure helpers used by views and renderers.

use robby_ir::{EdgeKind, Kind, Node, kind};

/// FNV-1a stable id for Mermaid / layout keys.
pub fn stable_id(input: &str) -> String {
    let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
    }
    format!("n_{hash:016x}")
}

/// Mermaid participant and node ids allow only a restricted alphabet.
pub fn alias(id: &str) -> String {
    let mut out = String::with_capacity(id.len());
    for ch in id.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, 'p');
    }
    if out.is_empty() {
        out.push_str("p_empty");
    }
    out
}

/// Sanitize text for a Mermaid label or message.
pub fn text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\n' | '\r' | '\t' | ';' | '#' => out.push(' '),
            '"' => out.push('\''),
            _ => out.push(ch),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn edge_label(kind: EdgeKind) -> &'static str {
    match kind {
        EdgeKind::Contains => "contains",
        EdgeKind::Calls => "calls",
        EdgeKind::Uses => "uses",
        EdgeKind::Imports => "imports",
        EdgeKind::Implements => "implements",
        EdgeKind::Extends => "extends",
        EdgeKind::Returns => "returns",
        EdgeKind::Field => "field",
        EdgeKind::References => "refs",
    }
}

pub fn short_kind(kind: &Kind) -> &str {
    kind.as_str().rsplit('.').next().unwrap_or(kind.as_str())
}

pub fn is_structural(kind: &Kind) -> bool {
    !matches!(
        kind.as_str(),
        kind::code::IMPORT
            | kind::code::FIELD
            | kind::code::VARIANT
            | kind::code::VERBATIM
            | kind::code::IMPL
    )
}

pub fn is_container(node: &Node) -> bool {
    matches!(
        node.kind.as_str(),
        kind::code::CRATE | kind::code::PACKAGE | kind::code::MODULE
    )
}

pub fn is_type(node: &Node) -> bool {
    matches!(
        node.kind.as_str(),
        kind::code::STRUCT
            | kind::code::CLASS
            | kind::code::ENUM
            | kind::code::TRAIT
            | kind::code::INTERFACE
            | kind::code::UNION
    )
}

pub fn strip_generics(name: &str) -> String {
    name.split('<').next().unwrap_or(name).trim().to_owned()
}

pub fn class_id(name: &str) -> String {
    alias(&strip_generics(name))
}

pub fn keyword(kind: robby_ir::BlockKind) -> &'static str {
    match kind {
        robby_ir::BlockKind::Alt => "alt",
        robby_ir::BlockKind::Opt => "opt",
        robby_ir::BlockKind::Loop => "loop",
        robby_ir::BlockKind::Par => "par",
        robby_ir::BlockKind::Break => "break",
    }
}
