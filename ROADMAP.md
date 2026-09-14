# ATC Engineering Roadmap

## Vision

Build the engineering and governance control plane used to develop, validate, audit and release the complete A-TownChain ecosystem.

---

## Phase 0 — Foundation

**Target: v0.1.x**

### Objectives

- [ ] Create Rust workspace
- [ ] Establish repository governance
- [ ] Implement `atc-core`
- [ ] Implement `atc-config`
- [ ] Establish schema conventions
- [ ] Establish error model
- [ ] Establish logging and tracing
- [ ] Establish CI
- [ ] Establish security baseline
- [ ] Establish initial documentation

### Exit Criteria

- Repository builds
- CI passes
- Governance files present
- Security baseline active
- Architecture documented

---

## Phase 1 — Standards Integration

**Target: v0.2.x**

- [ ] Integrate `atc-standards`
- [ ] Implement standards registry client
- [ ] Implement standard resolver
- [ ] Implement requirement resolver
- [ ] Implement standards version pinning
- [ ] Implement standards validation
- [ ] Implement machine-readable requirement mapping
- [ ] Implement conformance reports

### Exit Criteria

A repository can be validated against selected ATC standards automatically.

---

## Phase 2 — Repository Intelligence

**Target: v0.3.x**

- [ ] Repository discovery
- [ ] Repository metadata
- [ ] Repository classification
- [ ] File inventory
- [ ] Repository lifecycle
- [ ] Ownership metadata
- [ ] Dependency discovery
- [ ] Repository health reporting

### Exit Criteria

The organization repository fleet can be discovered and evaluated programmatically.

---

## Phase 3 — Policy & Governance Engine

**Target: v0.4.x**

- [ ] Policy schema
- [ ] Policy resolver
- [ ] Policy evaluator
- [ ] Governance roles
- [ ] Separation of duties
- [ ] Governance gates
- [ ] Exceptions
- [ ] Expiration handling
- [ ] Fail-closed enforcement

### Exit Criteria

Engineering operations can be evaluated against explicit policies.

---

## Phase 4 — Audit & Evidence

**Target: v0.5.x**

- [ ] Audit engine
- [ ] Evidence schema
- [ ] Evidence collection
- [ ] Evidence verification
- [ ] Evidence export
- [ ] Provenance
- [ ] Evidence integrity
- [ ] Derived state engine

### Exit Criteria

Repository and release states can be derived from verifiable evidence.

---

## Phase 5 — GitHub Integration

**Target: v0.6.x**

- [ ] GitHub App integration
- [ ] Repository discovery
- [ ] Commit integration
- [ ] Pull request integration
- [ ] Checks integration
- [ ] Actions integration
- [ ] Release integration
- [ ] Controlled write operations

### Exit Criteria

ATC Engineering can safely operate against the A-TownChain GitHub organization within explicitly authorized scopes.

---

## Phase 6 — Security Engineering

**Target: v0.7.x**

- [ ] Dependency auditing
- [ ] Secret detection
- [ ] SBOM
- [ ] SAST integration
- [ ] License validation
- [ ] Artifact hashing
- [ ] Supply-chain validation
- [ ] Security evidence

### Exit Criteria

Security validation becomes an enforceable release gate.

---

## Phase 7 — Repository Bootstrap

**Target: v0.8.x**

- [ ] Repository templates
- [ ] Governance templates
- [ ] GitHub workflow generation
- [ ] Documentation generation
- [ ] CODEOWNERS generation
- [ ] Standards mapping generation
- [ ] Repository initialization CLI

### Exit Criteria

New A-TownChain repositories can be initialized from governed templates.

---

## Phase 8 — Release Engineering

**Target: v0.9.x**

- [ ] Build orchestration
- [ ] Release candidate generation
- [ ] Release gates
- [ ] Artifact verification
- [ ] Provenance
- [ ] Approval workflow
- [ ] Release promotion
- [ ] Devnet release support
- [ ] Testnet release support
- [ ] Mainnet release support

### Exit Criteria

A complete evidence-based release lifecycle is operational.

---

## Phase 9 — AI Engineering Governance

**Target: v0.10.x**

- [ ] Agent identity
- [ ] Agent scopes
- [ ] Agent authorization
- [ ] Agent task model
- [ ] Agent evidence
- [ ] Agent audit
- [ ] Human approval boundaries
- [ ] AI engineering workflow enforcement

### Exit Criteria

AI agents can participate in engineering workflows without bypassing governance controls.

---

## Phase 10 — Production Platform

**Target: v1.0.0**

- [ ] Stable domain APIs
- [ ] Stable schemas
- [ ] Stable CLI
- [ ] Production GitHub integration
- [ ] Organization-wide governance
- [ ] Production release gates
- [ ] Evidence-backed readiness state
- [ ] Security hardening
- [ ] Reproducible builds
- [ ] Complete operational documentation

### v1.0 Definition

ATC Engineering v1.0 is production-ready when it can govern and validate the complete applicable A-TownChain repository lifecycle without relying on undocumented manual steps.

---

## Long-Term

Potential future capabilities:

- Distributed workers
- Remote execution
- Web dashboard
- Multi-organization governance
- Advanced technology radar
- Reproducible-build verification
- Artifact transparency
- Enterprise federation
- Advanced AI engineering orchestration
