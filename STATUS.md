# STATUS

## Current main evidence snapshot — 2026-10-12

This snapshot is bound to main commit `2e91ec61ac6680566cbe9b525406a9752c31e539`. It supersedes the older fleet-audit snapshot recorded against `470e6a9f500fdade0ea33065d30dd76b58d6492b`; that older result must be treated as historical, not current evidence.

- **Fleet Audit run:** [38228214277](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277) — completed with **failure** on the exact main SHA above.
- **Engineering self-test:** job [114740882505](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277/job/114740882505) — **success**.
- **Fleet audit job:** [114740951240](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277/job/114740951240) — **failure**. Step 5, “Audit, syntax-check and test repositories”, returned exit code 1. The audit summary and uploaded evidence artifact are available in the run.
- **Evidence artifact:** [fleet-audit-results](https://github.com/A-TownChain-Okosystems/atc-engineering/actions/runs/38228214277/artifacts/11738510943), archive SHA-256 `2c32f65478f1a15446e2a3d4cea4cd9f2c064155f48c347674cf738f1ddb1c39`.

### Fleet findings requiring remediation or further evidence

The run summary flagged these repositories as requiring remediation and/or additional verification evidence:

- `a-townchain-ecosystem`
- `globus-os`
- `aurora-ai`
- `atc-standards`
- `genesis-engine`
- `a-townchain`
- `a-townchain-os-docs`
- `genesis-chronicles`
- `atc-shivacore`
- `a-townchain-os`
- `atc-contracts`
- `atc-toolchain`
- `atc-sdk`
- `atc-ide`
- `atc-zkp`
- `atc-explorer`
- `atc-wallet`

These are **FINDINGS**, not a claim that each repository has a confirmed code defect. The audit summary groups audit and/or native-verification requirements; each repository needs its own exact finding, log, exit result, and remediation/evidence disposition. Do not mark these repositories VERIFIED from this summary alone.

### Concrete findings extracted from the failed fleet-audit log

The run log contains more specific evidence than the aggregate repository summary. The following findings are directly visible in job `114740951240` and should be handled in the owning repositories, not patched in this governance repository.

| Repository | Exact log evidence | Classification | Required next action |
|---|---|---|---|
| `atc-explorer` | npm dependency resolution failed: root selects `typescript@7.0.2`, while `ts-jest@29.4.14` declares peer range `typescript >=4.3 <7`. The subsequent test command reports `jest: not found`. | Dependency/toolchain failure; the missing Jest executable may be a consequence of the failed install, not a separate root cause. | Choose a mutually compatible TypeScript / Jest / ts-jest set; update and lock the dependency graph; rerun clean install and tests on the resulting exact SHA. Do not use `--force` or `--legacy-peer-deps` to mask the conflict. |
| `atc-wallet` | Rust compiler reports literal `\\n` text in `src/lib.rs` at lines 8 and 11 (for example `pub mod tx;\\npub mod wallet;`), followed by parser errors and failed compilation. | Confirmed source-format/compile failure in the audited checkout. | Replace literal escape text with actual line breaks; run formatter, `cargo check`, and `cargo test` on the fixed SHA. |
| `atc-wallet` | Audit reports `AUDIT-003` unresolved TODO/FIXME markers in both `modules/atc-wallet/src/main.rs` and `src/main.rs`. | Open code-completeness finding; duplicated paths also appear in consistency findings. | Triage each marker against intended behaviour, implement or explicitly track the residual, and add regression tests where behaviour is completed. Do not remove markers solely to satisfy the scanner. |
| `atc-wallet` | Consistency findings report identical/duplicate content across root and `modules/atc-wallet` paths, including `wallet/ecdsa.py`, `setup.py`, `src/main.rs`, `src/crypto.py`, `src/wallet.py`, and `src/__init__.py`. | Possible duplicate implementation / SSOT drift; duplicate detection alone does not prove the copies are erroneous. | Compare the copies, identify the canonical path and runtime/build references, then consolidate or document an intentional mirror with an automated drift check. Do not delete a copy before proving ownership and callers. |

**Scope limitation:** these are actionable examples extracted from the available failed job log, not a complete finding-by-finding export for all 17 flagged repositories. The current log summary does not establish the exact root cause for every flagged repository. Each remaining repository must be re-audited and have its own finding IDs, file/line references, source SHA, job/step evidence, owner, and remediation disposition before it can be closed.

**Security priority:** these general fleet findings do not replace or close the separately tracked 16 prioritized vulnerabilities. Those require their own advisory/CVE or internal finding identifier, affected component and version range, canonical owner, fix commit, regression test, and exact-SHA verification evidence.


- Machine-readable 17-entry ledger: [`docs/engineering/FLEET_AUDIT_FINDINGS_2026-10-12.json`](docs/engineering/FLEET_AUDIT_FINDINGS_2026-10-12.json). Each repository entry preserves `observed_commit_sha: null` where the audited checkout SHA could not be established; the engineering repo's main SHA is not substituted for repository source SHAs.

### Mainline implementation gaps still open

- The organization-wide function/component catalog is specified, but the catalog specification explicitly says the inventory has not yet been extracted and implementation is pending.
- Complete, reproducible coverage across active, archived, inaccessible, and private repositories still needs generated manifests and coverage evidence; parser failures and exclusions must be explicit.
- The remediation-intake workflow is evidence collection only. It does not repair code, and its documentation records that it examines the first 100 workflow runs without pagination.
- The existing implementation gaps listed below remain open until code plus exact-SHA verification evidence closes them.

---

| Feld | Wert |
|---|---|
| Status | **IMPLEMENTED (Phase 1, Foundation Wave 2)** |
| Version | v0.1.0 |
| Engineering Loop | DISCOVER → DOCUMENT → AUDIT → CLASSIFY → REMEDIATE → IMPACT-UPDATE → RE-AUDIT → VERIFY → COMPLETE/BLOCKED |
| Evidence Plane | `atc-evidence` implementiert — `ATC-EVD-001` v1.0.0 |
| Finding Registry | `atc-findings` implementiert — `ATC-FND-001` v1.0.0 |
| Impact Graph | `atc-graph` implementiert — typed nodes, relations, deterministic traversal |
| Verification | `atc-verification` implementiert — fail-closed result model |
| Readiness | fail-closed; `READY` darf nur nach gültiger Verification, Governance, SoD und erforderlicher Human Approval entstehen |

## Vorhanden

- `atc-core` — Domain IDs, Errors und zentrale Engineering-Domain-Typen.
- `atc-config` — fail-closed Konfiguration und Standards-Version-Pinning.
- `atc-standards` — Registry-/Repository-Adapter.
- `atc-maintenance` — Audit, MAINT-001-Klassifizierung M0–M3, Readiness, sichere Remediation und iterative Engineering-Schleife.
- `atc-requirements` — Requirements-Grundlage.
- `atc-gates` — fail-closed Control-/Release-Gates und SoD-Prüfung.
- `atc-audit-cli` — Repository-Audit und Engineering-Loop CLI.
- `atc-evidence` — maschinenlesbare Evidence Records mit Schema, Provenance und Integrity-Feldern.
- `atc-findings` — Finding Registry mit erzwungenem Lifecycle und append-only Registry-Persistenz.
- `atc-graph` — typisierter Dependency-/Impact-Graph mit deterministischer Traversierung.
- `atc-verification` — Verification-Resultate und fail-closed Readiness-Auswertung.

## Finding Lifecycle

```text
DETECTED → CLASSIFIED → PLANNED → FIXED → VERIFIED → CLOSED
                         ↘ BLOCKED
```

Ungültige Übergänge und doppelte Finding-IDs werden abgelehnt.

## Noch nicht vollständig implementiert

- Orchestrator-Integration von `atc-findings` in jeden Audit-Lauf;
- automatische Befüllung des vollständigen Impact-Graphs aus Repository-/Cargo-/Workflow-Daten;
- ausführbare Build/Test/Security/Conformance-Runner im Verification Engine;
- automatische Evidence-Erzeugung aus jedem Verification Result;
- generischer `ImplementationExecutor` für autorisierte semantische Codeänderungen;
- vollständige Policy-/Governance-Orchestrierung;
- GitHub Provider/PR/Checks/Release Adapter;
- Organization-wide Orchestrator;
- produktive GitHub-App-Integration.

## Verification Hinweis

Diese Welle wurde über GitHub Source Operations geschrieben. Ein erfolgreicher Cargo-Build, Test-, Format-, Clippy- oder CI-Lauf wird daraus **nicht** abgeleitet. Runtime-Verifikation bleibt eine separate Evidence-erzeugende Phase.
