#!/usr/bin/env python3
"""Fail-closed validation and manifest generation for ATC build evidence."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path


REPORT_SCHEMA = "atc.gate-report/v1"
MANIFEST_SCHEMA = "atc.build-evidence/v1"
FULL_SHA = re.compile(r"[0-9a-f]{40}")


def validate_event_identity(event_name: str, event_ref: str, expected_sha: str,
                            checked_out_sha: str, pr_head_sha: str) -> str:
    """Validate event-specific identity and return the normalized PR-head field."""
    if not FULL_SHA.fullmatch(expected_sha):
        raise ValueError("expected source SHA is not a full 40-character SHA")
    if not FULL_SHA.fullmatch(checked_out_sha) or checked_out_sha != expected_sha:
        raise ValueError("checked-out HEAD does not match the workflow event SHA")
    if not event_ref:
        raise ValueError("workflow event ref is missing")

    if event_name == "push":
        if not event_ref.startswith("refs/heads/") or event_ref == "refs/heads/":
            raise ValueError("push event ref must identify a branch under refs/heads/")
        if pr_head_sha:
            raise ValueError("PR head SHA must be absent for a push event without PR context")
        return "N/A (push event: no PR-head context)"
    if event_name == "pull_request":
        if not FULL_SHA.fullmatch(pr_head_sha):
            raise ValueError("pull_request event requires a full 40-character PR head SHA")
        return pr_head_sha
    if pr_head_sha:
        raise ValueError("PR head SHA is only valid when a pull_request event provides PR context")
    return f"N/A ({event_name} event: no PR-head context)"


def validate_report(report_path: Path, expected_sha: str, checked_out_sha: str,
                    run_id: str, run_attempt: str) -> dict:
    """Validate all inputs before any evidence manifest is emitted."""
    if not FULL_SHA.fullmatch(expected_sha):
        raise ValueError("expected source SHA is not a full 40-character SHA")
    if not FULL_SHA.fullmatch(checked_out_sha) or checked_out_sha != expected_sha:
        raise ValueError("checked-out HEAD does not match the workflow event SHA")
    if not run_id.isdigit() or not run_attempt.isdigit():
        raise ValueError("workflow run identity is missing or malformed")
    try:
        report = json.loads(report_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise ValueError(f"gate report is missing or invalid JSON: {exc}") from exc
    if not isinstance(report, dict):
        raise ValueError("gate report must be a JSON object")
    if report.get("schema_version") != REPORT_SCHEMA:
        raise ValueError("unsupported or missing gate-report schema_version")
    return report


def build_manifest(report_path: Path, binary_path: Path, expected_sha: str,
                   checked_out_sha: str, run_id: str, run_attempt: str,
                   repository: str, workflow_ref: str, report: dict,
                   event_name: str = "", event_ref: str = "",
                   pr_head_sha: str = "") -> dict:
    if not binary_path.is_file():
        raise ValueError("expected release binary is missing")
    normalized_pr_head = (validate_event_identity(event_name, event_ref, expected_sha,
                                                   checked_out_sha, pr_head_sha)
                          if event_name else (pr_head_sha or "N/A (event context not supplied)"))
    return {
        "schema_version": MANIFEST_SCHEMA,
        "repository": repository,
        "source_sha": expected_sha,
        "checked_out_sha": checked_out_sha,
        "event_name": event_name or "unspecified",
        "event_ref": event_ref or workflow_ref,
        "pr_head_sha": normalized_pr_head,
        "release_binary_path": str(binary_path),
        "release_binary_sha256": hashlib.sha256(binary_path.read_bytes()).hexdigest(),
        "workflow_ref": workflow_ref,
        "run_id": int(run_id),
        "run_attempt": int(run_attempt),
        "gate_report_path": str(report_path),
        "gate_report_sha256": hashlib.sha256(report_path.read_bytes()).hexdigest(),
        "gate_report_schema": report["schema_version"],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--expected-sha", required=True)
    parser.add_argument("--checked-out-sha", required=True)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--run-attempt", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--workflow-ref", required=True)
    parser.add_argument("--event-name", required=True)
    parser.add_argument("--event-ref", required=True)
    parser.add_argument("--pr-head-sha", default="")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    try:
        report = validate_report(args.report, args.expected_sha, args.checked_out_sha,
                                 args.run_id, args.run_attempt)
        manifest = build_manifest(args.report, args.binary, args.expected_sha,
                                  args.checked_out_sha, args.run_id, args.run_attempt,
                                  args.repository, args.workflow_ref, report,
                                  args.event_name, args.event_ref, args.pr_head_sha)
    except (ValueError, OSError) as exc:
        raise SystemExit(f"BLOCKED: {exc}") from exc

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(manifest, sort_keys=True, separators=(",", ":")) + "\n",
                           encoding="utf-8")
    print(json.dumps(manifest, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
