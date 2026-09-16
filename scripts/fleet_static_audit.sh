#!/usr/bin/env bash
set -u

# Offline repository audit. Run from a checked-out organization workspace:
#   ./scripts/fleet_static_audit.sh /path/to/repos
# The script intentionally performs no network access.

ROOT=${1:-.}
FAILED=0
TOTAL=0
PASS=0
OUT="${ROOT}/FLEET_STATIC_AUDIT.txt"
: > "$OUT"

log() { printf '%s\n' "$*" | tee -a "$OUT"; }

for repo in "$ROOT"/*; do
  [ -d "$repo/.git" ] || continue
  name=$(basename "$repo")
  TOTAL=$((TOTAL + 1))
  repo_failed=0
  log "=== $name ==="

  required=(README.md LICENSE SECURITY.md CHANGELOG.md CODEOWNERS STATUS.md ARCHITECTURE.md)
  for f in "${required[@]}"; do
    if [ ! -f "$repo/$f" ] && [ ! -f "$repo/.github/$f" ]; then
      log "MISSING: $f"
      repo_failed=1
    fi
  done

  if find "$repo/.github/workflows" -maxdepth 1 -type f \( -name '*.yml' -o -name '*.yaml' \) 2>/dev/null | grep -q .; then
    log "CI: workflow present"
  else
    log "CI: workflow missing"
    repo_failed=1
  fi

  if grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules \
      '(BEGIN (RSA|EC|OPENSSH|PRIVATE) KEY|AKIA[0-9A-Z]{16}|ghp_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|password[[:space:]]*=[[:space:]]*["'"'][^"'"']+["'"'])' \
      "$repo" >/tmp/atc-secret-scan.$$ 2>/dev/null; then
    log "SECURITY: possible credential pattern"
    cat /tmp/atc-secret-scan.$$ >> "$OUT"
    repo_failed=1
  else
    log "SECURITY: credential pattern scan clean"
  fi
  rm -f /tmp/atc-secret-scan.$$

  if grep -RInE --exclude-dir=.git --exclude-dir=target --exclude-dir=node_modules \
      '(TODO|FIXME|HACK|XXX)[ :]' "$repo" >/tmp/atc-todo-scan.$$ 2>/dev/null; then
    log "QUALITY: TODO/FIXME markers present"
    head -50 /tmp/atc-todo-scan.$$ >> "$OUT"
  else
    log "QUALITY: no TODO/FIXME markers found"
  fi
  rm -f /tmp/atc-todo-scan.$$

  if find "$repo" -name Cargo.toml -not -path '*/target/*' -not -path '*/archive/*' | grep -q .; then
    if command -v cargo >/dev/null 2>&1; then
      while IFS= read -r manifest; do
        log "RUST: check $manifest"
        if ! cargo check --manifest-path "$manifest" --all-targets >>"$OUT" 2>&1; then repo_failed=1; fi
        if ! cargo test --manifest-path "$manifest" >>"$OUT" 2>&1; then repo_failed=1; fi
      done < <(find "$repo" -name Cargo.toml -not -path '*/target/*' -not -path '*/archive/*' | sort -u)
    else
      log "RUST: cargo unavailable; deferred"
    fi
  fi

  if find "$repo" -name '*.py' -not -path '*/.git/*' -not -path '*/archive/*' | grep -q .; then
    if command -v python3 >/dev/null 2>&1; then
      if ! python3 -m compileall -q "$repo" -x '(^|/)(\.git|\.venv|node_modules|target|archive)(/|$)' >>"$OUT" 2>&1; then repo_failed=1; fi
    fi
  fi

  if find "$repo" -name '*.sh' -not -path '*/.git/*' -not -path '*/archive/*' | grep -q .; then
    while IFS= read -r script; do
      if ! bash -n "$script" >>"$OUT" 2>&1; then repo_failed=1; fi
    done < <(find "$repo" -name '*.sh' -not -path '*/.git/*' -not -path '*/archive/*' | sort -u)
  fi

  # Exact duplicate source/config files, excluding generated/build/VCS data.
  if command -v sha256sum >/dev/null 2>&1; then
    dup=$(find "$repo" -type f \
      -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/node_modules/*' -not -path '*/archive/*' \
      -print0 | xargs -0 sha256sum 2>/dev/null | sort | awk 'seen[$1]++ {print $1}' | sort -u | wc -l)
    if [ "$dup" -gt 0 ]; then log "QUALITY: $dup duplicate-content hash group(s)"; fi
  fi

  if [ "$repo_failed" -eq 0 ]; then
    PASS=$((PASS + 1)); log "RESULT: PASS"; else FAILED=$((FAILED + 1)); log "RESULT: FINDINGS"; fi
done

log "=== SUMMARY ==="
log "Repositories scanned: $TOTAL"
log "Pass: $PASS"
log "Findings: $FAILED"
[ "$FAILED" -eq 0 ]
