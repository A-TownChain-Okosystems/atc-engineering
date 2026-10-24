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

## Reproduzierbares Push-Evidence-Verfahren

1. Den Commit auf den Zielbranch pushen und den daraus entstandenen GitHub Actions-Lauf anhand des Push-Events ermitteln. Ein PR-Lauf oder ein Merge-Ref-Lauf ist kein Ersatz.
2. Den aktuellen Branch-HEAD erneut über GitHub lesen. Der Lauf ist nur zulässig, wenn `event == push`, `event_ref == refs/heads/<branch>`, `head_sha == current branch HEAD` und der ausgecheckte SHA identisch sind.
3. Run-ID, Attempt, erforderliche Jobs und Steps sowie deren Abschlussstatus und Logs erfassen. Bei fehlenden oder mehrdeutigen Daten bleibt der Status `BLOCKED`.
4. Das Artefakt desselben Laufs herunterladen. GitHub-Artefakt-Digest mit dem heruntergeladenen ZIP vergleichen und `SHA256SUMS` gegen sämtliche erwarteten Dateien prüfen. Manifest, Binärdatei und Gate-Report müssen zu genau demselben Lauf und Commit gehören.
5. Vor der Cross-Repository-Übergabe den Quell-SHA mit dem freigegebenen `atc-engineering/main`-HEAD und dem kanonischen Census vergleichen. Bei abweichendem Census-SHA den Consumer nicht als erfolgreich werten; Census zuerst aktualisieren und die Prüfungen auf dem neuen Head erneut ausführen.
6. Die Consumer-Ausführung separat mit der unabhängig geprüften Run-ID und dem erwarteten SHA nachweisen. Erst wenn Herkunft, Integrität, Census-Bindung und alle erforderlichen Jobs erfolgreich sind, darf `E2E_VERIFIED` vergeben werden.

Das Auslösen eines Workflows allein ist kein Verifikationsnachweis. Dieser Abschnitt dokumentiert das Verfahren und ersetzt keine tatsächlichen Run-/Artefakt-Ergebnisse.

## Governance und Freigabe

`decision: ALLOW` ist keine eigenständige organisatorische Freigabe. `sod_valid: false` oder `human_approval: false` bleiben explizite offene Governance-Bedingungen. CI-Erfolg und Governance-Freigabe müssen getrennt berichtet werden.

Statuszuordnung:

- `CI_VERIFIED`: alle anwendbaren technischen Checks bestehen auf dem exakt ausgewiesenen Commit, einschließlich Run-/Job-/Step- und Artefaktnachweis.
- `VERIFIED`: zusätzlich sind sämtliche fachlichen Abnahmekriterien und anwendbaren Governance-Bedingungen nachgewiesen.
- `BLOCKED`: erforderliche Evidenz, Event-Kontext oder Freigaben fehlen.

Es erfolgt kein automatischer Merge aufgrund eines grünen Workflows.
