#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 2 contract artifacts."""

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
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_02_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase2.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_generated_in_source.invalid.json"
    ): "layout.generated_in_source",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_missing_privacy_class.invalid.json"
    ): "privacy.missing_class",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_reason_without_correction.invalid.json"
    ): "reason.correction_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_untyped_public_ref.invalid.json"
    ): "ref.untyped",
}

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
REQUIRED_CONTRACTS = {
    "source_layout",
    "typed_ref_primitive",
    "lifecycle_primitives",
    "privacy_class_rule",
    "reason_code_entry",
    "correction_field",
}
REQUIRED_OBJECT_FAMILIES = {
    "person",
    "organization",
    "tenant",
    "node",
    "app",
    "native_service",
    "service_account",
    "system_service",
    "central_ai_actor",
    "overasset",
    "namespace",
    "route",
    "package",
    "workload",
    "queue",
    "lease",
    "receipt",
    "evidence",
    "secret_reference",
}
REQUIRED_PRIVACY_CLASSES = {
    "public",
    "tenant_private",
    "regulated",
    "encrypted_private",
    "user_content",
    "system_service_only",
    "redacted_diagnostic",
}
REQUIRED_REASON_CODES = {
    "schema.unsupported_version",
    "validation.typed_ref_required",
    "policy.privacy_class_missing",
    "dependency.generated_source_misplaced",
}


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
    assert_true(
        schema.get("$id", "").endswith("/shared_schema_package.schema.json"),
        "shared schema package schema id drifted",
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
    ]:
        assert_true(property_name in props, f"schema missing root property {property_name}")

    for def_name in [
        "source_layout",
        "typed_ref_primitive",
        "lifecycle_primitives",
        "privacy_class_rule",
        "reason_code_entry",
        "correction_field",
        "privacy_class",
        "reason_code",
    ]:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    assert_true(defs["schema_version"].get("const") == SUPPORTED_SCHEMA, "wrong schema version")
    assert_true(
        set(defs["privacy_class"].get("enum", [])) == REQUIRED_PRIVACY_CLASSES,
        "privacy classes drifted",
    )
    assert_true(
        props["identifier_primitives"].get("minItems") == len(REQUIRED_OBJECT_FAMILIES),
        "identifier primitive minimum must cover all common object families",
    )
    assert_true(
        props["reason_code_registry"].get("minItems") == len(REQUIRED_REASON_CODES),
        "reason-code registry minimum drifted",
    )


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase2")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase2")
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


def validate_phase2_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") != "shared-schema-package":
        errors.append("schema.family")
    if record.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.version")

    layout = record.get("layout", {})
    if layout.get("generated_in_source_directories") is not False:
        errors.append("layout.generated_in_source")
    source_roots = set(layout.get("approved_source_roots", []))
    for required_root in [
        "packages/schemas/README.md",
        "packages/schemas/overrid_contracts/v0",
        "packages/schemas/overrid_contracts/codegen_manifest.json",
        "packages/schemas/overrid_contracts/README.md",
    ]:
        if required_root not in source_roots:
            errors.append("layout.source_root_missing")
    generated_roots = layout.get("generated_output_roots", [])
    for generated_root in generated_roots:
        if generated_root.startswith("packages/schemas/overrid_contracts/v0/") or generated_root.endswith(
            ".schema.json"
        ):
            errors.append("layout.generated_in_source")
    if "packages/schemas/overrid_contracts/src/lib.rs" not in generated_roots:
        errors.append("layout.rust_projection_missing")

    primitives = record.get("identifier_primitives", [])
    families = {item.get("object_family") for item in primitives if isinstance(item, dict)}
    missing_families = REQUIRED_OBJECT_FAMILIES - families
    if missing_families:
        errors.append("ref.object_family_missing")
    for primitive in primitives:
        if not isinstance(primitive, dict):
            errors.append("ref.invalid")
            continue
        if primitive.get("unstructured_string_allowed") is not False or not primitive.get("ref_prefix"):
            errors.append("ref.untyped")
        if primitive.get("public_object_required") is True and primitive.get("versioned_when_external") is not True:
            errors.append("ref.unversioned_public")
        if primitive.get("object_family") == "secret_reference":
            if primitive.get("privacy_class") != "encrypted_private" or primitive.get("public_object_required") is True:
                errors.append("privacy.secret_ref_public")

    lifecycle = record.get("lifecycle_primitives", {})
    if lifecycle.get("trace_id") != "trace_":
        errors.append("lifecycle.trace")
    if lifecycle.get("idempotency_key") != "idem_":
        errors.append("lifecycle.idempotency")
    if lifecycle.get("request_hash") != "hash_" or lifecycle.get("payload_hash") != "hash_":
        errors.append("lifecycle.hash")
    if lifecycle.get("compatibility_window", {}).get("previous_major_supported") is not True:
        errors.append("lifecycle.compatibility_window")
    if not str(lifecycle.get("deprecation_metadata", {}).get("migration_plan_ref", "")).startswith("migration:"):
        errors.append("lifecycle.migration_plan")

    privacy_classes = record.get("privacy_classes", [])
    privacy_names = {item.get("privacy_class") for item in privacy_classes if isinstance(item, dict)}
    if REQUIRED_PRIVACY_CLASSES - privacy_names:
        errors.append("privacy.missing_class")
    for privacy_class in privacy_classes:
        if not isinstance(privacy_class, dict):
            errors.append("privacy.invalid")
            continue
        name = privacy_class.get("privacy_class")
        if name not in {"public", "redacted_diagnostic"} and privacy_class.get("requires_redaction") is not True:
            errors.append("privacy.redaction_required")

    reason_codes = record.get("reason_code_registry", [])
    reason_names = {item.get("reason_code") for item in reason_codes if isinstance(item, dict)}
    if REQUIRED_REASON_CODES - reason_names:
        errors.append("reason.missing")
    if not any(item.get("unsupported_version") is True for item in reason_codes if isinstance(item, dict)):
        errors.append("reason.unsupported_version_missing")
    for reason in reason_codes:
        if not isinstance(reason, dict):
            errors.append("reason.invalid")
            continue
        if "." not in str(reason.get("reason_code", "")):
            errors.append("reason.format")
        if not reason.get("correction_fields"):
            errors.append("reason.correction_missing")
        if reason.get("documented") is not True or reason.get("rust_first_enum_source") is not True:
            errors.append("reason.documentation")
        if not str(reason.get("policy_ref", "")).startswith("policy:"):
            errors.append("reason.policy_ref")
        if not str(reason.get("audit_ref", "")).startswith("audit:"):
            errors.append("reason.audit_ref")

    return sorted(set(errors))


