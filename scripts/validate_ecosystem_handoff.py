#!/usr/bin/env python3
"""Fail-closed validator for evidence manifests handed to the ecosystem control plane."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path

SHA40 = re.compile(r"^[0-9a-f]{40}$")
SHA256 = re.compile(r"^[0-9a-f]{64}$")
MANIFEST_SCHEMA = "atc.build-evidence/v1"


def digest(path: Path) -> str:
    if not path.is_file():
        raise ValueError(f"required evidence file is missing: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate(manifest_path: Path, expected_repository: str, expected_sha: str,
             artifact_root: Path) -> dict:
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        raise ValueError(f"manifest is missing or invalid JSON: {exc}") from exc
    if not isinstance(manifest, dict):
        raise ValueError("manifest must be a JSON object")
    if manifest.get("schema_version") != MANIFEST_SCHEMA:
        raise ValueError("unsupported or missing schema_version")
    if manifest.get("repository") != expected_repository:
        raise ValueError("manifest repository does not match expected repository")
    source_sha = manifest.get("source_sha")
    checked_out_sha = manifest.get("checked_out_sha")
    if not isinstance(expected_sha, str) or not SHA40.fullmatch(expected_sha):
        raise ValueError("expected SHA must be a full 40-character lowercase SHA")
    if source_sha != expected_sha or checked_out_sha != expected_sha:
        raise ValueError("manifest source/checkout SHA does not match expected exact head")
    if not isinstance(manifest.get("run_id"), int) or manifest["run_id"] <= 0:
        raise ValueError("run_id must be a positive integer")
    if not isinstance(manifest.get("run_attempt"), int) or manifest["run_attempt"] <= 0:
        raise ValueError("run_attempt must be a positive integer")
    event_name = manifest.get("event_name")
    event_ref = manifest.get("event_ref")
    pr_head_sha = manifest.get("pr_head_sha")
    if not isinstance(event_name, str) or not event_name:
        raise ValueError("event_name is required")
    if not isinstance(event_ref, str) or not event_ref:
        raise ValueError("event_ref is required")
    if event_name == "push":
        if not event_ref.startswith("refs/heads/") or event_ref == "refs/heads/":
            raise ValueError("push event requires a branch ref")
        if pr_head_sha != "N/A (push event: no PR-head context)":
            raise ValueError("push event requires explicit N/A PR-head context")
    elif event_name == "pull_request":
        if not isinstance(pr_head_sha, str) or not SHA40.fullmatch(pr_head_sha):
            raise ValueError("pull_request event requires a full PR-head SHA")
    elif not (isinstance(pr_head_sha, str) and pr_head_sha.startswith("N/A (")):
        raise ValueError("event without PR context requires explicit N/A PR-head context")

    for path_field, hash_field in (
        ("release_binary_path", "release_binary_sha256"),
        ("gate_report_path", "gate_report_sha256"),
    ):
        path_value = manifest.get(path_field)
        expected_digest = manifest.get(hash_field)
        if not isinstance(path_value, str) or not path_value:
            raise ValueError(f"{path_field} is required")
        if not isinstance(expected_digest, str) or not SHA256.fullmatch(expected_digest):
            raise ValueError(f"{hash_field} must be a full SHA-256 digest")
        path = Path(path_value)
        if not path.is_absolute():
            path = Path.cwd() / path
        if digest(path) != expected_digest:
            raise ValueError(f"{hash_field} does not match file bytes")

    # Verify that the uploaded copy of the binary is the same byte sequence
    # whose digest the build manifest records.
    uploaded_binary = artifact_root / "atc-audit-cli"
    if digest(uploaded_binary) != manifest["release_binary_sha256"]:
        raise ValueError("uploaded release binary differs from the manifested binary")
    return manifest


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--expected-sha", required=True)
    parser.add_argument("--artifact-root", type=Path, required=True)
    args = parser.parse_args()
    try:
        manifest = validate(args.manifest, args.repository, args.expected_sha, args.artifact_root)
    except (OSError, ValueError) as exc:
        raise SystemExit(f"BLOCKED: {exc}") from exc
    print(json.dumps({
        "status": "PASS_HANDOFF_INTEGRITY",
        "repository": manifest["repository"],
        "source_sha": manifest["source_sha"],
        "run_id": manifest["run_id"],
        "run_attempt": manifest["run_attempt"],
    }, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
