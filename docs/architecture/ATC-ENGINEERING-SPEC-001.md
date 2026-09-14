# ATC Engineering — Repository Specification v1.0.0

| Feld | Wert |
|---|---|
| Repository | `A-TownChain-Okosystems/atc-engineering` |
| Version | 1.0.0 |
| Status | DRAFT → Ziel CANDIDATE |
| Primary Language | Rust |
| License | gemäß zentraler ATC-Lizenzpolitik |
| Authority | A-TownChain-Okosystems |
| Standards Authority | `atc-standards` |
| Owner-Direktive | 14.09.2026 — Michael Wroblewski |

---

## 1. Zweck

`atc-engineering` ist die zentrale Softwareplattform für:

- Software Engineering
- Repository Governance
- Standards Enforcement
- Compliance Validation
- Security Validation
- CI/CD Orchestration
- Build Engineering
- Release Engineering
- Evidence Collection
- Audit Automation
- AI-Agent Governance
- Repository Bootstrap
- Technology/Dependency Intelligence

### Normative Abgrenzung

```
atc-standards
    │
    │ normative rules
    ▼
atc-engineering
    │
    │ enforcement / validation / automation
    ▼
A-TownChain Repository Fleet
```

`atc-engineering` darf **keine normative Regel stillschweigend überschreiben**, die in `atc-standards` definiert ist.

---

## 2. Architekturprinzip

Die Plattform basiert auf fünf Ebenen:

```
┌─────────────────────────────────────────────┐
│              GOVERNANCE PLANE               │
│ Policies / Approvals / Gates / Roles        │
├─────────────────────────────────────────────┤
│              KNOWLEDGE PLANE                │
│ Standards / Schemas / Rules / Registry      │
├─────────────────────────────────────────────┤
│             ENGINEERING PLANE               │
│ Build / Test / Audit / Security / Release   │
├─────────────────────────────────────────────┤
│              EVIDENCE PLANE                  │
│ Evidence / Provenance / Attestations        │
├─────────────────────────────────────────────┤
│              EXECUTION PLANE                │
│ Git / GitHub / CI / Runners / Repositories  │
└─────────────────────────────────────────────┘
```

---

## 3. Architekturmodell

### Control Plane

Die Control Plane entscheidet, was erlaubt ist und welche Gates erfüllt werden müssen.

```
Configuration
     │
     ▼
Standards Registry
     │
     ▼
Policy Engine
     │
     ▼
Governance Engine
     │
     ▼
Gate Engine
```

### Execution Plane

Die Execution Plane führt autorisierte Aktionen aus.

```
Gate
 │
 ├── Repository operation
 ├── Build
 ├── Test
 ├── Audit
 ├── Security scan
 ├── Release
 └── Evidence generation
```

### Grundregel

```
CONTROL PLANE ≠ EXECUTION PLANE
```

Eine Policy darf eine Operation erlauben, aber die Policy Engine führt die Operation nicht selbst aus.

---

## 4. „No Evidence, No Trust"

Das zentrale Integritätsprinzip lautet:

> Declared state is not trusted state.

Beispiel: `status: production_ready` ist lediglich eine Behauptung.
Der tatsächliche Status wird aus Evidence abgeleitet:

```
Repository + Commit + Standards + Tests + Security + Audit
+ Approvals + Release evidence = Derived State
```

Daraus:

```
NOT_READY
    ↓
DEVELOPMENT
    ↓
VALIDATED
    ↓
TESTNET_READY
    ↓
PRODUCTION_READY
```

---

## 5. Rust Workspace

Die Root-Datei `Cargo.toml` definiert ein Workspace-Modell.

