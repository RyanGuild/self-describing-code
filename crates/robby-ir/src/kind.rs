//! Node kinds.
//!
//! A kind is a namespaced, open string rather than a closed enum. This is a
//! deliberate constraint: Robby must eventually describe things that are not
//! source code (runtime observations, agent decisions, tool calls) without a
//! schema version bump. The namespace carries the meaning:
//!
//! - `code.*`    — static structure parsed from source
//! - `runtime.*` — things observed while running
//! - `agent.*`   — reserved for Robby's own processes
//!
//! Unknown namespaces must always round-trip untouched.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A namespaced, open-ended kind tag.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(transparent)]
pub struct Kind(String);

impl Kind {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The namespace prefix, e.g. `code` in `code.function`.
    pub fn namespace(&self) -> &str {
        self.0.split('.').next().unwrap_or(&self.0)
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<&str> for Kind {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Static structure parsed from source.
pub mod code {
    pub const WORKSPACE: &str = "code.workspace";
    pub const CRATE: &str = "code.crate";
    /// npm / workspace package root (TypeScript and similar).
    pub const PACKAGE: &str = "code.package";
    pub const MODULE: &str = "code.module";
    pub const FILE: &str = "code.file";
    pub const STRUCT: &str = "code.struct";
    pub const CLASS: &str = "code.class";
    pub const ENUM: &str = "code.enum";
    pub const UNION: &str = "code.union";
    pub const TRAIT: &str = "code.trait";
    pub const INTERFACE: &str = "code.interface";
    pub const IMPL: &str = "code.impl";
    pub const FUNCTION: &str = "code.function";
    pub const METHOD: &str = "code.method";
    pub const CONST: &str = "code.const";
    pub const STATIC: &str = "code.static";
    pub const TYPE_ALIAS: &str = "code.type_alias";
    pub const MACRO: &str = "code.macro";
    pub const FIELD: &str = "code.field";
    pub const VARIANT: &str = "code.variant";
    pub const BLOCK: &str = "code.block";
    pub const STATEMENT: &str = "code.statement";
    pub const EXPRESSION: &str = "code.expression";
    pub const IMPORT: &str = "code.import";
    pub const COMMENT: &str = "code.comment";
    pub const EXTERN_CRATE: &str = "code.extern_crate";
    pub const FOREIGN_MOD: &str = "code.foreign_mod";
    pub const TRAIT_ALIAS: &str = "code.trait_alias";
    pub const VERBATIM: &str = "code.verbatim";
    pub const UNKNOWN: &str = "code.unknown";
}

/// Things observed while running.
pub mod runtime {
    pub const TRACE: &str = "runtime.trace";
    pub const SPAN: &str = "runtime.span";
    pub const EVENT: &str = "runtime.event";
}

/// Reserved for Robby's own processes. Nothing produces these yet.
pub mod agent {
    pub const DECISION: &str = "agent.decision";
    pub const ACTION: &str = "agent.action";
    pub const OBSERVATION: &str = "agent.observation";
}

/// True when `kind` is understood by the current version of the tool.
///
/// Unknown kinds are never an error — they are rendered generically and
/// reported as a soft gap, so an older viewer can still display a newer graph.
pub fn is_known(kind: &Kind) -> bool {
    matches!(kind.namespace(), "code" | "runtime" | "agent")
}
