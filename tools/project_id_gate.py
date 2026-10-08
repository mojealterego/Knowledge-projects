"""Fail closed when a proposed numeric portfolio ID is already used.

Run before creating *any* new project: python tools/project_id_gate.py 124
Only scans the current checkout; re-check the current remote main before merging.
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

_ID = re.compile(r"^(?P<id>[0-9]+)-")


def existing_claims(root: Path, number: int) -> list[str]:
    """Return current portfolio artifacts with that numeric prefix.

    Recognizes canonical Markdown files as well as README-backed project
    directories. Multiple artifacts of the same project are still 'reserved'.
    """
    projects = root / "projekty"
    if not projects.is_dir():
        raise FileNotFoundError(f"Missing projects directory: {projects}")
    if not isinstance(number, int) or number < 0:
        raise ValueError("Project number must be a non-negative integer")
    result = []
    for entry in projects.iterdir():
        m = _ID.match(entry.name)
        if m and int(m.group("id")) == number and (
            entry.is_file() and entry.suffix.lower() == ".md"
            or entry.is_dir() and (entry / "README.md").is_file()
        ):
            result.append(str(entry.relative_to(root)))
    return sorted(result)


def main() -> int:
    parser = argparse.ArgumentParser(description="Check new Knowledge-projects numeric ID availability")
    parser.add_argument("number", type=int, help="Proposed numeric project ID")
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent.parent)
    args = parser.parse_args()
    claimed = existing_claims(args.root, args.number)
    print(json.dumps({"project_id": args.number, "available": not claimed, "claimed_by": claimed}, ensure_ascii=False))
    return 1 if claimed else 0


if __name__ == "__main__":
    raise SystemExit(main())
