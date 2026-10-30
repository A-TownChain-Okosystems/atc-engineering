//! Deterministic engineering release gates.
//! `UNKNOWN` is never promoted: missing evidence is a blocking condition.

mod report;
pub use report::GateReport;

use atc_core::DerivedState;

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
    pub evidence: Vec<EvidenceCheck>,
    pub sod: SeparationOfDuties,
    pub requested_state: DerivedState,
}

pub fn evaluate(input: &ReadinessInput) -> GateDecision {
    // An empty collection is not proof that every required control passed.
    // Required controls omitted by an upstream producer must never become ALLOW.
    if input.evidence.is_empty() || input.evidence.iter().any(|e| e.status.is_blocking()) {
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

    fn check(control_id: &str, status: EvidenceStatus) -> EvidenceCheck {
        EvidenceCheck {
            control_id: control_id.into(),
            status,
        }
    }

    fn input(evidence: Vec<EvidenceCheck>, sod: SeparationOfDuties) -> ReadinessInput {
        ReadinessInput {
            evidence,
            sod,
            requested_state: DerivedState::ProductionReady,
        }
    }

    #[test]
    fn empty_evidence_blocks_even_with_valid_sod_and_approval() {
        assert_eq!(evaluate(&input(Vec::new(), sod(true))), GateDecision::Block);
    }

    #[test]
    fn missing_required_evidence_blocks_partial_collection() {
        let evidence = vec![
            check("SEC-001", EvidenceStatus::Pass),
            check("GOV-002", EvidenceStatus::Missing),
        ];
        assert_eq!(evaluate(&input(evidence, sod(true))), GateDecision::Block);
    }

    #[test]
    fn unknown_evidence_type_blocks() {
        let evidence = vec![
            check("SEC-001", EvidenceStatus::Pass),
            check("UNKNOWN-999", EvidenceStatus::Unknown),
        ];
        assert_eq!(evaluate(&input(evidence, sod(true))), GateDecision::Block);
    }

    #[test]
    fn failed_evidence_blocks() {
        let evidence = vec![
            check("SEC-001", EvidenceStatus::Pass),
            check("SEC-002", EvidenceStatus::Fail),
        ];
        assert_eq!(evaluate(&input(evidence, sod(true))), GateDecision::Block);
    }

    #[test]
    fn production_requires_human_approval() {
        assert_eq!(
            evaluate(&input(vec![check("SEC-001", EvidenceStatus::Pass)], sod(false))),
            GateDecision::Block
        );
    }

    #[test]
    fn invalid_separation_of_duties_blocks_production() {
        let invalid_sod = SeparationOfDuties {
            coder: "same".into(),
            validator: "same".into(),
            auditor: "auditor".into(),
            release_authority: "release".into(),
            human_approval: true,
        };
        assert_eq!(
            evaluate(&input(vec![check("SEC-001", EvidenceStatus::Pass)], invalid_sod)),
            GateDecision::Block
        );
    }

    #[test]
    fn complete_passing_evidence_and_valid_approval_allow() {
        let evidence = vec![
            check("SEC-001", EvidenceStatus::Pass),
            check("GOV-002", EvidenceStatus::Pass),
        ];
        assert_eq!(evaluate(&input(evidence, sod(true))), GateDecision::Allow);
    }

    #[test]
    fn empty_role_is_invalid() {
        let mut roles = sod(true);
        roles.coder.clear();
        assert!(!roles.valid());
    }
}
