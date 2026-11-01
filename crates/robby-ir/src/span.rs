//! Source positions and spans.
//!
//! Byte offsets are the load-bearing field. Line and column are for humans and
//! for the source viewer; the coverage partition is computed entirely in bytes
//! so it is independent of encoding assumptions and newline style.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A single point in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column, counted in characters.
    pub column: u32,
    /// 0-based byte offset into the file.
    pub byte: u32,
}

impl Position {
    pub fn new(line: u32, column: u32, byte: u32) -> Self {
        Self { line, column, byte }
    }

    /// Resolve a byte offset to a line/column position.
    ///
    /// Out-of-range offsets clamp to the end of the source rather than panicking:
    /// a slightly wrong span must never take down the whole render.
    pub fn at(source: &str, byte: u32) -> Self {
        let byte = byte.min(source.len() as u32);
        let prefix = &source[..byte as usize];
        let line = prefix.matches('\n').count() as u32 + 1;
        let line_start = prefix.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = prefix[line_start..].chars().count() as u32 + 1;
        Self { line, column, byte }
    }
}

/// A half-open byte range within one file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Span {
    /// Repository-relative path, always with forward slashes.
    pub file: String,
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(file: impl Into<String>, source: &str, start: u32, end: u32) -> Self {
        let end = end.max(start).min(source.len() as u32);
        Self {
            file: file.into(),
            start: Position::at(source, start),
            end: Position::at(source, end),
        }
    }

    pub fn byte_range(&self) -> (u32, u32) {
        (self.start.byte, self.end.byte)
    }

    pub fn is_empty(&self) -> bool {
        self.start.byte >= self.end.byte
    }

    /// `file:line:column`, suitable for editor links.
    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.file, self.start.line, self.start.column)
    }
}
