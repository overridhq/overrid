#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 9 release and CI contracts."""

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
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_09_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase9ReleaseCiContract::canonical().validate()"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase9.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_release_gate_missing.invalid.json"
    ): "schema.release_gate_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_untyped_payload.invalid.json"
    ): "schema.generated_contract_consumption_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_coverage_missing.invalid.json"
    ): "schema.schema_coverage_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_docs_backlink_missing.invalid.json"
    ): "schema.generated_docs_backlink_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_product_bypass.invalid.json"
    ): "schema.product_hardening_bypass",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase9_projection_authority.invalid.json"
    ): "schema.rust_projection_authority_drift",
}

REQUIRED_RELEASE_GATES = {
    "schema_lint",
    "rust_generation_projection",
    "fixture_validation",
    "compatibility_report",
    "redaction_check",
    "generated_docs",
    "consumer_impact_report",
}
REQUIRED_CONSUMER_SURFACES = {
    "service",
    "sdk",
    "cli",
    "worker",
    "node_agent",
    "ui",
    "adapter",
    "test",
}
REQUIRED_COVERAGE_OBJECTS = {
    "command_envelope",
    "audit_event",
    "package_manifest",
    "ledger_ref",
    "view_model",
}
REQUIRED_DOC_ARTIFACTS = {
    "schema_reference",
    "reason_code_reference",
    "migration_notes",
    "compatibility_report",
    "fixture_examples",
    "consumer_registry_view",
}
REQUIRED_PRODUCT_SURFACES = {
    "docdex",
    "mcoda",
    "codali",
    "admin_developer_ui",
    "cli",
    "sdk",
    "adapters",
    "ai_gateway",
    "encrypted_docdex_rag",
}
REQUIRED_CONTRACTS = {
    "phase9_release_ci_contract",
    "phase9_release_workflow_gate",
    "phase9_contract_consumption_lint",
    "phase9_schema_coverage_report",
    "phase9_documentation_publishing_item",
    "phase9_product_hardening_check",
    "phase9_rust_projection",
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


def phase9_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase9_release_ci_contract", record)
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
    assert_true("Phase 9" in schema.get("title", ""), "schema title missing Phase 9")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase9_release_ci_contract" in props, "schema missing Phase 9 root property")
    for def_name in REQUIRED_CONTRACTS | {
        "phase9_release_gate",
        "phase9_consumer_surface",
        "phase9_generated_target",
        "phase9_documentation_artifact",
        "phase9_product_surface",
    }:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")
    assert_true(
        REQUIRED_RELEASE_GATES <= set(defs["phase9_release_gate"].get("enum", [])),
        "schema Phase 9 release gate enum drifted",
    )
    assert_true(
        REQUIRED_CONSUMER_SURFACES <= set(defs["phase9_consumer_surface"].get("enum", [])),
        "schema Phase 9 consumer surface enum drifted",
    )
    assert_true(
        REQUIRED_DOC_ARTIFACTS <= set(defs["phase9_documentation_artifact"].get("enum", [])),
        "schema Phase 9 docs artifact enum drifted",
    )
    assert_true(
        REQUIRED_PRODUCT_SURFACES <= set(defs["phase9_product_surface"].get("enum", [])),
        "schema Phase 9 product surface enum drifted",
    )


def validate_phase9_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase9_payload(record)

    gates = payload.get("release_workflow_gates", [])
    gate_names = {gate.get("gate_name") for gate in gates if isinstance(gate, dict)}
    if gates and not REQUIRED_RELEASE_GATES <= gate_names:
        errors.append("schema.release_gate_missing")
    for gate in gates:
        if not isinstance(gate, dict):
            errors.append("schema.release_gate_missing")
            continue
        if (
            gate.get("gate_name") not in REQUIRED_RELEASE_GATES
            or gate.get("required_before_release") is not True
            or gate.get("ci_blocks_release") is not True
            or not str(gate.get("artifact_ref", "")).startswith("artifact:")
            or not str(gate.get("blocked_reason_code", "")).startswith("schema.")
        ):
            errors.append("schema.release_gate_missing")

    lints = payload.get("contract_consumption_lints", [])
    lint_surfaces = {lint.get("consumer_surface") for lint in lints if isinstance(lint, dict)}
    if lints and not REQUIRED_CONSUMER_SURFACES <= lint_surfaces:
        errors.append("schema.generated_contract_consumption_missing")
    for lint in lints:
        if not isinstance(lint, dict):
            errors.append("schema.generated_contract_consumption_missing")
            continue
        if (
            lint.get("consumer_surface") not in REQUIRED_CONSUMER_SURFACES
            or lint.get("generated_contract_required") is not True
            or lint.get("untyped_payload_allowed") is not False
            or lint.get("private_schema_fork_allowed") is not False
            or lint.get("ad_hoc_string_parsing_allowed") is not False
            or not str(lint.get("review_checklist_ref", "")).startswith("checklist:")
        ):
            errors.append("schema.generated_contract_consumption_missing")

    reports = payload.get("schema_coverage_reports", [])
    report_objects = {report.get("object_family") for report in reports if isinstance(report, dict)}
    if reports and not REQUIRED_COVERAGE_OBJECTS <= report_objects:
        errors.append("schema.schema_coverage_missing")
    for report in reports:
        if not isinstance(report, dict):
            errors.append("schema.schema_coverage_missing")
            continue
        coverage = report.get("fixture_coverage", {})
        if (
            not str(report.get("build_phase", "")).startswith("master_phase:")
            or not report.get("service_family")
            or not report.get("object_family")
            or not report.get("privacy_class")
            or not report.get("validation_status")
            or int(coverage.get("valid_fixture_count", 0)) < 1
            or int(coverage.get("invalid_fixture_count", 0)) < 1
            or not report.get("generated_targets")
            or report.get("release_status") != "validated"
            or report.get("draft_contract_consumption_allowed") is not False
        ):
            errors.append("schema.schema_coverage_missing")

    docs = payload.get("documentation_publishing_items", [])
    doc_artifacts = {item.get("artifact_kind") for item in docs if isinstance(item, dict)}
    if docs and not REQUIRED_DOC_ARTIFACTS <= doc_artifacts:
        errors.append("schema.generated_docs_backlink_missing")
    for item in docs:
        if not isinstance(item, dict):
            errors.append("schema.generated_docs_backlink_missing")
            continue
        if (
            item.get("artifact_kind") not in REQUIRED_DOC_ARTIFACTS
            or not str(item.get("output_path", "")).startswith(
                "packages/schemas/overrid_contracts/generated/docs/"
            )
            or item.get("source_schema") != CANONICAL_SCHEMA
            or item.get("owning_sds") != "docs/sds/foundation/shared_schema_package.md"
            or item.get("build_plan_phase_gate") != "SUB BUILD PLAN #7 Phase 9"
            or item.get("backlinks_required") is not True
            or item.get("non_authoritative") is not True
        ):
            errors.append("schema.generated_docs_backlink_missing")

    products = payload.get("product_hardening_checks", [])
    product_surfaces = {product.get("product_surface") for product in products if isinstance(product, dict)}
    if products and not REQUIRED_PRODUCT_SURFACES <= product_surfaces:
        errors.append("schema.product_hardening_bypass")
    for product in products:
        if not isinstance(product, dict):
            errors.append("schema.product_hardening_bypass")
            continue
        if (
            product.get("product_surface") not in REQUIRED_PRODUCT_SURFACES
            or product.get("generated_contract_consumption_required") is not True
            or product.get("overgate_envelope_required") is not True
            or product.get("stable_errors_required") is not True
            or product.get("privacy_classification_required") is not True
            or product.get("redaction_required") is not True
            or product.get("bypass_allowed") is not False
        ):
            errors.append("schema.product_hardening_bypass")

    rust_projection = payload.get("rust_projection")
    if isinstance(rust_projection, dict):
        if (
            rust_projection.get("path") != RUST_OUTPUT
            or rust_projection.get("validator_entrypoint") != VALIDATOR_ENTRYPOINT
            or rust_projection.get("non_authoritative") is not True
        ):
            errors.append("schema.rust_projection_authority_drift")

    source_inputs = set(payload.get("source_hash_inputs", []))
    if source_inputs and not REQUIRED_SOURCE_INPUTS <= source_inputs:
        errors.append("schema.source_input_missing")

    return errors


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase9")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase9")
    assert_true(entry.get("source_of_truth") == "json_schema", "Phase 9 source of truth drifted")
    assert_true(entry.get("rust_first_validation") is True, "Phase 9 must remain Rust-first")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "Phase 9 canonical schema drifted")
    rust = entry.get("rust_projection", {})
    assert_true(rust.get("path") == RUST_OUTPUT, "Phase 9 Rust output path drifted")
    assert_true(rust.get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Phase 9 validator entrypoint drifted")
    assert_true(rust.get("non_authoritative") is True, "Phase 9 Rust projection became authority")
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "Phase 9 contracts list incomplete")
    fixtures = entry.get("fixtures", {})
    assert_true(str(VALID_FIXTURES[0]) in fixtures.get("valid", []), "Phase 9 valid fixture missing from manifest")
    for fixture in INVALID_FIXTURES:
        assert_true(str(fixture) in fixtures.get("invalid", []), f"Phase 9 invalid fixture missing: {fixture}")
    assert_true(not validate_phase9_record(entry), "manifest Phase 9 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase9_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid, got {errors}")
    for fixture, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase9_record(load_json(fixture))
        assert_true(expected_error in errors, f"{fixture} missing expected error {expected_error}; got {errors}")


