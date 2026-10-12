import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "scripts" / "validate_build_evidence.py"
SPEC = importlib.util.spec_from_file_location("validate_build_evidence", MODULE_PATH)
validator = importlib.util.module_from_spec(SPEC)
assert SPEC and SPEC.loader
SPEC.loader.exec_module(validator)


class BuildEvidenceValidationTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.report = self.root / "gate-report.json"
        self.report.write_text(json.dumps({
            "schema_version": "atc.gate-report/v1",
            "decision": "ALLOW",
            "requested_state": "TESTNET_READY",
            "blocking_controls": [],
            "sod_valid": True,
            "human_approval": True,
        }), encoding="utf-8")
        self.sha = "a" * 40

    def tearDown(self):
        self.temp.cleanup()

    def write_report(self, **updates):
        report = json.loads(self.report.read_text(encoding="utf-8"))
        report.update(updates)
        self.report.write_text(json.dumps(report), encoding="utf-8")
        return report

    def validate(self, report_path=None, expected_sha=None, checked_out_sha=None, run_id="123", run_attempt="1"):
        return validator.validate_report(
            report_path or self.report,
            expected_sha if expected_sha is not None else self.sha,
            checked_out_sha if checked_out_sha is not None else self.sha,
            run_id,
            run_attempt,
        )

    def test_accepts_valid_allow_report_and_exact_sha(self):
        report = self.validate()
        self.assertEqual(report["schema_version"], "atc.gate-report/v1")
        self.assertEqual(report["decision"], "ALLOW")

    def test_block_report_is_preserved_as_blocked_evidence(self):
        report = self.write_report(decision="BLOCK", sod_valid=False, human_approval=False)
        self.assertEqual(self.validate()["decision"], "BLOCK")
        binary = self.root / "atc-audit-cli"
        binary.write_bytes(b"test binary")
        manifest = validator.build_manifest(
            self.report, binary, self.sha, self.sha, "123", "1",
            "owner/repo", "workflow-ref", report,
        )
        self.assertEqual(manifest["gate_decision"], "BLOCK")
        self.assertFalse(manifest["readiness_authorized"])
        self.assertFalse(manifest["sod_valid"])
        self.assertFalse(manifest["human_approval"])

    def test_allow_with_blocking_controls_fails_closed(self):
        self.write_report(blocking_controls=["SEC-001"])
        with self.assertRaisesRegex(ValueError, "ALLOW report has non-empty blocking_controls"):
            self.validate()

    def test_allow_with_invalid_separation_of_duties_fails_closed(self):
        self.write_report(sod_valid=False)
        with self.assertRaisesRegex(ValueError, "ALLOW report separation-of-duties"):
            self.validate()

    def test_allow_without_human_approval_fails_closed(self):
        self.write_report(human_approval=False)
        with self.assertRaisesRegex(ValueError, "ALLOW report human approval"):
            self.validate()

    def test_unknown_decision_fails_closed(self):
        self.write_report(decision="PENDING")
        with self.assertRaisesRegex(ValueError, "decision must be ALLOW or BLOCK"):
            self.validate()

    def test_malformed_blocking_controls_fail_closed(self):
        self.write_report(blocking_controls="SEC-001")
        with self.assertRaisesRegex(ValueError, "blocking_controls"):
            self.validate()

    def test_malformed_governance_booleans_fail_closed(self):
        self.write_report(sod_valid="false")
        with self.assertRaisesRegex(ValueError, "sod_valid must be a boolean"):
            self.validate()
        self.write_report(sod_valid=True, human_approval=1)
        with self.assertRaisesRegex(ValueError, "human_approval must be a boolean"):
            self.validate()

    def test_missing_requested_state_fails_closed(self):
        self.write_report(requested_state="")
        with self.assertRaisesRegex(ValueError, "requested_state"):
            self.validate()

    def test_missing_report_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "missing or invalid JSON"):
            self.validate(report_path=self.root / "missing.json")

    def test_invalid_json_fails_closed(self):
        self.report.write_text("{not-json", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "missing or invalid JSON"):
            self.validate()

    def test_non_object_report_fails_closed(self):
        self.report.write_text("[]", encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "JSON object"):
            self.validate()

    def test_missing_or_wrong_schema_fails_closed(self):
        self.report.write_text(json.dumps({"schema_version": "wrong/v9"}), encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "schema_version"):
            self.validate()

    def test_malformed_expected_sha_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "full 40-character SHA"):
            self.validate(expected_sha="abc")

    def test_sha_foreign_checkout_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "does not match"):
            self.validate(checked_out_sha="b" * 40)

    def test_malformed_checkout_sha_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "does not match"):
            self.validate(checked_out_sha="not-a-sha")

    def test_missing_or_malformed_run_identity_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "run identity"):
            self.validate(run_id="")
        with self.assertRaisesRegex(ValueError, "run identity"):
            self.validate(run_attempt="x")

    def test_manifest_hashes_report_and_binary(self):
        binary = self.root / "atc-audit-cli"
        binary.write_bytes(b"test binary")
        report = json.loads(self.report.read_text(encoding="utf-8"))
        manifest = validator.build_manifest(
            self.report, binary, self.sha, self.sha, "123", "1",
            "owner/repo", "owner/repo/.github/workflows/engineering-ci.yml@refs/heads/main",
            report,
        )
        self.assertEqual(manifest["source_sha"], self.sha)
        self.assertEqual(manifest["gate_report_sha256"], hashlib.sha256(self.report.read_bytes()).hexdigest())
        self.assertEqual(manifest["release_binary_sha256"], hashlib.sha256(binary.read_bytes()).hexdigest())
        self.assertTrue(manifest["readiness_authorized"])

    def test_missing_binary_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "release binary is missing"):
            validator.build_manifest(
                self.report, self.root / "missing-binary", self.sha, self.sha, "123", "1",
                "owner/repo", "workflow-ref", json.loads(self.report.read_text(encoding="utf-8")),
            )


if __name__ == "__main__":
    unittest.main()
