#!/usr/bin/env python3
"""Read-only organization-wide PR red-gate and merge-conflict census."""
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ORG = os.environ.get("ATC_ORG", "A-TownChain-Okosystems")
OUT = Path(os.environ.get("ATC_TRIAGE_DIR", "triage-results"))
OUT.mkdir(parents=True, exist_ok=True)
records = []
blockers = []

def gh(*args):
    result = subprocess.run(["gh", *args], text=True, capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or "gh command failed")
    return result.stdout

def run_json(*args):
    return json.loads(gh(*args))

try:
    names = run_json("repo", "list", ORG, "--limit", "200", "--json", "name")
except Exception as exc:
    print("BLOCKED: repository discovery failed:", exc, file=sys.stderr)
    sys.exit(2)

for item in names:
    repo = ORG + "/" + item["name"]
    try:
        prs = run_json("pr", "list", "--repo", repo, "--state", "open", "--limit", "200",
                       "--json", "number,title,url,headRefOid,mergeStateStatus")
    except Exception as exc:
        blockers.append({"repository": repo, "classification": "BLOCKED_PR_DISCOVERY", "error": str(exc)})
        continue
    for pr in prs:
        try:
            checks = run_json("pr", "checks", str(pr["number"]), "--repo", repo,
                              "--json", "name,state,link,workflow")
        except Exception as exc:
            records.append({"repository": repo, "pr": pr["number"], "url": pr["url"],
                            "head_sha": pr.get("headRefOid"), "merge_state": pr.get("mergeStateStatus"),
                            "classification": "BLOCKED_CHECK_DISCOVERY", "error": str(exc)})
            continue
        red_states = {"FAILURE", "ERROR", "CANCELLED", "TIMED_OUT", "ACTION_REQUIRED"}
        red = [check for check in checks if check.get("state") in red_states]
        merge_state = pr.get("mergeStateStatus") or "UNKNOWN"
        classification = []
        if red:
            classification.append("RED_GATES")
        if merge_state == "CONFLICTING":
            classification.append("MERGE_CONFLICT")
        if classification:
            records.append({"repository": repo, "pr": pr["number"], "title": pr["title"],
                            "url": pr["url"], "head_sha": pr.get("headRefOid"),
                            "merge_state": merge_state, "classification": classification,
                            "red_gates": red})

(OUT / "findings.json").write_text(json.dumps({
    "generated_at": datetime.now(timezone.utc).isoformat(),
    "organization": ORG, "findings": records, "blockers": blockers,
    "safety": "Read-only snapshot. No pushes, merges, gate changes, or fixes are performed."
}, indent=2) + "\n", encoding="utf-8")

lines = ["# Pull-request Red-Gate and Merge-Conflict Triage", "",
         "Generated: " + datetime.now(timezone.utc).isoformat(), "",
         "| Repository | PR | Head SHA | Merge state | Red gates | Classification |",
         "|---|---:|---|---|---|---|"]
for row in records:
    red_names = ", ".join(check.get("name", "unnamed") for check in row.get("red_gates", [])) or "unknown"
    cls = ", ".join(row.get("classification", []))
    lines.append("| {repository} | [#{pr}]({url}) | {sha} | {merge} | {red} | {cls} |".format(
        repository=row["repository"], pr=row["pr"], url=row["url"],
        sha=row.get("head_sha") or "MISSING", merge=row.get("merge_state") or "UNKNOWN",
        red=red_names, cls=cls))
for row in blockers:
    lines.append("- BLOCKED discovery for {}: {}".format(row["repository"], row["error"]))
lines.extend(["", "## Interpretation", "",
              "- This report is a snapshot, not evidence that a fix has passed.",
              "- Findings are tied to the reported PR head SHA; rerun after every head change.",
              "- Discovery errors are BLOCKED and never counted as green.",
              "- This tool is read-only: it never pushes, merges, or weakens gates.",
              "- Remediation requires Run, Job, Step, exit/log evidence, a scoped fix, and fresh required checks on the exact new head SHA.",
              ""])
(OUT / "SUMMARY.md").write_text("\n".join(lines), encoding="utf-8")
print("\n".join(lines))
if blockers:
    sys.exit(2)
