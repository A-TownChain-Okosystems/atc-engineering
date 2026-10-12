import importlib.util
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "org_function_catalog.py"
SPEC = importlib.util.spec_from_file_location("org_function_catalog", SCRIPT)
catalog = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(catalog)


class CatalogTests(unittest.TestCase):
    def test_git_blob_hash_matches_known_empty_blob(self):
        self.assertEqual(catalog.blob_sha(b""), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391")

    def test_python_ast_finds_functions_and_methods_with_lines(self):
        source = "def top(x):\n    return x\n\nclass Thing:\n    def method(self):\n        pass\n"
        records = catalog.extract_python("owner/repo", "example.py", source, "a" * 40, "b" * 40)
        self.assertEqual([r["qualified_name"] for r in records], ["top", "Thing", "Thing.method"])
        self.assertEqual(records[0]["source_start_line"], 1)
        self.assertEqual(records[2]["source_start_line"], 5)
        self.assertTrue(all(r["observed_commit_sha"] == "b" * 40 for r in records))

    def test_invalid_python_is_reported(self):
        with self.assertRaises(ValueError):
            catalog.extract_python("owner/repo", "broken.py", "def broken(:\n", "a" * 40, "b" * 40)

    def test_rust_extraction_is_explicitly_heuristic(self):
        records = catalog.extract_heuristic(
            "owner/repo", "src/lib.rs", "rust", "pub fn simple_hash(value: u32) -> u32 { value }\n",
            "a" * 40, "b" * 40
        )
        self.assertEqual(len(records), 1)
        self.assertEqual(records[0]["qualified_name"], "simple_hash")
        self.assertTrue(records[0]["limitations"])

    def test_scan_indexes_unsupported_files_instead_of_silently_omitting(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "image.bin").write_bytes(b"binary")
            result = catalog.scan(root, "owner/sample")
            self.assertEqual(len(result["files"]), 1)
            self.assertEqual(result["files"][0]["status"], "UNSUPPORTED")
            self.assertTrue(result["limitations"])


if __name__ == "__main__":
    unittest.main()
