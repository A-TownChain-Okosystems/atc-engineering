import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "validate_bidirectional_reconciliation.py"
SPEC = importlib.util.spec_from_file_location("validate_bidirectional_reconciliation", SCRIPT)
module = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(module)

FORMATS = ("json", "markdown", "python", "rust", "shell", "toml", "unknown", "yaml")


def inventory():
    return {
        "repository": "owner/repo",
        "observed_commit_sha": "a" * 40,
        "files": [{"path": "src.py", "blob_sha": "b" * 40, "status": "ANALYSED"}],
        "symbols": [{
            "symbol_id": "symbol-1", "path": "src.py", "source_blob_sha": "b" * 40,
            "signature": "def run(x)", "qualified_name": "run", "symbol_kind": "function",
            "source_start_line": 1, "source_end_line": 2,
        }],
        "exclusions": [],
        "parser_coverage": {
            fmt: {
                "parser_mode": {
                    "json": "stdlib_json",
                    "markdown": "commonmark_ast",
                    "python": "python_ast",
                    "rust": "tree_sitter_rust_cst",
                    "shell": "tree_sitter_bash_cst",
                    "toml": "stdlib_tomllib",
                    "unknown": "explicit_unknown_classifier",
                    "yaml": "pyyaml_ast",
                }[fmt],
                "status": "AUTHORITATIVE_COMPLETE",
            }
            for fmt in FORMATS
        },
    }


class BidirectionalReconciliationTests(unittest.TestCase):
    def test_matching_authoritative_inventories_reconcile(self):
        source, catalog = inventory(), inventory()
        report = module.reconcile(source, catalog)
        self.assertEqual(report["status"], "RECONCILED")
        self.assertEqual(report["counts"]["blocking_findings"], 0)

    def test_missing_parser_group_blocks(self):
        source, catalog = inventory(), inventory()
        del source["parser_coverage"]["yaml"]
        report = module.reconcile(source, catalog)
        self.assertEqual(report["status"], "BLOCKED")
        self.assertIn("PARSER_GROUP_MISSING", {item["code"] for item in report["findings"]})

    def test_heuristic_parser_cannot_be_authoritative(self):
        source, catalog = inventory(), inventory()
        source["parser_coverage"]["rust"] = {"parser_mode": "heuristic_regex", "status": "HEURISTIC_PARTIAL"}
        report = module.reconcile(source, catalog)
        self.assertIn("PARSER_NOT_AUTHORITATIVE", {item["code"] for item in report["findings"]})

    def test_source_only_declaration_blocks_both_way_claim(self):
        source, catalog = inventory(), inventory()
        catalog["symbols"] = []
        report = module.reconcile(source, catalog)
        self.assertIn("SOURCE_DECLARATION_MISSING_FROM_CATALOG", {item["code"] for item in report["findings"]})

    def test_orphan_catalog_entry_blocks(self):
        source, catalog = inventory(), inventory()
        source["symbols"] = []
        report = module.reconcile(source, catalog)
        self.assertIn("CATALOG_ENTRY_WITHOUT_SOURCE_DECLARATION", {item["code"] for item in report["findings"]})

    def test_blob_mismatch_blocks(self):
        source, catalog = inventory(), inventory()
        catalog["files"][0]["blob_sha"] = "c" * 40
        report = module.reconcile(source, catalog)
        self.assertIn("FILE_BLOB_MISMATCH", {item["code"] for item in report["findings"]})


if __name__ == "__main__":
    unittest.main()
