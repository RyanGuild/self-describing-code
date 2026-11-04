//! A report is many rendered views, statically assembled into an artifact.
//!
//! Views are data. Renderers turn one view into text. A [`Report`] collects
//! those rendered sections (plus optional prose) into a single byte-stable
//! artifact: a Markdown page, a terminal dump, or a `.mmd` file.

use std::fmt::Write as _;

/// One titled block inside a report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: Option<String>,
    pub body: String,
}

impl Section {
    pub fn untitled(body: impl Into<String>) -> Self {
        Self {
            title: None,
            body: body.into(),
        }
    }

    pub fn titled(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            body: body.into(),
        }
    }

    pub fn is_blank(&self) -> bool {
        self.body.trim().is_empty()
    }
}

/// Static artifact built from rendered views.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report {
    pub sections: Vec<Section>,
}

impl Report {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, section: Section) {
        if !section.is_blank() {
            self.sections.push(section);
        }
    }

    pub fn push_body(&mut self, body: impl Into<String>) {
        self.push(Section::untitled(body));
    }

    pub fn push_titled(&mut self, title: impl Into<String>, body: impl Into<String>) {
        self.push(Section::titled(title, body));
    }

    /// Markdown artifact: optional `##` headings, bodies as-is (callers fence diagrams).
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        for (index, section) in self.sections.iter().enumerate() {
            if index > 0 && !out.ends_with("\n\n") {
                if out.ends_with('\n') {
                    out.push('\n');
                } else {
                    out.push_str("\n\n");
                }
            }
            if let Some(title) = &section.title {
                let _ = writeln!(out, "## {title}\n");
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }

    /// Terminal artifact: rule-separated titled sections.
    pub fn to_terminal(&self, width: usize) -> String {
        let rule = "─".repeat(width.clamp(24, 78));
        let mut out = String::new();
        for section in &self.sections {
            if let Some(title) = &section.title {
                if !out.is_empty() {
                    out.push('\n');
                }
                let _ = writeln!(out, "{title}\n{rule}");
            } else if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }

    /// Raw concatenation (e.g. a single `.mmd` diagram body).
    pub fn to_plain(&self) -> String {
        let mut out = String::new();
        for section in &self.sections {
            out.push_str(&section.body);
            if !section.body.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
}