| Crate | Verantwortung |
|---|---|
| `atc-core` | Fundamentale Domain Types: IDs, States, Errors, Timestamps, References, Common Traits, Result Types |
| `atc-config` | Konfiguration: `atc-engineering.toml`, `repository.yaml`, `workspace.yaml`, `policy.yaml` |
| `atc-standards` | Adapter für `atc-standards`: Registry laden, Standards/Requirements/Dependencies resolven, Versionen validieren, Rules kompilieren |
| `atc-policy` | Policy Engine: Policy, Rule, Condition, Action, Exception, Expiration |
| `atc-governance` | Governance Engine: Roles, Responsibilities, Approvals, Separation of Duties, Governance Gates, Exceptions, Escalations |
| `atc-repository` | Repository Intelligence: Discovery, Metadata, Classification, File Inventory, State, Lifecycle, Ownership, Dependencies |
| `atc-github` | GitHub Integration Layer: Repositories, Branches, Commits, PRs, Issues, Actions, Releases, Checks, CODEOWNERS, Metadata |
| `atc-audit` | Audit Engine: Repository/Organization/Standard/Release/Architecture audits |
| `atc-evidence` | Evidence Engine: erzeugt unveränderliche Evidence Records |
| `atc-security` | Security Validation: Dependency Audit, Secret Detection, SBOM, License Validation, SAST, Container Scanning, Supply-Chain Verification |
| `atc-build` | Build Orchestration: Build, Cross-Build, Artifact Generation, Toolchain Validation |
| `atc-test` | Test Orchestration: Unit, Integration, System, Compliance, Conformance, Regression |
| `atc-release` | Release Engineering: Versioning, Release Candidates, Release Gates, Artifact Verification, Provenance, Promotion |
| `atc-agent` | AI Engineering Agent Governance: Agent Identity, Scope, Allowed Actions, Task, Plan, Execution, Evidence, Approval |
| `atc-template` | Repository Bootstrap Templates |
| `atc-schema` | Canonical Schemas |

GitHub bleibt eine externe Execution-/Source-Control-Plattform.

---

## 6. Verzeichnisstruktur

```
atc-engineering/
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── security.yml
│   │   ├── compliance.yml
│   │   ├── audit.yml
│   │   └── release.yml
│   ├── CODEOWNERS
│   ├── dependabot.yml
│   └── pull_request_template.md
│
├── crates/
│   ├── atc-core/
│   ├── atc-config/
│   ├── atc-standards/
│   ├── atc-policy/
│   ├── atc-governance/
│   ├── atc-repository/
│   ├── atc-github/
│   ├── atc-audit/
│   ├── atc-evidence/
│   ├── atc-security/
│   ├── atc-build/
│   ├── atc-test/
│   ├── atc-release/
│   ├── atc-agent/
│   ├── atc-template/
│   └── atc-schema/
│
├── cli/
│   └── atc-engine/
│
├── config/
│   ├── defaults/
│   ├── schemas/
│   └── examples/
│
├── policies/
│   ├── repository/
│   ├── security/
│   ├── release/
│   ├── governance/
│   └── agent/
│
├── standards/
│   ├── registry/
│   └── mappings/
│
├── schemas/
│   ├── repository/
│   ├── evidence/
│   ├── policy/
│   ├── audit/
│   ├── release/
│   └── agent/
│
├── templates/
│   ├── repository/
│   ├── rust/
│   ├── python/
│   ├── atclang/
│   └── github/
│
├── validators/
│   ├── filesystem/
│   ├── metadata/
│   ├── governance/
│   ├── security/
│   ├── standards/
│   └── architecture/
│
├── generators/
│   ├── workflows/
│   ├── documentation/
│   ├── manifests/
│   └── reports/
│
├── integrations/
│   ├── github/
│   ├── git/
│   └── ci/
│
├── docs/
│   ├── architecture/
│   ├── governance/
│   ├── engineering/
│   ├── security/
│   ├── operations/
│   └── adr/
│
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── conformance/
│   ├── fixtures/
│   └── golden/
│
├── scripts/
│
├── AGENTS.md
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
├── STATUS.md
├── Cargo.toml
└── Cargo.lock
```

---

## 7. CLI

Das primäre Interface ist `atc-engine`.

### Repository Commands

```
atc-engine repo init
atc-engine repo inspect
atc-engine repo audit
atc-engine repo validate
atc-engine repo inventory
atc-engine repo classify
```

Beispiele:

```
atc-engine repo audit atc-node
atc-engine repo inventory atclang
```

---

## 8. Organization Commands

```
atc-engine org discover
atc-engine org inventory
atc-engine org audit
atc-engine org compliance
atc-engine org report
```

Beispiel: `atc-engine org audit A-TownChain-Okosystems`

Output:

```
Organization Compliance

Repositories:       26
Compliant:          19
Warnings:            5
Critical:            2

P0: 0
P1: 2
P2: 11
P3: 24

Overall: 91.4%
```

---

## 9. Standards CLI

```
atc-engine standards list
atc-engine standards show ATC-STD-000
atc-engine standards validate
atc-engine standards sync
atc-engine standards compile
```

Beispiel: `atc-engine standards validate atc-node`

---