def check_fixtures() -> None:
    for fixture_path in VALID_FIXTURES:
        errors = validate_phase2_record(load_json(fixture_path))
        assert_true(not errors, f"{fixture_path} should be valid but had errors: {errors}")

    for fixture_path, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase2_record(load_json(fixture_path))
        assert_true(
            expected_error in errors,
            f"{fixture_path} should fail with {expected_error}; got {errors}",
        )


def check_text_surfaces() -> None:
    sub_plan = read_text(SUB_PLAN)
    for item in ["**2.1 ", "**2.2 ", "**2.3 ", "**2.4 ", "**2.5 "]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "`packages/schemas` source layout",
        "Common id/ref schema module",
        "Common lifecycle and envelope primitive module",
        "Privacy classification schema",
        "Reason-code registry schema",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    contracts_readme = read_text(CONTRACTS_README)
    for expected in [
        "Shared Schema Package Phase 2 contracts define",
        "`source_layout`",
        "`typed_ref_primitive`",
        "`lifecycle_primitives`",
        "`reason_code_entry`",
        "compatibility report roots",
        "internal binary projection roots",
    ]:
        assert_contains(contracts_readme, expected, CONTRACTS_README)

    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "shared_schema_package.schema.json", SCHEMAS_README)
    assert_contains(schemas_readme, "typed refs", SCHEMAS_README)
    assert_contains(schemas_readme, "compatibility-report roots", SCHEMAS_README)
    assert_contains(schemas_readme, "internal binary projection roots", SCHEMAS_README)

    rust_projection = read_text(RUST_PROJECTION)
    for expected in [
        "SHARED_SCHEMA_PACKAGE_SCHEMA_FAMILY",
        "SUPPORTED_SHARED_SCHEMA_PACKAGE_SCHEMA_VERSION",
        "shared_schema_package_contract_set",
        "SharedSchemaPhase2Contract",
        "SharedSchemaTypedRefPrimitive",
        "SharedSchemaPrivacyClass",
        "SharedSchemaReasonCodeEntry",
        "required_shared_schema_typed_refs",
        "compatibility_report_roots",
        "internal_binary_projection_roots",
        "generated_in_source_directories",
    ]:
        assert_contains(rust_projection, expected, RUST_PROJECTION)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 2", PHASE_PLAN)
    assert_contains(phase_plan, "Canonical JSON Schema remains the source of truth", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 2 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)

    suite_validator = read_text(SUITE_VALIDATOR)
    assert_contains(
        suite_validator,
        'Path("scripts/validate_shared_schema_package_phase2.py")',
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
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema"])
    print("Shared Schema Package Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
