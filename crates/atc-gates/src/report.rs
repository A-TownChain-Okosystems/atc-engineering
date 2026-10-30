//! Stable, machine-readable gate reporting.
//! The report intentionally exposes no secret material and is suitable for CI evidence.

use crate::{blocking_controls, evaluate, GateDecision, ReadinessInput};

pub const REPORT_SCHEMA_VERSION: &str = "atc.gate-report/v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateReport {
    pub schema_version: &'static str,
    pub decision: GateDecision,
    pub requested_state: &'static str,
    pub blocking_controls: Vec<String>,
    pub sod_valid: bool,
    pub human_approval: bool,
}

impl GateReport {
    pub fn from_input(input: &ReadinessInput) -> Self {
        Self {
            schema_version: REPORT_SCHEMA_VERSION,
            decision: evaluate(input),
            requested_state: input.requested_state.as_str(),
            blocking_controls: blocking_controls(input),
            sod_valid: input.sod.valid(),
            human_approval: input.sod.human_approval,
        }
    }

    pub fn is_allowed(&self) -> bool {
        self.decision == GateDecision::Allow
    }

    /// Deterministic JSON without introducing a serialization dependency into the gate crate.
    pub fn to_json(&self) -> String {
        let controls = self
            .blocking_controls
            .iter()
            .map(|control| format!("\"{}\"", escape_json(control)))
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "{{\"schema_version\":\"{}\",\"decision\":\"{}\",\"requested_state\":\"{}\",\"blocking_controls\":[{}],\"sod_valid\":{},\"human_approval\":{}}}",
            escape_json(self.schema_version),
            self.decision.as_str(),
            escape_json(self.requested_state),
            controls,
            self.sod_valid,
            self.human_approval,
        )
    }
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EvidenceCheck, EvidenceStatus, SeparationOfDuties};
    use atc_core::DerivedState;

    fn sod() -> SeparationOfDuties {
        SeparationOfDuties {
            coder: "coder".into(),
            validator: "validator".into(),
            auditor: "auditor".into(),
            release_authority: "release".into(),
            human_approval: true,
        }
    }

    #[test]
    fn report_preserves_blocking_controls() {
        let input = ReadinessInput {
            required_controls: vec!["SEC-001".into(), "GOV-002".into()],
            evidence: vec![
                EvidenceCheck {
                    control_id: "SEC-001".into(),
                    status: EvidenceStatus::Pass,
                },
                EvidenceCheck {
                    control_id: "GOV-002".into(),
                    status: EvidenceStatus::Unknown,
                },
            ],
            sod: sod(),
            requested_state: DerivedState::TestnetReady,
        };
        let report = GateReport::from_input(&input);
        assert_eq!(report.decision, GateDecision::Block);
        assert_eq!(report.blocking_controls, vec!["GOV-002"]);
        assert!(!report.is_allowed());
    }

    #[test]
    fn report_exposes_missing_required_controls() {
        let input = ReadinessInput {
            required_controls: vec!["SEC-001".into(), "GOV-002".into()],
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Pass,
            }],
            sod: sod(),
            requested_state: DerivedState::TestnetReady,
        };
        let report = GateReport::from_input(&input);
        assert_eq!(report.decision, GateDecision::Block);
        assert_eq!(report.blocking_controls, vec!["GOV-002"]);
    }

    #[test]
    fn empty_evidence_report_is_blocked_and_explained() {
        let input = ReadinessInput {
            required_controls: vec!["SEC-001".into()],
            evidence: vec![],
            sod: sod(),
            requested_state: DerivedState::ProductionReady,
        };
        let report = GateReport::from_input(&input);
        assert_eq!(report.decision, GateDecision::Block);
        assert!(report
            .blocking_controls
            .contains(&"EVIDENCE_LIST_EMPTY".to_string()));
        assert!(report.blocking_controls.contains(&"SEC-001".to_string()));
    }

    #[test]
    fn json_is_versioned_and_deterministic() {
        let input = ReadinessInput {
            required_controls: vec!["SEC-001".into()],
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Pass,
            }],
            sod: sod(),
            requested_state: DerivedState::ProductionReady,
        };
        assert_eq!(GateReport::from_input(&input).to_json(), "{\"schema_version\":\"atc.gate-report/v1\",\"decision\":\"ALLOW\",\"requested_state\":\"PRODUCTION_READY\",\"blocking_controls\":[],\"sod_valid\":true,\"human_approval\":true}");
    }
}
