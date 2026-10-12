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


def validate_report(report_path: Path, expected_sha: str, checked_out_sha: str,
                    run_id: str, run_attempt: str) -> dict:
    """Validate build provenance and preserve, but never override, the gate result."""
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
    decision = report.get("decision")
    if decision not in ("ALLOW", "BLOCK"):
        raise ValueError("gate report decision must be ALLOW or BLOCK")
    blocking_controls = report.get("blocking_controls")
    if not isinstance(blocking_controls, list) or any(
        not isinstance(control, str) or not control for control in blocking_controls
    ):
        raise ValueError("gate report has missing or malformed blocking_controls")
    if type(report.get("sod_valid")) is not bool:
        raise ValueError("gate report sod_valid must be a boolean")
    if type(report.get("human_approval")) is not bool:
        raise ValueError("gate report human_approval must be a boolean")
    if not isinstance(report.get("requested_state"), str) or not report["requested_state"].strip():
        raise ValueError("gate report requested_state is missing or malformed")
    # ALLOW is a governance claim and is accepted only with all required controls.
    # BLOCK is valid evidence of a blocked decision and must remain blocked downstream.
    if decision == "ALLOW":
        if blocking_controls:
            raise ValueError("ALLOW report has non-empty blocking_controls")
        if report["sod_valid"] is not True:
            raise ValueError("ALLOW report separation-of-duties validation is not true")
        if report["human_approval"] is not True:
            raise ValueError("ALLOW report human approval is not true")
    return report


def build_manifest(report_path: Path, binary_path: Path, expected_sha: str,
                   checked_out_sha: str, run_id: str, run_attempt: str,
                   repository: str, workflow_ref: str, report: dict) -> dict:
    if not binary_path.is_file():
        raise ValueError("expected release binary is missing")
    return {
        "schema_version": MANIFEST_SCHEMA,
        "repository": repository,
        "source_sha": expected_sha,
        "checked_out_sha": checked_out_sha,
        "release_binary_path": str(binary_path),
        "release_binary_sha256": hashlib.sha256(binary_path.read_bytes()).hexdigest(),
        "workflow_ref": workflow_ref,
        "run_id": int(run_id),
        "run_attempt": int(run_attempt),
        "gate_report_path": str(report_path),
        "gate_report_sha256": hashlib.sha256(report_path.read_bytes()).hexdigest(),
        "gate_report_schema": report["schema_version"],
        "gate_decision": report["decision"],
        "requested_state": report["requested_state"],
        "blocking_controls": report["blocking_controls"],
        "sod_valid": report["sod_valid"],
        "human_approval": report["human_approval"],
        "readiness_authorized": (
            report["decision"] == "ALLOW"
            and not report["blocking_controls"]
            and report["sod_valid"] is True
            and report["human_approval"] is True
        ),
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
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    try:
        report = validate_report(args.report, args.expected_sha, args.checked_out_sha,
                                 args.run_id, args.run_attempt)
        manifest = build_manifest(args.report, args.binary, args.expected_sha,
                                  args.checked_out_sha, args.run_id, args.run_attempt,
                                  args.repository, args.workflow_ref, report)
    except (ValueError, OSError) as exc:
        raise SystemExit(f"BLOCKED: {exc}") from exc

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(manifest, sort_keys=True, separators=(",", ":")) + "\n",
                           encoding="utf-8")
    print(json.dumps(manifest, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
