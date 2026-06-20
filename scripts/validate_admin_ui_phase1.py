#!/usr/bin/env python3
"""Validate the Admin and Developer UI Phase 1 documentation gate."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
SDS = Path("docs/sds/foundation/admin_developer_ui.md")
SERVICE = Path("docs/service_catalog/foundation/admin_developer_ui.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_01_progress.md")

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    MASTER,
    CROSSWALK,
    TECH_STACK,
    PHASE_PLAN,
    PHASE_PROGRESS,
]

REQUIRED_PHASE_1_GATE_HEADINGS = [
    "#### Link Attachment Matrix",
    "#### Frozen Client Boundary",
    "#### Phase Entry Prerequisite Matrix",
    "#### Read-Only-First Capability Gates",
    "#### Documentation Update Rule",
]

REQUIRED_CAPABILITIES = [
    "admin.session.read",
    "admin.summary.read",
    "admin.timeline.read",
    "admin.diagnostics.copy",
    "admin.actions.submit",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def section(text: str, heading: str, next_heading_level: str = "## ") -> str:
    marker = f"{heading}\n"
    start = text.find(marker)
    if start == -1:
        raise AssertionError(f"Missing heading: {heading}")
    body_start = start + len(marker)
    end = text.find(f"\n{next_heading_level}", body_start)
    if end == -1:
        end = len(text)
    return text[body_start:end]


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def validate_sub_plan() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #1 - Admin and Developer UI", SUB_PLAN)
    assert_contains(text, "Attached SDS: [docs/sds/foundation/admin_developer_ui.md]", SUB_PLAN)

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(text, "## Phase 1: SDS Alignment, Scope Freeze, And Implementation Gates")
    for item in range(1, 6):
        label = f"**1.{item} "
        assert_contains(phase_1, label, SUB_PLAN)
    for work_item in re.finditer(r"- \*\*1\.[1-5] .+?(?=\n- \*\*1\.|\n### Phase 1 Gate Outputs)", phase_1, re.S):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for heading in REQUIRED_PHASE_1_GATE_HEADINGS:
        assert_contains(phase_1, heading, SUB_PLAN)
    for capability in REQUIRED_CAPABILITIES:
        assert_contains(phase_1, capability, SUB_PLAN)

    guardrails = [
        "The UI surface may be TypeScript",
        "Core admin APIs, authorization filters, audit emission, policy checks, and read-model assembly belong in Rust services behind Overgate.",
        "No direct PostgreSQL, Redis, S3, Vault, node-agent, Seal Ledger, or Overwatch storage access may appear in UI code.",
        "Review must reject UI-only changes that invent platform state",
    ]
    for guardrail in guardrails:
        assert_contains(text, guardrail, SUB_PLAN)


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)

    assert_contains(sds, "[SUB BUILD PLAN #1 - Admin and Developer UI]", SDS)
    assert_contains(sds, "not a privileged backdoor", SDS)
    assert_contains(sds, "Overgate-admin APIs", SDS)

    assert_contains(service, "[SUB BUILD PLAN #1 - Admin and Developer UI]", SERVICE)
    assert_contains(service, "## Phase 1 Implementation Gates", SERVICE)
    assert_contains(service, "direct UI access to service storage", SERVICE)
    assert_contains(service, "First Product Integration", SERVICE)

    assert_contains(master, "SDS #1: [Admin and Developer UI]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #1 - Admin and Developer UI]", MASTER)
    assert_contains(master, "First build point remains Phase 6", MASTER)

    assert_contains(crosswalk, "| SDS #1 | [Admin and Developer UI]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #1 - Admin and Developer UI]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 6: First Product Integration]", CROSSWALK)

    assert_contains(tech_stack, "Core grid, control-plane, execution, storage, policy, accounting, and backbone services should be implemented in Rust.", TECH_STACK)
    assert_contains(tech_stack, "Operator/developer UI", TECH_STACK)
    assert_contains(tech_stack, "TypeScript web UI is acceptable", TECH_STACK)
    assert_contains(tech_stack, "never become a privileged backdoor", TECH_STACK)

    assert_contains(phase_plan, "Complete the Phase 1 gate work", PHASE_PLAN)
    assert_contains(phase_plan, "Keep this pass documentation-only", PHASE_PLAN)


def validate_local_markdown_links() -> None:
    link_pattern = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")
    missing: list[str] = []

    for path in SCOPED_DOCS:
        text = read(path)
        for raw_target in link_pattern.findall(text):
            parsed = urlparse(raw_target)
            if parsed.scheme or raw_target.startswith("#"):
                continue
            target_without_anchor = raw_target.split("#", 1)[0]
            if not target_without_anchor:
                continue
            decoded = unquote(target_without_anchor)
            target = (REPO_ROOT / path.parent / decoded).resolve()
            try:
                target.relative_to(REPO_ROOT)
            except ValueError as exc:
                raise AssertionError(f"{path} links outside repo: {raw_target}") from exc
            if not target.exists():
                missing.append(f"{path}: {raw_target}")

    if missing:
        joined = "\n".join(f"- {item}" for item in missing)
        raise AssertionError(f"Missing local Markdown link targets:\n{joined}")


def main() -> int:
    checks = [
        validate_sub_plan,
        validate_cross_doc_alignment,
        validate_local_markdown_links,
    ]
    for check in checks:
        check()
    print("Admin UI Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
