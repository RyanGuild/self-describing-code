//! Type-level contracts for IR and pipeline invariants.
//!
//! These traits make illegal merges and unchecked writes harder without closing
//! open kinds or forcing a schema bump. See Project
//! `docs/traits-and-crate-splits.md`.

use crate::coverage::{CoverageReport, FileCoverage, Gap};
use crate::kind::Kind;
use crate::process::Process;

/// Merge an observed value into a static (or earlier) one without dropping prior
/// structure.
///
/// For [`Process`], matching is by `(from, to, label)`: observed steps mark
/// matches; unmatched observed steps append; static-only steps stay dashed.
pub trait Absorb {
    fn absorb(&mut self, observed: &Self);
}

impl Absorb for Process {
    fn absorb(&mut self, observed: &Self) {
        for p in &observed.participants {
            if !self.participants.iter().any(|q| q.id == p.id) {
                self.participants.push(p.clone());
            }
        }

        let mut matched = vec![false; observed.steps.len()];
        for step in &mut self.steps {
            if let Some(idx) = observed
                .steps
                .iter()
                .position(|o| o.from == step.from && o.to == step.to && o.label == step.label)
            {
                step.observed = true;
                matched[idx] = true;
            }
        }
        for (idx, step) in observed.steps.iter().enumerate() {
            if !matched[idx] {
                self.steps.push(step.clone());
            }
        }
    }
}

/// Open-kind namespace queries. Unknown namespaces must still round-trip.
pub trait KindNamespace {
    fn namespace(&self) -> &str;

    fn is_code(&self) -> bool {
        self.namespace() == "code"
    }

    fn is_runtime(&self) -> bool {
        self.namespace() == "runtime"
    }

    fn is_agent(&self) -> bool {
        self.namespace() == "agent"
    }

    /// `code` | `runtime` | `agent` — soft gap otherwise, never a hard error.
    fn is_known_namespace(&self) -> bool {
        matches!(self.namespace(), "code" | "runtime" | "agent")
    }
}

impl KindNamespace for Kind {
    fn namespace(&self) -> &str {
        Kind::namespace(self)
    }
}

/// Coverage completeness gate: unknown gaps fail; trivia does not.
pub trait CoverageGate {
    fn passes(&self) -> bool;

    fn failing_gaps(&self) -> Vec<(&FileCoverage, &Gap)>;
}

impl CoverageGate for CoverageReport {
    fn passes(&self) -> bool {
        self.is_complete()
    }

    fn failing_gaps(&self) -> Vec<(&FileCoverage, &Gap)> {
        self.failures().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kind;
    use crate::process::{Step, StepKind};

    #[test]
    fn absorb_trait_marks_matching_steps_observed() {
        let mut static_proc = Process::new();
        static_proc.participant("a", "A", kind::Kind::new(kind::code::FUNCTION));
        static_proc.participant("b", "B", kind::Kind::new(kind::code::FUNCTION));
        static_proc.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "call".into(),
            kind: StepKind::Call,
            observed: false,
            span: None,
            blocks: Vec::new(),
        });

        let mut observed = Process::new();
        observed.participant("a", "A", kind::Kind::new(kind::code::FUNCTION));
        observed.participant("b", "B", kind::Kind::new(kind::code::FUNCTION));
        observed.step(Step {
            from: "a".into(),
            to: "b".into(),
            label: "call".into(),
            kind: StepKind::Call,
            observed: true,
            span: None,
            blocks: Vec::new(),
        });

        Absorb::absorb(&mut static_proc, &observed);
        assert!(static_proc.steps[0].observed);
        assert_eq!(static_proc.steps.len(), 1);
    }

    #[test]
    fn kind_namespace_helpers() {
        let code = Kind::new(kind::code::FUNCTION);
        assert!(code.is_code());
        assert!(code.is_known_namespace());
        assert!(!code.is_agent());

        let weird = Kind::new("plugin.widget");
        assert!(!weird.is_known_namespace());
        assert_eq!(weird.namespace(), "plugin");
    }
}
