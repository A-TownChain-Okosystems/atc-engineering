use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use atc_core::DerivedState;
use atc_gates::{EvidenceCheck, EvidenceStatus, GateReport, ReadinessInput, SeparationOfDuties};
use atc_maintenance::{audit_repository, FindingKind};

fn env_value(name: &str, fallback: &str) -> String {
    env::var(name).unwrap_or_else(|_| fallback.to_string())
}

fn env_bool(name: &str) -> bool {
    matches!(env::var(name).as_deref(), Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES"))
}

fn main() {
    let mut args = env::args_os().skip(1);
    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let gate_requested = args.any(|arg| arg == "--gate");
    let requested_state = env_value("ATC_REQUESTED_STATE", "TESTNET_READY");

    let report = match audit_repository(&root) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("AUDIT ERROR: {error}");
            std::process::exit(2);
        }
    };

    for finding in &report.findings {
        let path = finding
            .path
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<repository>".to_string());
        println!(
            "{:?} {} {}: {}",
            finding.kind, finding.code, path, finding.message
        );
    }

    if gate_requested {
        let state = match DerivedState::from_str(&requested_state) {
            Ok(state) => state,
            Err(error) => {
                eprintln!("GATE ERROR: {error}");
                std::process::exit(2);
            }
        };

        let evidence = if report.findings.is_empty() {
            vec![EvidenceCheck {
                control_id: "AUDIT-REPOSITORY".into(),
                status: EvidenceStatus::Pass,
            }]
        } else {
            report
                .findings
                .iter()
                .map(|finding| EvidenceCheck {
                    control_id: finding.code.clone(),
                    status: match finding.kind {
                        FindingKind::Security => EvidenceStatus::Fail,
                        FindingKind::Error => EvidenceStatus::Fail,
                        FindingKind::Consistency => EvidenceStatus::Unknown,
                        FindingKind::Connectivity => EvidenceStatus::Unknown,
                    },
                })
                .collect()
        };

        let sod = SeparationOfDuties {
            coder: env_value("ATC_CODER", ""),
            validator: env_value("ATC_VALIDATOR", ""),
            auditor: env_value("ATC_AUDITOR", ""),
            release_authority: env_value("ATC_RELEASE_AUTHORITY", ""),
            human_approval: env_bool("ATC_HUMAN_APPROVAL"),
        };
        let gate = GateReport::from_input(&ReadinessInput {
            evidence,
            sod,
            requested_state: state,
        });
        println!("GATE_REPORT {}", gate.to_json());

        if !gate.is_allowed() {
            std::process::exit(1);
        }
    }

    if report.is_clean() {
        println!("AUDIT PASS: no findings");
        return;
    }

    let security = report.has_kind(FindingKind::Security);
    let connectivity = report.has_kind(FindingKind::Connectivity);
    let consistency = report.has_kind(FindingKind::Consistency);
    let errors = report.has_kind(FindingKind::Error);
    eprintln!(
        "AUDIT FAIL: errors={errors} security={security} consistency={consistency} connectivity={connectivity} findings={}",
        report.findings.len()
    );
    std::process::exit(1);
}
