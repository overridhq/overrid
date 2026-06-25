#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 8 downstream domain expansion contracts."""

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
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_08_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase8DomainExpansionContract::canonical().validate()"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase8.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase8_execution_private_duplicate.invalid.json"
    ): "schema.generated_contract_consumption_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase8_accounting_pricing_assumption.invalid.json"
    ): "schema.accounting_assumption_forbidden",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase8_storage_raw_secret.invalid.json"
    ): "schema.storage_secret_ref_invalid",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase8_ai_private_payload.invalid.json"
    ): "schema.private_payload_leakage",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase8_typescript_authority.invalid.json"
    ): "schema.typescript_authority_drift",
}

REQUIRED_DOMAIN_FAMILIES = {
    "execution_scheduling",
    "trust_policy_verification",
    "accounting_rights_settlement",
    "data_storage_namespace_secret_refs",
    "ai_docdex_mobile_native_ades",
}
REQUIRED_GUARDRAILS = {
    "generated_contract_consumption",
    "owner_phase_gate",
    "strict_unknown_field_rejection",
    "ref_only_secret_storage",
    "evidence_backed_namespace_ownership",
    "no_conventional_cloud_storage",
    "no_pricing_revenue_blockchain",
    "privacy_classified_ai_user_content",
    "typescript_second_projection",
}
REQUIRED_ACCOUNTING_FORBIDDEN_TERMS = {
    "price schedule",
    "revenue projection",
    "customer count",
    "blockchain",
    "NFT",
}
REQUIRED_CONTRACTS = {
    "phase8_domain_expansion_contract",
    "phase8_domain_module",
    "phase8_guardrail_check",
    "phase8_typescript_projection_requirement",
    "phase8_rust_projection",
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


def phase8_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase8_domain_expansion_contract", record)
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
    assert_true("Phase 8" in schema.get("title", ""), "schema title missing Phase 8")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase8_domain_expansion_contract" in props, "schema missing Phase 8 root property")
    for def_name in REQUIRED_CONTRACTS | {"phase8_domain_family", "phase8_required_guardrail"}:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")
    assert_true(
        REQUIRED_DOMAIN_FAMILIES <= set(defs["phase8_domain_family"].get("enum", [])),
        "schema Phase 8 domain family enum drifted",
    )
    assert_true(
        REQUIRED_GUARDRAILS <= set(defs["phase8_required_guardrail"].get("enum", [])),
        "schema Phase 8 guardrail enum drifted",
    )


def validate_phase8_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase8_payload(record)

    modules = payload.get("domain_modules", [])
    module_families = {module.get("domain_family") for module in modules if isinstance(module, dict)}
    if modules and not REQUIRED_DOMAIN_FAMILIES <= module_families:
        errors.append("schema.domain_owner_gate_missing")
    for module in modules:
        if not isinstance(module, dict):
            errors.append("schema.domain_owner_gate_missing")
            continue
        domain = module.get("domain_family")
        if domain not in REQUIRED_DOMAIN_FAMILIES or not module.get("module_name"):
            errors.append("schema.domain_owner_gate_missing")
        if not str(module.get("owning_master_phase", "")).startswith("master_phase:") or not module.get("owning_service_families"):
            errors.append("schema.domain_owner_gate_missing")
        if module.get("generated_contract_consumption_required") is not True or module.get("private_duplicate_types_allowed") is not False:
            errors.append("schema.generated_contract_consumption_missing")
        if module.get("strict_unknown_fields_required") is not True:
            errors.append("schema.strict_unknown_field_required")
        if module.get("raw_secret_values_allowed") is not False or module.get("untyped_refs_allowed") is not False:
            errors.append("schema.storage_secret_ref_invalid")
        if module.get("raw_private_payload_allowed") is not False:
            errors.append("schema.private_payload_leakage")
        if module.get("runtime_authority_stays_with_owner") is not True:
            errors.append("schema.domain_owner_gate_missing")

    guardrails = payload.get("guardrail_checks", [])
    guardrail_names = {guardrail.get("guardrail_name") for guardrail in guardrails if isinstance(guardrail, dict)}
    if guardrails and not REQUIRED_GUARDRAILS <= guardrail_names:
        errors.append("schema.strict_unknown_field_required")
    for guardrail in guardrails:
        if not isinstance(guardrail, dict):
            errors.append("schema.strict_unknown_field_required")
            continue
        name = guardrail.get("guardrail_name")
        terms = set(guardrail.get("forbidden_terms", []))
        if (
            name not in REQUIRED_GUARDRAILS
            or guardrail.get("domain_family") not in REQUIRED_DOMAIN_FAMILIES
            or not str(guardrail.get("reason_code", "")).startswith("schema.")
            or guardrail.get("enforced") is not True
            or not terms
        ):
            if name == "no_pricing_revenue_blockchain":
                errors.append("schema.accounting_assumption_forbidden")
            elif name in {"ref_only_secret_storage", "no_conventional_cloud_storage"}:
                errors.append("schema.storage_secret_ref_invalid")
            elif name == "privacy_classified_ai_user_content":
                errors.append("schema.private_payload_leakage")
            elif name == "typescript_second_projection":
                errors.append("schema.typescript_authority_drift")
            elif name == "generated_contract_consumption":
                errors.append("schema.generated_contract_consumption_missing")
            elif name == "owner_phase_gate":
                errors.append("schema.domain_owner_gate_missing")
            else:
                errors.append("schema.strict_unknown_field_required")
        if name == "no_pricing_revenue_blockchain" and not REQUIRED_ACCOUNTING_FORBIDDEN_TERMS <= terms:
            errors.append("schema.accounting_assumption_forbidden")
        if name == "privacy_classified_ai_user_content" and "raw private payload" not in terms:
            errors.append("schema.private_payload_leakage")

    typescript = payload.get("typescript_projection_requirement")
    if isinstance(typescript, dict):
        if (
            typescript.get("generated_from_canonical_schema") is not True
            or typescript.get("rust_fixture_gate_required") is not True
            or typescript.get("browser_safe_redaction_required") is not True
            or typescript.get("source_authority_allowed") is not False
            or typescript.get("protobuf_public_authority_allowed") is not False
            or typescript.get("blocked_until_rust_and_fixtures_stable") is not True
        ):
            errors.append("schema.typescript_authority_drift")

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
    entry = manifest.get("shared_schema_package_phase8")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase8")
    assert_true(entry.get("source_of_truth") == "json_schema", "Phase 8 source of truth drifted")
    assert_true(entry.get("rust_first_validation") is True, "Phase 8 must remain Rust-first")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "Phase 8 canonical schema drifted")
    rust = entry.get("rust_projection", {})
    assert_true(rust.get("path") == RUST_OUTPUT, "Phase 8 Rust output path drifted")
    assert_true(rust.get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Phase 8 validator entrypoint drifted")
    assert_true(rust.get("non_authoritative") is True, "Phase 8 Rust projection became authority")
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "Phase 8 contracts list incomplete")
    fixtures = entry.get("fixtures", {})
    assert_true(str(VALID_FIXTURES[0]) in fixtures.get("valid", []), "Phase 8 valid fixture missing from manifest")
    for fixture in INVALID_FIXTURES:
        assert_true(str(fixture) in fixtures.get("invalid", []), f"Phase 8 invalid fixture missing: {fixture}")
    assert_true(not validate_phase8_record(entry), "manifest Phase 8 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase8_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid but returned {sorted(set(errors))}")
    for fixture, expected in INVALID_FIXTURES.items():
        errors = validate_phase8_record(load_json(fixture))
        assert_true(expected in errors, f"{fixture} did not fail with {expected}; got {sorted(set(errors))}")


