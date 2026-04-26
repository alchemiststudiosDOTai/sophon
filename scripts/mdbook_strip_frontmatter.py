#!/usr/bin/env python3
"""mdBook preprocessor that strips YAML frontmatter from rendered chapters."""

from __future__ import annotations

import json
import sys
from typing import Any


def strip_frontmatter(content: str) -> str:
    """Remove a leading YAML frontmatter block from Markdown content."""
    if not content.startswith("---\n"):
        return content

    end = content.find("\n---\n", 4)
    if end == -1:
        return content

    return content[end + len("\n---\n") :]


def strip_content_fields(value: Any) -> None:
    """Recursively strip frontmatter from any mdBook chapter content field."""
    if isinstance(value, dict):
        content = value.get("content")
        if isinstance(content, str):
            value["content"] = strip_frontmatter(content)

        for child in value.values():
            strip_content_fields(child)
        return

    if isinstance(value, list):
        for item in value:
            strip_content_fields(item)


def main(argv: list[str]) -> int:
    if len(argv) >= 2 and argv[1] == "supports":
        return 0

    _context, book = json.load(sys.stdin)
    strip_content_fields(book)
    json.dump(book, sys.stdout)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
