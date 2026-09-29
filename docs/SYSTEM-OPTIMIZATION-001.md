# SYSTEM-OPTIMIZATION-001 — A-TownChain Ecosystem Technology & Architecture Optimization

**Date:** 2026-09-29  
**Status:** PROPOSED / implementation plan  
**Scope:** A-TownChain, ATCLang, ATC-VM, GlobusOS/ShivaCore, Aurora AI, Genesis Engine, SDK/wallet/indexer/storage, standards, and ecosystem integration.

## 1. Objective

Optimize the complete system without weakening existing governance or creating parallel SSOTs.

Priority order:

1. Correctness and canonical ownership
2. Determinism and protocol compatibility
3. Security and isolation
4. Evidence/CI integrity
5. Performance and scalability
6. Developer productivity
7. Operational simplicity

The governing architecture remains **Standalone First, Ecosystem Second**. Core repositories retain their own source, build, tests, release and API contracts. The ecosystem repository integrates and verifies; it does not become a second implementation.

## 2. Current high-impact gaps found

### O-001 — Transaction monetary type drift — CRITICAL

The canonical transaction specification already requires monetary transaction values as `u128`, while the current Rust wallet transaction and SDK still expose `amount: u64`. The TypeScript signing preimage also documents/encodes `amount` as u64.

Impact:
- protocol/SDK/wallet type mismatch;
- possible signing-vector incompatibility;
- loss of the 18-decimal monetary contract;
- cross-language serialization risk.

Required action:
- define one canonical transaction schema;
- use `u128` for economic amounts;
- define the exact wire encoding for u128;
- regenerate Rust/TypeScript SDK types and signing vectors;
- add positive and negative cross-language vectors;
- block release until Rust ↔ TypeScript ↔ wallet ↔ node vectors match.

### O-002 — Duplicate ATC-VM implementation — HIGH

The ecosystem audit identifies two VM paths:
- `components/atc-vm`
- `components/a-townchain/components/vm`

The audit reports divergence in `vm.rs` and `context.rs`, with the root workspace using the former while the nested path contains legacy transaction-domain material.

Required action:
- declare exactly one VM SSOT;
- migrate consumers to it;
- remove/archive the duplicate implementation;
- add a CI duplicate-path guard;
- reject legacy `ATC-TX-DOMAIN` in active source.

### O-003 — Cryptographic placeholder in chain path — CRITICAL

The current development path contains FNV-1a as a deterministic sequencing/root-hash placeholder. The repository itself states it is not cryptographic.

Required action:
- prohibit FNV-1a from security, commitment, state-root, transaction-ID or consensus-security roles;
- use reviewed cryptographic primitives for security-critical hashing;
- keep any experimental custom hash strictly isolated behind an explicit non-security feature boundary;
- require external cryptographic review before mainnet use.

A custom cryptographic hash must not be adopted merely because it is locally deterministic.

### O-004 — Toolchain floors are materially stale/inconsistent — HIGH

Several repository documents still specify Rust 1.75-era minimums. The current official stable Rust release is 1.98.1 as of 2026-09-03. Rust 1.98.1 also fixes a 1.98.0 miscompilation affecting trait-object vtables.

Required action:
- standardize the supported toolchain through one organization toolchain policy;
- baseline production CI on a pinned, reproducible Rust toolchain;
- use the current stable release after compatibility validation;
- prohibit per-repository undocumented toolchain drift.

### O-005 — Python baseline drift — MEDIUM

Repositories still expose Python 3.10 requirements while other parts of the ecosystem already use Python 3.11+.

Required action:
- standardize Python 3.11+ for supported Python tooling;
- pin CI matrices explicitly;
- remove stale 3.10 claims unless a compatibility requirement is demonstrated.

### O-006 — Umbrella subtree copies create synchronization complexity — HIGH

The ecosystem repository uses subtree synchronization. This is useful as integration evidence but creates a second physical copy of source history and therefore a drift surface.

Required action:
- retain subtree only where integration/build evidence genuinely requires it;
- make source-repository commit SHA the authoritative identity;
- add an automated source-SHA ↔ subtree-SHA consistency record;
- never infer implementation or verification from subtree presence;
- prevent direct domain development commits in the umbrella.

### O-007 — Status vocabulary needs one machine-enforced state model — HIGH

The standards repository explicitly distinguishes APPROVED, IMPLEMENTED, AUDITED and PRODUCTION_READY. The new AI lifecycle adds evidence-based states.

Required action:
- define one cross-repository status ontology:
  `DISCOVERED → DESIGNED → REGISTERED → IMPLEMENTED → BUILT → UNIT-TESTED → INTEGRATION-TESTED → E2E-VERIFIED → CI-VERIFIED → DOCUMENTED → VERIFIED → CANONICAL → RELEASE-READY`;
