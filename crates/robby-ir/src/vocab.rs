//! Closed vocabularies stored in the document.
//!
//! Agent enrichment writes these onto `agent.decision` nodes. Views read them
//! back. The string form is the serde name; producers and views share that
//! one mapping.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Relevant,
    Noise,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Relevant => "relevant",
            Self::Noise => "noise",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "relevant" => Some(Self::Relevant),
            "noise" => Some(Self::Noise),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NoteKind {
    UsageExample,
    ImplementationNote,
}

impl NoteKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsageExample => "usage_example",
            Self::ImplementationNote => "implementation_note",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "usage_example" => Some(Self::UsageExample),
            "implementation_note" => Some(Self::ImplementationNote),
            _ => None,
        }
    }

    /// Prose label for Markdown and the terminal report.
    pub fn label(self) -> &'static str {
        match self {
            Self::UsageExample => "usage example",
            Self::ImplementationNote => "implementation note",
        }
    }
}

/// Byte range into a code window. Offsets are UTF-8 bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Highlight {
    pub start: u32,
    pub end: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wire<T: serde::Serialize>(value: T) -> String {
        serde_json::to_value(value)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned()
    }

    #[test]
    fn names_match_serde() {
        assert_eq!(wire(Verdict::Relevant), Verdict::Relevant.as_str());
        assert_eq!(wire(Verdict::Noise), Verdict::Noise.as_str());
        assert_eq!(
            wire(NoteKind::UsageExample),
            NoteKind::UsageExample.as_str()
        );
        assert_eq!(
            wire(NoteKind::ImplementationNote),
            NoteKind::ImplementationNote.as_str()
        );
        assert_eq!(Verdict::parse("relevant"), Some(Verdict::Relevant));
        assert_eq!(NoteKind::parse("nope"), None);
    }
}
