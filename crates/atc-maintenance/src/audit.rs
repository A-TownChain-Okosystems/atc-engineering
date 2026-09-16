//! Repository audit primitives for the organization-wide engineering gate.
//!
//! The scanner is deliberately dependency-free and fail-closed: findings are
//! derived from the checked-out repository tree and do not depend on claims in
//! documentation.

use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::{collections::hash_map::DefaultHasher, ffi::OsStr};

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

    if let Err(error) = audit_readme_identity(root, &mut report) {
        return Err(error);
    }

    // Keep the duplicate index at repository scope. A per-directory index can
    // miss duplicates that are split across source/configuration directories.
    let mut duplicate_candidates: HashMap<u64, PathBuf> = HashMap::new();
    scan_tree(root, root, &mut duplicate_candidates, &mut report)?;
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

fn audit_readme_identity(root: &Path, report: &mut AuditReport) -> Result<(), std::io::Error> {
    let Some(repo_name) = root.file_name().and_then(OsStr::to_str) else {
        return Ok(());
    };
    let readme = fs::read_to_string(root.join("README.md"))?;
    if !readme
        .to_ascii_lowercase()
        .contains(&repo_name.to_ascii_lowercase())
    {
        report.findings.push(Finding {
            kind: FindingKind::Consistency,
            code: "CONS-AUDIT-002",
            path: Some(PathBuf::from("README.md")),
            message: format!("README does not identify repository `{repo_name}`"),
        });
    }
    Ok(())
}

fn scan_tree(
    root: &Path,
    current: &Path,
    duplicate_candidates: &mut HashMap<u64, PathBuf>,
    report: &mut AuditReport,
) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(&path);

        if relative
            .components()
            .any(|component| component.as_os_str() == ".git")
        {
            continue;
        }

        if path.is_dir() {
            scan_tree(root, &path, duplicate_candidates, report)?;
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

        if contains_dangerous_shell_pattern(path.extension().and_then(OsStr::to_str), text) {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-002",
                path: Some(relative.to_path_buf()),
                message: "potentially unsafe command execution/download pattern detected".into(),
            });
        }

        if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
            && text.contains("[workspace")
            && !root.join("Cargo.lock").is_file()
        {
            report.findings.push(Finding {
                kind: FindingKind::Consistency,
                code: "CONS-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: "Rust workspace has no committed Cargo.lock evidence".into(),
            });
        }

        if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml") {
            audit_cargo_path_dependencies(root, relative, text, report);
        }

        if relative.starts_with(Path::new(".github/workflows"))
            && text.contains("uses:")
            && !text.contains("permissions:")
        {
            report.findings.push(Finding {
                kind: FindingKind::Security,
                code: "SEC-AUDIT-003",
                path: Some(relative.to_path_buf()),
                message:
                    "GitHub Actions workflow uses actions without an explicit permissions policy"
                        .into(),
            });
        }

        let is_source = matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("rs" | "py" | "ts" | "tsx" | "js" | "jsx" | "sh" | "bash")
        );
        let is_audit_document = relative == Path::new("docs/ENGINEERING_AUDIT.md");
        if is_source && !is_audit_document && (text.contains("TODO") || text.contains("FIXME")) {
            report.findings.push(Finding {
                kind: FindingKind::Error,
                code: "AUDIT-003",
                path: Some(relative.to_path_buf()),
                message: "unresolved TODO/FIXME marker requires triage".into(),
            });
        }

        if is_duplicate_candidate(&path) {
            let mut hasher = DefaultHasher::new();
            bytes.hash(&mut hasher);
            let fingerprint = hasher.finish();
            if let Some(first) = duplicate_candidates.insert(fingerprint, relative.to_path_buf()) {
                report.findings.push(Finding {
                    kind: FindingKind::Consistency,
                    code: "CONS-AUDIT-003",
                    path: Some(relative.to_path_buf()),
                    message: format!("duplicate file content matches {}", first.display()),
                });
            }
        }
    }
    Ok(())
}

fn audit_cargo_path_dependencies(
    root: &Path,
    relative: &Path,
    text: &str,
    report: &mut AuditReport,
) {
    for line in text.lines() {
        let Some(raw) = line.split_once("path") else {
            continue;
        };
        let Some((_, value)) = raw.1.split_once('=') else {
            continue;
        };
        let path = value.trim().trim_matches('"').trim_matches('\'');
        if path.is_empty() {
            continue;
        }
        let manifest_dir = relative.parent().unwrap_or_else(|| Path::new("."));
        if !root.join(manifest_dir).join(path).exists() {
            report.findings.push(Finding {
                kind: FindingKind::Connectivity,
                code: "CONN-AUDIT-001",
                path: Some(relative.to_path_buf()),
                message: format!("Cargo path dependency does not exist: {path}"),
            });
        }
    }
}

fn is_duplicate_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rs" | "py" | "ts" | "tsx" | "js" | "jsx" | "sh" | "bash" | "toml" | "yaml" | "yml")
    )
}

fn is_text_candidate(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some(
            "rs" | "toml" | "yaml" | "yml" | "json" | "md" | "txt" | "py" | "ts" | "tsx"
                | "js" | "jsx" | "sh" | "bash"
        )
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

fn contains_dangerous_shell_pattern(extension: Option<&str>, text: &str) -> bool {
    if !matches!(extension, Some("sh" | "bash" | "yml" | "yaml")) {
        return false;
    }
    let normalized = text.replace('\n', " ");
    (normalized.contains("curl ") || normalized.contains("wget "))
        && (normalized.contains("| sh") || normalized.contains("| bash"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_patterns_are_detected() {
        assert!(contains_secret_pattern("prefix ghp_1234567890suffix"));
        assert!(contains_secret_pattern(
            "-----BEGIN OPENSSH PRIVATE KEY-----"
        ));
        assert!(!contains_secret_pattern("public documentation only"));
    }

    #[test]
    fn extensions_are_restricted_to_text_inputs() {
        assert!(is_text_candidate(Path::new("main.rs")));
        assert!(is_text_candidate(Path::new("README.md")));
        assert!(!is_text_candidate(Path::new("texture.png")));
    }

    #[test]
    fn unsafe_download_pipelines_are_detected() {
        assert!(contains_dangerous_shell_pattern(
            Some("sh"),
            "curl https://example.test/a.sh | sh"
        ));
        assert!(!contains_dangerous_shell_pattern(
            Some("sh"),
            "curl https://example.test/a.sh -o a.sh"
        ));
        assert!(!contains_dangerous_shell_pattern(
            Some("rs"),
            "curl https://example.test/a.sh | sh"
        ));
    }

    #[test]
    fn duplicate_candidates_are_limited_to_source_and_config() {
        assert!(is_duplicate_candidate(Path::new("a.rs")));
        assert!(is_duplicate_candidate(Path::new("a.yml")));
        assert!(!is_duplicate_candidate(Path::new("README.md")));
    }
}
