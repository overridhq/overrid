#!/usr/bin/env python3
"""Validate SDK Phase 3 generated Rust SDK skeleton gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_03_progress.md")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_GENERATED = Path("packages/sdk/src/generated.rs")
SDK_READ = Path("packages/sdk/src/read.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE3_SYMBOLS = [
    "pub mod generated;",
    "pub mod read;",
    "SDK_PHASE3_CAPABILITY_PROFILE",
    "SDK_PHASE3_GENERATED_MODELS_PATH",
    "SdkModuleBoundary",
    "sdk_package_boundary",
    "validate_sdk_package_boundary",
    "SdkGeneratedModelDescriptor",
    "sdk_generated_model_descriptors",
    "validate_generated_model_descriptors",
    "SdkVersionReport",
    "sdk_version_report",
    "SdkTracePolicy",
    "SdkRedactionDefaults",
    "SdkCredentialProviderRef",
    "ConfiguredSdkClient",
    "configure_client",
    "SdkControlPlaneReadKind",
    "SdkPagination",
    "SdkReadRequest",
    "build_control_plane_read_request",
    "control_plane_reader_capability",
]

REQUIRED_PHASE3_TESTS = [
    "phase3_package_boundary_and_generated_models_are_validated",
    "phase3_configure_client_applies_trace_redaction_and_provider_refs",
    "phase3_configure_client_blocks_non_loopback_local_without_explicit_confirmation",
    "phase3_read_helpers_preserve_control_plane_refs",
    "phase3_version_report_names_schema_revision_and_features",
]

REQUIRED_MODEL_OBJECTS = [
    "phase1_command",
    "tenant",
    "identity",
    "key_metadata",
    "manifest",
    "queue_status",
    "audit_ref",
    "error",
]

REQUIRED_READ_KINDS = [
    "Tenant",
    "Identity",
    "KeyMetadata",
    "Manifest",
    "QueueStatus",
    "AuditRef",
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


def validate_sub_plan_phase3() -> None:
    text = read(SUB_PLAN)
    phase_3 = section(text, "## Phase 3: Generated Rust SDK Skeleton")
    for item in range(1, 6):
        assert_contains(phase_3, f"**3.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*3\.[1-5] .+?(?=\n- \*\*3\.|\n## Phase 4:)",
        phase_3,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "generated-and-handwritten split",
        "Typed client builder",
        "Read-only API helpers",
        "Version metadata surfaced on requests and through local diagnostics",
    ]:
        assert_contains(phase_3, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 3 gate",
        "generated/handwritten package boundary",
        "Add generated Rust model descriptors",
        "Add Rust `configure_client` semantics",
        "Add read-only Phase 1 control-plane request helpers",
        "Add `scripts/validate_sdk_phase3.py`",
        "cargo test -p overrid-sdk",
        "Do not add runtime services",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Saved the repo-scoped Phase 3 session context",
        "Docdex impact graphs",
        "Docdex impact diagnostics",
        "Local delegation was attempted",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    lib = read(SDK_LIB)
    generated = read(SDK_GENERATED)
    read_helpers = read(SDK_READ)
    readme = read(SDK_README)
    combined = "\n".join([lib, generated, read_helpers])

    for expected in REQUIRED_PHASE3_SYMBOLS + REQUIRED_PHASE3_TESTS:
        assert_contains(combined, expected, SDK_LIB)
    for expected in REQUIRED_MODEL_OBJECTS:
        assert_contains(generated, expected, SDK_GENERATED)
    for expected in REQUIRED_READ_KINDS:
        assert_contains(read_helpers, expected, SDK_READ)
    for expected in [
        "GeneratedContractProjection",
        "contract_authority: false",
        "GENERATED_CONTRACT_STATUS",
        "SUPPORTED_SCHEMA_VERSION",
        "stable_enum_mapping: true",
        "reason_code_object: true",
        "phase_1_control_plane_bootstrap",
    ]:
        assert_contains(generated, expected, SDK_GENERATED)
    for expected in [
        "x-overrid-schema-version",
        "x-overrid-request-id",
        "x-overrid-trace-id",
        "x-overrid-read-only",
        "SDK_PHASE3_DEFAULT_PAGE_LIMIT",
        "SDK_PHASE3_MAX_PAGE_LIMIT",
    ]:
        assert_contains(read_helpers, expected, SDK_READ)
    for expected in [
        "## Phase 3 Generated Rust SDK Skeleton",
        "sdk_package_boundary()",
        "sdk_generated_model_descriptors()",
        "configure_client()",
        "build_control_plane_read_request()",
        "sdk_version_report()",
    ]:
        assert_contains(readme, expected, SDK_README)


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    generated = read(SDK_GENERATED)
    lib = read(SDK_LIB)
    for expected in [
        "| SDKs | Generated Rust SDK first",
        "TypeScript/web bindings from the same contracts",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    assert_contains(generated, "packages/schemas/overrid_contracts", SDK_GENERATED)
    assert_contains(lib, "pub use overrid_contracts", SDK_LIB)


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase3.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk"])
    assert_contains(result.stdout + result.stderr, "test result: ok.", Path("cargo test -p overrid-sdk"))


def main() -> int:
    checks = [
        validate_sub_plan_phase3,
        validate_phase_docs,
        validate_sdk_code_and_readme,
        validate_tech_stack_alignment,
        validate_wrapper,
        validate_rust_tests,
    ]
    for check in checks:
        check()
    print("SDK Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
