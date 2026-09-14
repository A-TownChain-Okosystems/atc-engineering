# ATC Engineering

> **A-TownChain Engineering & Governance Platform**

"atc-engineering" is the central engineering and governance control plane for the A-TownChain ecosystem.

It provides the software required to build, validate, audit, govern and release the A-TownChain repository fleet.

## Mission

«Build software through controlled, evidence-based engineering.»

ATC Engineering follows the principle:

**«No Evidence, No Trust.»**

Repository state, compliance state and release readiness are derived from authoritative inputs and verifiable evidence rather than declarations.

## Responsibilities

ATC Engineering provides:

- Repository discovery and intelligence
- Standards enforcement
- Policy evaluation
- Governance gates
- Repository auditing
- Security validation
- Build and test orchestration
- Evidence generation
- Release engineering
- Repository bootstrapping
- GitHub integration
- AI-agent engineering governance

## Architectural Boundary

ATC Engineering is not the normative source of standards.

The authority chain is:

```
atc-standards
      │
      │ normative standards
      ▼
atc-engineering
      │
      │ enforcement and automation
      ▼
A-TownChain repository fleet
```

"atc-standards" defines **what** must be true.
"atc-engineering" validates and enforces those requirements.

## Core Architecture

```
┌─────────────────────────────────────┐
│         Governance Plane            │
│ Policies / Roles / Approvals / Gates│
├─────────────────────────────────────┤
│          Knowledge Plane             │
│ Standards / Schemas / Rules          │
├─────────────────────────────────────┤
│         Engineering Plane            │
│ Build / Test / Audit / Security     │
├─────────────────────────────────────┤
│           Evidence Plane            │
│ Evidence / Provenance / Attestation  │
├─────────────────────────────────────┤
│          Execution Plane            │
│ Git / GitHub / CI / Runners          │
└─────────────────────────────────────┘
```

## Repository

The implementation is primarily written in Rust.

```
crates/
  atc-core
  atc-config
  atc-standards
  atc-policy
  atc-governance
  atc-repository
  atc-github
  atc-audit
  atc-evidence
  atc-security
  atc-build
  atc-test
  atc-release
  atc-agent
  atc-template
  atc-schema
```

## CLI

The primary interface is:

```
atc-engine
```

Examples:

```
atc-engine repo audit atc-node
atc-engine org audit A-TownChain-Okosystems
atc-engine standards validate atc-node
atc-engine policy evaluate --repo atc-node --operation release
atc-engine evidence collect --repo atc-node
atc-engine release validate atc-node
```

## Governance

ATC Engineering uses separation of duties.

Relevant roles include:

- Coder
- Validator
- Auditor
- Release Authority
- Human Approver

AI agents may operate only within explicitly authorized scopes.

Production release requires valid evidence and human governance approval.

## Failure Policy

ATC Engineering is fail-closed.

Missing, invalid or unverifiable evidence blocks the affected operation.

```
UNKNOWN
  ↓
BLOCKED
```

## Standards

The canonical standards source is:

"A-TownChain-Okosystems/atc-standards"

Standards are resolved, compiled and evaluated by ATC Engineering.

## Security

Security is a release requirement.

The platform is designed to support:

- dependency auditing
- SBOM generation
- SAST
- secret detection
- license validation
- artifact integrity
- provenance
- supply-chain security

See "SECURITY.md".

## Development

```
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Status

The repository is initially under active development.

See:

- "STATUS.md"
- "ROADMAP.md"
- "ARCHITECTURE.md"

## License

See "LICENSE".
