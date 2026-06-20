#!/usr/bin/env python3
"""Run the Admin and Developer UI validation suite."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]
VALIDATORS = [
    Path("scripts/validate_admin_ui_phase1.py"),
    Path("scripts/validate_admin_ui_phase2.py"),
    Path("scripts/validate_admin_ui_phase3.py"),
]


def main() -> int:
    for validator in VALIDATORS:
        path = REPO_ROOT / validator
        if not path.is_file():
            print(f"missing validator: {validator}", file=sys.stderr)
            return 1
        result = subprocess.run([sys.executable, str(path)], cwd=REPO_ROOT)
        if result.returncode != 0:
            return result.returncode
    print("Admin UI validation suite passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
