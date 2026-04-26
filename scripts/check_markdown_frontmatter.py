#!/usr/bin/env python3
"""Validate required Markdown frontmatter for tracked docs."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path


REQUIRED_KEYS = ("title", "when_to_read", "summary", "ontology_relations")
RELATION_KEYS = ("relation", "target", "note")


def tracked_markdown_files() -> list[Path]:
    result = subprocess.run(
        ["git", "ls-files", "*.md"],
        check=True,
        capture_output=True,
        text=True,
    )
    skipped = {"AGENTS.md", "README.md", "docs/SUMMARY.md"}
    return [Path(line) for line in result.stdout.splitlines() if line and line not in skipped]


def frontmatter_lines(path: Path) -> tuple[list[str] | None, str | None]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return None, "missing YAML frontmatter at top of file"

    end = text.find("\n---\n", 4)
    if end == -1:
        return None, "missing closing YAML frontmatter delimiter"

    return text[4:end].splitlines(), None


def top_level_keys(lines: list[str]) -> set[str]:
    keys: set[str] = set()
    for line in lines:
        if not line or line.startswith((" ", "\t", "-")):
            continue
        if ":" in line:
            keys.add(line.split(":", 1)[0].strip())
    return keys


def ontology_relation_errors(lines: list[str]) -> list[str]:
    errors: list[str] = []
    entries: list[set[str]] = []
    current: set[str] | None = None
    in_relations = False

    for line in lines:
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue

        if not line.startswith((" ", "\t", "-")) and ":" in line:
            in_relations = line.split(":", 1)[0].strip() == "ontology_relations"
            current = None
            continue

        if not in_relations:
            continue

        if stripped.startswith("- "):
            if current is not None:
                entries.append(current)
            current = set()
            remainder = stripped[2:]
            if ":" in remainder:
                current.add(remainder.split(":", 1)[0].strip())
            continue

        if current is not None and ":" in stripped:
            current.add(stripped.split(":", 1)[0].strip())

    if current is not None:
        entries.append(current)

    if not entries:
        return ["ontology_relations must contain at least one relation entry"]

    for index, entry in enumerate(entries, start=1):
        missing = [key for key in RELATION_KEYS if key not in entry]
        if missing:
            errors.append(
                f"ontology_relations entry {index} missing key(s): {', '.join(missing)}"
            )

    return errors


def validate(path: Path) -> list[str]:
    lines, frontmatter_error = frontmatter_lines(path)
    if frontmatter_error is not None:
        return [frontmatter_error]

    assert lines is not None
    keys = top_level_keys(lines)
    errors = [
        f"missing required frontmatter key: {key}"
        for key in REQUIRED_KEYS
        if key not in keys
    ]

    if "ontology_relations" in keys:
        errors.extend(ontology_relation_errors(lines))

    return errors


def main() -> int:
    failures: dict[Path, list[str]] = {}
    for path in tracked_markdown_files():
        errors = validate(path)
        if errors:
            failures[path] = errors

    if failures:
        print("Markdown frontmatter validation failed:", file=sys.stderr)
        for path, errors in failures.items():
            print(f"- {path}", file=sys.stderr)
            for error in errors:
                print(f"  - {error}", file=sys.stderr)
        return 1

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
