//! Deterministic engineering release gates.
//! `UNKNOWN` is never promoted: missing evidence is a blocking condition.

mod report;
pub use report::GateReport;

use atc_core::DerivedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus { Pass, Fail, Missing, Unknown }

impl EvidenceStatus {
    pub const fn is_blocking(self) -> bool { !matches!(self, Self::Pass) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecision { Allow, Block }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCheck {
    pub control_id: &'static str,
    pub status: EvidenceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDuties {
    pub coder: &'static str,
    pub validator: &'static str,
    pub auditor: &'static str,
    pub release_authority: &'static str,
    pub human_approval: bool,
}

impl SeparationOfDuties {
    pub fn valid(&self) -> bool {
        self.coder != self.validator
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
    if input.evidence.iter().any(|e| e.status.is_blocking()) {
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
        SeparationOfDuties { coder: "agent-a", validator: "agent-b", auditor: "agent-c", release_authority: "human-r", human_approval: approved }
    }

    #[test]
    fn missing_evidence_blocks() {
        let input = ReadinessInput { evidence: vec![EvidenceCheck { control_id: "SEC-001", status: EvidenceStatus::Missing }], sod: sod(true), requested_state: DerivedState::TestnetReady };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn production_requires_human_approval() {
        let input = ReadinessInput { evidence: vec![EvidenceCheck { control_id: "SEC-001", status: EvidenceStatus::Pass }], sod: sod(false), requested_state: DerivedState::ProductionReady };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn all_controls_pass_allows() {
        let input = ReadinessInput { evidence: vec![EvidenceCheck { control_id: "SEC-001", status: EvidenceStatus::Pass }], sod: sod(true), requested_state: DerivedState::ProductionReady };
        assert_eq!(evaluate(&input), GateDecision::Allow);
    }

    #[test]
    fn duplicate_principals_block_production() {
        let input = ReadinessInput {
            evidence: vec![EvidenceCheck { control_id: "SEC-001", status: EvidenceStatus::Pass }],
            sod: SeparationOfDuties { coder: "same", validator: "same", auditor: "auditor", release_authority: "release", human_approval: true },
            requested_state: DerivedState::ProductionReady,
        };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }
}
