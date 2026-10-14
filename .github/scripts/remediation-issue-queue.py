#!/usr/bin/env python3
"""Turn triage findings into idempotent, SHA-bound remediation issues.

This worker intentionally creates tracking issues only. It never edits source,
pushes branches, resolves conflicts, changes checks, or merges pull requests.
"""
import json
import os
import subprocess
import sys
from pathlib import Path

ORG = os.environ.get("ATC_ORG", "A-TownChain-Okosystems")
CONTROL_REPO = os.environ.get("ATC_CONTROL_REPO", ORG + "/atc-engineering")
INPUT = Path(os.environ.get("ATC_TRIAGE_DIR", "triage-results")) / "findings.json"
TOKEN = os.environ.get("ATC_REMEDIATION_TOKEN") or os.environ.get("GITHUB_TOKEN")
GH = os.environ.get("GH_CLI", "gh")


def gh(args, *, token=None, check=True):
    env = os.environ.copy()
    if token:
        env["GH_TOKEN"] = token
    proc = subprocess.run([GH, *args], text=True, capture_output=True, env=env)
    if check and proc.returncode:
        raise RuntimeError(proc.stderr.strip() or "gh command failed")
    return proc


def main():
    if not INPUT.is_file():
        print("BLOCKED: triage findings file missing", file=sys.stderr)
        return 2
    payload = json.loads(INPUT.read_text(encoding="utf-8"))
    findings = payload.get("findings", [])
    blockers = payload.get("blockers", [])
    if not TOKEN:
        print("BLOCKED: no issue-write token; findings artifact remains authoritative", file=sys.stderr)
        return 2

    failures = 0
    created = 0
    existing = 0
    for row in findings:
        repo = row.get("repository", "UNKNOWN")
        number = row.get("pr", "UNKNOWN")
        sha = row.get("head_sha")
        if not sha or len(sha) != 40:
            print(f"BLOCKED: {repo} PR {number} has no valid exact head SHA")
            failures += 1
            continue
        title = f"[PR Remediation] {repo}#{number} @ {sha[:12]}"
        search = gh(["issue", "list", "--repo", CONTROL_REPO, "--state", "open",
                     "--search", f'"{title}" in:title', "--json", "number,title,url"],
                    token=TOKEN, check=False)
        if search.returncode:
            print(f"BLOCKED: issue deduplication lookup failed for {repo}#{number}: {search.stderr.strip()}")
            failures += 1
            continue
        try:
            matches = json.loads(search.stdout)
        except json.JSONDecodeError:
            print(f"BLOCKED: issue lookup returned invalid JSON for {repo}#{number}")
            failures += 1
            continue
        if any(item.get("title") == title for item in matches):
            existing += 1
            continue

        classification = row.get("classification", ["UNKNOWN"])
        body = [
            "## Evidence-bound finding", "",
            f"- Repository: {repo}",
            f"- Pull request: {row.get('url', 'unknown')}",
            f"- PR number: {number}",
            f"- Exact observed head SHA: `{sha}`",
            f"- Merge state: {row.get('merge_state', 'UNKNOWN')}",
            f"- Classification: {', '.join(classification) if isinstance(classification, list) else classification}",
            f"- Red checks: {', '.join(c.get('name', 'unnamed') for c in row.get('red_gates', [])) or 'none reported'}",
            "",
            "## Required remediation sequence",
            "1. Re-fetch the PR and confirm its current head SHA still equals the SHA above; otherwise mark this record STALE and recapture evidence.",
            "2. Capture the actual workflow Run → Job → Step → exit code and relevant logs.",
            "3. Classify the root cause and propose the smallest additive fix.",
            "4. Preserve both sides of any merge conflict; ambiguous semantic choices require human review.",
            "5. Re-run required checks and inspect their logs on the exact new head SHA.",
            "6. Mark VERIFIED only when all required gates pass on that exact SHA; otherwise keep BLOCKED or RESIDUAL.",
            "",
            "## Safety invariants",
            "- No automatic merge, force-push, gate weakening, test deletion, or allowlist expansion to conceal defects.",
            "- Cryptography, consensus, protocol/standards, VM, kernel/TCB, dependency/security, and ambiguous conflict fixes require human review.",
            "- This issue records a finding; it is not evidence that a fix has been applied.",
            "",
            f"Source report timestamp: {payload.get('generated_at', 'unknown')}",
        ]
        create = gh(["issue", "create", "--repo", CONTROL_REPO, "--title", title,
                     "--body", "\n".join(body), "--label", "bug"],
                    token=TOKEN, check=False)
        if create.returncode:
            print(f"BLOCKED: could not create issue for {repo}#{number}: {create.stderr.strip()}")
            failures += 1
        else:
            created += 1
            print(f"CREATED: {create.stdout.strip()} for {repo}#{number}@{sha}")

    print(f"SUMMARY: findings={len(findings)} created={created} existing={existing} blockers={len(blockers)} failures={failures}")
    return 2 if blockers or failures else 0


if __name__ == "__main__":
    sys.exit(main())
