# ITERATIVE-REMEDIATION-001 — Closed-Loop Engineering Remediation

**Status:** IMPLEMENTED

## Objective

ATC Engineering shall not stop after detecting a finding. It shall re-audit after every safe remediation and continue until the repository is clean, no further safe progress is possible, or a hard iteration limit is reached.

## Control Loop

```text
DISCOVER
  -> AUDIT
  -> CLASSIFY
  -> SAFE REMEDIATION
  -> UPDATE AFFECTED ARTIFACTS
  -> RE-AUDIT
       | clean -> VERIFIED CLEAN
       | changed -> loop
       | unchanged -> BLOCKED
```

## Safety Model

The remediation engine is deliberately conservative. It may automatically apply only deterministic fixes whose correctness can be established from the local repository state.

Currently implemented safe remediations:

- create `docs/ENGINEERING_AUDIT.md` when the required file is missing;
- add canonical repository identity to `README.md` when the audit proves the identity is absent.

The engine does **not** silently rewrite:

- application or kernel source code;
- credentials or security-sensitive material;
- dependency declarations;
- shell download/execution pipelines;
- unresolved TODO/FIXME work;
- duplicate files;
- CI permissions;
- semantic configuration.

Those findings remain `blocked` and must be resolved through an explicit engineering change followed by another audit.

## Deterministic Termination

`repair_until_stable` accepts a hard `max_iterations` bound. It also calculates a deterministic finding signature. If two consecutive audits produce the same finding set, the engine stops instead of looping indefinitely.

A clean result is emitted only after a fresh audit reports zero findings.

## CLI

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository --repair
cargo run --release -p atc-audit-cli -- /path/to/repository --repair --max-repair-iterations 12
```

The command prints every applied repair and a final summary. A non-clean result remains fail-closed.

## Evidence

Each applied repair records:

- iteration number;
- deterministic repair action;
- affected path;
- final audit state.

This preserves the platform principle **No Evidence, No Trust**: remediation is not considered complete merely because a write occurred; it is complete only after re-audit verification.
