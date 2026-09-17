# ATC Engineering — Implementation Status

## Current implementation

The repository now contains a dependency-free `atc-gates` crate implementing the first normative Control Plane release gate:

- evidence is fail-closed (`Pass` is the only non-blocking status);
- `UNKNOWN`, `Missing` and `Fail` block the operation;
- production readiness requires separation of duties;
- coder, validator, auditor and release authority must be distinct principals;
- production readiness additionally requires explicit human approval;
- the gate is deterministic and covered by unit tests.

This gate complements the existing audit/readiness implementation in `atc-maintenance`; it does not replace repository-specific security, build, test or standards validation.

## Control flow

```text
Standards → Controls → Evidence → Gate
                              ↓
                    PASS → continue
                    other → BLOCK
```

For production:

```text
Evidence PASS
    +
SoD valid
    +
Human approval
    ↓
PRODUCTION_READY
```

Any missing prerequisite remains `NOT_READY`/blocked.

## Verification

The workspace should be validated with:

```bash
cargo fmt --all -- --check
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release
```

## Boundary

`atc-standards` remains the normative source. `atc-engineering` is the enforcement and automation layer. The gate implementation therefore deliberately consumes evidence and state; it does not redefine standards.
