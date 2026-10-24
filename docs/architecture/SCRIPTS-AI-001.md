# Scripts-KI Architecture

**Status:** PROPOSED — architecture and requirements only  
**System:** ShivaCore / Globus OS / Game Ecosystem  
**Repository role:** ATC Engineering integration contract

## 1. Purpose

Scripts-KI is the Script Intelligence Platform for context-aware script authoring, analysis, verification, testing and runtime integration across the ShivaCore / Globus OS / Game ecosystem.

It is not defined as a generic AI code generator. Its responsibility is to transform high-level intent and game/system context into validated script representations and controlled runtime artifacts.

## 2. Core capabilities

- Script Understanding
- Script Generation
- Script Analysis
- Script Debugging
- Script Optimization
- Script Testing
- Script Conversion
- Script Security
- Script Runtime Intelligence

## 3. Domain coverage

The platform shall support explicit contracts for:

- Gameplay
- Quests
- Characters
- Creatures
- Items
- Weapons
- World events
- Missions
- UI
- Animation
- Audio
- VFX
- Multiplayer
- Economy
- Blockchain integration
- Build/deployment/testing automation

Presence of a domain adapter does not constitute implementation. Each adapter requires source, tests and evidence.

## 4. Canonical pipeline

```text
Natural Language / Visual Script / Source Script
                    |
                    v
             Script Architect
                    |
                    v
            Semantic Script IR
                    |
          +---------+---------+
          |                   |
          v                   v
      Analyzer             Security
          |                   |
          +---------+---------+
                    |
                    v
                Compiler
                    |
                    v
       Runtime / WASM / Native / Web
                    |
                    v
      Game / Globus OS / ShivaCore
```

## 5. Script IR

The Script IR is the canonical semantic boundary between authoring surfaces and runtime targets.

Required conceptual fields:

- metadata
- imports
- variables
- constants
- functions
- events
- conditions
- actions
- state
- dependencies
- permissions
- security policy

Visual scripting and text scripting shall compile to the same IR. Runtime-specific source generation shall not become the canonical semantic representation.

## 6. Language and runtime targets

Planned authoring/target languages include:

- ATCLang
- Rust
- C++
- C#
- TypeScript
- JavaScript
- Lua
- Python

Runtime targets include:

- WASM
- native execution
- web
- supported game-engine adapters

The existence of a language adapter is not evidence that the language is implemented.

## 7. AI agent architecture

The platform may contain specialized agents:

- Script Architect
- Script Generator
- Refactor Agent
- Debug Agent
- Test Agent
- Performance Agent
- Security Agent
- Documentation Agent
- Migration Agent
- Runtime Agent

Agents must operate under explicit authorization and repository/system scope.

## 8. Context integration

Scripts-KI shall consume semantic context from:

- Character AI
- Quest AI
- World AI
- Creature AI
- Items AI
- Weapons AI
- Game runtime/state
- applicable Globus OS / ShivaCore interfaces

Integration must use explicit contracts. Hidden cross-system coupling is prohibited.

## 9. Security boundary

Untrusted or AI-generated scripts must not receive ambient authority.

Required conceptual pipeline:

```text
Script
  -> Parser
  -> Static Analyzer
  -> Permission Checker
  -> Sandbox
  -> Verifier
  -> Runtime
```

Capabilities must be explicit, least-privilege and auditable. Examples:

- CAP_WORLD_READ
- CAP_WORLD_WRITE
- CAP_PLAYER_READ
- CAP_PLAYER_WRITE
- CAP_INVENTORY_READ
- CAP_INVENTORY_WRITE
- CAP_NETWORK
- CAP_BLOCKCHAIN_READ
- CAP_BLOCKCHAIN_WRITE
- CAP_FILE_READ
- CAP_FILE_WRITE
- CAP_SYSTEM_EXEC

Capability names are architectural examples until formally standardized.

## 10. Verification requirements

Scripts-KI must support evidence-producing validation for:

- parser correctness
- IR validity
- dependency resolution
- permission/capability checks
- resource limits
- sandbox boundaries
- deterministic behavior where required
- unit tests
- integration tests
- negative/security tests
- compiler/runtime compatibility

A generated script, generated file or passing local generation step does not prove implementation completeness.

## 11. Proposed repository decomposition

A future standalone repository may use:

```text
scripts-ai/
├── docs/
├── crates/
│   ├── script-core/
│   ├── script-parser/
│   ├── script-ir/
│   ├── script-compiler/
│   ├── script-runtime/
│   ├── script-analyzer/
│   ├── script-debugger/
│   ├── script-tester/
│   ├── script-security/
│   ├── script-ai/
│   └── script-sdk/
├── languages/
├── runtime/
├── templates/
├── examples/
├── tests/
├── benchmarks/
└── tools/
```

This decomposition is a target architecture, not a claim that these components currently exist.

## 12. Governance and evidence

Scripts-KI follows the repository governance principle:

**No Evidence, No Trust.**

Implementation status shall distinguish at minimum:

- PRESENT
- DESIGNED
- IMPLEMENTED
- TESTED
- CI-VERIFIED
- E2E-VERIFIED

The canonical standards source remains `atc-standards`. `atc-engineering` consumes and enforces the applicable requirements; it does not replace the normative standards source.

## 13. Initial implementation sequence

1. Define Scripts-KI system boundary and ownership.
2. Define Script IR schema and versioning.
3. Define capability/security model.
4. Implement parser and IR validation.
5. Implement one reference runtime target.
6. Implement deterministic verifier and negative tests.
7. Implement AI generation against the IR rather than directly against runtime source.
8. Add compiler/adapter targets incrementally.
9. Integrate Character/Quest/World/Creature/Items/Weapons context through explicit APIs.
10. Establish CI and evidence gates before claiming production readiness.

## 14. Non-goals

The architecture does not currently claim:

- a production Script runtime
- a production compiler
- a production AI agent
- blockchain script execution
- unrestricted system execution
- Unity/Unreal integration
- production marketplace functionality

Those require separate implementation and evidence.

## 15. Acceptance principle

The architecture is considered integrated only when the relevant implementation exists in the authoritative repository, has deterministic tests, passes applicable CI on the exact commit under review, and satisfies the applicable ATC standards and governance gates.
