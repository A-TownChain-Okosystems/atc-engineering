//! Stable, machine-readable gate reporting.
//! The report intentionally exposes no secret material and is suitable for CI evidence.

use crate::{evaluate, GateDecision, ReadinessInput};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateReport {
    pub decision: GateDecision,
    pub blocking_controls: Vec<&'static str>,
    pub sod_valid: bool,
    pub human_approval: bool,
}

impl GateReport {
    pub fn from_input(input: &ReadinessInput) -> Self {
        let blocking_controls = input
            .evidence
            .iter()
            .filter(|e| e.status.is_blocking())
            .map(|e| e.control_id)
            .collect();

        Self {
            decision: evaluate(input),
            blocking_controls,
            sod_valid: input.sod.valid(),
            human_approval: input.sod.human_approval,
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.decision == GateDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EvidenceCheck, EvidenceStatus, SeparationOfDuties};
    use atc_core::DerivedState;

    #[test]
    fn report_preserves_blocking_controls() {
        let input = ReadinessInput {
            evidence: vec![
                EvidenceCheck { control_id: "SEC-001", status: EvidenceStatus::Pass },
                EvidenceCheck { control_id: "GOV-002", status: EvidenceStatus::Unknown },
            ],
            sod: SeparationOfDuties {
                coder: "coder", validator: "validator", auditor: "auditor",
                release_authority: "release", human_approval: true,
            },
            requested_state: DerivedState::TestnetReady,
        };
        let report = GateReport::from_input(&input);
        assert_eq!(report.decision, GateDecision::Block);
        assert_eq!(report.blocking_controls, vec!["GOV-002"]);
        assert!(!report.is_allowed());
    }
}
