# PR Red-Gate Remediation Loop

**Status:** PARTIALLY IMPLEMENTED — discovery plus idempotent issue queue; source mutation and conflict repair remain deliberately disabled.

## Automated stages

1. Inventory open PRs, red check states, merge conflicts, and discovery blockers.
2. Bind every finding to the observed PR head SHA.
3. Create one central tracking issue per repository/PR/head SHA, with deduplication.
4. Record a bounded remediation procedure and safety invariants in each issue.
5. Re-run the census on the next scheduled run. A changed head SHA produces a new evidence record rather than reusing stale verification.
6. Close a finding only after an authorized reviewer/worker confirms exact-SHA Run → Job → Step → log evidence and fresh required checks.

## Required configuration

- `ATC_FLEET_READ_TOKEN`: read access to organization repositories for the scanner.
- `ATC_REMEDIATION_TOKEN`: write access to Issues in **atc-engineering only**. Do not grant organization-wide source write access to this token.
- The workflow's built-in token also needs `issues: write` to create central issues when the optional token is absent.
- The queue does not apply labels, so it does not depend on an optional label being present.

## Deliberate boundary

This is not a generic source-code repair bot. It does not infer or execute arbitrary patches from CI logs, push to PR branches, resolve semantic merge conflicts, change permissions, or merge. Those operations require repository-specific tests, least-privilege credentials, idempotency and rollback guarantees. A later executor must use an explicit allowlist of deterministic fix classes and open a separate draft PR; never commit directly to a contributor branch.

## Acceptance evidence

- Python syntax and unit tests pass.
- Manual workflow run emits triage artifacts and attempts issue queueing.
- Known red gate/conflict appears with exact head SHA.
- Repeated run does not duplicate the issue for the same repository/PR/SHA.
- Missing report, invalid SHA, failed lookup, and missing write permission fail closed.
- No source mutation or merge is performed.
