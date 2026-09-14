# ATC-ENGINEERING-SPEC-001 — atc-engineering Repository Specification v1.0

| Feld | Wert |
|---|---|
| Spec-ID | ATC-ENGINEERING-SPEC-001 |
| Version | 1.0.0 (2026-09-14) |
| Auslöser | Owner-Direktive 14.09.2026 — Engineering Control Plane als zentrales Repository (ausdrücklich kein Ersatz für atc-standards) |
| Status | SPECIFIED — Implementation gemäß ROADMAP.md Phase 1ff. |
| Sprache | Rust (kanonisch, AD-008: Infrastruktur) — Python/TypeScript nur für dokumentierte Integrations-Glue |
| Registry | ATC-REPO-ENG-001 · Layer L7 · Domain engineering_governance · Criticality C2 · Security S2 |

---

## 1. Purpose & Scope

> A-TownChain Engineering & Governance Platform — Software platform for
> building, validating, governing, auditing and releasing the A-TownChain
> ecosystem.

atc-engineering ist die **Engineering Control Plane** des Ökosystems. Sie
operiert auf der Repository-Fleet (L0–L7), nicht auf Chain-Daten. Sie
implementiert **keine Konsens-Semantik** und verletzt damit AD-008 nicht.

### 1.1 Drei-Ebenen-Modell

```
┌────────────────────────────────────────────────────┐
│  1. KNOWLEDGE / AUTHORITY   — atc-standards         │
│     "WHAT MUST BE TRUE?"                            │
├────────────────────────────────────────────────────┤
│  2. ENGINEERING / GOVERNANCE — atc-engineering      │
│     "HOW IS IT BUILT AND VERIFIED?"                 │
├────────────────────────────────────────────────────┤
│  3. PRODUCT / RUNTIME — ATCLang, ShivaCore,         │
│     A-TownChain, ATC-VM, Aurora, GlobusOS, Genesis │
└────────────────────────────────────────────────────┘
```

### 1.2 Verhältnis zu atc-standards (kardinale Regel)

**Standards-Fluss ist einseitig:**

```
atc-standards (normatives SSOT)
        │  READ-ONLY
        ▼
Standards Registry (registry/standards.yaml, repositories.yaml)
        ▼
Rule Compiler (atc-engineering)
        ▼
Maschinenlesbare Policies (policies/)
        ▼
Repository-Validatoren (execution)
```

