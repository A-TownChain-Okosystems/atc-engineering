# A-TownChain-Okosystems — Repository Audit Matrix

> Date: 2026-09-16
> Status: **IN PROGRESS**
> Purpose: one auditable inventory for the organization-wide engineering audit.

## Audit contract

Each repository is processed through:

`DISCOVER → CLASSIFY → DOCUMENT → IMPLEMENT/FIX → TEST → RE-AUDIT → VERIFY → DOCUMENT STATE`

Required dimensions:

- code, syntax and logic
- functional behavior and tests
- security and dependency risk
- architecture and interfaces
- integration/connectivity
- completeness/stubs/TODO/FIXME/HACK
- duplicates/dead/legacy sources
- README/STATUS/SECURITY/CHANGELOG
- roadmap/TODO/sprints/wiki
- contradiction detection and resolution
- class/category/family/tag classification

A repository is **COMPLETE** only when all applicable dimensions have current evidence. `IN PROGRESS` is the default until verified.

## Fleet

| Repository | Audit state | Known current focus |
|---|---|---|
| `.github` | IN PROGRESS | org governance / shared automation |
| `atc-standards` | IN PROGRESS | standards registry/conformance consistency |
| `atclang` | IN PROGRESS | compiler/VM boundary and implementation completeness |
| `a-townchain` | IN PROGRESS | deterministic L1 / consensus integration |
| `a-townchain-os` | IN PROGRESS | system integration/deployment surface |
| `a-townchain-os-docs` | IN PROGRESS | stale roadmap/TODO/wiki claims |
| `atc-shivacore` | IN PROGRESS | historical source boundary; canonical source moved |
| `atc-contracts` | IN PROGRESS | contract/ATVM integration |
| `atc-node` | IN PROGRESS | node/runtime integration |
| `atc-compute` | IN PROGRESS | compute/runtime interfaces |
| `atc-oracle` | IN PROGRESS | oracle/security integration |
| `atc-sdk` | IN PROGRESS | SDK structure/docs consistency |
| `atc-marketplace` | IN PROGRESS | application/API integration |
| `atc-wallet` | IN PROGRESS | key/security/integration |
| `atc-vm` | IN PROGRESS | deterministic VM / ATCLang boundary |
| `atc-zkp` | IN PROGRESS | proof verification/integration |
| `aurora-ai` | IN PROGRESS | AI boundary / OS integration |
| `globus-os` | **P1 OPEN** | canonical ShivaCore LKM stub + symbol export/import semantics |
| `genesis-engine` | IN PROGRESS | engine subsystem completeness |
| `genesis-chronicles` | IN PROGRESS | game/application integration |
| `atc-mining` | IN PROGRESS | mining/runtime integration |
| `atc-indexer` | IN PROGRESS | chain/indexer consistency |
| `atc-interop` | IN PROGRESS | cross-chain/interface security |
| `atc-launchpad` | IN PROGRESS | deployment/application integration |
| `atc-explorer` | IN PROGRESS | indexer/API/UI integration |
| `atc-storage` | IN PROGRESS | persistence/runtime correctness |
| `atc-algorithm` | IN PROGRESS | deterministic algorithm/consensus correctness |
| `atc-ide` | IN PROGRESS | SDK/compiler/toolchain integration |
| `genesis-franchise-factory` | IN PROGRESS | game/economy integration |
| `atc-engineering` | ACTIVE | audit/control plane itself under audit |
| `demo-repository` | IN PROGRESS | private-repository access/evidence path |

## Confirmed findings currently tracked

### GlobusOS #18
`DependencyGraph::dependencies()` is an `unimplemented!()` placeholder. Class **P1**, category **correctness/completeness**, family **kernel/LKM/dependency-resolution**.

### GlobusOS #19
`with_export()` and `ModuleBuilder::export()` incorrectly register exported symbols as imports. Class **P1**, category **logic/API semantics**, family **kernel/LKM/symbol-resolution/reference-accounting**.

## CI mode

When GitHub Actions execution/logs are unavailable or incomplete, source-level audit continues. Runtime findings are not fabricated. A finding requiring CI proof remains open until verifiable CI evidence exists.

## Completion rule

Do not replace `IN PROGRESS` with `COMPLETE` based on historical wiki/README claims. Completion requires current source inspection plus verification evidence appropriate to the repository.