- make non-applicable states explicit;
- prohibit upward state claims without evidence;
- generate all views from SSOT.

## 3. Target technology baseline

### Core systems

| Domain | Target baseline | Rule |
|---|---|---|
| Systems language | Rust | Primary language for chain, VM, kernel-facing OS and performance-critical engine |
| Async runtime | Tokio | Standardize rather than mixing runtimes |
| Serialization | Explicit canonical binary schema + serde adapters | JSON for human/config boundaries, not consensus wire format |
| Hashing | Reviewed standard cryptographic primitives | No custom hash in security boundary without external review |
| Storage | RocksDB-class embedded KV where workload requires it | Benchmark against current workload before migration |
| Networking | QUIC-capable transport for suitable control/data paths | Keep protocol framing and consensus semantics above transport |
| Observability | OpenTelemetry-compatible traces/metrics/logs | Correlate exact commit, node, task and evidence IDs |
| Web/API | Typed schema-first interfaces | Versioned contracts; generated clients where useful |
| WASM execution | Wasmtime/WASI Component Model where sandboxed WASM is actually required | Do not replace the canonical ATC-VM without an architecture decision |
| AI boundary | Provider-neutral model/runtime adapters | No model provider becomes a privileged system dependency |
| Game runtime | Rust deterministic core + platform-specific rendering backends | AI commands enter simulation through validated deterministic commands |

Rust 1.98.1 is the current stable baseline verified from the official Rust release channel. urlRust 1.98.1 releasehttps://blog.rust-lang.org/releases/latest/

OpenTelemetry Rust currently provides traces, metrics and logs through its Rust API/SDK ecosystem, with the major signal components documented as Beta. It is therefore suitable as an observability contract, but telemetry itself must not become a consensus dependency. urlOpenTelemetry Rust documentationhttps://opentelemetry.io/docs/languages/rust/

RocksDB 11.7.0 is a current 2026 release and includes storage/recovery and scan improvements; it is a candidate, not an automatic migration target. Benchmarking must precede adoption. urlRocksDB releaseshttps://github.com/facebook/rocksdb/releases

## 4. Canonical target architecture

```text
                    ┌──────────────────────────────┐
                    │       Governance / SSOT      │
                    │ Standards · Registry · ADRs  │
                    └──────────────┬───────────────┘
                                   │
          ┌────────────────────────┼────────────────────────┐
          │                        │                        │
     ShivaCore                 A-TownChain              Genesis
      / GlobusOS                 L1/Node                 Engine
          │                        │                        │
     IPC/Capability          ATCLang → ATC-VM         Deterministic
          │                        │                  Simulation Core
       Aurora                State/Consensus                 │
          │                        │                  Editor/Production
          └────────────────────────┼────────────────────────┘
                                   │
                           Typed Integration APIs
                                   │
                        Ecosystem Verification Layer
```

### Boundary rules

- ShivaCore is TCB/kernel only.
- GlobusOS owns OS services/userspace.
- Aurora owns AI/runtime/policy orchestration, never implicit kernel authority.
- ATCLang owns language/compiler contracts.
- ATC-VM owns deterministic contract execution.
- A-TownChain owns L1 state/network/node orchestration.
- atc-algorithm owns canonical consensus/economic algorithm contracts.
- SDK/wallet own client-facing protocol construction and signing.
- Genesis owns deterministic game/runtime infrastructure.
- Ecosystem repositories verify integration and evidence only.

## 5. Required shared contracts

Create or consolidate these contracts before broad feature expansion:

1. **ATC Identity Contract** — chain/network/genesis identity.
2. **ATC Transaction Contract** — canonical field types, encoding, signing domain and vectors.
3. **ATC ABI/IR Contract** — ATCLang → IR/ABI → bytecode.
4. **ATC VM Contract** — verifier/execution/state-transition boundary.
5. **State Root / Commitment Contract**.
6. **P2P Protocol Contract** — handshake, framing, peer identity, chain identity, sync.
7. **Storage Contract** — persistence, snapshots, recovery, migration.
8. **Capability/IPC Contract** — ShivaCore ↔ GlobusOS ↔ Aurora.
9. **AI Tool/Policy Contract** — capability, approval, audit, provenance.
10. **Genesis Deterministic Simulation Contract**.
11. **Evidence Contract** — source SHA, test, CI run/job/step/log, result.
12. **Release Contract** — reproducible artifact, SBOM/provenance, signatures, rollback.

## 6. CI/CD optimization

Every core repository gets the same conceptual gates, adapted to its domain:

```text
Gate 0 Existing-First
      ↓
Contract / Registry
      ↓
Format / Lint
      ↓
Build / Type Check
      ↓
Unit
      ↓
Integration
      ↓
Security / Dependency
      ↓
Determinism / Conformance
      ↓
E2E
      ↓
Exact-SHA Evidence
      ↓
Release / Provenance
```

