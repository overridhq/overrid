#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 3 contract artifacts."""

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
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_03_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase3.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_command_missing_tenant_actor.invalid.json"
    ): "command.tenant_actor_refs",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_identity_raw_secret.invalid.json"
    ): "identity.raw_secret",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_api_error_missing_reason_trace.invalid.json"
    ): "api_error.reason_trace_correction",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_event_private_payload.invalid.json"
    ): "event.private_payload_leakage",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_manifest_bad_secret_ref.invalid.json"
    ): "manifest.secret_ref",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase3_queue_key_material.invalid.json"
    ): "credential.private_key_material",
}

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
REQUIRED_MODULES = {
    "identity",
    "tenant",
    "command",
    "api_error",
    "event",
    "audit",
    "workload_manifest",
    "resource_manifest",
    "registry_metadata",
    "queue_and_lease",
    "credential_key_metadata",
}
REQUIRED_CONTRACTS = REQUIRED_MODULES | {"phase3_contract_module", "phase3_contract_rule"}
CATEGORY_KEYS = [
    "identity_tenant_modules",
    "command_api_error_modules",
    "event_audit_modules",
    "manifest_registry_modules",
    "queue_lease_credential_key_modules",
]


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
    assert_true(
        schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema",
        "shared schema package schema must use JSON Schema draft 2020-12",
    )
    assert_true(schema.get("additionalProperties") is False, "root schema must reject unknown fields")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})

    for property_name in [
        "schema_family",
        "schema_version",
        "layout",
        "identifier_primitives",
        "lifecycle_primitives",
        "privacy_classes",
        "reason_code_registry",
        "phase3_contract_modules",
    ]:
        assert_true(property_name in props, f"schema missing root property {property_name}")

    for def_name in [
        "phase3_contract_modules",
        "phase3_contract_module",
        "phase3_contract_rule",
        "privacy_class",
        "reason_code",
    ]:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    module_def = defs["phase3_contract_module"]
    assert_true(module_def.get("additionalProperties") is False, "phase3 module must reject unknown fields")
    assert_true(
        set(module_def.get("properties", {}).get("module_name", {}).get("enum", [])) == REQUIRED_MODULES,
        "phase3 module names drifted",
    )
    assert_true(
        module_def.get("properties", {}).get("source_of_truth", {}).get("const") == "json_schema",
        "phase3 module source of truth must be JSON Schema",
    )
    assert_true(
        module_def.get("properties", {}).get("runtime_authority", {}).get("const") == "owning_service",
        "phase3 runtime authority must stay with owning services",
    )


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase3")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase3")
    assert_true(entry.get("schema_family") == "shared-schema-package", "wrong schema family")
    assert_true(entry.get("schema_version") == SUPPORTED_SCHEMA, "wrong schema version")
    assert_true(entry.get("canonical_schema") == str(SCHEMA), "wrong canonical schema path")
    assert_true(entry.get("source_of_truth") == "json_schema", "JSON Schema must be source of truth")
    assert_true(entry.get("rust_first_validation") is True, "Rust-first validation flag missing")
    assert_true(
        entry.get("rust_projection", {}).get("non_authoritative") is True,
        "Rust projection must be non-authoritative",
    )
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "manifest contract list drifted")

    listed_valid = {Path(path) for path in entry.get("fixtures", {}).get("valid", [])}
    assert_true(listed_valid == set(VALID_FIXTURES), "manifest valid fixture list drifted")
    listed_invalid = {Path(path) for path in entry.get("fixtures", {}).get("invalid", [])}
    assert_true(listed_invalid == set(INVALID_FIXTURES), "manifest invalid fixture list drifted")

    ownership = entry.get("schema_ownership", {})
    for field in [
        "owning_service_family",
        "downstream_consumers",
        "release_status",
        "privacy_class",
        "compatibility_class",
        "review_authority",
        "consumer_notes",
    ]:
        assert_true(ownership.get(field) not in (None, "", []), f"manifest missing ownership {field}")


def phase3_modules(record: dict[str, Any]) -> list[dict[str, Any]]:
    modules = record.get("phase3_contract_modules", {})
    if not isinstance(modules, dict):
        return []
    flattened: list[dict[str, Any]] = []
    for key in CATEGORY_KEYS:
        for module in modules.get(key, []):
            if isinstance(module, dict):
                flattened.append(module)
    return flattened


def has_item(module: dict[str, Any], key: str, item: str) -> bool:
    value = module.get(key, [])
    return isinstance(value, list) and item in value


