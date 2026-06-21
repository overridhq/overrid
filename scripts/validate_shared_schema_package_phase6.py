#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 6 fixture and artifact contracts."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SCHEMA = Path("packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json")
MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
CONTRACTS_README = Path("packages/schemas/overrid_contracts/README.md")
SCHEMAS_README = Path("packages/schemas/README.md")
RUST_PROJECTION = Path("packages/schemas/overrid_contracts/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_007_shared_schema_package.md")
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_06_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase6FixtureContract::canonical().validate()"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase6.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase6_missing_seed.invalid.json"
    ): "schema.fixture_seed_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase6_ambiguous_error.invalid.json"
    ): "schema.ambiguous_parser_error",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase6_golden_drift.invalid.json"
    ): "schema.golden_fixture_drift",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase6_local_stack_production_data.invalid.json"
    ): "schema.local_stack_fixture_unsafe",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase6_artifact_overwatch_event.invalid.json"
    ): "schema.artifact_authority_drift",
}

REQUIRED_FIXTURE_FAMILIES = {
    "primitive",
    "identity",
    "tenant",
    "command",
    "api_error",
    "event",
    "audit",
    "manifest",
    "resource",
    "queue",
    "lease",
    "usage",
    "policy",
    "key_metadata",
}
REQUIRED_NEGATIVE_CASES = {
    "unsupported_version",
    "unknown_field",
    "missing_envelope",
    "privacy_missing",
    "malformed_ref",
    "stale_reason_code",
    "migration_needed",
    "deprecated_payload",
}
REQUIRED_GOLDEN_ENVELOPES = {"command", "event", "audit", "usage", "ledger", "api_error"}
REQUIRED_ARTIFACTS = {
    "schema_lint",
    "generated_output_diff",
    "fixture_pass_fail_counts",
    "redaction_check",
    "compatibility_report",
}
REQUIRED_CONTRACTS = {
    "phase6_fixture_contract",
    "phase6_valid_fixture_builder",
    "phase6_invalid_fixture_builder",
    "phase6_golden_envelope_fixture",
    "phase6_local_stack_contract",
    "phase6_integration_harness_contract",
    "phase6_validation_artifact",
    "phase6_rust_projection",
}
REQUIRED_SOURCE_INPUTS = {CANONICAL_SCHEMA, MANIFEST_PATH, BUILD_PLAN_PATH, TECH_STACK_PATH}


