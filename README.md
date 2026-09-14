# atc-engineering

> **A-TownChain Engineering & Governance Platform**
> Software platform for building, validating, governing, auditing and
> releasing the A-TownChain ecosystem.

**Status: SPECIFIED** — Specification v1.0 verabschiedet (Owner-Direktive
14.09.2026), Implementation gemäß [ROADMAP.md](ROADMAP.md).

## Zweck

`atc-engineering` ist die **Engineering Control Plane** des A-TownChain-Ökosystems.
Sie beantwortet die Frage: *Wie wird sichergestellt, dass die Standards gelten —
und wie wird daraus Software gebaut?*

| Ebene | Repository | Frage |
|---|---|---|
| 1. Knowledge / Authority | [`atc-standards`](https://github.com/A-TownChain-Okosystems/atc-standards) | **What must be true?** |
| 2. Engineering / Governance | **`atc-engineering`** (dieses Repo) | **How is it built and verified?** |
| 3. Product / Runtime | ATCLang, ShivaCore, A-TownChain, ATC-VM, Aurora, GlobusOS, Genesis | Was läuft? |

## Kernprinzipien

1. **Standards-Fluss ist einseitig:** atc-engineering *liest* Regeln aus
   atc-standards — niemals umgekehrt. atc-standards bleibt normatives SSOT.
2. **No Evidence, No Trust:** Zustände (`PRODUCTION_READY` etc.) werden nie
   behauptet, sondern aus Commits, CI-Ergebnissen, Audits und Approvals
   **abgeleitet** (Evidence Engine).
3. **AD-008-konform:** Infrastruktur-Software → Rust als kanonische
   Implementierungssprache; ATCLang/ATVM als Ausführungsgrenze greift hier
   nicht, da keine Konsens-Semantik implementiert wird.
4. **Control Plane ≠ Product Plane:** Dieses Repo produziert keine
   Blockchain-Runtime, sondern die Plattform, die die Produkt-Repos baut,
   prüft, governiert und released.

## Kernmodule (Spec v1.0)

- **Repository Manager** — Fleet-Discovery, Klassifikation, Lifecycle, Health
- **Standards Engine** — Rule Compiler: ATC-Standards → maschinenlesbare Policies → Validatoren
- **Audit Engine** — `atc-engine audit` je Repo oder org-weit
- **Repository Bootstrapper** — `atc-engine repo init`: standardisierte Repo-Erstellung
- **CI/CD Generator** — Workflow-Generierung aus Engineering-Policy
- **Policy Engine** — Merge-/Release-Gates, Approvals, Required Checks
- **Evidence Engine** — Status-Derivation aus Evidenz-Bundles
- **AI Engineering Agent Interface** — Task/Scope/Standards/Actions/Evidence pro Auftrag
- **Dependency & Technology Intelligence** — Vulnerabilities, EOL, Technology Radar
- **Release Engineering** — Devnet → Testnet → Release Gate → Mainnet

Vollständige Specification: [docs/architecture/ATC-ENGINEERING-SPEC-001.md](docs/architecture/ATC-ENGINEERING-SPEC-001.md)

## Architektur (kurz)

```
CONTROL PLANE                EXECUTION PLANE
Governance Engine            GitHub Repos
Engineering Engine     →     CI/CD Runners
Evidence Engine              Local Runtime
     │
atc-standards (liest)
```

## Beteiligung

Änderungen an dieser Plattform folgen dem etablierten SCR-Prozess
(atc-standards). Agenten: [AGENTS.md](AGENTS.md) beachten.

## Lizenz

Apache-2.0 — siehe [LICENSE](LICENSE).
