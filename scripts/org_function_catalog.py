#!/usr/bin/env python3
"""Deterministic, offline first-pass catalog generator.

Python symbols are parsed with ast. Other languages are file-inventoried and only
common declaration forms are heuristically extracted; such records explicitly carry
a heuristic limitation and MUST NOT be treated as complete API documentation.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
import os
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path

SCHEMA_VERSION = "1.0.0"
SKIP_DIRS = {".git", "target", "node_modules", ".venv", "venv", "dist", "build", "__pycache__", "vendor", "third_party"}
EXTENSIONS = {
    ".py": "python", ".rs": "rust", ".js": "javascript", ".jsx": "javascript",
    ".ts": "typescript", ".tsx": "typescript", ".go": "go", ".java": "java",
    ".cs": "csharp", ".c": "c", ".h": "c", ".cc": "cpp", ".cpp": "cpp",
    ".hpp": "cpp", ".sh": "shell", ".bash": "shell", ".yml": "yaml",
    ".yaml": "yaml", ".toml": "toml", ".json": "json", ".md": "markdown",
}
PATTERNS = {
    "rust": re.compile(r"^\s*(?:(?:pub(?:\([^)]*\))?)\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>]*>)?\s*\(", re.M),
    "javascript": re.compile(r"^\s*(?:(?:export|async|static|public|private)\s+)*(?:function\s+([\w$]+)|(?:const|let|var)\s+([\w$]+)\s*=\s*(?:async\s*)?(?:function|\([^\n]*\)\s*=>|[\w$]+\s*=>))", re.M),
    "typescript": re.compile(r"^\s*(?:(?:export|declare|async|public|private|static)\s+)*function\s+([\w$]+)\s*(?:<[^>]*>)?\s*\(", re.M),
    "go": re.compile(r"^\s*func\s+(?:\([^)]*\)\s*)?([A-Za-z_][A-Za-z0-9_]*)\s*\(", re.M),
    "java": re.compile(r"^\s*(?:(?:public|protected|private|static|final|synchronized|abstract)\s+)+[\w<>?,.\[\]]+\s+([A-Za-z_$][\w$]*)\s*\(", re.M),
    "csharp": re.compile(r"^\s*(?:(?:public|protected|private|static|async|virtual|override|sealed)\s+)+[\w<>?,.\[\]]+\s+([A-Za-z_]\w*)\s*\(", re.M),
    "c": re.compile(r"^\s*(?:[\w*]+\s+)+([A-Za-z_]\w*)\s*\([^;{}]*\)\s*\{", re.M),
    "cpp": re.compile(r"^\s*(?:[\w:*&<>]+\s+)+([A-Za-z_]\w*)\s*\([^;{}]*\)\s*(?:const\s*)?\{", re.M),
    "shell": re.compile(r"^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*\(\s*\)\s*\{", re.M),
}

def git_sha(root: Path) -> str:
    env_sha = os.environ.get("GITHUB_SHA", "")
    if re.fullmatch(r"[0-9a-f]{40,64}", env_sha):
        return env_sha
    try:
        value = subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"], text=True, stderr=subprocess.DEVNULL).strip()
        if re.fullmatch(r"[0-9a-f]{40,64}", value):
            return value
    except (OSError, subprocess.CalledProcessError):
        # A repository without a readable Git checkout has no trustworthy SHA.
        return "0" * 40
    return "0" * 40

def blob_sha(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()

def classify(path: Path) -> str:
    parts = {part.lower() for part in path.parts}
    if parts & {"archive", "archived", "legacy"}:
        return "archived"
    if parts & {"vendor", "third_party", "third-party"}:
        return "vendored"
    if "generated" in parts or path.name.endswith(".generated.ts"):
        return "generated"
    if parts & {"tests", "test", "fixtures", "__tests__"} or path.name.startswith("test_") or ".test." in path.name or ".spec." in path.name:
        return "test"
    if path.suffix.lower() in {".md", ".rst", ".adoc"} or path.name.lower().startswith("readme"):
        return "documentation"
    if path.suffix.lower() in {".yml", ".yaml", ".toml", ".json", ".ini", ".cfg"}:
        return "configuration"
    return "first_party"

def symbol_record(repo: str, rel: str, language: str, kind: str, name: str, start: int, end: int,
                  signature: str, visibility: str, source_sha: str, commit: str, limitations: list[str]) -> dict:
    stable = hashlib.sha256(f"{repo}:{rel}:{kind}:{name}:{start}".encode()).hexdigest()
    return {
        "schema_version": SCHEMA_VERSION, "symbol_id": stable, "repository": repo, "path": rel,
        "language": language, "symbol_kind": kind, "qualified_name": name, "signature": signature,
        "visibility": visibility, "summary": "", "parameters": [], "return_type": None, "errors": [],
        "side_effects": [], "security_boundary": None, "dependencies": [], "callers": [], "callees": [],
        "tests": [], "doc_url_or_path": None, "source_start_line": start, "source_end_line": max(start, end),
        "source_blob_sha": source_sha, "observed_commit_sha": commit, "status": "DETECTED",
        "canonical_owner": repo, "evidence": [f"{rel}:{start}-{max(start, end)}"], "limitations": limitations,
    }

def extract_python(repo: str, rel: str, text: str, sha: str, commit: str) -> list[dict]:
    try:
        tree = ast.parse(text, filename=rel)
    except SyntaxError as exc:
        raise ValueError(f"python parse error line {exc.lineno}: {exc.msg}") from exc
    lines = text.splitlines()
    records = []
    def walk(body, prefix=""):
        for node in body:
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                name = f"{prefix}.{node.name}" if prefix else node.name
                start = node.lineno
                end = getattr(node, "end_lineno", start) or start
                signature = lines[start - 1].strip() if 0 < start <= len(lines) else node.name
                records.append(symbol_record(repo, rel, "python", "method" if prefix else "function", name, start, end, signature,
                    "private" if node.name.startswith("_") else "public", sha, commit, []))
            elif isinstance(node, ast.ClassDef):
                name = f"{prefix}.{node.name}" if prefix else node.name
                start = node.lineno
                end = getattr(node, "end_lineno", start) or start
                signature = lines[start - 1].strip() if 0 < start <= len(lines) else node.name
                records.append(symbol_record(repo, rel, "python", "class", name, start, end, signature,
                    "private" if node.name.startswith("_") else "public", sha, commit, []))
                walk(node.body, name)
    walk(tree.body)
    return records

def extract_heuristic(repo: str, rel: str, language: str, text: str, sha: str, commit: str) -> list[dict]:
    pattern = PATTERNS.get(language)
    if not pattern:
        return []
    lines = text.splitlines()
    out = []
    for match in pattern.finditer(text):
        name = next((g for g in match.groups() if g), None)
        if not name:
            continue
        start = text.count("\n", 0, match.start()) + 1
        signature = lines[start - 1].strip() if start <= len(lines) else name
        vis = "public" if re.search(r"^\\s*pub\\b|^\\s*export\\b|^\\s*public\\b", signature) else "unknown"
        out.append(symbol_record(repo, rel, language, "function", name, start, start, signature, vis, sha, commit,
            ["heuristic declaration match; body range, overloads, visibility, and signature may be incomplete"]))
    return out

def scan(root: Path, repo: str) -> dict:
    root = root.resolve()
    commit = git_sha(root)
    files, symbols, limitations = [], [], []
    for path in sorted(root.rglob("*")):
        if not path.is_file() or any(part in SKIP_DIRS for part in path.parts):
            continue
        try:
            rel = path.relative_to(root).as_posix()
            data = path.read_bytes()
        except (OSError, ValueError) as exc:
            limitations.append(f"file read failure: {path}: {type(exc).__name__}")
            continue
        language = EXTENSIONS.get(path.suffix.lower(), "unknown")
        classification = classify(Path(rel))
        sha = blob_sha(data)
        entry = {"path": rel, "language": language, "classification": classification, "blob_sha": sha, "status": "ANALYSED"}
        files.append(entry)
        if language not in {"python", *PATTERNS.keys()}:
            entry["status"] = "UNSUPPORTED"
            entry["reason"] = "file inventoried; symbol extraction parser not implemented for this format"
            continue
        if classification in {"vendored", "generated", "archived"}:
            entry["status"] = "SKIPPED"
            entry["reason"] = f"classification={classification}; indexed but excluded from symbol extraction"
            continue
        try:
            content = data.decode("utf-8")
            if language == "python":
                symbols.extend(extract_python(repo, rel, content, sha, commit))
            elif language in PATTERNS:
                symbols.extend(extract_heuristic(repo, rel, language, content, sha, commit))
            else:
                entry["status"] = "UNSUPPORTED"
                entry["reason"] = "no symbol parser implemented"
        except UnicodeDecodeError:
            entry["status"] = "UNSUPPORTED"
            entry["reason"] = "not UTF-8 text; file indexed only"
        except ValueError as exc:
            entry["status"] = "PARSE_ERROR"
            entry["reason"] = str(exc)
            limitations.append(f"{rel}: {exc}")
    if commit == "0" * 40:
        limitations.append("repository commit SHA unavailable; output cannot be VERIFIED")
    if any(f["status"] in {"UNSUPPORTED", "PARSE_ERROR", "SKIPPED"} for f in files):
        limitations.append("one or more files were skipped, unsupported, or failed parsing; catalog is not complete")
    return {
        "schema_version": SCHEMA_VERSION, "repository": repo, "observed_commit_sha": commit,
        "generated_at": datetime.now(timezone.utc).isoformat(), "status": "DETECTED",
        "files": files, "symbols": sorted(symbols, key=lambda s: (s["path"], s["source_start_line"], s["qualified_name"])),
        "limitations": limitations,
    }

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", nargs="?", default=".", help="repository checkout to inventory")
    parser.add_argument("--repository", default=None, help="canonical owner/repository label")
    parser.add_argument("--output", default="-", help="JSON output path, or - for stdout")
    args = parser.parse_args()
    root = Path(args.root)
    if not root.is_dir():
        parser.error(f"not a directory: {root}")
    repo = args.repository or root.name
    catalog = scan(root, repo)
    rendered = json.dumps(catalog, ensure_ascii=False, sort_keys=True, indent=2) + "\n"
    if args.output == "-":
        print(rendered, end="")
    else:
        target = Path(args.output)
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(rendered, encoding="utf-8")
    print(f"CATALOG: repository={repo} commit={catalog['observed_commit_sha']} files={len(catalog['files'])} symbols={len(catalog['symbols'])} limitations={len(catalog['limitations'])}", file=__import__("sys").stderr)
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
