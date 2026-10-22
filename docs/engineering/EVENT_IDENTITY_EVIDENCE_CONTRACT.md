# Event-spezifischer Vertrag für CI-Evidenz

Status: implementierter Vertrag auf dem jeweiligen Commit; die tatsächliche Erfüllung gilt erst nach erfolgreicher CI-Verifikation dieses Commits.

## Gemeinsame Identität

Jeder Lauf muss `source_sha` und `checked_out_sha` als vollständige 40-stellige SHA erfassen. Beide müssen exakt übereinstimmen und den von GitHub für das konkrete Event bereitgestellten `github.sha` bezeichnen. Run-ID, Attempt, Repository, Workflow-Ref, Event-Typ und Event-Ref werden ebenfalls dokumentiert.

Ein SHA-256-Manifest belegt nur die Integrität der gehashten Bytes. Es ersetzt weder Event-Semantik noch die Prüfung, ob alle erforderlichen Evidenzdateien vorhanden und korrekt zugeordnet sind.

## Event-Regeln

### `push`

- `event_ref` muss ein Branch-Ref unter `refs/heads/` sein.
- `source_sha` und `checked_out_sha` müssen dem Commit des ausgelösten Push-Laufs entsprechen.
- Ein Push stellt keinen PR-Head-Kontext bereit. `pr_head_sha` wird deshalb explizit als `N/A (push event: no PR-head context)` im Manifest und in `pr-head-sha.txt` ausgewiesen. Ein leeres Feld ist nicht zulässig.

### `pull_request`

- `source_sha` und `checked_out_sha` müssen den für den Workflow-Event ausgecheckten Commit bezeichnen. Bei einem normalen `pull_request`-Lauf kann das der PR-Merge-Ref sein.
- `pr_head_sha` muss zusätzlich als vollständige 40-stellige SHA aus `github.event.pull_request.head.sha` vorliegen.
- PR-Head-SHA und getesteter Merge-Commit sind getrennte Identitäten. Sie dürfen nicht gegenseitig substituiert werden. Der PR-Head muss im Review außerdem gegen den aktuellen PR-Head verglichen werden; ein Event-Snapshot allein belegt nicht, dass der PR inzwischen unverändert ist.

### Andere Events

Für Events ohne PR-Head-Kontext wird `pr_head_sha` ausdrücklich als `N/A (<event> event: no PR-head context)` ausgewiesen. Ein unerwarteter PR-Head-Wert blockiert die Validierung.

## Automatisierte Ablehnungsfälle

Die Regressionstests müssen mindestens abdecken:

- gültigen Push mit Branch-Ref und passender Checkout-SHA;
- gültigen Pull-Request mit getrennt erfasster PR-Head-SHA;
- fehlende oder fehlerhafte PR-Head-SHA bei `pull_request`;
- unerwarteten PR-Head-Wert bei `push`;
- Tag-/Nicht-Branch-Ref bei `push`;
- ungültige SHA oder abweichenden Checkout-SHA;
- fehlenden Gate-Report, falsches Schema und fehlendes Release-Binary.

## Governance und Freigabe

`decision: ALLOW` ist keine eigenständige organisatorische Freigabe. `sod_valid: false` oder `human_approval: false` bleiben explizite offene Governance-Bedingungen. CI-Erfolg und Governance-Freigabe müssen getrennt berichtet werden.

Statuszuordnung:
- `CI_VERIFIED`: alle anwendbaren technischen Checks bestehen auf dem exakt ausgewiesenen Commit, einschließlich Run-/Job-/Step- und Artefaktnachweis.
- `VERIFIED`: zusätzlich sind sämtliche fachlichen Abnahmekriterien und anwendbaren Governance-Bedingungen nachgewiesen.
- `BLOCKED`: erforderliche Evidenz, Event-Kontext oder Freigaben fehlen.

Es erfolgt kein automatischer Merge aufgrund eines grünen Workflows.
