# ADR-ATC-TOOLCHAIN-HYBRID-001: Hybrid Toolchain Architecture

- **Status:** Accepted as the selected architecture direction; repository ratification and implementation remain pending review and evidence.
- **Decision date:** 2026-10-12
- **Decision owner:** A-TownChain architecture/governance
- **Scope:** `A-TownChain-Okosystems/atc-toolchain`, with enforcement and evidence coordination in `atc-engineering`
- **Related repositories:** `atclang`, `a-townchain`, `atc-standards`

## 1. Context

The current `atc-toolchain/main` describes a Python stdlib-first R1 skeleton for governance and build orchestration. The `anchor/compiler-baseline` branch describes a Rust-first deterministic toolchain baseline, including bytecode verification and artifact tooling. These are different implementation directions and must not silently become competing sources of truth.

The selected architecture is **Option C — Hybrid with explicit boundaries**. This ADR records the target architecture. It does not claim that the components described here are already implemented, tested, verified, production-ready, or released.

## 2. Decision

1. Python is responsible for governance and build orchestration.
2. Rust is used for security-sensitive binary and artifact operations where its typed, memory-safe implementation provides a clear benefit.
3. Canonical language, bytecode semantics, and VM execution remain owned by their designated repositories. Toolchain modules integrate with these authorities; they do not replace them.
4. The existing R1 Python baseline remains the orchestration/governance starting point. Rust components from `anchor/compiler-baseline` may be adopted selectively only after source-SHA review, interface-contract review, tests, and provenance checks.
5. The architecture decision does not authorize a merge, release, production claim, or a migration that has not passed its required gates.

## 3. Single Sources of Truth and ownership

| Concern | Canonical owner | Toolchain responsibility |
|---|---|---|
| ATCLang language specification and canonical compiler responsibility | `atclang` | Invoke through a versioned compiler interface; do not establish a competing language or compiler authority |
| ATC-VM implementation and execution semantics | `a-townchain/components/vm` | Validate artifacts and integrate with the canonical VM; do not implement a second VM |
| Blockchain algorithm/consensus | `a-townchain/components/algorithm` | Orchestration and interface integration only; no competing consensus implementation |
| Normative standards and standard registry | `atc-standards` | Consume and enforce published standards; do not redefine normative requirements |
| Governance, CI policy, evidence and release-gate enforcement | `atc-engineering` | Own engineering controls and evidence policy, not product implementations |
| Cross-repository developer tooling | `atc-toolchain` | CLI, orchestration adapters, bytecode/artifact validation tools, packaging and evidence adapters |

A local cache, adapter, test fixture, or validator is not a new canonical authority. If sources conflict, fail closed and raise an explicit conflict for the canonical owner; do not silently choose one definition.

## 4. Runtime and language boundaries

The intended flow is:

```text
ATCLang source
  -> canonical compiler interface (owned by atclang)
  -> ATCB artifact
  -> Rust structural bytecode validator
  -> artifact integrity and provenance checks
  -> reproducible build/package result
  -> canonical ATC-VM (a-townchain/components/vm)
  -> evidence collection and policy evaluation (atc-engineering)
```

Python governance/orchestration coordinates stages and records their inputs, outputs, exit codes, logs, and artifact digests. It must not reinterpret bytecode or override validator failures. The validator must not execute programs as a substitute for the canonical VM.

## 5. Language and bytecode contracts

### 5.1 Compiler interface

The compiler boundary must be versioned and machine-checkable. Its contract must define:
- compiler identity and exact source/build revision;
- input encoding, source identity and input digest;
- target format/version and explicit configuration;
- output artifact path or transport, artifact digest, and structured diagnostics;
- deterministic error codes and non-zero exit behavior for invalid input;
- provenance fields sufficient to bind source, compiler, configuration and output.

The toolchain must call the canonical ATCLang compiler interface. Until its actual interface and implementation are inspected and pinned by exact SHA, this ADR does not prescribe an unverified CLI or ABI.

### 5.2 ATCB format

The proposed structural header contract is:

| Field | Encoding | Size |
|---|---|---:|
| Magic | ASCII `ATCB` | 4 bytes |
| Format version | unsigned 16-bit big-endian | 2 bytes |
| Instruction count | unsigned 32-bit big-endian | 4 bytes |
| Instruction stream | canonical format defined by the normative specification | variable |

The 10-byte header is a target contract, not independent proof that every repository currently implements it. The canonical format specification and the canonical VM/parser behavior must be inspected and reconciled before implementation is extended. The meaning and framing of instructions, opcode assignments, operand encoding, resource limits and semantic validity must not be invented or independently redefined by the toolchain.

`ATCB` is the selected magic in this target contract. An alternative such as `ATC1` must not be accepted or emitted as a silent fallback. Any existing conflicting definition is a release blocker until resolved by the canonical owners and documented with exact source SHAs.

### 5.3 Validator and artifact contracts

The Rust validator must:
- reject invalid magic, unsupported versions, truncated or malformed data, invalid instruction framing and unknown opcodes where defined by the canonical format;
- enforce the limits and validity rules published by the canonical format/VM specifications;
- return stable, machine-readable error codes and fail closed;
- never repair, reinterpret, or silently normalize invalid input;
- expose a versioned interface and tests for valid, invalid, boundary and adversarial inputs.

Artifact tooling must use specified canonical serialization, cryptographic digests, and explicit provenance. A digest proves identity of bytes, not semantic correctness or safety.

## 6. Responsibility split