def check_docs() -> None:
    sub_plan = read_text(SUB_PLAN)
    assert_contains(sub_plan, "## Phase 8: Downstream Domain Schema Expansion", SUB_PLAN)
    assert_contains(sub_plan, "8.1 Add execution and scheduling schemas", SUB_PLAN)
    assert_contains(sub_plan, "8.5 Add AI, Docdex, mobile, native-app, and ADES-facing schemas", SUB_PLAN)

    contracts_readme = read_text(CONTRACTS_README)
    assert_contains(contracts_readme, "Shared Schema Package Phase 8", CONTRACTS_README)
    assert_contains(contracts_readme, VALIDATOR_ENTRYPOINT, CONTRACTS_README)

    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "Shared Schema Package Phase 8", SCHEMAS_README)
    assert_contains(schemas_readme, "owner-gated downstream domain schema expansion", SCHEMAS_README)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 8", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 8 Progress", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)


def check_rust_projection() -> None:
    rust = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase8DomainExpansionContract",
        "SharedSchemaPhase8DomainModule",
        "SharedSchemaPhase8GuardrailCheck",
        "SharedSchemaPhase8TypeScriptProjectionRequirement",
        "SharedSchemaPhase8RustProjection",
        VALIDATOR_ENTRYPOINT,
        "REQUIRED_SHARED_SCHEMA_PHASE8_DOMAIN_FAMILIES",
        "REQUIRED_SHARED_SCHEMA_PHASE8_GUARDRAILS",
    ]:
        assert_contains(rust, expected, RUST_PROJECTION)


def check_suite_wiring() -> None:
    suite = read_text(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_shared_schema_package_phase8.py")', SUITE_VALIDATOR)


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
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase8"])
    print("Shared Schema Package Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
