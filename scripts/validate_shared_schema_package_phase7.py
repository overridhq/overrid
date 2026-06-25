#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 7 compatibility gate contracts."""

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
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_07_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase7CompatibilityContract::canonical().validate()"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase7.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase7_missing_migration.invalid.json"
    ): "schema.migration_metadata_required",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase7_deprecation_missing_window.invalid.json"
    ): "schema.deprecation_window_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase7_silent_downgrade.invalid.json"
    ): "schema.schema_version_unsupported",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase7_authority_gate_missing.invalid.json"
    ): "schema.authority_migration_gate_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase7_consumer_unknown.invalid.json"
    ): "schema.consumer_impact_missing",
}

REQUIRED_CHANGE_CLASSIFICATIONS = {
    "additive",
    "deprecated",
    "breaking",
    "blocked",
    "migration_required",
}
REQUIRED_BREAKING_KINDS = {
    "field_removal",
    "type_narrowing",
    "envelope_change",
    "signing_input_change",
    "privacy_class_change",
}
REQUIRED_AUTHORITY_MODULES = {
    "identity",
    "tenant",
    "command",
    "api_error",
    "event",
    "audit",
    "queue",
    "usage",
    "oru",
    "seal_ledger",
    "policy",
    "asset",
    "namespace",
    "credential",
    "secret_ref",
    "overvault",
    "overbase",
    "overstore",
    "native_app",
    "mobile",
    "ai",
    "docdex_rag",
    "ades",
    "cross_client_compatibility",
}
REQUIRED_CONSUMER_KINDS = {
    "service",
    "sdk",
    "cli",
    "adapter",
    "native_app",
    "mobile_client",
    "test_fixture",
}
REQUIRED_CONTRACTS = {
    "phase7_compatibility_contract",
    "phase7_schema_comparison",
    "phase7_deprecation_metadata",
    "phase7_stable_major_support",
    "phase7_authority_migration_gate",
    "phase7_consumer_impact_report",
    "phase7_rust_projection",
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


def phase7_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase7_compatibility_contract", record)
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
    assert_true("Phase 7" in schema.get("title", ""), "schema title missing Phase 7")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase7_compatibility_contract" in props, "schema missing Phase 7 root property")
    for def_name in REQUIRED_CONTRACTS | {
        "phase7_change_classification",
        "phase7_breaking_change_kind",
        "phase7_authority_sensitive_module",
        "phase7_consumer_kind",
    }:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    assert_true(
        REQUIRED_CHANGE_CLASSIFICATIONS <= set(defs["phase7_change_classification"].get("enum", [])),
        "schema Phase 7 change classification enum drifted",
    )
    assert_true(
        REQUIRED_BREAKING_KINDS <= set(defs["phase7_breaking_change_kind"].get("enum", [])),
        "schema Phase 7 breaking-kind enum drifted",
    )
    assert_true(
        REQUIRED_AUTHORITY_MODULES <= set(defs["phase7_authority_sensitive_module"].get("enum", [])),
        "schema Phase 7 authority module enum drifted",
    )
    assert_true(
        REQUIRED_CONSUMER_KINDS <= set(defs["phase7_consumer_kind"].get("enum", [])),
        "schema Phase 7 consumer kind enum drifted",
    )


def validate_phase7_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase7_payload(record)

    comparisons = payload.get("schema_comparisons", [])
    comparison_classifications = {
        comparison.get("change_classification")
        for comparison in comparisons
        if isinstance(comparison, dict)
    }
    comparison_kinds = {
        comparison.get("breaking_change_kind")
        for comparison in comparisons
        if isinstance(comparison, dict)
    }
    if comparisons and not REQUIRED_CHANGE_CLASSIFICATIONS <= comparison_classifications:
        errors.append("schema.comparison_classification_missing")
    if comparisons and not REQUIRED_BREAKING_KINDS <= comparison_kinds:
        errors.append("schema.comparison_kind_missing")
    for comparison in comparisons:
        if not isinstance(comparison, dict):
            errors.append("schema.comparison_invalid")
            continue
        classification = comparison.get("change_classification")
        if classification not in REQUIRED_CHANGE_CLASSIFICATIONS:
            errors.append("schema.comparison_invalid")
        if classification in {"breaking", "migration_required"}:
            if comparison.get("breaking_change_kind") not in REQUIRED_BREAKING_KINDS:
                errors.append("schema.comparison_invalid")
        if classification in {"breaking", "migration_required", "deprecated", "blocked"} and (
            comparison.get("migration_metadata_required") is not True
            or comparison.get("migration_metadata_present") is not True
        ):
            errors.append("schema.migration_metadata_required")
        if classification in {"breaking", "migration_required", "blocked"} and (
            comparison.get("blocked_without_migration") is not True
        ):
            errors.append("schema.migration_metadata_required")
        if classification == "additive" and (
            comparison.get("migration_metadata_required") is not False
            or comparison.get("migration_metadata_present") is not False
            or comparison.get("blocked_without_migration") is not False
        ):
            errors.append("schema.comparison_invalid")
        if (
            not str(comparison.get("previous_schema_ref", "")).startswith("schema:")
            or not str(comparison.get("next_schema_ref", "")).startswith("schema:")
            or not str(comparison.get("compatibility_report_ref", "")).startswith("compat:")
        ):
            errors.append("schema.comparison_invalid")

    deprecations = payload.get("deprecation_metadata", [])
    if deprecations and len(deprecations) < 2:
        errors.append("schema.deprecation_window_missing")
    for deprecation in deprecations:
        if not isinstance(deprecation, dict):
            errors.append("schema.deprecation_window_missing")
            continue
        if (
            not deprecation.get("object_family")
            or not deprecation.get("deprecated_field")
            or not deprecation.get("replacement_mapping")
            or not deprecation.get("consumer_list")
            or not deprecation.get("first_deprecated_version")
            or not deprecation.get("last_supported_version")
            or not deprecation.get("migration_reason")
            or not deprecation.get("owner")
            or not deprecation.get("support_window")
            or deprecation.get("active_consumers_require_notes") is not True
        ):
            errors.append("schema.deprecation_window_missing")

    support = payload.get("stable_major_support")
    if isinstance(support, dict):
        if (
            support.get("policy_name") != "current_plus_previous_stable_major"
            or support.get("previous_stable_major_supported") is not True
            or not support.get("external_consumer_gate")
            or support.get("unsupported_version_reason_code") != "schema.schema_version_unsupported"
            or support.get("silent_downgrade_allowed") is not False
            or support.get("contract_tests_required") is not True
        ):
            errors.append("schema.schema_version_unsupported")

    gates = payload.get("authority_sensitive_migration_gates", [])
    gate_modules = {gate.get("module") for gate in gates if isinstance(gate, dict)}
    if gates and not REQUIRED_AUTHORITY_MODULES <= gate_modules:
        errors.append("schema.authority_migration_gate_missing")
    for gate in gates:
        if not isinstance(gate, dict):
            errors.append("schema.authority_migration_gate_missing")
            continue
        if gate.get("module") not in REQUIRED_AUTHORITY_MODULES or any(
            gate.get(flag) is not True
            for flag in [
                "migration_plan_required",
                "consumer_impact_required",
                "rollback_guidance_required",
                "stable_error_behavior_required",
                "owner_signoff_required",
                "release_blocked_without_gate",
            ]
        ):
            errors.append("schema.authority_migration_gate_missing")

    reports = payload.get("consumer_impact_reports", [])
    report_kinds = {report.get("consumer_kind") for report in reports if isinstance(report, dict)}
    if reports and not REQUIRED_CONSUMER_KINDS <= report_kinds:
        errors.append("schema.consumer_impact_missing")
    for report in reports:
        if not isinstance(report, dict):
            errors.append("schema.consumer_impact_missing")
            continue
        if (
            report.get("consumer_kind") not in REQUIRED_CONSUMER_KINDS
            or not report.get("schema_module")
            or not report.get("field_path")
            or not report.get("consumers")
            or not str(report.get("owner_signoff_ref", "")).startswith("signoff:")
            or not str(report.get("compatibility_report_ref", "")).startswith("compat:")
            or not str(report.get("migration_notes_ref", "")).startswith("migration:")
            or report.get("breaking_change_requires_report") is not True
            or report.get("identified_consumers_required") is not True
        ):
            errors.append("schema.consumer_impact_missing")

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
    entry = manifest.get("shared_schema_package_phase7")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase7")
    assert_true(entry.get("source_of_truth") == "json_schema", "Phase 7 source of truth drifted")
    assert_true(entry.get("rust_first_validation") is True, "Phase 7 must remain Rust-first")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "Phase 7 canonical schema drifted")
    rust = entry.get("rust_projection", {})
    assert_true(rust.get("path") == RUST_OUTPUT, "Phase 7 Rust output path drifted")
    assert_true(rust.get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Phase 7 validator entrypoint drifted")
    assert_true(rust.get("non_authoritative") is True, "Phase 7 Rust projection became authority")
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "Phase 7 contracts list incomplete")
    fixtures = entry.get("fixtures", {})
    assert_true(str(VALID_FIXTURES[0]) in fixtures.get("valid", []), "Phase 7 valid fixture missing from manifest")
    for fixture in INVALID_FIXTURES:
        assert_true(str(fixture) in fixtures.get("invalid", []), f"Phase 7 invalid fixture missing: {fixture}")
    assert_true(not validate_phase7_record(entry), "manifest Phase 7 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase7_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid but returned {sorted(set(errors))}")
    for fixture, expected in INVALID_FIXTURES.items():
        errors = validate_phase7_record(load_json(fixture))
        assert_true(expected in errors, f"{fixture} did not fail with {expected}; got {sorted(set(errors))}")


