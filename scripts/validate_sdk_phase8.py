#!/usr/bin/env python3
"""Validate SDK Phase 8 fixtures, contract-test, and artifact gates."""

from __future__ import annotations

from pathlib import Path
import json
import re
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_08_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_FIXTURES = Path("packages/sdk/src/fixtures.rs")
SDK_README = Path("packages/sdk/README.md")
LOCAL_FIXTURE_SET = Path("packages/sdk/fixtures/phase8/local_sdk_fixture_set.valid.json")
GOLDEN_CORPUS = Path("packages/sdk/fixtures/phase8/golden_cross_language_corpus.valid.json")
VALIDATION_ARTIFACTS = Path("packages/sdk/fixtures/phase8/validation_artifacts_manifest.valid.json")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE8_SYMBOLS = [
    "pub mod fixtures;",
    "pub use fixtures::*;",
    "SDK_PHASE8_CAPABILITY_PROFILE",
    "SDK_PHASE8_FIXTURE_ROOT",
    "SDK_PHASE8_LOCAL_FIXTURE_SET_PATH",
    "SDK_PHASE8_GOLDEN_CORPUS_PATH",
    "SDK_PHASE8_VALIDATION_ARTIFACTS_PATH",
    "SdkPhase8FixtureKind",
    "SdkPhase8LocalFixtureRecord",
    "SdkPhase8LocalFixtureCorpus",
    "sdk_phase8_local_fixture_corpus",
    "validate_phase8_local_fixture_corpus",
    "SdkPhase8ContractTestKind",
    "SdkPhase8ContractTestDescriptor",
    "sdk_phase8_contract_tests",
    "validate_phase8_contract_tests",
    "SdkPhase8GoldenFixtureKind",
    "SdkPhase8GoldenFixtureDescriptor",
    "sdk_phase8_golden_fixtures",
    "validate_phase8_golden_fixtures",
    "SdkPhase8SecurityRedactionCheck",
    "sdk_phase8_security_redaction_checks",
    "validate_phase8_security_redaction_checks",
    "SdkPhase8ValidationArtifactKind",
    "SdkPhase8ValidationArtifact",
    "sdk_phase8_validation_artifacts",
    "validate_phase8_validation_artifacts",
    "validate_phase8_fixture_artifact_files",
    "SdkPhase8Error",
]

REQUIRED_PHASE8_TESTS = [
    "phase8_local_fixtures_are_deterministic_redacted_resettable_and_non_production",
    "phase8_contract_tests_target_public_local_stack_routes_and_record_owner_blockers",
    "phase8_golden_fixtures_gate_cross_language_bindings_on_rust_corpus",
    "phase8_security_redaction_checks_block_secret_bearing_artifacts",
    "phase8_validation_artifacts_are_docdex_indexed_not_runtime_events",
    "phase8_fixture_artifact_files_match_declared_paths_and_markers",
]

REQUIRED_LOCAL_FIXTURE_KINDS = {
    "tenant",
    "actor",
    "credential",
    "command_envelope",
    "manifest",
    "signature",
    "idempotency_entry",
    "error",
    "usage_ref",
    "audit_ref",
}

REQUIRED_GOLDEN_CASES = {
    "golden_request_envelope",
    "golden_canonical_signing_input",
    "golden_response_error",
    "golden_manifest_validation",
    "golden_idempotency_case",
    "golden_redaction_case",
}

REQUIRED_VALIDATION_ARTIFACTS = {
    "phase8_schema_generation_artifact",
    "phase8_contract_test_artifact",
    "phase8_signing_golden_artifact",
    "phase8_idempotency_artifact",
    "phase8_redaction_artifact",
    "phase8_compatibility_artifact",
    "phase8_docs_alignment_artifact",
}

REQUIRED_VALIDATION_ARTIFACT_KINDS = {
    "schema_generation",
    "contract_tests",
    "signing_golden_checks",
    "idempotency_behavior",
    "redaction_checks",
    "compatibility_checks",
    "docs_alignment",
}

REQUIRED_CONTRACT_TEST_KINDS = {
    "signed_command_submission",
    "duplicate_idempotency",
    "stable_error_preservation",
    "status_read",
}

EXPECTED_LOCAL_METADATA = {
    "schema_version": "overrid.v0",
    "environment": "local",
    "deterministic_seed": "sdk-phase8-local-seed-v0",
    "reset_marker": "local-dev-resettable-fixture-set",
    "redaction_profile": "phase8_redacted_test_fixture_refs_only",
    "tenant_id": "tenant:local-fixture",
    "actor_id": "actor:local-developer",
}

