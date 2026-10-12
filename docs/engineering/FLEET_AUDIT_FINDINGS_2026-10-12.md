# Fleet Audit Findings — 2026-10-12

**Evidence status:** `FAILED / RESIDUAL`  
**Source main SHA:** `2e91ec61ac6680566cbe9b525406a9752c31e539`  
**Workflow run:** [38228214277](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277)  
**Failing job:** [114740951240 — Audit complete repository fleet](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277/job/114740951240)  
**Failing step:** `Audit, syntax-check and test repositories`, step 5, exit code `1`  
**Evidence artifact:** [fleet-audit-results](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277/artifacts/11738510943)  
**Artifact ZIP SHA-256:** `2c32f65478f1a15446e2a3d4cea4cd9f2c064155f48c347674cf738f1ddb1c39`

This is a log-derived triage record, not a claim that all findings are security vulnerabilities. The audit emits different classes: `AUDIT-002` missing governance documentation, `AUDIT-003` TODO/FIXME requiring triage, `SEC-AUDIT-003` workflow permissions policy, `CONS-AUDIT-001` lockfile evidence, `CONS-AUDIT-002` repository identity in README, and `CONS-AUDIT-003` duplicate content. Duplicate content requires ownership/SSOT review before deletion or relocation.

## Findings grouped by repository

| Repository | Evidence observed in the run | Required next action |
|---|---|---|
| `a-townchain-ecosystem` | `AUDIT-002`: missing `docs/ENGINEERING_AUDIT.md`. `CONS-AUDIT-003`: duplicate workflow/configuration and source files under nested component snapshots. `AUDIT-003`: TODO/FIXME in nested `a-townchain-os-docs/aistudio` views and `wiki/kai-os/code/backend/api/kai_routes.py`. | Decide canonical owners and whether each duplicate is an intentional snapshot, generated copy, or active implementation. Add the required audit document or correct the policy contract. Triage TODOs individually. |
| `globus-os` | `AUDIT-003`: TODO/FIXME markers in `modules/atc-windows-edition/src/{app,settings,wallet,node,explorer}.rs` and `modules/atc-linux-edition/src/{app,settings,wallet,node,explorer}.rs`. `CONS-AUDIT-003`: Linux/Windows edition source duplicates and duplicate CI files under `ci-cd-fix/`. Aggregate: 17 findings; audit marked error, security, and consistency categories. | Triage each TODO by behavior/security impact; establish whether edition copies are intentionally shared or drift-prone; inspect the exact security finding in the uploaded artifact before remediation. |
| `aurora-ai` | `AUDIT-003`: TODO/FIXME in `modules/atc-aistudio/src/components/SessionExportView.tsx`, `ProjectHubView.tsx`, and `SoftwareKnowledgeDbView.tsx`. Aggregate: 4 findings; audit marked error and security categories. | Triage the three UI markers and inspect the artifact for the fourth/security finding; do not infer that the markers themselves are security defects. |
| `atc-standards` | `SEC-AUDIT-003`: `.github/workflows/sdk.yml` has no explicit GitHub Actions permissions policy. | Add least-privilege explicit workflow permissions after checking every job/action requirement; preserve required gates and re-run security checks. |
| `genesis-engine` | `SEC-AUDIT-003`: `.github/workflows/sdk.yml` has no explicit GitHub Actions permissions policy. | Add least-privilege explicit workflow permissions after checking every job/action requirement; preserve required gates and re-run security checks. |
| `a-townchain` | `CONS-AUDIT-003`: duplicate workflow, compliance/lifecycle, determinism tool, and explorer source files across component directories. | Compare every flagged path with canonical SSOT ownership. Classify active implementation vs generated/snapshot copy; no blind deletion. |
| `a-townchain-os-docs` | `CONS-AUDIT-002`: README does not identify repository `a-townchain-os-docs`. `AUDIT-003`: TODO/FIXME in the three `aistudio/src/components` views listed above. Aggregate: 4 findings. | Correct repository identity in README and triage each TODO with source-linked disposition. |
| `genesis-chronicles` | `CONS-AUDIT-003`: `api/marketplace_routes.py` duplicates `modules/genesis-chronicles/api/marketplace_routes.py`. | Confirm canonical route implementation and whether the second path is a maintained compatibility shim or an unintended duplicate. |
| `atc-shivacore` | `SEC-AUDIT-003`: `.github/workflows/sdk.yml` has no explicit GitHub Actions permissions policy. | Add and verify least-privilege workflow permissions; do not confuse this repository-level CI finding with kernel/TCB verification. |
| `a-townchain-os` | `CONS-AUDIT-001`: `kai-os/Cargo.toml` workspace has no committed `Cargo.lock` evidence. | Decide whether the workspace is an application/binary requiring a lockfile; if so, commit it and verify reproducible dependency resolution. |
| `atc-contracts` | `CONS-AUDIT-003`: `contracts/solidity/test/ATCBridge.test.js` duplicates `modules/atc-contracts/contracts/solidity/test/ATCBridge.test.js`. | Identify canonical test path and ensure CI executes the authoritative copy; do not delete without checking workflow references. |
| `atc-toolchain` | `AUDIT-002`: missing required `docs/ENGINEERING_AUDIT.md`. | Add the governance record or update the audited requirement through reviewed policy change; do not bypass the audit rule. |
| `atc-sdk` | `SEC-AUDIT-003`: `.github/workflows/sdk.yml` has no explicit GitHub Actions permissions policy. | Add least-privilege explicit permissions and verify workflow behavior. |
| `atc-ide` | `AUDIT-003`: TODO/FIXME markers in `src/components/CustomLinterPanel.tsx`, `AutoSyncHubModal.tsx`, `AuditPanel.tsx`, `src/App.tsx`, `src/utils/projectSyncEngine.ts`, `auditEngine.ts`, and `src/apps/TodoApp.tsx`. Aggregate: 7 findings. | Triage each marker, assign disposition/owner, and add regression tests where behavior is implemented. |
| `atc-zkp` | `CONS-AUDIT-001`: `Cargo.toml` workspace has no committed `Cargo.lock` evidence. | Confirm workspace type; if an application/binary, commit the lockfile and verify reproducible dependency resolution. |
| `atc-explorer` | `CONS-AUDIT-003`: seven duplicate source/config files between root and `modules/atc-explorer/` (`vite.config.ts`, `src/search.ts`, `index.ts`, `api.ts`, `transactions.ts`, `addresses.ts`, `blocks.ts`). | Verify canonical explorer implementation and build/test entry points before consolidating duplicates. |
| `atc-wallet` | `AUDIT-003`: TODO/FIXME in root and `modules/atc-wallet/src/main.rs`. `CONS-AUDIT-003`: duplicate Python/Rust files across root and module paths. Native Rust check/test emitted `unknown start of token: \\\\` and `expected one of ! or ::, found keyword mod`; compilation failed although the aggregate audit summary also contains passing test invocations. | Inspect the exact source lines causing invalid backslash tokens, repair the canonical file, add a regression test, resolve duplicate ownership, and rerun cargo check/test on the new exact SHA. |

