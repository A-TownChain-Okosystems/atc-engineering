# Engineering Audit

Repository: `atc-engineering`
Status: BASELINE
Last verified: 2026-09-15

This repository is the engineering control plane. Its own audit implementation is subject to the same evidence rules it enforces: deterministic behavior, explicit findings, fail-closed gates and verified remediation.

Current implementation includes `atc-maintenance::audit_repository` and `atc-integration::IntegrationGraph`.
