import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).with_name("remediation-issue-queue.py")


class RemediationIssueQueueTests(unittest.TestCase):
    def test_missing_report_fails_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            env = os.environ.copy()
            env["ATC_TRIAGE_DIR"] = tmp
            env.pop("ATC_REMEDIATION_TOKEN", None)
            env.pop("GITHUB_TOKEN", None)
            result = subprocess.run([sys.executable, str(SCRIPT)], env=env,
                                    text=True, capture_output=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("BLOCKED", result.stderr)

    def test_missing_token_fails_closed_even_with_report(self):
        with tempfile.TemporaryDirectory() as tmp:
            Path(tmp, "findings.json").write_text(json.dumps({
                "generated_at": "2026-10-11T00:00:00Z",
                "findings": [], "blockers": []
            }), encoding="utf-8")
            env = os.environ.copy()
            env["ATC_TRIAGE_DIR"] = tmp
            env.pop("ATC_REMEDIATION_TOKEN", None)
            env.pop("GITHUB_TOKEN", None)
            result = subprocess.run([sys.executable, str(SCRIPT)], env=env,
                                    text=True, capture_output=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("no issue-write token", result.stderr)


if __name__ == "__main__":
    unittest.main()
