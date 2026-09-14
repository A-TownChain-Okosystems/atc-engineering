# STATUS

| Feld | Wert |
|---|---|
| Status | **IMPLEMENTED (Phase 0, partiell)** (Statusleiter: SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED) |
| Version | v1.0.0 (Specification final, Owner-Direktive 14.09.2026) |
| Letzte Änderung | 2026-09-14 — Voll-Spec v1.0 + initiale Kerndokumente übernommen |
| Implementation | Phase 0 gestartet: `atc-core` + `atc-config` implementiert (0.1.0), 39/39 Tests, clippy `-D warnings` sauber, Release-Build OK; Phase 1 gestartet: `atc-standards` liest die echte Registry (505 Standards, 31 Repos) |
| Registry | ATC-REPO-ENG-001 (atc-standards/repositories.yaml, C2/S2/L7) |

## Was existiert

- Rust-Workspace (`Cargo.toml`, resolver 2, edition 2021, zero-dependency):
  - `crates/atc-core` 0.1.0 — Domain Types: `RepositoryId`/`StandardId`/`EvidenceId` (fail-closed Validierung), `DerivedState` (NOT_READY→…→PRODUCTION_READY, `FromStr` fail-closed, Advance-Regeln), `CoreError`/`Result` (kein unwrap im Bibliothekspfad), `Timestamp` (deterministisch, keine Systemuhr im Kern)
  - `crates/atc-config` 0.1.0 — Phase-0-Subset von `atc-engineering.toml`: Sektionen `[repository]`/`[standards]`/`[evidence]`, Standards Version Pinning inkl. `minimum<=version`-Prüfung und Wildcard-Ablehnung, Duplikat-/Unbekannt-Fehler mit Zeilenkontext
  - `crates/atc-standards` 0.1.0 — Phase-1-Registry-Client: Inline-Flow-Map-Parser (fail-closed), `StandardsRegistry` (Lifecycle idea→retired, Duplikat-/Status-Validierung, `require_normative_released`), `RepoRegistry` (Fleet-Metadaten, `governed()`, Duplikat-Prüfung), `examples/load_registry.rs` — lädt die **echte** atc-standards-Registry: 505 Standards (465 approved / 40 draft), 31 Repos (29 governed)
  - `config/examples/atc-engineering.toml` — kanonisches Beispiel
  - `atc-core::version` — `semver_key`/`is_semver` public (keine Duplikate); `RepositoryId` akzeptiert Punkt (`.github`-Repo)
- Evidenz (2026-09-14, lokal, Rust 1.98.1 stable): `cargo fmt --check` OK · `cargo clippy --workspace --all-targets --all-features -- -D warnings` = 0 Fehler · `cargo test --workspace` = 39/39 PASS (atc-core 14, atc-config 7, atc-standards 18 — frühere Angabe „atc-core 7, atc-config 11" war vertauscht) · `cargo build --release` OK
- Specification v1.0.0 vollständig: `docs/architecture/ATC-ENGINEERING-SPEC-001.md` (42 Abschnitte: 5-Ebenen-Modell, 16 Crates, CLI-Suite, Governance State Machine, Gates G0–G9, Evidence/Repository-State-Schemas, GitHub-App-Pfad, Version Pinning, Supply-Chain, P0–P3, v1.0-DoD)
- Initiale Kerndokumente verbindlich: `README.md`, `AGENTS.md`, `ROADMAP.md` (Phase 0–10), `ARCHITECTURE.md`
- Governance-Dokumentation (`docs/governance/`), Engineering-Docs (`docs/engineering/`), Threat-Model (`docs/security/`)
- Evidence-Skeleton (`.atc/evidence/evidence.yaml`)
- Struktur-Validierung in CI (`.github/workflows/ci.yml` — Owner-Push via `ci-fix/apply.sh` offen)

## Was NICHT existiert (ehrlich)

- 13 der 16 Crates noch nicht implementiert; kein CLI (`atc-engine`).
- Keine Policy-/Audit-/Evidence-Engine — nur deren Spezifikation.
- Tests bisher nur lokal, nicht in CI (Workflow liegt in `ci-fix/`, Owner-Push offen).
- Rust 1.98.1 im Sandbox; Formatierung auf 2024er-Edition-Anforderungen geprüft mit fmt.
- GitHub App existiert nicht (langfristiges Ziel §23; bis dahin PAT mit Least Privilege).

## Nächster Schritt

Phase-1-Rest: Requirement-Resolver (§24: REQ-STD-XXX-NNN maschinenlesbar), Version-Pinning gegen Registry-Version, Konformitäts-Reports; parallel CI live schalten (Owner-Push `ci-fix/apply.sh`).