## 10. Policy CLI

```
atc-engine policy list
atc-engine policy show
atc-engine policy validate
atc-engine policy evaluate
```

Beispiel:

```
atc-engine policy evaluate \
  --repo atc-node \
  --operation release
```

---

## 11. Audit CLI

```
atc-engine audit repository
atc-engine audit organization
atc-engine audit standard
atc-engine audit release
```

Mit JSON: `atc-engine audit repository atc-node --format json`

---

## 12. Evidence CLI

```
atc-engine evidence collect
atc-engine evidence verify
atc-engine evidence export
atc-engine evidence inspect
```

Beispiel:

```
atc-engine evidence collect \
  --repo atc-node \
  --commit abc123
```

---

## 13. Build/Test

```
atc-engine build
atc-engine test
atc-engine test compliance
atc-engine test security
atc-engine test conformance
```

---

## 14. Release

```
atc-engine release inspect
atc-engine release prepare
atc-engine release validate
atc-engine release approve
atc-engine release promote
```

Wichtig: `release promote` darf **niemals ausschließlich aus einer lokalen Konfiguration heraus funktionieren**. Es benötigt gültige Release Evidence.

---

## 15. Agent Commands

```
atc-engine agent inspect
atc-engine agent authorize
atc-engine agent task
atc-engine agent validate
atc-engine agent evidence
```

Beispiel:

```
atc-engine agent task \
  --repository atc-node \
  --scope "network subsystem"
```

---

## 16. Governance-Modell

Die Plattform verwendet **Separation of Duties**. Mindestens folgende Rollen:

- CODER
- VALIDATOR
- AUDITOR
- RELEASE_AUTHORITY
- HUMAN_APPROVER

Eine einzelne Identität darf nicht automatisch alle Rollen besitzen.

### AI-Agent

```
AI Agent
    │
    ├── can inspect
    ├── can plan
    ├── can modify authorized scope
    ├── can test
    └── can generate evidence

aber:

AI Agent
    X
    └── cannot independently authorize production release
```

---

## 17. Governance State Machine

```
IDEA
 │
 ▼
PROPOSED
 │
 ▼
PLANNED
 │
 ▼
IMPLEMENTING
 │
 ▼
VALIDATING
 │
 ▼
AUDITING
 │
 ▼
REVIEW
 │
 ▼
APPROVED
 │
 ▼
RELEASE_CANDIDATE
 │
 ▼
PRODUCTION_READY
 │
 ▼
RELEASED
```

Bei einem kritischen Fehler: `ANY STATE → BLOCKED`

---

## 18. Policy Gates

Jede kritische Aktion besitzt Gates:

| Gate | Prüfung |
|---|---|
| G0 | Identity |
| G1 | Scope |
| G2 | Standards |
| G3 | Architecture |
| G4 | Tests |
| G5 | Security |
| G6 | Evidence |
| G7 | Review |
| G8 | Approval |
| G9 | Release |

Eine Release-Aktion:

```
G0 ✓  G1 ✓  G2 ✓  G3 ✓  G4 ✓  G5 ✓  G6 ✓  G7 ✓  G8 ✓  G9 → EXECUTE
```

Fehlt ein Gate: `RELEASE = BLOCKED`

---

## 19. Evidence Schema

```yaml
evidence:
  id: EVD-2026-000001
  type: test-result

  subject:
    repository: atc-node
    commit: abc123

  producer:
    system: atc-engineering
    component: atc-test

  timestamp: 2026-09-14T10:00:00Z

  result:
    status: PASS

  inputs:
    standards:
      - ATC-STD-000
      - ATC-STD-016

  artifacts:
    - test-report.json

  provenance:
    workflow: ci.yml
    run_id: 123456

  integrity:
    algorithm: SHA-256
    digest: "..."

  immutable: true
```

---

## 20. Evidence Types

Mindestens:

BUILD · TEST · SECURITY_SCAN · SAST · DEPENDENCY_SCAN · SBOM · LICENSE_SCAN · STANDARD_VALIDATION · POLICY_EVALUATION · AUDIT · REVIEW · APPROVAL · RELEASE · ARTIFACT · PROVENANCE

---

## 21. Repository State Schema

```yaml
repository:
  id: ATC-REPO-001
  name: atc-node

  lifecycle:
    phase: DEVELOPMENT

  derived_state:
    state: VALIDATED

  source:
    commit: abc123
    branch: main

  governance:
    policy: ATC-POLICY-001

  standards:
    registry_version: "1.3.0"

  evidence:
    required: true
    valid: true
```

