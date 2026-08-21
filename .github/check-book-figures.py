#!/usr/bin/env python3
"""Verify that every figure asset referenced by the book exists."""

from __future__ import annotations

import re
import sys
from pathlib import Path

IMAGE_REFERENCE = re.compile(r"!\[[^\]]*\]\(([^)\s]+)\)")


def check_book(book_dir: Path) -> int:
    """Return zero when all referenced figure assets are regular files."""
    missing: list[tuple[Path, int, Path]] = []
    checked = 0

    for page in sorted(book_dir.rglob("*.md")):
        for line_number, line in enumerate(
            page.read_text(encoding="utf-8").splitlines(), start=1
        ):
            for reference in IMAGE_REFERENCE.finditer(line):
                href = reference.group(1)
                if "figures/" not in href:
                    continue

                target = (page.parent / href).resolve()
                if target.is_file():
                    checked += 1
                else:
                    missing.append((page, line_number, target))

    if missing:
        print("BOOK_FIGURES_MISSING: referenced figure assets not found:", file=sys.stderr)
        for page, line_number, target in missing:
            print(f"  - {page}:{line_number} -> {target}", file=sys.stderr)
        return 1

    print(f"BOOK_FIGURES_IN_SYNC: {checked} referenced figure asset(s)")
    return 0


def main() -> int:
    """Run the figure check for the supplied book directory."""
    if len(sys.argv) != 2:
        print(f"usage: {Path(sys.argv[0]).name} BOOK_DIR", file=sys.stderr)
        return 2
    book_dir = Path(sys.argv[1])
    if not book_dir.is_dir():
        print(f"book directory not found: {book_dir}", file=sys.stderr)
        return 2
    return check_book(book_dir)


if __name__ == "__main__":
    raise SystemExit(main())
