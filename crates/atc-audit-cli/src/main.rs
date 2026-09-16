use std::env;
use std::path::PathBuf;

use atc_maintenance::{audit_repository, FindingKind};

fn main() {
    let root = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    match audit_repository(&root) {
        Ok(report) => {
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
        Err(error) => {
            eprintln!("AUDIT ERROR: {error}");
            std::process::exit(2);
        }
    }
}
