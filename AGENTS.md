# AGENTS.md — Verbindliche Regeln für KI-Agenten in atc-engineering

> Gilt für alle Agenten, die an diesem Repository arbeiten.
> Meta-Regelwerk: `AGENT_MASTERRULES.md` (A-TownChain-Okosystems/.github)
> Agent-Governance: ATC-AI-GOV v1.0 (Fail Closed, Evidence First, No Self-Certification).

## 1. Rolle dieses Repos

atc-engineering ist Infrastruktur (Engineering Control Plane). Es implementiert
**keine Konsens-Semantik** — AD-008 wird nicht verletzt. Kanonische Sprache:
**Rust**. Python/TypeScript nur mit dokumentiertem Integrationsvorteil
(GitHub-API-Glue), nie im Kern-Pfad.

## 2. Reality-Check (Rule 1)

Jede Aktion mit Statussymbol kennzeichnen: ✅ ausgeführt (API-Evidenz) ·
🔄 vorbereitet · 📋 geplant · 🔲 simuliert · ❌ fehlgeschlagen · ⚠️ unklar.
**CLAIMED != PASS.** Kein `IMPLEMENTED` ohne Test-Evidenz.

## 3. Grenzen der Agenten-Autorität

- Agenten führen Beschlüsse aus, fassen sie nicht (ATC-STD-000 §34).
- Normative Änderungen → SCR in atc-standards mit Owner-Mandat.
- Implementation hier → ROADMAP-Phasen + Tests + Evidenz.
- Niemals: Standards-Registry in atc-standards direkt editieren ohne Mandat.

## 4. Engineering-Pflichten

- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` vor jedem Push.
- Kein `unwrap()` in Policy-/Audit-/Evidence-kritischem Code (Owner-Regel;
  identisch zur Konsens-Regel: fail-closed mit expliziten Fehlertypen).
- Determinismus wo möglich: keine Wall-Clock-, keine RNG-Abhängigkeit in
  Audit-/Policy-Entscheidungen; reproduzierbare Generierung (feste Sortierung).
- Commits referenzieren ROADMAP-Phase bzw. SCR-ID.

## 5. Standards-Anbindung

Dieses Repo unterliegt u.a.: ATC-STD-000 (Verfassung), ATC-STD-VERSION-001,
ATC-STD-README-001, ATC-STD-MD-001, ATC-AI-GOV-Familie, ATC-STD-ENG-001
(Determinismus First-Class für D-CRITICAL-Komponenten), Registry-Compliance
V-01..V-16. Audit-Score ≥ 85 = GATE bestanden.

## 6. AI-Execution-Prinzip des Ökosystems

AI may propose. ATCLang specifies. ATVM executes. ATC commits.
In diesem Repo (Infrastruktur): AI proposes → Owner genehmigt → Rust
implementiert → CI verifiziert.
