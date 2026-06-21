#!/usr/bin/env python3
"""Validate SDK Phase 1 boundary and compatibility gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys
from urllib.parse import unquote, urlparse


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
SDS = Path("docs/sds/foundation/sdk.md")
SERVICE = Path("docs/service_catalog/foundation/sdk.md")
MASTER = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_01_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_01_progress.md")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

SCOPED_DOCS = [
    SUB_PLAN,
    SDS,
    SERVICE,
    MASTER,
    CROSSWALK,
    TECH_STACK,
    PHASE_PLAN,
    PHASE_PROGRESS,
    SDK_README,
]

REQUIRED_PHASE1_DECISIONS = [
    "Rust-first binding",
    "TypeScript/web second target",
    "credential-provider-only signing",
    "bounded idempotency cache retention",
    "separate Mobile SDK boundary",
    "current-plus-previous stable major compatibility",
]

REQUIRED_METADATA_SYMBOLS = [
    "SDK_NAME",
    "SDK_VERSION",
    "SDK_LANGUAGE_BINDING",
    "SDK_CAPABILITY_PROFILE",
    "SDK_CURRENT_STABLE_MAJOR",
    "SDK_PREVIOUS_STABLE_MAJOR",
    "SDK_UNSUPPORTED_VERSION_REASON_CODE",
    "SDK_UNSUPPORTED_SCHEMA_REASON_CODE",
    "SdkCompatibilityMetadata",
    "sdk_compatibility_metadata",
    "SdkReleaseChecklist",
    "sdk_release_checklist",
    "SdkCompatibilityRejection",
    "check_sdk_compatibility",
    "compatibility_metadata",
]

REQUIRED_REASON_CODES = [
    "unsupported_sdk_version",
    "schema_version_unsupported",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")
    return result


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


def validate_sub_plan_phase1() -> None:
    text = read(SUB_PLAN)
    assert_contains(text, "# SUB BUILD PLAN #6 - SDK", SUB_PLAN)
    assert_contains(text, "Attached SDS: [docs/sds/foundation/sdk.md]", SUB_PLAN)

    phase_headings = re.findall(r"^## Phase (\d+):", text, flags=re.MULTILINE)
    if phase_headings != [str(number) for number in range(1, 11)]:
        raise AssertionError(f"{SUB_PLAN} must contain Phase 1 through Phase 10 in order")

    phase_1 = section(text, "## Phase 1: SDS Attachment, Boundary, And Version Gates")
    for item in range(1, 6):
        assert_contains(phase_1, f"**1.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*1\.[1-5] .+?(?=\n- \*\*1\.|\n## Phase 2:)",
        phase_1,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")

    for expected in [
        "generated/versioned client package",
        "not a runtime service",
        "not a runtime service, policy authority, secret store, billing service, queue, scheduler, or storage layer",
        "master Phase 1",
        "Phase 0 shared schemas",
        "Phase 6 product-integration hardening",
        "unsupported_sdk_version",
        "schema_version_unsupported",
    ]:
        assert_contains(phase_1, expected, SUB_PLAN)
    for expected in REQUIRED_PHASE1_DECISIONS:
        assert_contains(phase_1, expected, SUB_PLAN)


def validate_cross_doc_alignment() -> None:
    sds = read(SDS)
    service = read(SERVICE)
    master = read(MASTER)
    crosswalk = read(CROSSWALK)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)

    assert_contains(sds, "[sub_build_plan_006_sdk.md]", SDS)
    assert_contains(sds, "not a standalone runtime service", SDS)
    assert_contains(sds, "current stable SDK major version and one previous stable major version", SDS)

    assert_contains(service, "[SUB BUILD PLAN #6 - SDK]", SERVICE)
    assert_contains(service, "Thin client in [Phase 1: Control-Plane Skeleton]", SERVICE)
    assert_contains(service, "hardened SDK in [Phase 6: First Product Integration]", SERVICE)
    assert_contains(service, "Generate supported language bindings from the shared schema package", SERVICE)

    assert_contains(master, "SDS #6: [SDK]", MASTER)
    assert_contains(master, "[SUB BUILD PLAN #6 - SDK]", MASTER)
    assert_contains(master, "First build point remains Phase 1", MASTER)

    assert_contains(crosswalk, "| SDS #6 | [SDK]", CROSSWALK)
    assert_contains(crosswalk, "[SUB BUILD PLAN #6 - SDK]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 1: Control-Plane Skeleton]", CROSSWALK)
    assert_contains(crosswalk, "[Phase 6: First Product Integration]", CROSSWALK)

    assert_contains(tech_stack, "| SDKs | Generated Rust SDK first", TECH_STACK)
    assert_contains(tech_stack, "TypeScript/web bindings from the same contracts", TECH_STACK)
    assert_contains(tech_stack, "not as the core runtime for the grid", TECH_STACK)

    for expected in [
        "Complete the Phase 1 gate work",
        "Add Rust SDK compatibility metadata and release checklist helpers",
        "cargo test -p overrid-sdk",
        "Do not add new runtime services",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Validation Evidence",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)


def validate_sdk_package() -> None:
    lib = read(SDK_LIB)
    readme = read(SDK_README)

    for expected in REQUIRED_METADATA_SYMBOLS + REQUIRED_REASON_CODES:
        assert_contains(lib, expected, SDK_LIB)

    for expected in [
        "sdk_compatibility_metadata_names_release_gate_values",
        "sdk_release_checklist_carries_required_phase1_gate_items",
        "compatibility_check_rejects_unsafe_sdk_or_schema_without_downgrade",
        "SDK_SUPPORTED_SCHEMA_VERSIONS",
        "SUPPORTED_SCHEMA_VERSION",
        "phase1-control-plane-thin-client",
    ]:
        assert_contains(lib, expected, SDK_LIB)

    for expected in [
        "## Phase 1 Release Gate",
        "First binding: Rust SDK only.",
        "sdk_compatibility_metadata()",
        "sdk_release_checklist()",
        "unsupported_sdk_version",
        "schema_version_unsupported",
        "Later TypeScript/web, mobile, Python, Swift, or Kotlin bindings",
    ]:
        assert_contains(readme, expected, SDK_README)


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase1.py")', VALIDATION_WRAPPER)


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


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk"])
    assert_contains(result.stdout + result.stderr, "test result: ok.", Path("cargo test -p overrid-sdk"))


def main() -> int:
    checks = [
        validate_sub_plan_phase1,
        validate_cross_doc_alignment,
        validate_sdk_package,
        validate_wrapper,
        validate_local_markdown_links,
        validate_rust_tests,
    ]
    for check in checks:
        check()
    print("SDK Phase 1 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