### Python: governance and orchestration
- build-pipeline and process coordination;
- manifest, configuration and policy processing;
- CI-gate orchestration and result aggregation;
- evidence registry, audit reports and residual tracking;
- release workflow coordination, without self-approving a release;
- deterministic input/output manifests and interface invocation.

### Rust: security-sensitive tooling
- structural bytecode and binary-format validation;
- format/version and bounds checking;
- artifact integrity, canonical serialization and hashing;
- robust handling of malformed or adversarial input;
- unit, property, fuzz and regression tests appropriate to the threat model.

The language choice is a responsibility boundary, not permission to duplicate canonical business logic. Rust does not become a second VM; Python does not become a second compiler or bytecode interpreter.

## 7. Determinism and provenance

A reproducible build must bind at least:
- source repository and exact commit SHA;
- canonical compiler identity and exact revision;
- toolchain identity and exact revision;
- format/specification version;
- explicit build configuration and target;
- dependency/lockfile state where applicable;
- input and output artifact digests;
- relevant environment/tool versions and the commands executed.

For equivalent declared inputs and pinned tool versions/configuration, repeated builds must produce byte-identical artifacts or report an explicitly specified, reviewable source of nondeterminism. Timestamps, filesystem traversal order, locale, host paths and ambient environment must not affect output unless normalized or explicitly part of the contract.

## 8. Migration rules

1. Inspect `atc-toolchain/main` and `anchor/compiler-baseline` at exact commit SHAs before selecting files or behavior for migration.
2. Produce a file-level diff and classify each item as governance/orchestration, security-sensitive tooling, canonical implementation, duplicate, obsolete, or unresolved.
3. Preserve the R1 governance baseline and required CI gates. Do not replace Python orchestration wholesale merely because Rust components exist.
4. Port or adapt only components that have a clear ownership assignment, versioned interface, tests, dependency review and evidence.
5. Do not copy a VM, consensus implementation or competing language implementation into `atc-toolchain`.
6. Do not silently resolve ATCB or compiler-interface conflicts. Record each conflict, owner, exact source SHAs and disposition.
7. Every migration change must be submitted through a reviewable branch and pull request. No direct-to-main mutation, merge, auto-merge or release is authorized by this ADR.

## 9. Phased implementation and acceptance criteria

| Phase | Deliverable | Acceptance criteria |
|---|---|---|
| P0 — Architecture freeze | This ADR and ownership matrix | Option C recorded; ownership and non-goals explicit; no implementation status implied |
| P1 — Baseline reconciliation | SHA-pinned comparison of R1 `main` and `anchor/compiler-baseline` | Exact 40-character SHAs; file-level differences; dependency and capability inventory; conflicts and residuals recorded |
| P2 — Interface contracts | Versioned compiler, validator, artifact and evidence contracts | Machine-readable schemas; deterministic errors; contract and negative tests; canonical owners and source SHAs identified |
| P3 — Rust validation tools | Bytecode and artifact validation components | Valid/invalid/boundary/adversarial tests; fail-closed behavior; fuzz/property testing as applicable; no duplicate VM |
| P4 — Orchestration integration | Python orchestration invoking versioned Rust tools | End-to-end CI on a pinned source SHA; failures propagate; artifacts and provenance are retained; existing gates remain enabled |
| P5 — Verification | Exact-SHA evidence package | Run → Job → Step → exit code/log evidence; artifact hashes; all mandatory gates pass or residuals remain explicitly open |

A phase is not accepted because a document or file exists. Acceptance requires the stated evidence for the exact tested commit.

## 10. Evidence and status vocabulary

The project must distinguish at least:
- **SPECIFIED** — a normative contract has been reviewed and recorded;
- **IMPLEMENTED** — code exists at an identified commit;
- **TESTED** — named tests ran against an exact commit with recorded outcomes;
- **VERIFIED** — all required evidence and gates pass for that exact commit;
- **RESIDUAL** — an unresolved defect, conflict, missing proof, or unverified requirement remains;
- **BLOCKED** — progress is prevented by a stated dependency, conflict, or failed gate.

A green CI job is evidence for the checks it actually ran, not a blanket production certification. Missing or incomplete evidence is not a pass.

## 11. Security and governance invariants

- Fail closed on malformed bytecode, unsupported versions, missing provenance, failed integrity checks and unknown mandatory evidence.
- Do not weaken or remove formatting, test, static-analysis, security, dependency or evidence gates to obtain a green result.
- Keep architecture approval, code review, merge approval and release authorization separate.
- Keep cryptographic, protocol, VM and standards conflicts under review by their canonical owners.
- Treat any mismatch between the validator and canonical VM/specification as a blocker until resolved.
- No production or mainnet readiness claim follows from this ADR.

## 12. Consequences

**Positive:** Python's current R1 governance base can be retained; Rust can be introduced where binary safety benefits; canonical ownership remains explicit; migration and verification become evidence-driven.

**Costs and constraints:** Two language ecosystems must be tested and versioned; interfaces and error semantics need maintenance; source-SHA reconciliation and conflict resolution are required before implementation claims can be upgraded.

## 13. Open items / residuals

- [ ] Record exact commit SHAs for `atc-toolchain/main`, `anchor/compiler-baseline`, `atclang` and canonical ATC-VM source.
- [ ] Reconcile the ATCB header, instruction framing, limits and rejection behavior against canonical specifications and source.
- [ ] Inspect the actual compiler interface before freezing CLI/ABI details.
- [ ] Establish Rust/Python interface and schema versioning policy.
- [ ] Capture baseline CI and security evidence before migration.
- [ ] Obtain repository review and merge approval for this ADR.

Until these items have evidence, the architecture direction is selected, while implementation conformance remains **RESIDUAL / NOT VERIFIED**.
