# Engineering Audit

Repository: `atc-engineering`
Status: ACTIVE / FLEET AUDIT CONTROL PLANE
Last verified: 2026-09-16

This repository is the engineering control plane for the A-TownChain-Okosystems repository fleet. Its own audit implementation is subject to the same evidence rules it enforces: deterministic behavior, explicit findings, fail-closed gates and verified remediation.

## Implemented controls

- `atc-maintenance::audit_repository` — repository-tree audit for governance files, README identity, credential patterns, unsafe shell download pipelines, Rust workspace lock evidence, Cargo path dependencies, workflow permission policy, TODO/FIXME markers and duplicate source/configuration content.
- `atc-integration::IntegrationGraph` — deterministic integration graph covering the current active repository architecture, including developer tooling, game/franchise tooling, compute, consensus, mining, explorer, marketplace and launchpad systems.
- `.github/workflows/fleet-audit.yml` — organization fleet audit with read-only permissions, dynamic repository discovery, private-repository support through `ATC_FLEET_READ_TOKEN`, isolated shallow clones, per-repository evidence and artifact retention.

## Verification model

The fleet audit is not a production-readiness claim. It is an evidence collection and defect-detection gate. Findings must be classified, documented, remediated, and then re-audited. Repository-native build, test, security, dependency and runtime gates remain authoritative for deep validation.

## Current fleet scope

The repository registry in `atc-standards` records 31 repositories as of 2026-09-15. The fleet workflow discovers the live GitHub organization inventory rather than relying on a hard-coded list, so newly added repositories are automatically included.

Private repositories require the organization secret `ATC_FLEET_READ_TOKEN`; without it the audit is intentionally fail-closed and records the repository as blocked instead of silently omitting it.
