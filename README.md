# ATC Engineering

> **A-TownChain Engineering & Governance Platform**

`atc-engineering` is the central engineering and governance control plane for the A-TownChain ecosystem.

It provides the software required to build, validate, audit, govern and release the A-TownChain repository fleet.

## Mission

**No Evidence, No Trust.**

Repository state, compliance state and release readiness are derived from authoritative inputs and verifiable evidence rather than declarations.

## Responsibilities

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

```text
atc-standards
      │ normative standards
      ▼
atc-engineering
      │ enforcement and automation
      ▼
A-TownChain repository fleet
```

`atc-standards` defines **what** must be true. `atc-engineering` validates and enforces those requirements.

## Core Architecture

```text
┌─────────────────────────────────────┐
│         Governance Plane            │
│ Policies / Roles / Approvals / Gates│
├─────────────────────────────────────┤
│          Knowledge Plane            │
│ Standards / Schemas / Rules         │
├─────────────────────────────────────┤
│         Engineering Plane           │
│ Build / Test / Audit / Security     │
├─────────────────────────────────────┤
│           Evidence Plane            │
│ Evidence / Provenance / Attestation │
├─────────────────────────────────────┤
│          Execution Plane            │
│ Git / GitHub / CI / Runners         │
└─────────────────────────────────────┘
```

## Current Implementation

The current main branch is **Phase 0 partial + Phase 1 partial**. The implemented workspace currently contains:

```text
crates/
  atc-core
  atc-config
  atc-standards
  atc-maintenance
```

The larger 16-crate target architecture is specified in `docs/architecture/ATC-ENGINEERING-SPEC-001.md` but is not yet implemented. Documentation must not present target crates as existing software.

## CLI

The target primary interface is `atc-engine`. **It is not yet implemented.**

Examples of the planned interface:

```text
atc-engine repo audit atc-node
atc-engine org audit A-TownChain-Okosystems
atc-engine standards validate atc-node
atc-engine policy evaluate --repo atc-node --operation release
atc-engine evidence collect --repo atc-node
atc-engine release validate atc-node
```

## Governance

ATC Engineering uses separation of duties: Coder, Validator, Auditor, Release Authority and Human Approver.

AI agents may operate only within explicitly authorized scopes. Production release requires valid evidence and human governance approval.

## Failure Policy

ATC Engineering is fail-closed. Missing, invalid or unverifiable evidence blocks the affected operation.

```text
UNKNOWN
  ↓
BLOCKED
```

## Standards

The canonical standards source is `A-TownChain-Okosystems/atc-standards`.

The `atc-standards` crate reads the canonical registry and validates lifecycle, identity, normative status and release status fail-closed. Standards remain owned by `atc-standards`; this repository is an enforcement consumer.

## Security

Security is a release requirement. The platform is designed to support dependency auditing, SBOM generation, SAST, secret detection, license validation, artifact integrity, provenance and supply-chain security.

See `SECURITY.md`.

## Development

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo build --release
```

## Status

See `STATUS.md` for the authoritative implementation/evidence state and `ROADMAP.md` for the planned progression.

Current status: **IMPLEMENTED (Phase 0, partial; Phase 1 partial)**. The repository is not a production platform yet.

## License

See `LICENSE`.
