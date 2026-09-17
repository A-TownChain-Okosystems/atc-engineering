//! Verification foundation. PASS is only emitted from an explicit verifier.

use atc_core::{CoreError, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationStatus { Pass, Fail, Blocked, NotExecuted, NotApplicable }

impl VerificationStatus {
    pub fn as_str(self) -> &'static str { match self { Self::Pass=>"PASS", Self::Fail=>"FAIL", Self::Blocked=>"BLOCKED", Self::NotExecuted=>"NOT_EXECUTED", Self::NotApplicable=>"NOT_APPLICABLE" } }
    pub fn is_pass(self) -> bool { matches!(self, Self::Pass) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationResult {
    pub check_id: String,
    pub status: VerificationStatus,
    pub evidence_ref: Option<String>,
    pub details: String,
}

impl VerificationResult {
    pub fn pass(check_id: &str, evidence_ref: &str, details: &str) -> Result<Self> {
        if check_id.trim().is_empty() || evidence_ref.trim().is_empty() { return Err(CoreError::MissingEvidence("PASS requires check id and evidence reference".into())); }
        Ok(Self { check_id:check_id.into(), status:VerificationStatus::Pass, evidence_ref:Some(evidence_ref.into()), details:details.into() })
    }
    pub fn blocked(check_id: &str, details: &str) -> Self { Self { check_id:check_id.into(), status:VerificationStatus::Blocked, evidence_ref:None, details:details.into() } }
}

#[derive(Clone, Debug, Default)]
pub struct VerificationReport { pub results: Vec<VerificationResult> }

impl VerificationReport {
    pub fn ready(&self) -> bool { !self.results.is_empty() && self.results.iter().all(|r| r.status.is_pass()) }
    pub fn add(&mut self, result: VerificationResult) { self.results.push(result); }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn pass_requires_evidence() { assert!(VerificationResult::pass("build", "", "").is_err()); } #[test] fn report_is_fail_closed() { let mut r=VerificationReport::default(); r.add(VerificationResult::blocked("test","not run")); assert!(!r.ready()); } }
