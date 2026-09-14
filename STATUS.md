# STATUS

| Feld | Wert |
|---|---|
| Status | **SPECIFIED** (Statusleiter: SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED) |
| Version | v1.0.0 (Specification final, Owner-Direktive 14.09.2026) |
| Letzte Änderung | 2026-09-14 — Voll-Spec v1.0 + initiale Kerndokumente übernommen |
| Implementation | docs_only — Crates/CLI gemäß ROADMAP Phase 0 offen |
| Registry | ATC-REPO-ENG-001 (atc-standards/repositories.yaml, C2/S2/L7) |

## Was existiert

- Specification v1.0.0 vollständig: `docs/architecture/ATC-ENGINEERING-SPEC-001.md` (42 Abschnitte: 5-Ebenen-Modell, 16 Crates, CLI-Suite, Governance State Machine, Gates G0–G9, Evidence/Repository-State-Schemas, GitHub-App-Pfad, Version Pinning, Supply-Chain, P0–P3, v1.0-DoD)
- Initiale Kerndokumente verbindlich: `README.md`, `AGENTS.md`, `ROADMAP.md` (Phase 0–10), `ARCHITECTURE.md`
- Governance-Dokumentation (`docs/governance/`), Engineering-Docs (`docs/engineering/`), Threat-Model (`docs/security/`)
- Evidence-Skeleton (`.atc/evidence/evidence.yaml`)
- Struktur-Validierung in CI (`.github/workflows/ci.yml` — Owner-Push via `ci-fix/apply.sh` offen)

## Was NICHT existiert (ehrlich)

- Kein Rust-Code, keine Crates, kein Workspace, kein CLI — alles SPECIFIED, nichts IMPLEMENTED.
- Keine Policy-Engine, keine Audit-Engine, keine Evidence-Engine — nur deren Spezifikation.
- GitHub App existiert nicht (langfristiges Ziel §23; bis dahin PAT mit Least Privilege).

## Nächster Schritt

ROADMAP Phase 0 — Foundation (v0.1.x): Rust-Workspace + `atc-core` + `atc-config` + CI-Baseline.
