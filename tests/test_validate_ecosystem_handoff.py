import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "validate_ecosystem_handoff.py"
SPEC = importlib.util.spec_from_file_location("validate_ecosystem_handoff", MODULE_PATH)
validator = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(validator)


class EcosystemHandoffTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.sha = "a" * 40
        self.binary = self.root / "target" / "release" / "atc-audit-cli"
        self.binary.parent.mkdir(parents=True)
        self.binary.write_bytes(b"binary bytes")
        self.uploaded = self.root / "artifacts" / "atc-audit-cli"
        self.uploaded.parent.mkdir(parents=True)
        self.uploaded.write_bytes(b"binary bytes")
        self.report = self.root / "artifacts" / "gate-report.json"
        self.report.write_text(json.dumps({"schema_version": "atc.gate-report/v1"}), encoding="utf-8")
        self.manifest_path = self.root / "artifacts" / "evidence-manifest.json"
        self.manifest = {
            "schema_version": "atc.build-evidence/v1",
            "repository": "A-TownChain-Okosystems/atc-engineering",
            "source_sha": self.sha,
            "checked_out_sha": self.sha,
            "event_name": "push",
            "event_ref": "refs/heads/ci/exact-head-sha-pr32",
            "pr_head_sha": "N/A (push event: no PR-head context)",
            "release_binary_path": str(self.binary),
            "release_binary_sha256": hashlib.sha256(self.binary.read_bytes()).hexdigest(),
            "workflow_ref": "owner/repo/.github/workflows/engineering-ci.yml@refs/heads/ci/test",
            "run_id": 123,
            "run_attempt": 1,
            "gate_report_path": str(self.report),
            "gate_report_sha256": hashlib.sha256(self.report.read_bytes()).hexdigest(),
            "gate_report_schema": "atc.gate-report/v1",
        }
        self.write_manifest()

    def tearDown(self):
        self.temp.cleanup()

    def write_manifest(self):
        self.manifest_path.write_text(json.dumps(self.manifest), encoding="utf-8")

    def validate(self):
        return validator.validate(
            self.manifest_path, "A-TownChain-Okosystems/atc-engineering",
            self.sha, self.root / "artifacts"
        )

    def test_accepts_exact_sha_and_matching_artifact_bytes(self):
        result = self.validate()
        self.assertEqual(result["source_sha"], self.sha)

    def test_rejects_wrong_repository(self):
        with self.assertRaisesRegex(ValueError, "repository"):
            validator.validate(self.manifest_path, "other/repo", self.sha, self.root / "artifacts")

    def test_rejects_stale_or_mismatched_head(self):
        self.manifest["checked_out_sha"] = "b" * 40
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, "exact head"):
            self.validate()

    def test_rejects_tampered_gate_report(self):
        self.report.write_text("tampered", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "gate_report_sha256"):
            self.validate()

    def test_rejects_uploaded_binary_mismatch(self):
        self.uploaded.write_bytes(b"different binary")
        with self.assertRaisesRegex(ValueError, "uploaded release binary"):
            self.validate()

    def test_rejects_missing_push_na_marker(self):
        self.manifest["pr_head_sha"] = ""
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, "explicit N/A"):
            self.validate()

    def test_rejects_push_tag_ref(self):
        self.manifest["event_ref"] = "refs/tags/v1"
        self.write_manifest()
        with self.assertRaisesRegex(ValueError, "branch ref"):
            self.validate()


if __name__ == "__main__":
    unittest.main()