def load_json(path: Path) -> Any:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"missing required file: {path}")
    with full_path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_text(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_contains(text: str, expected: str, source: Path) -> None:
    assert_true(expected in text, f"{source} missing required text: {expected}")


def phase6_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase6_fixture_contract", record)
    return payload if isinstance(payload, dict) else {}


def check_required_files() -> None:
    for path in [
        SCHEMA,
        MANIFEST,
        CONTRACTS_README,
        SCHEMAS_README,
        RUST_PROJECTION,
        SUB_PLAN,
        PHASE_PLAN,
        PHASE_PROGRESS,
        TECH_STACK,
        SUITE_VALIDATOR,
        *VALID_FIXTURES,
        *INVALID_FIXTURES.keys(),
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_schema_surface(schema: dict[str, Any]) -> None:
    assert_true(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "schema draft drifted")
    assert_true(schema.get("additionalProperties") is False, "root schema must reject unknown fields")
    assert_true("Phase 6" in schema.get("title", ""), "schema title missing Phase 6")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase6_fixture_contract" in props, "schema missing Phase 6 root property")
    for def_name in REQUIRED_CONTRACTS | {
        "phase6_fixture_family",
        "phase6_negative_case",
        "phase6_golden_envelope_family",
        "phase6_validation_artifact_kind",
    }:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    assert_true(
        REQUIRED_FIXTURE_FAMILIES <= set(defs["phase6_fixture_family"].get("enum", [])),
        "schema Phase 6 fixture family enum drifted",
    )
    assert_true(
        REQUIRED_NEGATIVE_CASES <= set(defs["phase6_negative_case"].get("enum", [])),
        "schema Phase 6 negative case enum drifted",
    )
    assert_true(
        REQUIRED_GOLDEN_ENVELOPES <= set(defs["phase6_golden_envelope_family"].get("enum", [])),
        "schema Phase 6 golden envelope enum drifted",
    )
    assert_true(
        REQUIRED_ARTIFACTS <= set(defs["phase6_validation_artifact_kind"].get("enum", [])),
        "schema Phase 6 artifact enum drifted",
    )


def validate_phase6_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase6_payload(record)

    valid_builders = payload.get("valid_fixture_builders", [])
    valid_by_family = {
        builder.get("fixture_family"): builder
        for builder in valid_builders
        if isinstance(builder, dict)
    }
    if valid_builders and not REQUIRED_FIXTURE_FAMILIES <= set(valid_by_family):
        errors.append("schema.fixture_family_missing")
    for builder in valid_builders:
        if not isinstance(builder, dict):
            errors.append("schema.fixture_builder_invalid")
            continue
        if not str(builder.get("deterministic_seed", "")).startswith("fixture-seed:"):
            errors.append("schema.fixture_seed_missing")
        if not str(builder.get("schema_ref", "")).startswith("schema:"):
            errors.append("schema.fixture_schema_ref_missing")
        if builder.get("stable_from_clean_checkout") is not True:
            errors.append("schema.fixture_seed_missing")
        if builder.get("privacy_defaults") != "redacted_refs_only":
            errors.append("schema.fixture_private_payload_forbidden")
        if builder.get("production_credentials_allowed") is not False:
            errors.append("schema.fixture_credentials_forbidden")
        if builder.get("private_payload_defaults_allowed") is not False:
            errors.append("schema.fixture_private_payload_forbidden")
        consumers = set(builder.get("consumers", []))
        if not {"integration_harness", "sdk", "cli"} <= consumers:
            errors.append("schema.fixture_consumer_missing")
        if not all("." in code for code in builder.get("expected_reason_codes", [])):
            errors.append("schema.reason_code_undocumented")

    invalid_builders = payload.get("invalid_fixture_builders", [])
    invalid_by_kind = {
        builder.get("case_kind"): builder
        for builder in invalid_builders
        if isinstance(builder, dict)
    }
    if invalid_builders and not REQUIRED_NEGATIVE_CASES <= set(invalid_by_kind):
        errors.append("schema.invalid_fixture_case_missing")
    for builder in invalid_builders:
        if not isinstance(builder, dict):
            errors.append("schema.invalid_fixture_case_missing")
            continue
        if builder.get("case_kind") not in REQUIRED_NEGATIVE_CASES:
            errors.append("schema.invalid_fixture_case_missing")
        if builder.get("payload_family") not in REQUIRED_FIXTURE_FAMILIES:
            errors.append("schema.fixture_family_missing")
        if "." not in str(builder.get("stable_reason_code", "")):
            errors.append("schema.reason_code_undocumented")
        if builder.get("ambiguous_parser_error_allowed") is not False:
            errors.append("schema.ambiguous_parser_error")
        if builder.get("case_kind") in {"migration_needed", "deprecated_payload"} and builder.get(
            "migration_metadata_required"
        ) is not True:
            errors.append("schema.migration_metadata_required")

    golden_fixtures = payload.get("golden_envelope_fixtures", [])
    golden_by_family = {
        fixture.get("envelope_family"): fixture
        for fixture in golden_fixtures
        if isinstance(fixture, dict)
    }
    if golden_fixtures and not REQUIRED_GOLDEN_ENVELOPES <= set(golden_by_family):
        errors.append("schema.golden_fixture_drift")
    for fixture in golden_fixtures:
        if not isinstance(fixture, dict):
            errors.append("schema.golden_fixture_drift")
            continue
        if fixture.get("envelope_family") not in REQUIRED_GOLDEN_ENVELOPES:
            errors.append("schema.golden_fixture_drift")
        if "#golden." not in str(fixture.get("fixture_path", "")):
            errors.append("schema.golden_fixture_drift")
        targets = set(fixture.get("round_trip_targets", []))
        if not {"rust", "cli", "sdk"} <= targets:
            errors.append("schema.golden_fixture_drift")
        if (
            fixture.get("stable_field_order") is not True
            or fixture.get("field_loss_allowed") is not False
            or fixture.get("ordering_drift_allowed") is not False
            or fixture.get("error_shape_changes_allowed") is not False
        ):
            errors.append("schema.golden_fixture_drift")

    local_stack = payload.get("local_stack_contract")
    if isinstance(local_stack, dict):
        if (
            local_stack.get("sds_ref") != "SDS #4 Local Development Stack"
            or "#local_stack_fixture_bundle" not in str(local_stack.get("fixture_bundle_path", ""))
            or local_stack.get("resettable") is not True
            or local_stack.get("deterministic_state") is not True
            or local_stack.get("test_only_marker_required") is not True
            or local_stack.get("production_data_enabled") is not False
            or local_stack.get("production_credentials_allowed") is not False
            or local_stack.get("raw_secret_defaults_allowed") is not False
        ):
            errors.append("schema.local_stack_fixture_unsafe")

    harness = payload.get("integration_harness_contract")
    if isinstance(harness, dict):
        if (
            harness.get("sds_ref") != "SDS #3 Integration Test Harness"
            or "#integration_harness_fixture_bundle" not in str(harness.get("fixture_bundle_path", ""))
            or harness.get("reuses_fixture_builders") is not True
            or harness.get("production_credentials_allowed") is not False
            or harness.get("private_payload_defaults_allowed") is not False
            or harness.get("stable_reason_codes_required") is not True
            or harness.get("redaction_required") is not True
        ):
            errors.append("schema.integration_harness_fixture_unsafe")

    artifacts = payload.get("validation_artifacts", [])
    artifacts_by_kind = {
        artifact.get("artifact_kind"): artifact
        for artifact in artifacts
        if isinstance(artifact, dict)
    }
    if artifacts and not REQUIRED_ARTIFACTS <= set(artifacts_by_kind):
        errors.append("schema.artifact_authority_drift")
    for artifact in artifacts:
        if not isinstance(artifact, dict):
            errors.append("schema.artifact_authority_drift")
            continue
        if (
            artifact.get("artifact_kind") not in REQUIRED_ARTIFACTS
            or not str(artifact.get("path", "")).startswith("artifacts/shared-schema-phase6/")
            or artifact.get("generated_by") != str(Path("scripts/validate_shared_schema_package_phase6.py"))
            or artifact.get("non_authoritative") is not True
            or artifact.get("overwatch_runtime_event") is not False
            or artifact.get("redaction_checked") is not True
        ):
            errors.append("schema.artifact_authority_drift")

    source_inputs = set(payload.get("source_hash_inputs", []))
    if source_inputs and not REQUIRED_SOURCE_INPUTS <= source_inputs:
        errors.append("schema.source_input_missing")

    rust_projection = payload.get("rust_projection")
    if isinstance(rust_projection, dict):
        if (
            rust_projection.get("path") != RUST_OUTPUT
            or rust_projection.get("validator_entrypoint") != VALIDATOR_ENTRYPOINT
            or rust_projection.get("non_authoritative") is not True
        ):
            errors.append("schema.rust_projection_authority_drift")

    return errors


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase6")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase6")
    assert_true(entry.get("source_of_truth") == "json_schema", "Phase 6 source of truth drifted")
    assert_true(entry.get("rust_first_validation") is True, "Phase 6 must remain Rust-first")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "Phase 6 canonical schema drifted")
    rust = entry.get("rust_projection", {})
    assert_true(rust.get("path") == RUST_OUTPUT, "Phase 6 Rust output path drifted")
    assert_true(rust.get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Phase 6 validator entrypoint drifted")
    assert_true(rust.get("non_authoritative") is True, "Phase 6 Rust projection became authority")
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "Phase 6 contracts list incomplete")
    fixtures = entry.get("fixtures", {})
    assert_true(str(VALID_FIXTURES[0]) in fixtures.get("valid", []), "Phase 6 valid fixture missing from manifest")
    for fixture in INVALID_FIXTURES:
        assert_true(str(fixture) in fixtures.get("invalid", []), f"Phase 6 invalid fixture missing: {fixture}")
    assert_true(not validate_phase6_record(entry), "manifest Phase 6 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase6_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid but returned {sorted(set(errors))}")
    for fixture, expected in INVALID_FIXTURES.items():
        errors = validate_phase6_record(load_json(fixture))
        assert_true(expected in errors, f"{fixture} did not fail with {expected}; got {sorted(set(errors))}")