atc-engineering **liest** Regeln aus atc-standards und kompiliert sie zu
Policies und Validatoren. atc-standards bleibt normative Autorität
(„Was muss gelten?"); atc-engineering beantwortet „Wie wird sichergestellt,
dass es gilt — und wie wird daraus Software gebaut?".

### 1.3 Was atc-engineering NICHT ist

- Kein Ersatz für atc-standards (keine normativen Texte)
- Kein DevOps-/Scripts-/Tools-/Admin-/CI-Sammlungs-Repo (Name bewusst:
  Plattform, nicht Hilfsmittel-Sammlung)
- Kein zweites Governance-SSOT (Registry bleibt in atc-standards)
- Kein Produkt-Repo (keine Blockchain-Runtime, keine Konsens-Logik)

---

## 2. Architektur

```
                ATC ENGINEERING PLATFORM

                     CONTROL PLANE
                          │
      ┌───────────────────┼───────────────────┐
      │                   │                   │
 Governance           Engineering           Evidence
 Engine                Engine               Engine
      │                   │                   │
      ▼                   ▼                   ▼
 Standards            Build/CI             Audit Records
 Policies             Testing              Evidence
 Gates                Release               Provenance
 Approvals            Deployment           Compliance
      │                   │                   │
      └───────────────────┼───────────────────┘
                          ▼
                   EXECUTION PLANE
                          │
      ┌───────────────────┼───────────────────┐
      ▼                   ▼                   ▼
   GitHub              CI/CD                Local
   Repos               Runners              Runtime
```

### 2.1 Engine-Verantwortlichkeiten

| Engine | Verantwortung | Kern-Crates |
|---|---|---|
| Governance Engine | Standards-Kompilierung, Policies, Gates, Approvals | atc-standards, atc-governance, atc-policy |
| Engineering Engine | Build, Test, Workflows, Bootstrapping, Release | atc-build, atc-test, atc-repository, atc-release |
| Evidence Engine | Evidenz-Bündel, Status-Derivation, Provenance, Compliance | atc-evidence, atc-audit, atc-security |

---

## 3. Verzeichnisstruktur

```
atc-engineering/
├── .github/
│   ├── workflows/          # eigene CI (generiert aus Policy, sobald Phase 3)
│   ├── CODEOWNERS
│   └── dependabot.yml      # (Phase 1)
├── crates/
│   ├── atc-core/           # Domänen-Typen, Fehler-Modell, Statusleiter
│   ├── atc-config/         # Registry-/Policy-Loader (YAML/JSON), Schema-Validierung
│   ├── atc-governance/     # Governance Engine: Gates, Approvals, SCR-Anbindung
│   ├── atc-policy/         # Policy Engine: Merge-/Release-Policies, Evaluation
│   ├── atc-standards/      # Standards Engine: Registry-Loader, Rule Compiler
│   ├── atc-audit/          # Audit Engine: Repo-/Org-Audits, Scoring
│   ├── atc-evidence/        # Evidence Engine: Bündel, Derivation, Provenance
│   ├── atc-repository/      # Repository Manager: Fleet, Lifecycle, Health
│   ├── atc-build/           # Build-Orchestrierung
│   ├── atc-test/            # Test-Orchestrierung, Determinism-Gates
│   ├── atc-security/        # Security-Scans, Dependency-Intelligence
│   ├── atc-release/         # Release Engineering: Devnet→Testnet→Mainnet
│   └── atc-agent/           # AI-Agent-Schnittstelle (ATC-AI-GOV-konform)
├── cli/
│   └── atc-engine/          # Binär-Crate: atc-engine CLI
├── policies/                # Generierte + handgepflegte Engineering-Policies
├── schemas/                 # JSON-Schemas: repository, policy, evidence
├── templates/               # repo/, rust/, python/, atclang/, github/
├── validators/              # Standard-Validatoren (je ATC-STD eine Regel-Datei)
├── integrations/
│   └── github/              # GitHub-API-Client (read-mostly), Checks, Webhooks
├── docs/
│   ├── architecture/        # SPEC (dieses Dokument), Architektur-Entscheidungen
│   ├── governance/          # Governance-Modell, Gate-Definitionen
│   ├── engineering/         # Module-Handbücher
│   ├── security/            # Security-Policy, Threat-Modell
│   └── operations/          # Betrieb, Runbooks
├── tests/                   # Integrations-Tests (CLI-End-to-End)
├── AGENTS.md · ARCHITECTURE.md · CHANGELOG.md · CONTRIBUTING.md
├── LICENSE · README.md · ROADMAP.md · SECURITY.md · STATUS.md
└── .atc/evidence/evidence.yaml   # Ehrlicher Evidenz-Stand (Statusleiter)
```

---

## 4. Rust-Crates

| Crate | Zweck | Abhängigkeiten |
|---|---|---|
| `atc-core` | Domänen-Typen (Repository, Layer, Status), Fehler-Modell (fail-closed, kein unwrap im Policy-Pfad), Statusleiter SPECIFIED→…→RELEASED | – |
| `atc-config` | Loader für repositories.yaml, standards.yaml, Policy-Dateien; Schema-Validierung | atc-core, serde, serde_yaml |
| `atc-standards` | Standards-Engine: Registry-Lesen, **Rule Compiler** (ATC-STD → maschinenlesbare Policy → Validatoren-Set) | atc-core, atc-config |
| `atc-policy` | Policy-Engine: Merge-Gates (Reviews, CI, Security-Scan), Release-Gates (Human-Approval, Evidence-Bundle, Changelog, Provenance); Evaluation fail-closed | atc-core, atc-config |
| `atc-governance` | Gate-Orchestrierung, Approval-Trail, SCR-Referenzen, Owner-Mandat-Prüfung | atc-policy, atc-evidence |
| `atc-repository` | Fleet-Discovery (GitHub-API), Klassifikation aus Registry, Lifecycle, Health, Maturity | atc-core, integrations/github |
| `atc-audit` | Audit-Engine: je Repo und org-weit; Kategorien Governance/Architecture/Security/Docs/Testing/CI/Dependencies/Standards; Score 0–100, GATE ab 85 | atc-standards, atc-repository, atc-evidence |
| `atc-evidence` | Evidenz-Bündel, **Status-Derivation** (state = f(commit, ci, security, standards, approvals)), Provenance, Compliance-Records | atc-core |
| `atc-build` | Build-Orchestrierung, Toolchain-Pinning, Reproducibility | atc-core |
| `atc-test` | Test-Orchestrierung inkl. **Determinism-Gates** (Org-Standard D-CRITICAL) | atc-core |
| `atc-security` | Security-Scan-Integration (cargo audit, etc.), Dependency-Intelligence: Vulns, EOL, License-Konflikte, Technology Radar | atc-core, integrations/github |
| `atc-release` | Release-Pipeline: Devnet→Testnet→Release-Gate→Mainnet als Policy-Kette mit Human-Approval | atc-policy, atc-evidence |
| `atc-agent` | AI-Agent-Schnittstelle: Task+Scope+Standards+AllowedActions+EvidenceRequirements je Auftrag; **keine implizite Autorität** | atc-governance |

Workspace: ein Cargo-Workspace, `cargo fmt`/`clippy -D warnings`/`test` in CI,
kein unwrap() in Policy-/Audit-/Evidence-kritischen Pfaden (Owner-Regel,
analog Konsens-Kritikalität).

---

## 5. CLI — `atc-engine`

```
atc-engine <command> [options]

Repository Manager:
  atc-engine repo list [--layer L5] [--domain services]
  atc-engine repo show <name>
  atc-engine repo health [--org]
  atc-engine repo init <name> [--template rust|python|atclang] [--layer L7]

Standards Engine:
  atc-engine standards compile          # Registry → Policies → Validatoren
  atc-engine standards validate <repo>

Audit:
  atc-engine audit <repo>              # Kategorie-Score + Findings P0–P3
  atc-engine audit --org               # gesamte Organisation

Policy:
  atc-engine policy check <repo>       # Merge-/Release-Policy auswerten
  atc-engine policy apply <repo>       # Branch-Protection-Vorschlag/Owner-Aktion

Generator:
  atc-engine generate workflows <repo> # CI aus Policy (reproduzierbar)
  atc-engine generate matrix           # Code-Quality-Matrix (SSOT: Registry)

Evidence:
  atc-engine evidence collect <repo>
  atc-engine evidence derive <repo>    # Status-Derivation (kein Claiming)
  atc-engine evidence verify <repo>    # Bündel gegen Realität prüfen

Release:
  atc-engine release gate <repo> [--env devnet|testnet|mainnet]
  atc-engine release status [--org]

Agent:
  atc-engine agent task <task.yaml>    # ATC-AI-GOV-konformer Auftrag
```

Audit-Output (Beispiel):

```
Repository: atc-node

Governance        PASS
Architecture      PASS
Security          PASS
Documentation     WARN
Testing           PASS
CI/CD             PASS
Dependencies      WARN
Standards         PASS

Compliance: 94/100 (GATE ≥ 85: PASS)
P0: 0 · P1: 1 · P2: 3 · P3: 5
```

---

## 6. Governance-Modell

### 6.1 Engineering Policy (Beispiel `policies/repository.yaml`)

```yaml
repository:
  id: ATC-REPO-NODE-001
  name: atc-node
  layer: L7
  domain: blockchain
  lifecycle: development
  criticality: critical

governance:
  standards:           # verbindlich über atc-standards Registry
    - ATC-STD-000
    - ATC-STD-VERSION-001
    - ATC-STD-ENG-001

required:
  security: true       # Security-Scan in CI
  codeowners: true
  ci: true
  changelog: true
  roadmap: true
  determinism: true    # D-CRITICAL-Repos: Determinism-Gate

merge:
  required_reviews: 2
  codeowners_required: true
  ci_required: true
  security_scan_required: true

release:
  human_approval: required
  evidence_bundle: required
  changelog: required
  provenance: required
```

### 6.2 Pipeline

```
Developer → PR → Engineering Policy ─┬─ Tests
                                     ├─ Security
                                     ├─ Standards
                                     ├─ Architecture
                                     ├─ Dependencies
                                     └─ Evidence
                          → Release Gate → Human Approval → Release
```

### 6.3 Release-Lifecycle (maschinenlesbar)

```
SOURCE → BUILD → TEST → DEVNET (Alpha→Beta) → TESTNET (Alpha→Beta)
       → RELEASE GATE → HUMAN APPROVAL → MAINNET
```

### 6.4 Agent-Governance (ATC-AI-GOV v1.0)

Agenten erhalten **keine implizite Autorität**. Jeder Auftrag besteht aus:

```
Task + Scope + Applicable Standards + Repository Policy
     + Allowed Actions + Evidence Requirements
```

Workflow: DISCOVER → UNDERSTAND → PLAN → IMPLEMENT → TEST → AUDIT →
DOCUMENT → REVIEW → COMMIT → PR → HUMAN APPROVAL → MERGE.

No Self-Certification: der Agent zertifiziert nie sein eigenes Ergebnis;
Evidence Engine verifiziert gegen die Realität (Commits, CI-Runs).

---

## 7. Daten- & Evidence-Schemas

### 7.1 Evidence-Bundle (`schemas/evidence-bundle.schema.json`)

```json
{
  "repository": "atc-node",
  "state": "PRODUCTION_READY",
  "derived_from": [
    "commit:abc123",
    "ci:passed",
    "security:passed",
    "standards:passed",
    "review:approved",
    "release:evidence-001"
  ],
  "generated_by": "atc-engine v0.4.0",
  "generated_at": "2026-09-14T12:00:00Z",
  "verifiable": true
}
```

**Kernprinzip:** `state` wird nie behauptet, sondern aus Evidenz **abgeleitet**
(DERIVED). Das ersetzt manuell gepflegte STATUS.md-Claims durch überprüfbare
Derivation — STATUS.md bleibt als menschenlesbare Projektion erhalten.

### 7.2 Repository-Metadaten (`schemas/repository.schema.json`)

```yaml
repository:
  id: ATC-REPO-001
  name: atc-node
  layer: L7
  domain: blockchain
  lifecycle: development
  criticality: critical
  canonical: true
  evidence: .atc/evidence/evidence.yaml
```

### 7.3 Status-Derivation (Evidence Engine)

```
state = f(repository_state, commit, ci_results, security_results,
          standards_validation, architecture_validation, approvals,
          release_evidence)
```

Konflikt-Regel: fehlende oder widersprüchliche Evidenz → Status bleibt
niedriger / wird auf UNKNOWN gesetzt. Fail-closed, kein Guessing.

---

## 8. Standards-Anbindung

| Standards-Verantwortung | Wo |
|---|---|
| Normative Standards, Registry, SCR-Prozess | atc-standards (SSOT) |
| Rule-Compiler (STD → Policy → Validator) | atc-engineering/crates/atc-standards |
| Validatoren-Ausführung je Repo | atc-engineering/validators/ |
| Findings (P0–P3) | atc-engineering → atc-standards Finding-Format |

Der Rule Compiler mappt z.B.:

```
ATC-STD-016 (File Inventory required)
   → Validator: docs/inventory vorhanden & aktuell
   → PASS / FAIL + Evidence
```

Governance-Findings folgen ATC-FINDING-Jahr-Nummer (ATC-AI-GOV).

---

## 9. GitHub-Integration

- **Read-mostly API-Client** (`integrations/github/`): Fleet-Discovery,
  Checks-Status, CI-Ergebnisse, Branch-Protection-Lesen.
- **Schreiboperationen** (Branch-Protection setzen, Required Checks) nur als
  **Owner-Vorschlag**: atc-engine erzeugt die exakte Konfiguration; der Owner
  bestätigt (Token-Scope des Agenten bleibt minimal — niemals Org-Admin).
- Webhook-Receiver (Phase 4+): PR-Events → Policy-Evaluation → Check-Status.
- Org-weite Audits: `--org` mit-pagination, Rate-Limit-fair.

## 10. CI/CD (dieses Repos)

Phase 0 (jetzt): Struktur-Validierung — Pflichtdokumente vorhanden,
SPEC-Version gebunden, Evidence-Datei ehrlich.

Phase 1+: `cargo fmt --check`, `cargo clippy --D warnings`, `cargo test`,
`cargo audit` (analog atc-vm RustSec-Gate), Struktur-Gate. Ab Phase 3 werden
die eigenen Workflows aus der Policy generiert (Hund-Futter-Prinzip: die
Plattform isst ihren eigenen Hund).

---

## 11. Compliance & Metrik

- Org-Audit V-01..V-16 (atc-standards), GATE ab Score 85.
- Determinismus: Audit-/Policy-Entscheidungen reproduzierbar (feste
  Sortierung, keine Wall-Clock-Abhängigkeit; ATC-STD-ENG-001).
- Registry-Eintrag: ATC-REPO-ENG-001, Layer L7, Domain engineering_governance,
  Criticality C2, Security S2, Maturity C, canonical: true (capability:
  engineering_governance).

## 12. Changelog der Spec

- v1.0.0 (2026-09-14): Erste verabschiedete Fassung (Owner-Direktive
  14.09.2026). Änderungen nur via SCR in atc-standards + Owner-Freigabe.