---

## 22. GitHub Integration

GitHub ist ein **Provider, nicht die normative Quelle**.

Integration über `atc-github`:

```
atc-github
      │
      ├── Repository API
      ├── Git API
      ├── Pull Request API
      ├── Actions API
      ├── Checks API
      ├── Release API
      └── Issue API
```

### Pull Request Lifecycle

```
PR → Discover → Policy Evaluation → Standards Validation → CI
   → Security → Audit → Review → Approval → Merge
```

---

## 23. GitHub App

Für produktiven Betrieb sollte `atc-engineering` langfristig über eine **GitHub App** arbeiten — statt eines permanenten persönlichen Tokens.

```
GitHub App → atc-engineering
```

Minimalprinzip:

> Least Privilege + Repository Scoping + Explicit Write Operations

Die App erhält nur die Berechtigungen, die für den jeweiligen Workflow notwendig sind.

---

## 24. Standards-Anbindung

`atc-standards` bleibt SSOT.

```
atc-standards
       │
       ▼
Registry
       │
       ▼
Standard Resolver
       │
       ▼
Requirement Resolver
       │
       ▼
Policy Compiler
       │
       ▼
Validators
```

Beispiel:

```
ATC-STD-016
     │
     ├── REQ-STD-016-001
     ├── REQ-STD-016-002
     └── REQ-STD-016-003
             │
             ▼
        Validator
```

Damit können einzelne Requirements maschinenlesbar geprüft werden.

---

## 25. Standards Version Pinning

Ein Repository darf nicht unkontrolliert gegen eine beliebige aktuelle Registry-Version validiert werden.

```yaml
standards:
  registry:
    source: A-TownChain-Okosystems/atc-standards
    version: 1.3.0

  policy:
    compatibility:
      minimum: 1.3.0
      maximum: 1.x
```

Für Major-Versionen ist ein expliziter Migration Path erforderlich.

---

## 26. CI/CD

### Pull Request

```
checkout ↓ format ↓ clippy ↓ unit tests ↓ integration tests
↓ conformance ↓ security ↓ dependency audit ↓ license
↓ standards validation ↓ policy validation ↓ evidence generation
```

---

## 27. Main Branch

`main` darf nur über kontrollierte Änderungen verändert werden.

Required:

```
PR + required checks + CODEOWNERS review + governance gate
```

---

## 28. Release Pipeline

```
Commit → CI → Audit → Security → Evidence → Release Candidate
       → Human Approval → Artifact Signing → Release → Provenance
```

---

## 29. Supply-Chain Security

Pflichtkomponenten:

- SBOM
- Dependency Locking
- Dependency Audit
- Artifact Hashing
- Release Provenance
- Build Metadata
- License Validation
- Secret Detection

Optional später:

- SLSA-compatible provenance
- Sigstore/Cosign
- Reproducible Builds
- Binary Transparency

---

## 30. Repository Bootstrap

`atc-engine repo init my-new-repo` führt zu:

```
Repository
 │
 ├── detect domain
 ├── select template
 ├── resolve standards
 ├── generate governance
 ├── generate CI
 ├── generate documentation
 └── validate
```

Templates: blockchain · os · kernel · language · vm · ai · sdk · service · game · documentation

---

## 31. Technology Stack

### Core

Rust · Cargo · Tokio · Serde · Clap · Tracing · thiserror/anyhow

### Daten

Für v1.0 wird zunächst dateibasierte, portable Schemas priorisiert:

YAML · JSON · JSON Schema · TOML

Eine Datenbank wird erst eingeführt, wenn der tatsächliche Persistenzbedarf nachgewiesen ist.

---

## 32. API Boundary

Die Architektur ist von Anfang an API-fähig:

```
CLI → Application Layer → Domain Layer → Adapters
```

Später: REST API · gRPC · Web UI · Aurora integration — ohne die Domain Engine neu zu schreiben.

---

## 33. Domain Layer

Dependency Direction:

```
            DOMAIN
             /      \
            /        \
       POLICY       GOVERNANCE
          \            /
           \          /
            APPLICATION
                 │
        ┌────────┼────────┐
        ▼        ▼        ▼
      GitHub    Git       CI
```

Die Domain darf nicht direkt von GitHub abhängig sein.

---

## 34. Failure Model

Die Plattform arbeitet grundsätzlich **fail closed**.

