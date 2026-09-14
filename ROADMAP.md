# Roadmap — atc-engineering

> Versionierung der Phasen gemäß ATC-STD-VERSION-001. Jede Phase schließt mit
> Test-Evidenz + STATUS-Update (Statusleiter SPECIFIED → IMPLEMENTED → TESTED …).

## Phase 0 — Bootstrap (abgeschlossen 2026-09-14)

- [x] Repository erstellt (Owner-Direktive 14.09.2026)
- [x] Specification v1.0 (ATC-ENGINEERING-SPEC-001)
- [x] README / AGENTS / ROADMAP / STATUS / SECURITY / ARCHITECTURE
- [x] Registry-Eintrag (atc-standards/registry/repositories.yaml)
- [x] CI: Struktur-Validierung

## Phase 1 — Core & Repository Manager (v0.1.0)

- [ ] Cargo-Workspace: atc-core, atc-config, atc-repository
- [ ] Repository-Manager: Discovery via GitHub-API, Klassifikation aus Registry
- [ ] `atc-engine repo list|show|health`
- [ ] Unit-Tests (fail-closed, kein unwrap in Policy-Pfaden)

## Phase 2 — Standards Engine & Audit (v0.2.0)

- [ ] atc-standards (Crate): Registry-Loader (YAML), Rule-Compiler
- [ ] Validatoren: Struktur, Dokumente, Standards-Bindung
- [ ] atc-audit: `atc-engine audit [--org]` mit Compliance-Score
- [ ] Evidence-Schema v1 + `.atc/evidence`-Integration

## Phase 3 — Policy, Generator, Bootstrapper (v0.3.0)

- [ ] atc-policy: Merge-/Release-Policies, Gate-Definition
- [ ] CI/CD-Generator: Workflow-Erzeugung aus Policy (reproduzierbar)
- [ ] Bootstrapper: `atc-engine repo init` (Templates, Standards-Bindung)

## Phase 4 — Evidence & Release Engineering (v0.4.0)

- [ ] atc-evidence: Status-Derivation aus Evidenz-Bündeln
- [ ] Release-Gate: Devnet → Testnet → Mainnet als Policy-Kette
- [ ] GitHub-Integration: Checks, Required Statuses, Branch-Protection-Vorschläge

## Phase 5 — Agent Interface & Technology Intelligence (v0.5.0)

- [ ] atc-agent: Task/Scope/Standards/Actions/Evidence-Schnittstelle (ATC-AI-GOV)
- [ ] Dependency-Intelligence: Vulns, EOL, Technology Radar
- [ ] `atc-engine audit --org` org-weit, Compliance-Dashboard

## Offen / blockiert

- Org-weite Branch-Protection-Automatisierung: Owner-Rechte erforderlich
  (Agent schlägt vor, Owner konfiguriert).
