//! Deterministic engineering release gates.
//! `UNKNOWN` is never promoted: missing evidence is a blocking condition.

mod report;
pub use report::GateReport;

use atc_core::DerivedState;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    Pass,
    Fail,
    Missing,
    Unknown,
}

impl EvidenceStatus {
    pub const fn is_blocking(self) -> bool {
        !matches!(self, Self::Pass)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Missing => "MISSING",
            Self::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecision {
    Allow,
    Block,
}

impl GateDecision {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "ALLOW",
            Self::Block => "BLOCK",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCheck {
    pub control_id: String,
    pub status: EvidenceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDuties {
    pub coder: String,
    pub validator: String,
    pub auditor: String,
    pub release_authority: String,
    pub human_approval: bool,
}

impl SeparationOfDuties {
    pub fn valid(&self) -> bool {
        !self.coder.is_empty()
            && !self.validator.is_empty()
            && !self.auditor.is_empty()
            && !self.release_authority.is_empty()
            && self.coder != self.validator
            && self.coder != self.auditor
            && self.coder != self.release_authority
            && self.validator != self.auditor
            && self.validator != self.release_authority
            && self.auditor != self.release_authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessInput {
    /// Every control required for this gate evaluation. This is an explicit
    /// caller-supplied contract; an empty list is not interpreted as "no controls".
    pub required_controls: Vec<String>,
    pub evidence: Vec<EvidenceCheck>,
    pub sod: SeparationOfDuties,
    pub requested_state: DerivedState,
}

/// Returns a deterministic list of control identifiers that prevent an ALLOW.
///
/// A required control is covered only by an explicit PASS for the same ID.
/// Unknown control IDs, malformed IDs, missing controls, and every non-PASS
/// evidence status remain blocking.
pub(crate) fn blocking_controls(input: &ReadinessInput) -> Vec<String> {
    let mut blocking = BTreeSet::new();
    let required: BTreeSet<&str> = input.required_controls.iter().map(String::as_str).collect();

    if input.evidence.is_empty() {
        blocking.insert("EVIDENCE_LIST_EMPTY".to_string());
    }
    if input.required_controls.is_empty() {
        blocking.insert("REQUIRED_CONTROLS_UNCONFIGURED".to_string());
    }
    if required.len() != input.required_controls.len()
        || input
            .required_controls
            .iter()
            .any(|control| control.trim().is_empty() || control.trim() != control)
    {
        blocking.insert("REQUIRED_CONTROLS_INVALID".to_string());
    }

    for control in &input.required_controls {
        if !input.evidence.iter().any(|evidence| {
            evidence.control_id == *control && evidence.status == EvidenceStatus::Pass
        }) {
            blocking.insert(control.clone());
        }
    }

    for evidence in &input.evidence {
        if evidence.control_id.trim().is_empty() {
            blocking.insert("EVIDENCE_CONTROL_ID_INVALID".to_string());
        } else if !required.contains(evidence.control_id.as_str()) {
            blocking.insert("UNKNOWN_CONTROL_ID".to_string());
        }

        if evidence.status.is_blocking() {
            blocking.insert(evidence.control_id.clone());
        }
    }

    blocking.into_iter().collect()
}

pub fn evaluate(input: &ReadinessInput) -> GateDecision {
    if !blocking_controls(input).is_empty() {
        return GateDecision::Block;
    }
    if input.requested_state == DerivedState::ProductionReady
        && (!input.sod.valid() || !input.sod.human_approval)
    {
        return GateDecision::Block;
    }
    GateDecision::Allow
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sod(approved: bool) -> SeparationOfDuties {
        SeparationOfDuties {
            coder: "agent-a".into(),
            validator: "agent-b".into(),
            auditor: "agent-c".into(),
            release_authority: "human-r".into(),
            human_approval: approved,
        }
    }

    fn input(
        required_controls: &[&str],
        evidence: Vec<EvidenceCheck>,
        requested_state: DerivedState,
    ) -> ReadinessInput {
        ReadinessInput {
            required_controls: required_controls.iter().map(|id| (*id).into()).collect(),
            evidence,
            sod: sod(true),
            requested_state,
        }
    }

    fn pass(control_id: &str) -> EvidenceCheck {
        EvidenceCheck {
            control_id: control_id.into(),
            status: EvidenceStatus::Pass,
        }
    }

    #[test]
    fn empty_evidence_blocks_even_with_valid_production_approvals() {
        let input = input(&["SEC-001"], vec![], DerivedState::ProductionReady);
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn missing_required_control_blocks_partial_evidence() {
        let input = input(
            &["SEC-001", "GOV-001"],
            vec![pass("SEC-001")],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn missing_evidence_status_blocks() {
        let input = input(
            &["SEC-001"],
            vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Missing,
            }],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn unknown_evidence_status_blocks() {
        let input = input(
            &["SEC-001"],
            vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Unknown,
            }],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn failed_evidence_blocks() {
        let input = input(
            &["SEC-001"],
            vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Fail,
            }],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn unknown_control_id_blocks() {
        let input = input(
            &["SEC-001"],
            vec![pass("SEC-001"), pass("UNREGISTERED-999")],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn empty_required_control_set_blocks() {
        let input = input(&[], vec![pass("SEC-001")], DerivedState::TestnetReady);
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn duplicate_required_controls_block() {
        let input = input(
            &["SEC-001", "SEC-001"],
            vec![pass("SEC-001")],
            DerivedState::TestnetReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn production_requires_human_approval() {
        let mut input = input(
            &["SEC-001"],
            vec![pass("SEC-001")],
            DerivedState::ProductionReady,
        );
        input.sod.human_approval = false;
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn invalid_separation_of_duties_blocks_production() {
        let mut input = input(
            &["SEC-001"],
            vec![pass("SEC-001")],
            DerivedState::ProductionReady,
        );
        input.sod.validator = input.sod.coder.clone();
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn all_required_controls_pass_with_valid_approval_allows() {
        let input = input(
            &["SEC-001", "GOV-001"],
            vec![pass("SEC-001"), pass("GOV-001")],
            DerivedState::ProductionReady,
        );
        assert_eq!(evaluate(&input), GateDecision::Allow);
    }

    #[test]
    fn empty_role_is_invalid() {
        let mut roles = sod(true);
        roles.coder.clear();
        assert!(!roles.valid());
    }
}