Unknown · Missing Evidence · Invalid Policy · Invalid Signature · Missing Approval · Failed Gate · Unsupported Standard · Unknown State

führt zu:

```
BLOCK  (nicht ALLOW)
```

---

## 35. Exceptions

```yaml
exception:
  id: EXC-001
  scope: repository
  repository: atc-node

  reason: "..."

  requested_by: ...
  approved_by:
    - ...

  expires_at: ...

  controls:
    compensating: true
```

Keine unbefristeten Governance Exceptions.

---

## 36. Audit Trail

Jede mutierende Aktion muss nachvollziehbar sein:

WHO · WHAT · WHEN · WHY · SCOPE · INPUT · RESULT · EVIDENCE · APPROVAL

Beispiel:

```
Actor: agent-001
Action: modify
Repository: atc-node
Scope: src/network/*
Commit: abc123
Policy: ATC-POLICY-001
Result: PASS
Evidence: EVD-001
```

---

## 37. Initiale Priorisierung

### P0

Core domain · Configuration · Standards integration · Policy engine · Repository engine · Audit engine · Evidence engine · CLI · GitHub read integration · CI

### P1

GitHub write operations · Repository bootstrap · Security engine · Release engine · Agent governance

### P2

Web API · Dashboard · Technology radar · Advanced provenance · Organization-wide orchestration

### P3

Distributed workers · Enterprise federation · Advanced analytics · Multi-organization governance

---

## 41. V1.0 Definition of Done

v1.0.0 wird nicht nur daran gemessen, ob der Rust-Code kompiliert. Die Plattform ist erst **PRODUCTION_READY**, wenn:

```
ATC ENGINEERING v1.0
                                  │
          ┌───────────────────────┼───────────────────────┐
          │                       │                       │
     Governance                Engineering             Evidence
          │                       │                       │
       Policies                Build/Test              Provenance
       Approvals               Security                Audit
       Gates                   Release                 Attestation
          │                       │                       │
          └───────────────────────┼───────────────────────┘
                                  │
                                  ▼
                         Repository Fleet
                                  │
                                  ▼
                       Derived System State
```

### P0

- [ ] Standards können geladen werden.
- [ ] Standards können validiert werden.
- [ ] Policies können ausgewertet werden.
- [ ] Repositories können auditiert werden.
- [ ] Evidence kann erzeugt und verifiziert werden.
- [ ] Gates können fail-closed arbeiten.
- [ ] GitHub kann integriert werden.
- [ ] CI/CD funktioniert.
- [ ] Security Gates funktionieren.
- [ ] Release Readiness ist evidence-basiert.

### P1

- [ ] Repository Bootstrap funktioniert.
- [ ] GitHub Write Operations sind scoped.
- [ ] Agent Governance funktioniert.
- [ ] Release Promotion funktioniert.

---

## 42. Strategische Einordnung

```
A-TOWNCHAIN
                         │
          ┌──────────────┴──────────────┐
          │                             │
      AUTHORITY                     EXECUTION
          │                             │
          ▼                             ▼
  ┌──────────────┐              ┌─────────────────┐
  │ atc-standards│              │ atc-engineering │
  └──────┬───────┘              └────────┬────────┘
         │                               │
         │ WHAT                          │ HOW
         │                               │
         ▼                               ▼
    Standards                       Engineering
    Requirements                    Governance
    Schemas                         Audit
    Normative Rules                 Build
                                    Test
                                    Security
                                    Release
                                         │
                                         ▼
                              ┌────────────────────┐
                              │ Product Repositories│
                              └────────────────────┘
```

**Wichtigste Leitplanke:** `atc-engineering` darf nicht zum „Mega-Repository" werden, das den eigentlichen Code von `atc-node`, `atclang`, ShivaCore, GlobusOS, Aurora usw. übernimmt. Es bleibt die **Engineering Control Plane**. Die eigentlichen Produkte bleiben in ihren spezialisierten Repositories.

Damit wird `atc-engineering` das System, mit dem die gesamte A-TownChain-Organisation ihre Software baut — inklusive GSEPF-Prinzipien, Standards, Agent-Governance und Evidence-basierten Release-Gates.

---

*Die initialen Kerndokumente `README.md`, `AGENTS.md` und `ROADMAP.md` sind normative Ableitungen dieser Spezifikation. Phase-Detailplanung in [ROADMAP.md](../../ROADMAP.md).*