def validate_module(module: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    name = str(module.get("module_name", "unknown"))

    if module.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append(f"{name}.schema_version")
    if not module.get("owning_service_family") or not module.get("downstream_consumers"):
        errors.append(f"{name}.ownership")
    if (
        module.get("source_of_truth") != "json_schema"
        or module.get("rust_projection_non_authoritative") is not True
        or module.get("runtime_authority") != "owning_service"
    ):
        errors.append(f"{name}.authority")
    if module.get("strict_unknown_field_rejection") is not True:
        errors.append(f"{name}.unknown_sensitive_fields")
        if name in {"event", "audit"}:
            errors.append(f"{name}.private_payload_leakage")
    if module.get("raw_secret_values_allowed") is not False:
        errors.append("identity.raw_secret" if name in {"identity", "tenant"} else f"{name}.raw_secret")
    if module.get("private_key_material_allowed") is not False:
        errors.append("credential.private_key_material")
    if module.get("untyped_capability_blobs_allowed") is not False:
        errors.append("manifest.untyped_capability_blob")

    if module.get("tenant_actor_refs_required") is True and not (
        has_item(module, "required_refs", "tenant_ref") and has_item(module, "required_refs", "actor_ref")
    ):
        errors.append(f"{name}.tenant_actor_refs")
    if module.get("append_only_record") is True and not has_item(module, "required_fields", "sequence"):
        errors.append(f"{name}.append_only_sequence")
    if module.get("privacy_class_required") is True and not has_item(module, "required_fields", "privacy_class"):
        errors.append(f"{name}.privacy_class")
    if module.get("reason_code_required") is True and not has_item(module, "required_fields", "reason_code"):
        errors.append(f"{name}.reason_code")
    if module.get("correction_shape_required") is True and not has_item(
        module, "required_fields", "correction_fields"
    ):
        errors.append(f"{name}.correction_shape")
    if module.get("typed_secret_refs_required") is True and not has_item(module, "required_refs", "secret_ref"):
        errors.append("manifest.secret_ref" if name.endswith("manifest") else f"{name}.secret_ref")

    if name == "command":
        for field in ["trace_id", "idempotency_key", "payload_hash", "signature_metadata"]:
            if not has_item(module, "required_fields", field):
                errors.append("command.envelope")
    if name == "api_error":
        if not all(
            has_item(module, "required_fields", field)
            for field in ["reason_code", "trace_id", "retryability", "correction_fields"]
        ):
            errors.append("api_error.reason_trace_correction")
    if name == "credential_key_metadata":
        if not all(
            has_item(module, "required_fields", field)
            for field in ["key_rotation", "revocation", "reason_code", "trace_id"]
        ):
            errors.append("credential.incomplete_metadata")
        if not has_item(module, "required_refs", "signer_ref"):
            errors.append("credential.signer_ref")

    for rule in module.get("validation_rules", []):
        if not isinstance(rule, dict):
            errors.append(f"{name}.rule")
            continue
        if not rule.get("rule_id") or "." not in str(rule.get("reason_code", "")):
            errors.append(f"{name}.rule")
        if not rule.get("correction_fields"):
            errors.append(f"{name}.rule_correction")
        if rule.get("enforcement") not in {
            "require",
            "reject",
            "preserve_append_only",
            "require_typed_ref",
        }:
            errors.append(f"{name}.rule_enforcement")

    return errors


def validate_phase3_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") != "shared-schema-package":
        errors.append("schema.family")
    if record.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.version")

    modules = phase3_modules(record)
    if not modules:
        errors.append("phase3.modules_missing")
    module_names = {str(module.get("module_name")) for module in modules}
    missing_modules = REQUIRED_MODULES - module_names
    if missing_modules:
        errors.append("module.missing")

    for module in modules:
        errors.extend(validate_module(module))

    return sorted(set(errors))


def check_fixtures() -> None:
    for fixture_path in VALID_FIXTURES:
        errors = validate_phase3_record(load_json(fixture_path))
        assert_true(not errors, f"{fixture_path} should be valid but had errors: {errors}")

    for fixture_path, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase3_record(load_json(fixture_path))
        assert_true(
            expected_error in errors,
            f"{fixture_path} should fail with {expected_error}; got {errors}",
        )


def check_text_surfaces() -> None:
    sub_plan = read_text(SUB_PLAN)
    for item in ["**3.1 ", "**3.2 ", "**3.3 ", "**3.4 ", "**3.5 "]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "`identity` and `tenant` schema modules",
        "`command` and `api_error` modules",
        "`event` and `audit` modules",
        "`workload_manifest`, `resource_manifest`, and registry metadata modules",
        "`queue_and_lease` and `credential_key_metadata` schema modules",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for path in [SCHEMAS_README, CONTRACTS_README]:
        text = read_text(path)
        assert_contains(text, "Phase 3", path)
        assert_contains(text, "identity", path)
        assert_contains(text, "credential", path)

    rust_projection = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase3Contract",
        "SharedSchemaPhase3ContractModule",
        "SharedSchemaPhase3ModuleFamily",
        "REQUIRED_SHARED_SCHEMA_PHASE3_MODULES",
        "shared_schema_phase3_contract_modules",
        "MissingTenantActorRefs",
        "PrivateKeyMaterialAllowed",
    ]:
        assert_contains(rust_projection, expected, RUST_PROJECTION)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 3", PHASE_PLAN)
    assert_contains(phase_plan, "Canonical JSON Schema remains the source of truth", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 3 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)

    suite_validator = read_text(SUITE_VALIDATOR)
    assert_contains(
        suite_validator,
        'Path("scripts/validate_shared_schema_package_phase3.py")',
        SUITE_VALIDATOR,
    )


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError(f"command failed with exit {result.returncode}: {' '.join(command)}")


def main() -> int:
    check_required_files()
    check_schema_surface(load_json(SCHEMA))
    check_manifest(load_json(MANIFEST))
    check_fixtures()
    check_text_surfaces()
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase3"])
    print("Shared Schema Package Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
