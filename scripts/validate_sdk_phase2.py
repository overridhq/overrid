#!/usr/bin/env python3
"""Validate SDK Phase 2 contract intake and local data-model gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
SDS = Path("docs/sds/foundation/sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
CONTRACT_AUTHORITY = Path("docs/specs/contract_authority.md")
REASON_CODES = Path("docs/specs/reason_codes_and_events.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_02_progress.md")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_README = Path("packages/sdk/README.md")
CLI_RUNNER = Path("packages/cli/src/runner.rs")
SCHEMA = Path("packages/schemas/overrid_contracts/v0/cli_command.schema.json")
CODEGEN_MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE2_SYMBOLS = [
    "SdkContractIntakeManifest",
    "sdk_contract_intake_manifest",
    "validate_contract_intake_manifest",
    "SdkConfigInput",
    "SdkConfigRecord",
    "SdkFeatureFlag",
    "SdkCredentialReferenceRecord",
    "SdkRequestContextRecord",
    "SdkSignedRequestRecord",
    "SdkCommandClass",
    "SdkIdempotencyEntry",
    "OverridErrorRecord",
    "SdkServiceCapabilityProfile",
    "SdkOptionalHelper",
    "SdkCapabilityDecision",
    "negotiate_sdk_capability",
    "SDK_CAPABILITY_UNAVAILABLE_REASON_CODE",
]

REQUIRED_PHASE2_TESTS = [
    "phase2_contract_intake_manifest_names_authoritative_sources",
    "phase2_config_accepts_explicit_local_overgate_contract",
    "phase2_config_rejects_missing_environment_and_unknown_flags",
    "phase2_config_rejects_implicit_live_endpoint_and_production_fixtures",
    "phase2_request_and_signed_records_preserve_refs_without_secret_material",
    "phase2_credential_records_reject_secret_like_values",
    "phase2_idempotency_records_are_bounded_and_skip_read_only_cache",
    "phase2_error_records_preserve_reason_trace_audit_and_retryability",
    "phase2_capability_negotiation_fails_closed_before_unsafe_helpers",
]

REQUIRED_CLI_SDK_ERROR_VARIANTS = [
    "SdkError::MissingEnvironment",
    "SdkError::MissingRequiredField",
    "SdkError::UnknownFeatureFlag",
    "SdkError::LiveEndpointConfirmationRequired",
    "SdkError::FixtureInProduction",
    "SdkError::SecretMaterialRejected",
    "SdkError::InvalidBodyHash",
    "SdkError::CapabilityUnavailable",
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


def validate_sub_plan_phase2() -> None:
    text = read(SUB_PLAN)
    phase_2 = section(text, "## Phase 2: Contract Intake And Local Client Data Model")
    for item in range(1, 6):
        assert_contains(phase_2, f"**2.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*2\.[1-5] .+?(?=\n- \*\*2\.|\n## Phase 3:)",
        phase_2,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "Contract intake manifest",
        "sdk_config",
        "credential_ref",
        "request_context",
        "signed_request",
        "idempotency_entry",
        "overrid_error",
        "Capability reader and compatibility decision table",
        "unsupported optional helpers fail with stable local errors before unsafe network calls",
    ]:
        assert_contains(phase_2, expected, SUB_PLAN)


def validate_source_authority_docs() -> None:
    sds = read(SDS)
    tech_stack = read(TECH_STACK)
    contract_authority = read(CONTRACT_AUTHORITY)
    reason_codes = read(REASON_CODES)
    schema = read(SCHEMA)
    codegen_manifest = read(CODEGEN_MANIFEST)

    for expected in [
        "`sdk_config`",
        "`credential_ref`",
        "`request_context`",
        "`signed_request`",
        "`idempotency_entry`",
        "`overrid_error`",
        "All objects must carry schema version metadata",
    ]:
        assert_contains(sds, expected, SDS)
    assert_contains(tech_stack, "| SDKs | Generated Rust SDK first", TECH_STACK)
    assert_contains(tech_stack, "TypeScript/web bindings from the same contracts", TECH_STACK)
    assert_contains(contract_authority, "`packages/schemas` is the language-neutral source of truth", CONTRACT_AUTHORITY)
    assert_contains(contract_authority, "Rust projection at `packages/schemas/overrid_contracts/src/lib.rs`", CONTRACT_AUTHORITY)
    assert_contains(reason_codes, "Error shapes must keep stable reason-code families", REASON_CODES)

    for expected in [
        '"cli_profile"',
        '"credential_reference"',
        '"trace_context"',
        '"signed_command_envelope"',
        '"local_idempotency_cache_record"',
        '"api_error"',
        '"capability_snapshot"',
    ]:
        assert_contains(schema, expected, SCHEMA)
    for expected in [
        '"canonical_schema": "packages/schemas/overrid_contracts/v0/cli_command.schema.json"',
        '"source_of_truth": "json_schema"',
        '"path": "packages/schemas/overrid_contracts/src/lib.rs"',
        '"status": "rust_projection_from_json_schema_source"',
    ]:
        assert_contains(codegen_manifest, expected, CODEGEN_MANIFEST)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 2 gate",
        "Add a Phase 2 contract intake manifest",
        "Add typed SDK configuration records",
        "Add `scripts/validate_sdk_phase2.py`",
        "cargo test -p overrid-sdk",
        "Do not add runtime services",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graphs",
        "Docdex impact diagnostics",
        "Local delegation was attempted",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    lib = read(SDK_LIB)
    readme = read(SDK_README)
    for expected in REQUIRED_PHASE2_SYMBOLS + REQUIRED_PHASE2_TESTS:
        assert_contains(lib, expected, SDK_LIB)
    for expected in [
        "SDK_CONTRACT_SCHEMA_PATH",
        "SDK_CONTRACT_CODEGEN_MANIFEST_PATH",
        "SDK_CONTRACT_RUST_PROJECTION_PATH",
        "sdk_capability_unavailable",
        "MissingEnvironment",
        "UnknownFeatureFlag",
        "LiveEndpointConfirmationRequired",
        "FixtureInProduction",
        "SecretMaterialRejected",
        "InvalidBodyHash",
        "CapabilityUnavailable",
    ]:
        assert_contains(lib, expected, SDK_LIB)
    for expected in [
        "## Phase 2 Contract Intake And Local Data Model",
        "sdk_contract_intake_manifest()",
        "SdkConfigRecord::from_input()",
        "SdkRequestContextRecord",
        "SdkSignedRequestRecord",
        "SdkIdempotencyEntry",
        "OverridErrorRecord",
        "negotiate_sdk_capability()",
        "sdk_capability_unavailable",
    ]:
        assert_contains(readme, expected, SDK_README)


def validate_cli_error_mapping() -> None:
    runner = read(CLI_RUNNER)
    for expected in REQUIRED_CLI_SDK_ERROR_VARIANTS:
        assert_contains(runner, expected, CLI_RUNNER)
    assert_contains(runner, "live_endpoint_confirmation_required", CLI_RUNNER)
    assert_contains(runner, "secret_material_rejected", CLI_RUNNER)
    assert_contains(runner, "invalid_body_hash", CLI_RUNNER)
    assert_contains(runner, "(ExitCodeClass::Phase, reason_code)", CLI_RUNNER)


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase2.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk"])
    assert_contains(result.stdout + result.stderr, "test result: ok.", Path("cargo test -p overrid-sdk"))


def main() -> int:
    checks = [
        validate_sub_plan_phase2,
        validate_source_authority_docs,
        validate_phase_docs,
        validate_sdk_code_and_readme,
        validate_cli_error_mapping,
        validate_wrapper,
        validate_rust_tests,
    ]
    for check in checks:
        check()
    print("SDK Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
