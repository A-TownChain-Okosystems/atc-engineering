//! ATC Finding Registry — machine-readable, fail-closed finding lifecycle.
//!
//! Lifecycle: DETECTED -> CLASSIFIED -> PLANNED -> FIXED -> VERIFIED -> CLOSED.
//! BLOCKED is terminal until an explicit engineering action changes the state.

use atc_core::{CoreError, Result};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingState {
    Detected,
    Classified,
    Planned,
    Fixed,
    Verified,
    Closed,
    Blocked,
}

impl FindingState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "DETECTED",
            Self::Classified => "CLASSIFIED",
            Self::Planned => "PLANNED",
            Self::Fixed => "FIXED",
            Self::Verified => "VERIFIED",
            Self::Closed => "CLOSED",
            Self::Blocked => "BLOCKED",
        }
    }

    fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Detected, Self::Classified)
                | (Self::Classified, Self::Planned)
                | (Self::Planned, Self::Fixed)
                | (Self::Fixed, Self::Verified)
                | (Self::Verified, Self::Closed)
                | (Self::Detected, Self::Blocked)
                | (Self::Classified, Self::Blocked)
                | (Self::Planned, Self::Blocked)
                | (Self::Fixed, Self::Blocked)
                | (Self::Verified, Self::Blocked)
        )
    }
}

impl fmt::Display for FindingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FindingSeverity { Info, Low, Medium, High, Critical }

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self { Self::Info=>"INFO", Self::Low=>"LOW", Self::Medium=>"MEDIUM", Self::High=>"HIGH", Self::Critical=>"CRITICAL" }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub id: String,
    pub repository: String,
    pub path: Option<String>,
    pub code: String,
    pub category: String,
    pub severity: FindingSeverity,
    pub state: FindingState,
    pub description: String,
    pub root_cause: String,
    pub implementation_plan: String,
    pub iteration: usize,
}

impl Finding {
    pub fn new(id: &str, repository: &str, code: &str, category: &str, severity: FindingSeverity, description: &str) -> Result<Self> {
        if id.trim().is_empty() || repository.trim().is_empty() || code.trim().is_empty() {
            return Err(CoreError::InvalidId("finding requires id, repository and code".into()));
        }
        Ok(Self { id:id.into(), repository:repository.into(), path:None, code:code.into(), category:category.into(), severity, state:FindingState::Detected, description:description.into(), root_cause:String::new(), implementation_plan:String::new(), iteration:1 })
    }

    pub fn transition(&mut self, next: FindingState) -> Result<()> {
        if !self.state.can_transition_to(next) {
            return Err(CoreError::InvalidState(format!("invalid finding transition {} -> {}", self.state, next)));
        }
        self.state = next;
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FindingRegistry { pub findings: Vec<Finding> }

impl FindingRegistry {
    pub fn register(&mut self, finding: Finding) -> Result<()> {
        if self.findings.iter().any(|f| f.id == finding.id) {
            return Err(CoreError::InvalidState(format!("duplicate finding id: {}", finding.id)));
        }
        self.findings.push(finding);
        Ok(())
    }

    pub fn get_mut(&mut self, id: &str) -> Result<&mut Finding> {
        self.findings.iter_mut().find(|f| f.id == id).ok_or_else(|| CoreError::InvalidId(format!("unknown finding: {id}")))
    }

    pub fn open_findings(&self) -> impl Iterator<Item=&Finding> {
        self.findings.iter().filter(|f| !matches!(f.state, FindingState::Closed))
    }

    pub fn write_json(&self, root: &Path) -> Result<()> {
        let dir = root.join(".atc/findings");
        fs::create_dir_all(&dir).map_err(|e| CoreError::InvalidState(format!("create findings store: {e}")))?;
        let path = dir.join("registry.json");
        if path.exists() { return Err(CoreError::InvalidState("finding registry is append-only; refusing overwrite".into())); }
        let mut out = String::from("{\n  \"schema\": \"ATC-FND-001\",\n  \"version\": \"1.0.0\",\n  \"findings\": [\n");
        for (i, f) in self.findings.iter().enumerate() {
            let comma = if i + 1 == self.findings.len() { "" } else { "," };
            out.push_str(&format!("    {{\"id\":\"{}\",\"repository\":\"{}\",\"code\":\"{}\",\"category\":\"{}\",\"severity\":\"{}\",\"state\":\"{}\",\"iteration\":{}}}{}\n", esc(&f.id), esc(&f.repository), esc(&f.code), esc(&f.category), f.severity.as_str(), f.state.as_str(), f.iteration, comma));
        }
        out.push_str("  ],\n  \"immutable\": true\n}\n");
        fs::write(path, out).map_err(|e| CoreError::InvalidState(format!("write findings registry: {e}")))
    }
}

fn esc(s: &str) -> String { s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n") }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn lifecycle_is_enforced() { let mut f=Finding::new("FND-1","repo","ERR-1","error",FindingSeverity::High,"x").unwrap(); assert!(f.transition(FindingState::Classified).is_ok()); assert!(f.transition(FindingState::Closed).is_err()); }
    #[test] fn duplicate_ids_are_rejected() { let f=Finding::new("FND-1","repo","ERR-1","error",FindingSeverity::Low,"x").unwrap(); let mut r=FindingRegistry::default(); r.register(f.clone()).unwrap(); assert!(r.register(f).is_err()); }
}
