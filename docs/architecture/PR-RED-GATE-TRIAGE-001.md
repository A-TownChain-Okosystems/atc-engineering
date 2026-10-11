# PR Red-Gate and Merge-Conflict Triage

**Status:** IMPLEMENTED — read-only detection; remediation execution remains a follow-up.

## Purpose

The scheduled/manual workflow inventories open pull requests across the organization, detects failing check states and GitHub's `CONFLICTING` merge state, and publishes both JSON and Markdown evidence as a workflow artifact.

## Execution

- Workflow: `.github/workflows/pr-red-gate-triage.yml`
- Scanner: `.github/scripts/triage-pr-gates.py`
- Schedule: every four hours; also manually dispatchable.
- Output: `triage-results/findings.json` and `triage-results/SUMMARY.md`.
- Organization-wide access: configure `ATC_FLEET_READ_TOKEN` as a repository Actions secret with read access to the intended repository fleet. The default token may not see other repositories; access failures are explicitly BLOCKED.

## Detection and evidence

Each finding records repository, PR number and URL, head SHA, merge state, classification, and red check names/metadata. A check-discovery error is recorded as BLOCKED, not treated as green. The report is a point-in-time census; after a head SHA changes, checks must be re-collected.

## Remediation contract

Detection is not remediation. For each red gate, the next worker must fetch the exact head's workflow run, job, step, exit code and relevant logs; classify the root cause; document the finding; apply a scoped fix; update affected documentation; and rerun required checks against the exact new head SHA. Merge conflicts must be resolved additively, preserving both sides and documenting any semantic choice.

Only deterministic, low-risk transformations may be automated without review. Compiler/test failures, dependency/security changes, cryptography, consensus, protocol, standards, kernel/TCB, permissions, and ambiguous conflicts require an explicit engineering review. Never delete tests, weaken gates, broaden allowlists to conceal defects, force-push, or auto-merge.

## Verification criteria for closure

A finding becomes VERIFIED only when all required checks pass for the exact updated PR head SHA, evidence is attributable to the real workflow run/job/step, and no unresolved residual remains. Stale checks, synthetic merge refs, missing jobs/logs, inaccessible repositories, and unknown states remain BLOCKED/RESIDUAL.

## Current limitation

This first increment is read-only: it detects and records failures/conflicts but does not yet push patches, create issue records for each finding, or resolve conflicts automatically. Those write-capable actions need a separately scoped remediation executor, idempotency controls, branch ownership checks, and exact-SHA verification gates. This limitation is intentional; the scanner must not imply that an error was fixed merely because it was found.