## Security-policy findings observed

The run emitted `SEC-AUDIT-003` for GitHub Actions workflows that do not declare an explicit permissions policy:

- `atc-standards/.github/workflows/sdk.yml`
- `genesis-engine/.github/workflows/sdk.yml`
- `atc-shivacore/.github/workflows/sdk.yml`
- `atc-sdk/.github/workflows/sdk.yml`

The full run log also emitted this rule for additional workflow paths. The complete artifact is the source of record; the list above is only the set directly mapped to repository sections in the inspected log output. This rule is a least-privilege configuration finding, not proof of a vulnerability exploit.

## Cross-cutting residuals

1. The audit reports duplicate files but does not prove whether each is erroneous. Resolve each against the architecture SSOT and build/test references before moving or deleting.
2. A TODO/FIXME marker is a triage finding, not proof of a defect. Each marker needs an owner, severity, disposition, and evidence.
3. The fleet job's aggregate summary does not provide a per-repository source SHA in the excerpt inspected here. Capture the exact audited SHA for each repository from the artifact before declaring any item VERIFIED.
4. The workflow's uploaded artifact is evidence collection; it does not remediate findings. A successful engineering self-test does not override the failed fleet job.
5. Re-run the fleet audit after repairs. Close findings only with exact SHA + Run → Job → Step → exit/log evidence. Keep unresolved or inaccessible items `BLOCKED`/`RESIDUAL`.

## Verification state

- Fleet audit: **FAILED** on source main SHA `2e91ec61ac6680566cbe9b525406a9752c31e539`.
- Engineering self-test in the same run: **SUCCESS**.
- Repository-specific remediation: **NOT VERIFIED** by this document.
- No production code, branch protection, or CI gate was changed by this triage record.