def check_docs() -> None:
    sub_plan = read_text(SUB_PLAN)
    assert_contains(sub_plan, "## Phase 7: Compatibility, Deprecation, And Migration Gates", SUB_PLAN)
    assert_contains(sub_plan, "7.1 Implement schema comparison", SUB_PLAN)
    assert_contains(sub_plan, "7.5 Build consumer impact reports", SUB_PLAN)

    contracts_readme = read_text(CONTRACTS_README)
    assert_contains(contracts_readme, "Shared Schema Package Phase 7", CONTRACTS_README)
    assert_contains(contracts_readme, "current-plus-previous stable major support", CONTRACTS_README)
    assert_contains(contracts_readme, VALIDATOR_ENTRYPOINT, CONTRACTS_README)

    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "Shared Schema Package Phase 7", SCHEMAS_README)
    assert_contains(schemas_readme, "schema comparison, deprecation metadata", SCHEMAS_README)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 7", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 7 Progress", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)


def check_rust_projection() -> None:
    rust = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase7CompatibilityContract",
        "SharedSchemaPhase7SchemaComparison",
        "SharedSchemaPhase7DeprecationMetadata",
        "SharedSchemaPhase7StableMajorSupport",
        "SharedSchemaPhase7AuthorityMigrationGate",
        "SharedSchemaPhase7ConsumerImpactReport",
        VALIDATOR_ENTRYPOINT,
        "schema.schema_version_unsupported",
        "REQUIRED_SHARED_SCHEMA_PHASE7_AUTHORITY_MODULES",
    ]:
        assert_contains(rust, expected, RUST_PROJECTION)


def check_suite_wiring() -> None:
    suite = read_text(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_shared_schema_package_phase7.py")', SUITE_VALIDATOR)


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
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase7"])
    print("Shared Schema Package Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
