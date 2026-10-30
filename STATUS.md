# STATUS

| Feld | Wert |
|---|---|
| Status | **IMPLEMENTED (Phase 1, Foundation Wave 2)** |
| Version | v0.1.0 |
| Engineering Loop | DISCOVER → DOCUMENT → AUDIT → CLASSIFY → REMEDIATE → IMPACT-UPDATE → RE-AUDIT → VERIFY → COMPLETE/BLOCKED |
| Evidence Plane | `atc-evidence` implementiert — `ATC-EVD-001` v1.0.0 |
| Finding Registry | `atc-findings` implementiert — `ATC-FND-001` v1.0.0 |
| Impact Graph | `atc-graph` implementiert — typed nodes, relations, deterministic traversal |
| Verification | `atc-verification` implementiert — fail-closed result model |
| Readiness | Gate-Prüfung blockiert explizit fehlende/fehlgeschlagene/unbekannte Evidence; leere Evidence-Sammlungen werden in PR 1 zusätzlich fail-closed behandelt. CI-Verifikation des PR-Heads steht aus. |

## Vorhanden

- `atc-core` — Domain IDs, Errors und zentrale Engineering-Domain-Typen.
- `atc-config` — fail-closed Konfiguration und Standards-Version-Pinning.
- `atc-standards` — Registry-/Repository-Adapter.
- `atc-maintenance` — Audit, MAINT-001-Klassifizierung M0–M3, Readiness, sichere Remediation und iterative Engineering-Schleife.
- `atc-requirements` — Requirements-Grundlage.
- `atc-gates` — Control-/Release-Gates; PR 1 ergänzt den expliziten Empty-Evidence-Block und Regressionstests für die blockierenden Evidence- und Production-Kontrollen.
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

## PR 1 — P0 Fail-Closed Evidence Correction

- Scope: ausschließlich `crates/atc-gates/src/lib.rs` und diese Statusdokumentation.
- Befund: Eine leere Evidence-Sammlung konnte die Evidence-Prüfung passieren, weil kein vorhandener Eintrag blockierend war.
- Korrektur: `evaluate()` blockiert jetzt leere Evidence-Sammlungen explizit.
- Regressionstests: leere Evidence, partielle Sammlung mit `Missing`, unbekannter Evidence-Status, fehlgeschlagene Evidence, ungültige SoD, Production ohne Human Approval sowie vollständige gültige Evidence.
- Schnittstellengrenze: `ReadinessInput` enthält derzeit keine normative Liste erwarteter Control-IDs. Die Vollständigkeit einer konkreten Pflichtkontrollmenge kann daher nur dann erkannt werden, wenn der vorgelagerte Producer fehlende Kontrollen als `EvidenceStatus::Missing` übergibt. Eine neue Registry oder ein neues Manifest ist ausdrücklich nicht Teil dieses PR.
- Status: Implementierung im PR-Branch; Exact-HEAD-CI und Evidence noch ausstehend. `VERIFIED` wird erst nach erfolgreicher Prüfung des exakten PR-Heads vergeben.

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

Ein erfolgreicher Cargo-Build, Test-, Format-, Clippy- oder CI-Lauf wird nicht aus GitHub Source Operations abgeleitet. Runtime-Verifikation bleibt eine separate Evidence-erzeugende Phase.
