# Architektur — atc-engineering

**Referenz:** Die verbindliche Gesamt-Specification liegt in
[docs/architecture/ATC-ENGINEERING-SPEC-001.md](docs/architecture/ATC-ENGINEERING-SPEC-001.md).
Dieses Dokument fasst die Kernentscheidungen zusammen.

## Position im Ökosystem

```
A-TOWNCHAIN ECOSYSTEM
        │
   ┌────┴─────────────┐
   │                  │
atc-standards    atc-engineering
Standards SSOT   Engineering Control Plane
   │                  │
   │        ┌─────────┼──────────┐
   │        ▼         ▼          ▼
   │   Governance  Engineering  Evidence
   │     Engine      Engine      Engine
   │        │         │           │
   │        └────┬────┴───────────┘
   │             ▼
   └────► Repository Fleet (L0–L7)
```

## Kernentscheidungen

| # | Entscheidung | Begründung |
|---|---|---|
| 1 | Control Plane, kein DevOps-Repo | Plattform mit CLI, Policy-, Audit-, Evidence-Engine — keine Script-Sammlung |
| 2 | Rust als Implementierungssprache | AD-008: Infrastruktur → Rust kanonisch; Nähe zu Git, CI/CD, Validatoren, Policy-Enforcement |
| 3 | Einseitiger Standards-Fluss | atc-engineering liest atc-standards; niemals umgekehrt |
| 4 | Evidence-Derivation statt Status-Pflege | `state = f(commit, ci, security, standards, approvals)` — kein manuelles STATUS.md-Claiming |
| 5 | Generator statt Kopie | Workflows/Repo-Templates werden aus Policy generiert — keine 30 manuellen Workflow-Varianten |
| 6 | Keine Konsens-Semantik | Plattform verletzt AD-008 nicht: keine on-chain Logik, reine Infrastruktur |

## Grenzen (was atc-engineering NICHT ist)

- Kein Ersatz für atc-standards (normative Autorität bleibt dort)
- Kein Produkt-Repo (keine Blockchain-Runtime)
- Kein zweites Governance-SSOT (Registry bleibt in atc-standards)
- Kein Agent mit impliziter Autorität (Agent-Governance: ATC-AI-GOV v1.0)
