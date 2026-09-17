# REQUIREMENTS-DISCOVERY-001 — Target Software Requirements Discovery

**Status:** IMPLEMENTED (initial deterministic engine)

## Purpose

`atc-engineering` must be able to inspect the software being built and derive an explicit list of what that target requires. The result is a **requirements profile**, not a declaration that the target is complete.

The engine answers:

1. Which technologies and execution models are observable?
2. Which mandatory software components are required?
3. Which non-code contents are required?
4. Which tests and verification layers are required?
5. Which documentation and governance artifacts are required?
6. Which security and release controls are required?
7. Which requirements are still missing?

## Evidence rule

A detected file is only a signal. It is not proof that the corresponding subsystem is correctly implemented.

```text
Target software
      ↓
Observable repository signals
      ↓
Target profile
      ↓
Required components/content/tests/docs/security/release controls
      ↓
Missing requirements
      ↓
Implementation plan
```

This preserves the engineering principle **No Evidence, No Trust**.

## Requirement classes

| ID family | Meaning |
|---|---|
| `COMP-*` | Runtime and architectural components |
| `CONT-*` | Required specifications and inventories |
| `TEST-*` | Verification and negative/fail-closed testing |
| `DOC-*` | Architecture and operations documentation |
| `GOV-*` | Ownership and change control |
| `SEC-*` | Security and supply-chain controls |
| `REL-*` | Reproducible build and release evidence |
| `TECH-*` | Technology-specific enforcement |

## Current discovery signals

The initial implementation detects Rust, Python, JavaScript, TypeScript, shell automation, Docker and GitHub Actions from repository structure. More detectors must be added for databases, network protocols, native toolchains, OS/kernel targets, blockchain nodes, smart contracts, AI/ML runtimes, storage, cryptography and hardware dependencies.

## CLI

```bash
cargo run --release -p atc-audit-cli -- /path/to/target --requirements
```

The command exits non-zero when the derived target profile still contains missing mandatory requirements.

## Non-goals

The discovery engine does not infer correctness from names, README claims, directory presence or commit messages. It also does not automatically mark a missing requirement as implemented.

## Next implementation layers

1. Standards Registry → requirement/control mapping.
2. Recursive source and manifest capability detection.
3. Component/dependency graph extraction.
4. Requirement-to-evidence traceability.
5. Repository-specific profiles and target classes.
6. Automatic implementation task generation.
7. Re-audit and requirement closure verification.