Required security/quality additions:
- dependency lockfile verification;
- SBOM generation;
- dependency/license policy;
- secret scanning;
- static analysis;
- fuzzing for parsers, serialization and consensus boundaries;
- property-based tests for deterministic state transitions;
- cross-language golden vectors;
- reproducible-build verification for release artifacts.

## 7. Performance optimization

Do not optimize by intuition.

Every performance change requires:
- benchmark baseline;
- workload definition;
- CPU/memory/IO/network measurements;
- regression threshold;
- exact commit;
- reproducible benchmark environment.

Priority benchmark suites:
1. transaction validation;
2. signature verification;
3. block execution;
4. state reads/writes;
5. state sync;
6. VM execution;
7. IPC;
8. Genesis ECS tick;
9. asset streaming;
10. Aurora inference/tool latency.

## 8. Security architecture

Adopt a uniform zero-trust boundary model:

```text
Identity → Capability → Policy → Authorization → Execution
                                  ↓
                               Audit
                                  ↓
                              Evidence
```

No component receives authority merely because it is inside the same repository or process.

For AI:
- deny-by-default tools;
- explicit capability grants;
- approval gates for privileged actions;
- immutable audit records;
- provenance for generated artifacts;
- no direct access to private keys;
- no direct consensus-state mutation.

For OS:
- capability-based IPC;
- least privilege;
- secure boot chain;
- signed packages;
- atomic A/B updates;
- rollback protection;
- hardware-backed key storage where available;
- explicit HAL boundary.

## 9. AI architecture

Aurora should remain model-agnostic:

```text
ModelHub
   ↓
Runtime
   ↓
Conversation / Memory
   ↓
RAG
   ↓
Agent Loop
   ↓
Policy / Capability Engine
   ↓
Tools / OS / Genesis / ATC APIs
```

All external model calls must terminate at a typed provider interface. Provider-specific semantics must not leak into the canonical policy or tool contracts.

The AI Development Lifecycle from SCR-0129 becomes the implementation process for this architecture after governance approval.

## 10. Immediate implementation waves

### Wave P0 — Protocol correctness
- Fix monetary type drift.
- Freeze canonical transaction schema.
- Generate Rust/TypeScript vectors.
- Remove legacy signing-domain paths.
- Consolidate VM SSOT.
- Remove FNV-1a from security-sensitive paths.

### Wave P1 — Build and toolchain
- Standardize Rust/Python toolchains.
- Introduce organization-wide toolchain manifest.
- Reproducible CI images.
- Lock dependency versions.
- SBOM/provenance.

### Wave P1 — Evidence
- Exact-SHA evidence bundle schema.
- Automated source-SHA/subtree-SHA verification.
- Machine-enforced lifecycle states.
- Cross-repository contract checks.

### Wave P2 — Runtime
- Canonical P2P protocol.
- Durable state/snapshot/recovery contract.
- VM verifier and execution conformance.
- multi-process/multi-node E2E.

### Wave P2 — OS/Aurora
- capability/IPC conformance;
- secure update path;
- identity/wallet isolation;
- Aurora policy/tool audit integration.

### Wave P3 — Genesis
- deterministic replay;
- fixed-timestep simulation contract;
- authoritative networking;
- asset/package provenance;
- AI command validation boundary.

## 11. Release blockers

The following MUST block a production/mainnet release:

- unresolved duplicate canonical implementation;
- transaction encoding/type mismatch;
- unaudited cryptography in a security boundary;
- missing exact-SHA evidence for mandatory gates;
- unverifiable dependency provenance;
- non-deterministic consensus/VM behavior;
- undocumented cross-repository contract drift;
- private-key exposure path;
- unsigned or rollback-unsafe system update path;
- documentation claiming a higher state than technical evidence.

## 12. Decision rule for technology changes

Technology is adopted only when it satisfies:

```Existing capability
    ↓
Measured deficiency
    ↓
Candidate comparison
    ↓
Security/licensing review
    ↓
Benchmark/conformance test
    ↓
Architecture decision
    ↓
Migration
    ↓
Exact-SHA evidence
```

No technology is introduced merely because it is newer.

## 13. Definition of optimized

The system is considered technically optimized only when:

- each critical capability has exactly one SSOT;
- interfaces are typed and versioned;
- deterministic paths are reproducible;
- security boundaries are explicit;
- every production claim has evidence;
- CI validates the actual commit;
- cross-repository contracts are automatically checked;
- observability is correlated across system boundaries;
- performance is benchmarked rather than assumed;
- documentation is generated/synchronized from authoritative state;
- integration cannot silently override standalone correctness.
