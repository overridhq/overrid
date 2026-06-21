#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 5 strict validation artifacts."""

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
RUST_PROJECTION = Path("packages/schemas/overrid_contracts/src/lib.rs")
GENERATED_DOCS = Path("packages/schemas/overrid_contracts/generated/docs/shared_schema_package_phase4_reference.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_007_shared_schema_package.md")
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_05_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase5ValidationContract::canonical().validate()"
SENTINEL_SECRET = "OVERRID_SENTINEL_SECRET_NEVER_EMIT"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase5.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_unknown_fields_allowed.invalid.json"
    ): "schema.unknown_sensitive_field",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_extension_map_untyped.invalid.json"
    ): "schema.extension_map_not_permitted",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_parse_helper_missing_reason.invalid.json"
    ): "schema.parse_helper_unstable",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_envelope_missing_trace.invalid.json"
    ): "schema.envelope_incomplete",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_free_form_reason_code.invalid.json"
    ): "schema.reason_code_undocumented",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase5_diagnostic_secret_leak.invalid.json"
    ): "schema.diagnostic_secret_leak",
}

REQUIRED_SENSITIVE_FAMILIES = {
    "command",
    "identity",
    "tenant",
    "credential",
    "signature",
    "api_error",
    "audit",
    "policy",
    "usage",
    "oru",
    "seal_ledger",
    "overasset",
    "dispute",
    "namespace_ownership",
}
REQUIRED_PARSE_ERRORS = {
    "schema.parse_malformed_payload",
    "schema.unsupported_version",
    "schema.missing_required_field",
    "schema.wrong_privacy_class",
    "schema.unknown_sensitive_field",
}
REQUIRED_ENVELOPES = {
    "command",
    "event",
    "audit",
    "usage",
    "ledger",
    "public_response",
}
REQUIRED_COMMAND_FIELDS = {
    "tenant_id",
    "actor_id",
    "trace_id",
    "idempotency_key",
    "command_type",
    "timestamp",
    "schema_version",
    "signature_metadata",
}
REQUIRED_REASON_DOMAINS = {
    "validation",
    "identity",
    "tenancy",
    "credentials",
    "policy",
    "queue",
    "execution",
    "accounting",
    "storage",
    "namespace",
    "ai",
    "compatibility",
}
REQUIRED_VALIDATION_REASONS = {
    "schema.unknown_sensitive_field",
    "schema.extension_map_not_permitted",
    "schema.parse_malformed_payload",
    "schema.unsupported_version",
    "schema.missing_required_field",
    "schema.wrong_privacy_class",
    "schema.envelope_incomplete",
    "schema.reason_code_undocumented",
    "schema.diagnostic_secret_leak",
}
REQUIRED_CONTRACTS = {
    "phase5_validation_contract",
    "phase5_strict_validation_defaults",
    "phase5_extension_map_rule",
    "phase5_parse_helper",
    "phase5_envelope_assertion",
    "phase5_reason_code_registry",
    "phase5_redaction_diagnostic",
    "phase5_rust_projection",
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


def phase5_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase5_validation_contract", record)
    return payload if isinstance(payload, dict) else {}


def check_required_files() -> None:
    for path in [
        SCHEMA,
        MANIFEST,
        CONTRACTS_README,
        RUST_PROJECTION,
        GENERATED_DOCS,
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
    assert_true("Phase 2 Through Phase 5" in schema.get("title", ""), "schema title missing Phase 5")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase5_validation_contract" in props, "schema missing Phase 5 root property")
    for def_name in REQUIRED_CONTRACTS | {
        "phase5_sensitive_family",
        "phase5_reason_domain",
        "phase5_envelope_family",
    }:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    sensitive_enum = set(defs["phase5_sensitive_family"].get("enum", []))
    assert_true(REQUIRED_SENSITIVE_FAMILIES <= sensitive_enum, "schema sensitive family enum drifted")
    domain_enum = set(defs["phase5_reason_domain"].get("enum", []))
    assert_true(REQUIRED_REASON_DOMAINS <= domain_enum, "schema reason domain enum drifted")
    envelope_enum = set(defs["phase5_envelope_family"].get("enum", []))
    assert_true(REQUIRED_ENVELOPES <= envelope_enum, "schema envelope family enum drifted")


def validate_phase5_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase5_payload(record)
    strict = payload.get("strict_validation_defaults") or {}
    if strict.get("unknown_fields_rejected_for_sensitive_families") is not True:
        errors.append("schema.unknown_sensitive_field")
    if strict.get("extension_maps_default_denied") is not True:
        errors.append("schema.extension_map_not_permitted")
    if not REQUIRED_SENSITIVE_FAMILIES <= set(strict.get("sensitive_families", [])):
        errors.append("schema.unknown_sensitive_field")
    for rule in strict.get("allowed_extension_maps", []):
        if (
            rule.get("surface") != "low_risk_metadata"
            or rule.get("namespace_prefix_required") is not True
            or rule.get("typed_values_required") is not True
            or rule.get("privacy_class_required") is not True
            or rule.get("compatibility_class_required") is not True
        ):
            errors.append("schema.extension_map_not_permitted")

    helpers = payload.get("parse_helpers", [])
    if not helpers:
        errors.append("schema.parse_helper_unstable")
    for helper in helpers:
        if (
            helper.get("schema_name") != "shared-schema-package"
            or helper.get("combines_schema_validation") is not True
            or helper.get("constructs_typed_object") is not True
            or helper.get("normalizes_errors") is not True
            or helper.get("checks_privacy_class") is not True
            or helper.get("references_reason_codes") is not True
            or not REQUIRED_PARSE_ERRORS <= set(helper.get("stable_error_reasons", []))
        ):
            errors.append("schema.parse_helper_unstable")

    envelopes = payload.get("envelope_assertions", [])
    by_envelope = {
        envelope.get("envelope_family"): envelope
        for envelope in envelopes
        if isinstance(envelope, dict)
    }
    if not REQUIRED_ENVELOPES <= set(by_envelope):
        errors.append("schema.envelope_incomplete")
    command = by_envelope.get("command", {})
    if (
        not REQUIRED_COMMAND_FIELDS <= set(command.get("required_fields", []))
        or not {"tenant_ref", "actor_ref", "signature_ref"} <= set(command.get("required_refs", []))
        or command.get("reason_code") != "schema.envelope_incomplete"
    ):
        errors.append("schema.envelope_incomplete")

    registries = payload.get("reason_code_registries", [])
    by_domain = {
        registry.get("domain"): registry
        for registry in registries
        if isinstance(registry, dict)
    }
    if not REQUIRED_REASON_DOMAINS <= set(by_domain):
        errors.append("schema.reason_code_undocumented")
    for registry in registries:
        if (
            registry.get("generated_enum_binding") != "SharedSchemaPhase5ReasonCode"
            or registry.get("free_form_replacements_allowed") is not False
            or not registry.get("reason_codes")
        ):
            errors.append("schema.reason_code_undocumented")
        for reason in registry.get("reason_codes", []):
            if "." not in str(reason):
                errors.append("schema.reason_code_undocumented")
    validation = by_domain.get("validation", {})
    if not REQUIRED_VALIDATION_REASONS <= set(validation.get("reason_codes", [])):
        errors.append("schema.reason_code_undocumented")

    diagnostics = payload.get("redaction_diagnostics", [])
    if not diagnostics:
        errors.append("schema.diagnostic_secret_leak")
    for diagnostic in diagnostics:
        if (
            diagnostic.get("sentinel_secret") != SENTINEL_SECRET
            or diagnostic.get("private_payload_leak_allowed") is not False
            or not diagnostic.get("redaction_hints")
            or not diagnostic.get("diagnostic_outputs")
        ):
            errors.append("schema.diagnostic_secret_leak")

    if not REQUIRED_SOURCE_INPUTS <= set(payload.get("source_hash_inputs", [])):
        errors.append("schema.source_trace_missing")
    rust = payload.get("rust_projection") or record.get("rust_projection") or {}
    if (
        rust.get("path") != RUST_OUTPUT
        or rust.get("validator_entrypoint") != VALIDATOR_ENTRYPOINT
        or rust.get("non_authoritative") is not True
    ):
        errors.append("schema.rust_projection_authority")
    return sorted(set(errors))


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase5")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase5")
    assert_true(entry.get("schema_family") == "shared-schema-package", "wrong schema family")
    assert_true(entry.get("schema_version") == SUPPORTED_SCHEMA, "wrong schema version")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "wrong canonical schema path")
    assert_true(entry.get("source_of_truth") == "json_schema", "JSON Schema must be source of truth")
    assert_true(entry.get("rust_first_validation") is True, "Rust-first validation flag missing")
    assert_true(entry.get("rust_projection", {}).get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Rust validator entrypoint drifted")
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

    assert_true(not validate_phase5_record(entry), "manifest Phase 5 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase5_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid, got {errors}")

    for fixture, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase5_record(load_json(fixture))
        assert_true(expected_error in errors, f"{fixture} should fail with {expected_error}, got {errors}")


def check_text_surfaces() -> None:
    sub_plan = read_text(SUB_PLAN)
    for expected in [
        "Phase 5: Strict Validators",
        "strict validation defaults",
        "parse helpers",
        "common envelope assertions",
        "reason-code registries",
        "redaction and privacy-aware diagnostics",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    contracts_readme = read_text(CONTRACTS_README)
    for expected in [
        "Shared Schema Package Phase 5",
        "SharedSchemaPhase5ValidationContract",
        "reject unknown fields",
        "list_reason_codes(domain)",
        "sentinel private material",
    ]:
        assert_contains(contracts_readme, expected, CONTRACTS_README)

    rust_projection = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase5ValidationContract",
        "SharedSchemaPhase5StrictValidationDefaults",
        "SharedSchemaPhase5ParseHelper",
        "SharedSchemaPhase5EnvelopeAssertion",
        "SharedSchemaPhase5ReasonCodeRegistry",
        "SharedSchemaPhase5RedactionDiagnostic",
        "REQUIRED_SHARED_SCHEMA_PHASE5_REASON_DOMAINS",
        "list_reason_codes",
    ]:
        assert_contains(rust_projection, expected, RUST_PROJECTION)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 5", PHASE_PLAN)
    assert_contains(phase_plan, "Canonical JSON Schema remains the contract authority", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 5 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)

    suite_validator = read_text(SUITE_VALIDATOR)
    assert_contains(
        suite_validator,
        'Path("scripts/validate_shared_schema_package_phase5.py")',
        SUITE_VALIDATOR,
    )

    generated_docs = read_text(GENERATED_DOCS)
    assert_true(SENTINEL_SECRET not in generated_docs, "sentinel secret leaked into generated docs")


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
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase5"])
    print("Shared Schema Package Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
