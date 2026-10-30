# STATUS

| Feld | Wert |
|---|---|
| Status | **IMPLEMENTED (Phase 1, Foundation Wave 2)** |
| Version | v0.1.0 |
| Engineering Loop | DISCOVER → DOCUMENT → AUDIT → CLASSIFY → REMEDIATE → IMPACT-UPDATE → RE-AUDIT → VERIFY → COMPLETE/BLOCKED |
| Evidence Plane | \`atc-evidence\` implementiert — \`ATC-EVD-001\` v1.0.0 |
| Finding Registry | \`atc-findings\` implementiert — \`ATC-FND-001\` v1.0.0 |
| Impact Graph | \`atc-graph\` implementiert — typed nodes, relations, deterministic traversal |
| Verification | \`atc-verification\` implementiert — fail-closed result model |
| Readiness | fail-closed; \`READY\` darf nur nach gültiger Verification, Governance, SoD und erforderlicher Human Approval entstehen |

## Vorhanden

- \`atc-core\` — Domain IDs, Errors und zentrale Engineering-Domain-Typen.
- \`atc-config\` — fail-closed Konfiguration und Standards-Version-Pinning.
- \`atc-standards\` — Registry-/Repository-Adapter.
- \`atc-maintenance\` — Audit, MAINT-001-Klassifizierung M0–M3, Readiness, sichere Remediation und iterative Engineering-Schleife.
- \`atc-requirements\` — Requirements-Grundlage.
- \`atc-gates\` — deterministische Control-/Release-Gates und SoD-Prüfung; P0-Härtung der Evidence-Vollständigkeit ist in PR-Prüfung.
- \`atc-audit-cli\` — Repository-Audit und Engineering-Loop CLI.
- \`atc-evidence\` — maschinenlesbare Evidence Records mit Schema, Provenance und Integrity-Feldern.
- \`atc-findings\` — Finding Registry mit erzwungenem Lifecycle und append-only Registry-Persistenz.
- \`atc-graph\` — typisierter Dependency-/Impact-Graph mit deterministischer Traversierung.
- \`atc-verification\` — Verification-Resultate und fail-closed Readiness-Auswertung.

## Finding Lifecycle

\`\`\`text
DETECTED → CLASSIFIED → PLANNED → FIXED → VERIFIED → CLOSED
                         ↘ BLOCKED
\`\`\`

Ungültige Übergänge und doppelte Finding-IDs werden abgelehnt.

## P0 Gate-Härtung — PR 1

**Status: IMPLEMENTIERUNG AUF BRANCH / CI- UND REVIEW-NACHWEIS AUSSTEHEND**

Ausgangsbefund auf \`main\` SHA \`38ab42e81847ac6bc03c2d288ec7a9b8a940c670\`:

- \`evaluate()\` blockierte vorhandene \`FAIL\`-, \`MISSING\`- und \`UNKNOWN\`-Evidence-Einträge.
- Eine leere Evidence-Liste wurde nicht explizit blockiert und konnte bei gültiger Production-SoD und Human Approval zu \`ALLOW\` führen.
- Das bisherige Eingabemodell enthielt keine explizite Liste erforderlicher Kontrollen. Eine nur teilweise vorhandene Evidence-Sammlung konnte deshalb nicht gegen einen vollständigen Pflichtumfang geprüft werden.

Korrektur im PR-Branch:

- Leere Evidence-Liste → \`BLOCK\`.
- Leere, doppelte oder ungültige Pflichtkontroll-Konfiguration → \`BLOCK\`.
- Jede Pflichtkontrolle benötigt einen expliziten \`PASS\`-Nachweis mit identischer Control-ID.
- Fehlende, unbekannte, fehlerhafte oder nicht bestandene Evidence → \`BLOCK\`.
- Der Audit-CLI bindet die Gate-Auswertung an die explizite Pflichtkontrolle \`AUDIT-REPOSITORY\`.
- Production behält die vorhandene Separation-of-Duties-Prüfung und die explizite Human-Approval-Pflicht.

Die Gate-Report-Schema-Version bleibt \`atc.gate-report/v1\`; fehlende Kontrollen werden in \`blocking_controls\` ausgewiesen. Owner-/Approver-Zuständigkeiten, Quoren, Branch Protection und Release-Autorisierung werden nicht geändert.

**Abnahme:** Positive und negative Regressionstests sind ergänzt. Cargo-Test, Format, Clippy, PR-HEAD-CI, Run→Job→Step-Logs und Artefaktintegrität sind erst nach Ausführung und Prüfung der Actions-Nachweise als verifiziert zu bewerten.

## Noch nicht vollständig implementiert

- Orchestrator-Integration von \`atc-findings\` in jeden Audit-Lauf;
- automatische Befüllung des vollständigen Impact-Graphs aus Repository-/Cargo-/Workflow-Daten;
- ausführbare Build/Test/Security/Conformance-Runner im Verification Engine;
- automatische Evidence-Erzeugung aus jedem Verification Result;
- generischer \`ImplementationExecutor\` für autorisierte semantische Codeänderungen;
- vollständige Policy-/Governance-Orchestrierung;
- GitHub Provider/PR/Checks/Release Adapter;
- Organization-wide Orchestrator;
- produktive GitHub-App-Integration.

## Verification Hinweis

Diese Welle wurde über GitHub Source Operations geschrieben. Ein erfolgreicher Cargo-Build, Test-, Format-, Clippy- oder CI-Lauf wird daraus **nicht** abgeleitet. Runtime-Verifikation bleibt eine separate Evidence-erzeugende Phase.
