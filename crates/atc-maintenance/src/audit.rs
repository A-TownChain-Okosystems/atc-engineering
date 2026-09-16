//! Repository audit primitives for the organization-wide engineering gate.
//!
//! The scanner is deliberately dependency-free and fail-closed: it reports
//! only evidence that can be derived from the checked-out repository tree.

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingKind {
    Error,
    Security,
    Consistency,
    Connectivity,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub kind: FindingKind,
    pub code: &'static str,
    pub path: Option<PathBuf>,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub findings: Vec<Finding>,
}

impl AuditReport {
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn has_kind(&self, kind: FindingKind) -> bool {
        self.findings.iter().any(|finding| finding.kind == kind)
    }
}

pub fn audit_repository(root: &Path) -> Result<AuditReport, std::io::Error> {
    let mut report = AuditReport::default();

    require_file(root, "README.md", "AUDIT-001", &mut report)?;
    require_file(root, "docs/ENGINEERING_AUDIT.md", "AUDIT-002", &mut report)?;

    scan_tree(root, root, &mut report)?;
    Ok(report)
}

fn require_file(
    root: &Path,
    relative: &str,
    code: &'static str,
    report: &mut AuditReport,
) -> Result<(), std::io::Error> {
    let path = root.join(relative);
    if !path.is_file() {
        report.findings.push(Finding {
            kind: FindingKind::Error,
            code,
            path: Some(PathBuf::from(relative)),
            message: format!("required governance file is missing: {relative}"),
        });
    }
    Ok(())
}

fn scan_tree(root: &Path, current: &Path, report: &mut AuditReport) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);

        if relative.components().any(|component| component.as_os_str() == ".git") {
            continue;
        }

        if path.is_dir() {
            scan_tree(root, &path, report)?;
            continue;
        }

        if !is_text_candidate(&path) {
            continue;
        }

        let bytes = fs::read(&path)?;
        let text = match std::str::from_utf8(&bytes) {
            Ok(text) => text,
            Err(_) => continue,
        };

        if contains_secret_pattern(text) {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: "high-confidence credential material pattern detected".into(),
            });
        }

        if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
            && text.contains("workspace")
            && !root.join("Cargo.lock").is_file()
        {
            report.findings.push(Finding {
                kind: FindingKind::Consistency,
                code: "CONS-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: "Rust workspace has no committed Cargo.lock evidence".into(),
            });
        }

        if text.contains("TODO") || text.contains("FIXME") {
            report.findings.push(Finding {
                kind: FindingKind::Error,
                code: "AUDIT-003",
                path: Some(relative.to_path_buf()),
                message: "unresolved TODO/FIXME marker requires triage".into(),
            });
        }
    }
    Ok(())
}

fn is_text_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "toml" | "yaml" | "yml" | "json" | "md" | "txt" | "py" | "ts" | "tsx" | "js" | "jsx" | "sh" | "bash")
    )
}

fn contains_secret_pattern(text: &str) -> bool {
    [
        "-----BEGIN RSA PRIVATE KEY-----",
        "-----BEGIN EC PRIVATE KEY-----",
        "-----BEGIN OPENSSH PRIVATE KEY-----",
        "ghp_",
        "github_pat_",
        "AKIA",
    ]
    .iter()
    .any(|pattern| text.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_patterns_are_detected() {
        assert!(contains_secret_pattern("prefix ghp_1234567890suffix"));
        assert!(contains_secret_pattern("-----BEGIN OPENSSH PRIVATE KEY-----"));
        assert!(!contains_secret_pattern("public documentation only"));
    }

    #[test]
    fn extensions_are_restricted_to_text_inputs() {
        assert!(is_text_candidate(Path::new("main.rs")));
        assert!(is_text_candidate(Path::new("README.md")));
        assert!(!is_text_candidate(Path::new("texture.png")));
    }
}
