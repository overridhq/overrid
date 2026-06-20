#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 1 gate semantics."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
SDS = Path("docs/sds/foundation/integration_test_harness.md")
SERVICE = Path("docs/service_catalog/foundation/integration_test_harness.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_01_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

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

REQUIRED_GATE_HEADINGS = [
    "#### Link Attachment Matrix",
    "#### Frozen Harness Boundary",
    "#### Gate Class Matrix",
    "#### Fail-Closed Dependency Contract",
    "#### Documentation Update Rule",
]

REQUIRED_GATE_CLASSES = [
    "`smoke`",
    "`contract_spine`",
    "`regression`",
    "`extended`",
    "`release_candidate`",
]

REQUIRED_BLOCKED_REASONS = [
    "`dependency.local_stack_unavailable`",
    "`dependency.schema_missing`",
    "`dependency.service_unavailable`",
    "`dependency.phase_tag_unsupported`",
    "`dependency.manifest_missing`",
    "`safety.non_local_profile`",
    "`safety.fixture_not_test_only`",
    "`dependency.phase_contract_not_ready`",
]

REQUIRED_BOUNDARY_TEXT = [
    "not a runtime service",
    "not a policy engine",
    "not a production event writer",
    "not a payment actor",
    "not a provider-side execution service",
    "Direct storage, event-log, object, queue, or local file inspection is allowed only for reset and diagnostics",
]

REQUIRED_ATTACHMENT_PATHS = [
    "docs/sds/foundation/integration_test_harness.md",
    "docs/service_catalog/foundation/integration_test_harness.md",
    "docs/build_plan/master_plan.md",
    "docs/build_plan/service_catalog_alignment.md",
    "docs/overrid_tech_stack_choice.md",
    "docs/planning/integration_test_harness_phase_01_plan.md",
    "docs/planning/integration_test_harness_phase_01_progress.md",
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
    assert_contains(text, "# SUB BUILD PLAN #3 - Integration Test Harness", SUB_PLAN)
    assert_contains(
        text,
        "Attached SDS: [docs/sds/foundation/integration_test_harness.md]",
        SUB_PLAN,
    )

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(
        text,
        "## Phase 1: SDS Attachment, Harness Boundary, And Gate Semantics",
    )
    for item in range(1, 6):
        assert_contains(phase_1, f"**1.{item} ", SUB_PLAN)

    for work_item in re.finditer(
        r"- \*\*1\.[1-5] .+?(?=\n- \*\*1\.|\n### Phase 1 Gate Outputs)",
        phase_1,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for heading in REQUIRED_GATE_HEADINGS:
        assert_contains(phase_1, heading, SUB_PLAN)
    for gate_class in REQUIRED_GATE_CLASSES:
        assert_contains(phase_1, gate_class, SUB_PLAN)
    for reason in REQUIRED_BLOCKED_REASONS:
        assert_contains(phase_1, reason, SUB_PLAN)
    for boundary in REQUIRED_BOUNDARY_TEXT:
        assert_contains(phase_1, boundary, SUB_PLAN)
    for path in REQUIRED_ATTACHMENT_PATHS:
        assert_contains(phase_1, path, SUB_PLAN)

    for phase in range(0, 14):
        if not re.search(rf"^\| {phase} \|", phase_1, flags=re.MULTILINE):
            raise AssertionError(f"{SUB_PLAN} gate class matrix is missing phase {phase}")

    required_doc_rule_terms = [
        "scenario manifests",
        "golden traces",
        "integration readiness",
        "planned",
        "blocked",
        "Release-candidate evidence",
    ]
    for term in required_doc_rule_terms:
        assert_contains(phase_1, term, SUB_PLAN)


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite_validator = read(SUITE_VALIDATOR)

    assert_contains(sds, "[sub_build_plan_003_integration_test_harness.md]", SDS)
    assert_contains(sds, "## Phase-Gate Boundary Decisions", SDS)
    for gate_class in REQUIRED_GATE_CLASSES:
        assert_contains(sds, gate_class, SDS)
    for reason in REQUIRED_BLOCKED_REASONS:
        assert_contains(sds, reason, SDS)

    assert_contains(service, "[SUB BUILD PLAN #3 - Integration Test Harness]", SERVICE)
    assert_contains(service, "## Phase 1 Implementation Gates", SERVICE)
    for gate_class in REQUIRED_GATE_CLASSES:
        assert_contains(service, gate_class, SERVICE)
    for reason in REQUIRED_BLOCKED_REASONS:
        assert_contains(service, reason, SERVICE)
    assert_contains(service, "scenario manifests", SERVICE)
    assert_contains(service, "golden traces", SERVICE)

    assert_contains(master, "SDS #3: [Integration Test Harness]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #3 - Integration Test Harness]", MASTER)
    assert_contains(master, "First build point remains Phase 0", MASTER)

    assert_contains(crosswalk, "| SDS #3 | [Integration Test Harness]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #3 - Integration Test Harness]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 0: Foundation]", CROSSWALK)

    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Overrid-shaped local stubs", TECH_STACK)
    assert_contains(tech_stack, "PostgreSQL, MySQL, MongoDB", TECH_STACK)
    assert_contains(tech_stack, "Redis, NATS, Kafka", TECH_STACK)

    assert_contains(phase_plan, "Complete SUB BUILD PLAN #3 Phase 1", PHASE_PLAN)
    assert_contains(phase_plan, "Do not introduce the Rust harness crate in this phase", PHASE_PLAN)
    assert_contains(phase_progress, "Integration Test Harness Phase 1 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    assert_contains(
        suite_validator,
        'Path("scripts/validate_integration_harness_phase1.py")',
        SUITE_VALIDATOR,
    )


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
    print("Integration Test Harness Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