FORBIDDEN_ARTIFACT_MARKERS = [
    "raw_private_key",
    "private_key_value",
    "bearer_token_value",
    "seed_phrase_value",
    '"signature_value"',
    '"raw_request_body"',
    "private_payload_value",
    "fixture_credential_material",
    '"production_default": true',
    '"overwatch_runtime_event": true',
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def read_json(path: Path) -> Any:
    return json.loads(read(path))


def workspace_version() -> str:
    workspace_toml = read(Path("Cargo.toml"))
    match = re.search(r"(?m)^version = \"([^\"]+)\"", workspace_toml)
    if not match:
        raise AssertionError("Cargo workspace package version is missing")
    return match.group(1)


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


def assert_artifact_safe(path: Path) -> None:
    text = read(path).lower()
    for forbidden in FORBIDDEN_ARTIFACT_MARKERS:
        if forbidden in text:
            raise AssertionError(f"{path} contains forbidden Phase 8 artifact marker: {forbidden}")


def validate_sub_plan_phase8() -> None:
    text = read(SUB_PLAN)
    phase_8 = section(text, "## Phase 8: Fixtures, Contract Tests, And Validation Artifacts")
    for item in range(1, 6):
        assert_contains(phase_8, f"**8.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*8\.[1-5] .+?(?=\n- \*\*8\.|\n## Phase 9:)",
        phase_8,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "local/test fixtures",
        "local stack",
        "golden request envelopes",
        "security and redaction tests",
        "validation artifacts",
        "Progress evidence records",
    ]:
        assert_contains(phase_8, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 8 gate",
        "Rust-first SDK",
        "packages/sdk/fixtures/phase8",
        "public-API contract-test descriptors",
        "TypeScript/web and later bindings",
        "Do not add new SDK dependencies",
        "Validation artifacts are CI/build evidence only",
        "cargo test -p overrid-sdk phase8_",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 8 gap discovery",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    fixtures = read(SDK_FIXTURES)
    readme = read(SDK_README)
    combined = f"{lib}\n{fixtures}"

    for expected in REQUIRED_PHASE8_SYMBOLS + REQUIRED_PHASE8_TESTS:
        assert_contains(combined, expected, SDK_FIXTURES)
    for expected in [
        'include_str!("../fixtures/phase8/local_sdk_fixture_set.valid.json")',
        'include_str!("../fixtures/phase8/golden_cross_language_corpus.valid.json")',
        'include_str!("../fixtures/phase8/validation_artifacts_manifest.valid.json")',
        "public_api_only: true",
        "uses_internal_service_mock: false",
        "typescript_web_release_blocked_until_pass: true",
        "later_bindings_blocked_until_pass: true",
        "overwatch_runtime_event: false",
        "docdex_index_expected: true",
        "production_default: false",
        "contains_private_material: false",
        "contains_raw_payload: false",
    ]:
        assert_contains(fixtures, expected, SDK_FIXTURES)
    for expected in [
        "## Phase 8 Fixtures, Contract Tests, And Validation Artifacts",
        "`sdk_phase8_local_fixture_corpus()`",
        "`validate_phase8_local_fixture_corpus()`",
        "`validate_phase8_fixture_artifact_files()`",
        "`sdk_phase8_contract_tests()`",
        "`validate_phase8_contract_tests()`",
        "`sdk_phase8_golden_fixtures()`",
        "`validate_phase8_golden_fixtures()`",
        "`sdk_phase8_security_redaction_checks()`",
        "`validate_phase8_security_redaction_checks()`",
        "`sdk_phase8_validation_artifacts()`",
        "`validate_phase8_validation_artifacts()`",
        "not Overwatch runtime events",
    ]:
        assert_contains(readme, expected, SDK_README)
    if "[dependencies]\noverrid-contracts" not in cargo:
        raise AssertionError(f"{SDK_CARGO} should remain dependency-light for Phase 8")
    for forbidden in ["serde", "reqwest", "tokio", "postgres", "redis", "kafka", "nats", "s3"]:
        if forbidden in cargo.lower():
            raise AssertionError(f"{SDK_CARGO} contains unexpected Phase 8 dependency: {forbidden}")


def validate_fixture_artifacts() -> None:
    local = read_json(LOCAL_FIXTURE_SET)
    golden = read_json(GOLDEN_CORPUS)
    artifacts = read_json(VALIDATION_ARTIFACTS)

    for path in (LOCAL_FIXTURE_SET, GOLDEN_CORPUS, VALIDATION_ARTIFACTS):
        assert_artifact_safe(path)

    if not local.get("test_only"):
        raise AssertionError(f"{LOCAL_FIXTURE_SET} must be test_only")
    if local.get("production_default") is not False:
        raise AssertionError(f"{LOCAL_FIXTURE_SET} must reject production defaults")
    if local.get("contains_private_material") is not False:
        raise AssertionError(f"{LOCAL_FIXTURE_SET} must avoid private material")
    if local.get("contains_raw_payload") is not False:
        raise AssertionError(f"{LOCAL_FIXTURE_SET} must avoid raw payloads")
    for key, expected in EXPECTED_LOCAL_METADATA.items():
        if key in {"tenant_id", "actor_id"}:
            continue
        if local.get(key) != expected:
            raise AssertionError(f"{LOCAL_FIXTURE_SET} has invalid {key}: {local.get(key)!r}")
    local_kinds = {item["kind"] for item in local.get("fixtures", [])}
    missing_local = REQUIRED_LOCAL_FIXTURE_KINDS - local_kinds
    if missing_local:
        raise AssertionError(f"{LOCAL_FIXTURE_SET} missing fixture kinds: {sorted(missing_local)}")
    local_fixture_ids = {item["fixture_id"] for item in local.get("fixtures", [])}
    if len(local_fixture_ids) != len(local.get("fixtures", [])):
        raise AssertionError(f"{LOCAL_FIXTURE_SET} fixture ids must be unique")
    for item in local.get("fixtures", []):
        for key, expected in EXPECTED_LOCAL_METADATA.items():
            if item.get(key) != expected:
                raise AssertionError(
                    f"{LOCAL_FIXTURE_SET} fixture {item.get('fixture_id')} has invalid {key}: "
                    f"{item.get(key)!r}"
                )
        for flag in ("production_default", "contains_private_material", "contains_raw_payload"):
            if item.get(flag) is not False:
                raise AssertionError(
                    f"{LOCAL_FIXTURE_SET} fixture {item.get('fixture_id')} has unsafe {flag}"
                )
        value_ref = item.get("value_ref", "")
        if not isinstance(value_ref, str) or ":" not in value_ref:
            raise AssertionError(f"{LOCAL_FIXTURE_SET} fixture {item.get('fixture_id')} has invalid value_ref")

    if golden.get("rust_required") is not True:
        raise AssertionError(f"{GOLDEN_CORPUS} must require Rust first")
    if golden.get("typescript_web_release_blocked_until_pass") is not True:
        raise AssertionError(f"{GOLDEN_CORPUS} must gate TypeScript/web release")
    if golden.get("later_bindings_blocked_until_pass") is not True:
        raise AssertionError(f"{GOLDEN_CORPUS} must gate later bindings")
    golden_cases = {item["case_name"] for item in golden.get("cases", [])}
    missing_golden = REQUIRED_GOLDEN_CASES - golden_cases
    if missing_golden:
        raise AssertionError(f"{GOLDEN_CORPUS} missing golden cases: {sorted(missing_golden)}")
    for key, expected in {
        "schema_version": "overrid.v0",
        "sdk_name": "overrid-rust-sdk",
        "sdk_version": workspace_version(),
        "language_binding": "rust",
        "redaction_profile": EXPECTED_LOCAL_METADATA["redaction_profile"],
    }.items():
        if golden.get(key) != expected:
            raise AssertionError(f"{GOLDEN_CORPUS} has invalid {key}: {golden.get(key)!r}")
    for case in golden.get("cases", []):
        if case.get("schema_version") != "overrid.v0":
            raise AssertionError(f"{GOLDEN_CORPUS} case {case.get('case_name')} has invalid schema version")
        source_refs = case.get("source_fixture_refs")
        if not source_refs or not set(source_refs).issubset(local_fixture_ids):
            raise AssertionError(
                f"{GOLDEN_CORPUS} case {case.get('case_name')} has invalid source_fixture_refs"
            )
        for flag in (
            "contains_private_material",
            "contains_raw_payload",
            "contains_signature_value",
            "raw_request_body_included",
        ):
            if case.get(flag) is not False:
                raise AssertionError(f"{GOLDEN_CORPUS} case {case.get('case_name')} has unsafe {flag}")
        if case.get("artifact_safe") is not True:
            raise AssertionError(f"{GOLDEN_CORPUS} case {case.get('case_name')} must be artifact_safe")

    if artifacts.get("docdex_index_expected") is not True:
        raise AssertionError(f"{VALIDATION_ARTIFACTS} must be Docdex-indexable")
    if artifacts.get("overwatch_runtime_event") is not False:
        raise AssertionError(f"{VALIDATION_ARTIFACTS} must not be runtime events")
    artifact_names = {item["name"] for item in artifacts.get("artifacts", [])}
    missing_artifacts = REQUIRED_VALIDATION_ARTIFACTS - artifact_names
    if missing_artifacts:
        raise AssertionError(
            f"{VALIDATION_ARTIFACTS} missing artifacts: {sorted(missing_artifacts)}"
        )
    artifact_kinds = {item["kind"] for item in artifacts.get("artifacts", [])}
    if artifact_kinds != REQUIRED_VALIDATION_ARTIFACT_KINDS:
        raise AssertionError(
            f"{VALIDATION_ARTIFACTS} has invalid artifact kinds: {sorted(artifact_kinds)}"
        )
    for item in artifacts.get("artifacts", []):
        if not item.get("retention_rule"):
            raise AssertionError(f"{VALIDATION_ARTIFACTS} artifact {item.get('name')} lacks retention_rule")
        if item.get("docdex_index_expected") is not True:
            raise AssertionError(f"{VALIDATION_ARTIFACTS} artifact {item.get('name')} must be Docdex-indexed")
        if item.get("overwatch_runtime_event") is not False:
            raise AssertionError(f"{VALIDATION_ARTIFACTS} artifact {item.get('name')} must not be runtime event")
        if item.get("progress_evidence_required") is not True:
            raise AssertionError(f"{VALIDATION_ARTIFACTS} artifact {item.get('name')} needs progress evidence")
    contract_tests = artifacts.get("contract_tests", [])
    contract_kinds = {item["kind"] for item in contract_tests}
    missing_contracts = REQUIRED_CONTRACT_TEST_KINDS - contract_kinds
    if missing_contracts:
        raise AssertionError(f"{VALIDATION_ARTIFACTS} missing contract tests: {sorted(missing_contracts)}")
    for item in contract_tests:
        route = item.get("route", "")
        if not (route.startswith("/v1/overgate/") or route.startswith("/v1/control-plane/")):
            raise AssertionError(f"{VALIDATION_ARTIFACTS} contract test has private route: {route}")
        if item.get("method") not in {"GET", "POST"}:
            raise AssertionError(f"{VALIDATION_ARTIFACTS} contract test has invalid method: {item.get('method')}")
        for flag, expected in {
            "public_api_only": True,
            "local_stack_contract": True,
            "uses_internal_service_mock": False,
            "owning_services_available": False,
        }.items():
            if item.get(flag) is not expected:
                raise AssertionError(
                    f"{VALIDATION_ARTIFACTS} contract test {item.get('name')} has invalid {flag}"
                )
        blocker = item.get("blocker", "")
        if "owning public Overgate/control-plane service" not in blocker:
            raise AssertionError(f"{VALIDATION_ARTIFACTS} contract test {item.get('name')} lacks blocker")
        if not item.get("assertions"):
            raise AssertionError(f"{VALIDATION_ARTIFACTS} contract test {item.get('name')} lacks assertions")


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    fixtures = read(SDK_FIXTURES)
    for expected in [
        "| SDKs | Generated Rust SDK first",
        "TypeScript/web bindings from the same contracts",
        "Signed command envelopes",
        "idempotency keys",
        "trace ids",
        "stable reason codes",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for forbidden in ["postgres", "redis", "kafka", "nats", "s3", "vault", "blockchain", "nft"]:
        if forbidden in fixtures.lower():
            raise AssertionError(f"{SDK_FIXTURES} contains forbidden stack/product boundary: {forbidden}")


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase8.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk", "phase8_"])
    if "phase8_" not in result.stdout and "phase8_" not in result.stderr:
        raise AssertionError("Phase 8 focused Rust tests did not run")


def main() -> int:
    validate_sub_plan_phase8()
    validate_phase_docs()
    validate_sdk_code_and_readme()
    validate_fixture_artifacts()
    validate_tech_stack_alignment()
    validate_wrapper()
    validate_rust_tests()
    print("SDK Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