def check_docs() -> None:
    sub_plan = read_text(SUB_PLAN)
    assert_contains(sub_plan, "## Phase 6: Fixtures, Golden Tests, And Integration-Harness Contracts", SUB_PLAN)
    assert_contains(sub_plan, "6.1 Build valid fixture builders", SUB_PLAN)
    assert_contains(sub_plan, "6.5 Publish validation artifacts", SUB_PLAN)

    contracts_readme = read_text(CONTRACTS_README)
    assert_contains(contracts_readme, "Shared Schema Package Phase 6", CONTRACTS_README)
    assert_contains(contracts_readme, "deterministic valid and invalid", CONTRACTS_README)
    assert_contains(contracts_readme, "fixture-builder metadata", CONTRACTS_README)
    assert_contains(contracts_readme, VALIDATOR_ENTRYPOINT, CONTRACTS_README)

    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "Shared Schema Package Phase 6", SCHEMAS_README)
    assert_contains(schemas_readme, "fixture builders, golden envelope fixtures", SCHEMAS_README)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 6", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 6 Progress", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)


def check_rust_projection() -> None:
    rust = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase6FixtureContract",
        "SharedSchemaPhase6ValidFixtureBuilder",
        "SharedSchemaPhase6InvalidFixtureBuilder",
        "SharedSchemaPhase6GoldenEnvelopeFixture",
        "SharedSchemaPhase6ValidationArtifact",
        VALIDATOR_ENTRYPOINT,
        "schema.fixture_validated",
        "SDS #3 Integration Test Harness",
        "SDS #4 Local Development Stack",
    ]:
        assert_contains(rust, expected, RUST_PROJECTION)


def check_suite_wiring() -> None:
    suite = read_text(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_shared_schema_package_phase6.py")', SUITE_VALIDATOR)


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError(f"command failed with exit {result.returncode}: {' '.join(command)}")


def main() -> int:
    check_required_files()
    schema = load_json(SCHEMA)
    manifest = load_json(MANIFEST)
    check_schema_surface(schema)
    check_manifest(manifest)
    check_fixtures()
    check_docs()
    check_rust_projection()
    check_suite_wiring()
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase6"])
    print("Shared Schema Package Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
