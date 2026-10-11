#!/usr/bin/env python3
"""Collect recent GitHub Actions failures without changing source or PR state.

The collector is intentionally read-only. It records the exact workflow head SHA,
run/job/step identifiers and available URLs. The Actions API does not expose a
process exit code for every step, so exit_code remains null unless a trusted
source provides it; this must never be promoted to VERIFIED evidence.
"""
from __future__ import annotations

import json
import os
import sys
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

API = "https://api.github.com"
TOKEN = os.environ.get("GITHUB_TOKEN", "")
REPOSITORY = os.environ.get("GITHUB_REPOSITORY", "")
CURRENT_RUN_ID = os.environ.get("GITHUB_RUN_ID", "")
CURRENT_SHA = os.environ.get("GITHUB_SHA", "")
OUT = Path(os.environ.get("ATC_TRIAGE_OUTPUT", "artifacts/ai-agent-triage"))


class ApiError(RuntimeError):
    pass


def api_get(path: str) -> dict:
    if not TOKEN or not REPOSITORY:
        raise ApiError("GITHUB_TOKEN or GITHUB_REPOSITORY is missing")
    request = urllib.request.Request(
        f"{API}{path}",
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {TOKEN}",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "ATC-AI-Agent-CI-Triage",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            data = json.loads(response.read().decode("utf-8"))
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError) as exc:
        raise ApiError(f"GitHub API request failed for {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise ApiError(f"GitHub API returned a non-object for {path}")
    return data


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    generated = datetime.now(timezone.utc).isoformat()
    try:
        payload = api_get(f"/repos/{REPOSITORY}/actions/runs?per_page=100")
        runs = payload.get("workflow_runs")
        if not isinstance(runs, list):
            raise ApiError("workflow_runs is missing or not an array")
        findings = []
        for run in runs:
            run_id = run.get("id")
            if not isinstance(run_id, int) or str(run_id) == CURRENT_RUN_ID:
                continue
            conclusion = run.get("conclusion")
            status = run.get("status")
            if status == "completed" and conclusion == "success":
                continue
            if status == "completed" and conclusion in ("skipped",):
                continue
            if status not in ("queued", "in_progress", "waiting", "requested", "completed"):
                continue
            jobs_data = api_get(f"/repos/{REPOSITORY}/actions/runs/{run_id}/jobs?per_page=100")
            jobs = jobs_data.get("jobs")
            if not isinstance(jobs, list):
                raise ApiError(f"jobs list missing for run {run_id}")
            nonpassing_jobs = []
            for job in jobs:
                job_conclusion = job.get("conclusion")
                job_status = job.get("status")
                if job_status == "completed" and job_conclusion == "success":
                    continue
                steps = []
                for step in job.get("steps") or []:
                    step_conclusion = step.get("conclusion")
                    step_status = step.get("status")
                    if step_status == "completed" and step_conclusion == "success":
                        continue
                    steps.append({
                        "name": step.get("name"),
                        "number": step.get("number"),
                        "status": step_status,
                        "conclusion": step_conclusion,
                        "exit_code": None,
                        "exit_code_note": "GitHub Actions Jobs API does not expose a reliable per-step process exit code",
                    })
                nonpassing_jobs.append({
                    "job_id": job.get("id"),
                    "name": job.get("name"),
                    "status": job_status,
                    "conclusion": job_conclusion,
                    "html_url": job.get("html_url"),
                    "steps": steps,
                })
            findings.append({
                "workflow": run.get("name"),
                "run_id": run_id,
                "run_attempt": run.get("run_attempt"),
                "status": status,
                "conclusion": conclusion,
                "head_sha": run.get("head_sha"),
                "sha_matches_current_triage_sha": run.get("head_sha") == CURRENT_SHA,
                "head_branch": run.get("head_branch"),
                "event": run.get("event"),
                "created_at": run.get("created_at"),
                "updated_at": run.get("updated_at"),
                "html_url": run.get("html_url"),
                "jobs": nonpassing_jobs,
            })
    except ApiError as exc:
        failure = {
            "schema_version": "1.0.0",
            "repository": REPOSITORY or None,
            "triage_commit_sha": CURRENT_SHA or None,
            "status": "BLOCKED",
            "generated_at": generated,
            "error": str(exc),
            "findings": [],
            "note": "Fail-closed: unreadable or incomplete API evidence is not a clean result.",
        }
        (OUT / "triage.json").write_text(json.dumps(failure, indent=2) + "\n", encoding="utf-8")
        (OUT / "summary.md").write_text(
            "# AI agent CI triage\n\n**Status: BLOCKED**\n\n"
            f"- Repository: {REPOSITORY or 'unknown'}\n"
            f"- Triage SHA: {CURRENT_SHA or 'unknown'}\n"
            f"- Error: {exc}\n\nNo clean result is inferred from missing evidence.\n",
            encoding="utf-8",
        )
        print(f"BLOCKED: {exc}", file=sys.stderr)
        return 2

    status = "FINDINGS" if findings else "NO_NONPASSING_RUNS_IN_WINDOW"
    report = {
        "schema_version": "1.0.0",
        "repository": REPOSITORY,
        "triage_commit_sha": CURRENT_SHA,
        "current_run_id": int(CURRENT_RUN_ID) if CURRENT_RUN_ID.isdigit() else None,
        "generated_at": generated,
        "status": status,
        "window": {"api": "GitHub Actions runs API", "limit": 100, "pagination": "first page only"},
        "finding_count": len(findings),
        "evidence_policy": {
            "source_mutations": False,
            "automatic_merge": False,
            "missing_api_evidence_is_pass": False,
            "step_exit_code_available_from_actions_api": False,
            "exit_code": None,
            "full_log_review_required_before_verified": True,
        },
        "findings": findings,
    }
    (OUT / "triage.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    lines = [
        "# AI agent CI triage",
        "",
        f"- Status: **{status}**",
        f"- Repository: `{REPOSITORY}`",
        f"- Triage SHA: `{CURRENT_SHA}`",
        f"- Generated: {generated}",
        f"- Non-passing/in-flight runs: {len(findings)}",
        "- Source mutations: none",
        "- Automatic merge: disabled",
        "- Note: a run inventory is not a substitute for full log and exit-code verification.",
        "",
    ]
    for item in findings:
        lines.extend([
            f"## {item.get('workflow') or 'Unnamed workflow'} — run {item['run_id']}",
            f"- Status/conclusion: `{item.get('status')}` / `{item.get('conclusion')}`",
            f"- Head SHA: `{item.get('head_sha')}`",
            f"- Current triage SHA match: `{item.get('sha_matches_current_triage_sha')}`",
            f"- Run: {item.get('html_url')}",
        ])
        if not item["jobs"]:
            lines.append("- No non-passing job details were returned; inspect the run directly.")
        for job in item["jobs"]:
            lines.append(f"- Job {job.get('job_id')} — {job.get('name')}: {job.get('status')} / {job.get('conclusion')} ({job.get('html_url')})")
            for step in job["steps"]:
                lines.append(f"  - Step {step.get('number')} — {step.get('name')}: {step.get('status')} / {step.get('conclusion')}; exit_code=null")
        lines.append("")
    (OUT / "summary.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"{status}: wrote {OUT / 'triage.json'} and {OUT / 'summary.md'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
