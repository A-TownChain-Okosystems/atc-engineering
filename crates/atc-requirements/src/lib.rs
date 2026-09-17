//! Target-software requirements discovery.
//!
//! This crate turns observable repository facts into an explicit implementation
//! plan. It does not claim that a component exists merely because a directory
//! or documentation entry exists. Required components are derived from detected
//! capabilities and remain `Missing` until implementation evidence is supplied.

use std::collections::BTreeSet;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Technology {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Shell,
    Docker,
    GitHubActions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequirementKind {
    Component,
    Content,
    Test,
    Documentation,
    Governance,
    Security,
    Integration,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementStatus {
    Missing,
    Planned,
    Present,
    Verified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    pub id: &'static str,
    pub kind: RequirementKind,
    pub name: &'static str,
    pub reason: &'static str,
    pub status: RequirementStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TargetProfile {
    pub technologies: BTreeSet<Technology>,
    pub requirements: Vec<Requirement>,
}

impl TargetProfile {
    pub fn missing(&self) -> impl Iterator<Item = &Requirement> {
        self.requirements
            .iter()
            .filter(|r| matches!(r.status, RequirementStatus::Missing))
    }

    pub fn is_complete(&self) -> bool {
        self.missing().next().is_none()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RepositorySignals {
    pub rust_workspace: bool,
    pub python_project: bool,
    pub javascript_project: bool,
    pub typescript_project: bool,
    pub shell_automation: bool,
    pub docker: bool,
    pub github_actions: bool,
}

pub fn detect_signals(root: &Path) -> std::io::Result<RepositorySignals> {
    let exists = |name: &str| root.join(name).exists();
    Ok(RepositorySignals {
        rust_workspace: exists("Cargo.toml"),
        python_project: exists("pyproject.toml") || exists("requirements.txt") || exists("setup.py"),
        javascript_project: exists("package.json"),
        typescript_project: exists("tsconfig.json"),
        shell_automation: exists("Makefile") || exists("scripts"),
        docker: exists("Dockerfile") || exists("docker-compose.yml") || exists("compose.yml"),
        github_actions: root.join(".github/workflows").is_dir(),
    })
}

pub fn analyze(signals: RepositorySignals) -> TargetProfile {
    let mut technologies = BTreeSet::new();
    if signals.rust_workspace { technologies.insert(Technology::Rust); }
    if signals.python_project { technologies.insert(Technology::Python); }
    if signals.javascript_project { technologies.insert(Technology::JavaScript); }
    if signals.typescript_project { technologies.insert(Technology::TypeScript); }
    if signals.shell_automation { technologies.insert(Technology::Shell); }
    if signals.docker { technologies.insert(Technology::Docker); }
    if signals.github_actions { technologies.insert(Technology::GitHubActions); }

    let mut requirements = vec![
        Requirement { id: "COMP-001", kind: RequirementKind::Component, name: "core runtime", reason: "Every target software system needs an explicit execution core.", status: RequirementStatus::Missing },
        Requirement { id: "COMP-002", kind: RequirementKind::Component, name: "configuration and schema layer", reason: "Configuration must be typed, validated and versioned.", status: RequirementStatus::Missing },
        Requirement { id: "COMP-003", kind: RequirementKind::Component, name: "integration/dependency layer", reason: "External and internal component contracts must be explicit.", status: RequirementStatus::Missing },
        Requirement { id: "COMP-004", kind: RequirementKind::Component, name: "observability and diagnostics", reason: "A maintainable target needs structured diagnostics and operational visibility.", status: RequirementStatus::Missing },
        Requirement { id: "CONT-001", kind: RequirementKind::Content, name: "authoritative requirements specification", reason: "The target must have a traceable statement of required behavior.", status: RequirementStatus::Missing },
        Requirement { id: "CONT-002", kind: RequirementKind::Content, name: "component/dependency inventory", reason: "Required components and dependencies must be enumerated before completeness can be assessed.", status: RequirementStatus::Missing },
        Requirement { id: "TEST-001", kind: RequirementKind::Test, name: "unit and integration tests", reason: "Component existence is insufficient without executable verification.", status: RequirementStatus::Missing },
        Requirement { id: "TEST-002", kind: RequirementKind::Test, name: "negative/fail-closed tests", reason: "Invalid and incomplete states must be proven to block where required.", status: RequirementStatus::Missing },
        Requirement { id: "DOC-001", kind: RequirementKind::Documentation, name: "architecture and operations documentation", reason: "Interfaces, lifecycle and operation must be reproducible by maintainers.", status: RequirementStatus::Missing },
        Requirement { id: "GOV-001", kind: RequirementKind::Governance, name: "ownership and change-control definition", reason: "Critical software requires explicit responsibility and controlled change paths.", status: RequirementStatus::Missing },
        Requirement { id: "SEC-001", kind: RequirementKind::Security, name: "security/threat and supply-chain controls", reason: "Security requirements must be explicit before release readiness.", status: RequirementStatus::Missing },
        Requirement { id: "REL-001", kind: RequirementKind::Release, name: "reproducible build and release evidence", reason: "A release requires verifiable build inputs and outputs.", status: RequirementStatus::Missing },
    ];

    if signals.rust_workspace {
        requirements.push(Requirement { id: "TECH-RUST-001", kind: RequirementKind::Component, name: "Rust workspace validation", reason: "Cargo workspace targets require formatting, linting and test validation.", status: RequirementStatus::Planned });
    }
    if signals.github_actions {
        requirements.push(Requirement { id: "TECH-CI-001", kind: RequirementKind::Integration, name: "CI enforcement", reason: "Detected GitHub Actions should enforce the target verification contract.", status: RequirementStatus::Planned });
    }

    TargetProfile { technologies, requirements }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_target_gets_rust_validation_requirement() {
        let profile = analyze(RepositorySignals { rust_workspace: true, ..Default::default() });
        assert!(profile.technologies.contains(&Technology::Rust));
        assert!(profile.requirements.iter().any(|r| r.id == "TECH-RUST-001"));
    }

    #[test]
    fn missing_requirements_prevent_completeness_claim() {
        let profile = analyze(RepositorySignals::default());
        assert!(!profile.is_complete());
        assert!(profile.missing().any(|r| r.id == "COMP-001"));
    }

    #[test]
    fn ci_signal_creates_enforcement_requirement() {
        let profile = analyze(RepositorySignals { github_actions: true, ..Default::default() });
        assert!(profile.requirements.iter().any(|r| r.id == "TECH-CI-001"));
    }
}