def check_docs_and_planning() -> None:
    sub_plan = read_text(SUB_PLAN)
    assert_contains(sub_plan, "## Phase 9: Package Release, CI Enforcement, And Consumer Registry", SUB_PLAN)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)

    contracts_readme = read_text(CONTRACTS_README)
    assert_contains(contracts_readme, "Shared Schema Package Phase 9", CONTRACTS_README)
    assert_contains(contracts_readme, VALIDATOR_ENTRYPOINT, CONTRACTS_README)

    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "Shared Schema Package Phase 9", SCHEMAS_README)
    assert_contains(schemas_readme, "release workflow, CI enforcement, consumer registry", SCHEMAS_README)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 9", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 9 Progress", PHASE_PROGRESS)


def check_rust_projection() -> None:
    source = read_text(RUST_PROJECTION)
    for expected in [
        "PHASE9_CANONICAL_SCHEMA_SOURCE",
        "PHASE9_VALIDATOR_SCRIPT",
        "SharedSchemaPhase9ReleaseWorkflowGate",
        "SharedSchemaPhase9ContractConsumptionLint",
        "SharedSchemaPhase9SchemaCoverageReport",
        "SharedSchemaPhase9DocumentationPublishingItem",
        "SharedSchemaPhase9ProductHardeningCheck",
        "SharedSchemaPhase9ReleaseCiContract",
        "SharedSchemaPhase9ContractError",
        VALIDATOR_ENTRYPOINT,
    ]:
        assert_contains(source, expected, RUST_PROJECTION)


def check_suite_wiring() -> None:
    suite = read_text(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_shared_schema_package_phase9.py")', SUITE_VALIDATOR)


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError(f"command failed: {' '.join(command)}")


def main() -> int:
    check_required_files()
    schema = load_json(SCHEMA)
    manifest = load_json(MANIFEST)
    check_schema_surface(schema)
    check_manifest(manifest)
    check_fixtures()
    check_docs_and_planning()
    check_rust_projection()
    check_suite_wiring()
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase9"])
    print("Shared Schema Package Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
