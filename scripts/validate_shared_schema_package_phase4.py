#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 4 generation contract artifacts."""

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
GENERATED_DOCS = Path("packages/schemas/overrid_contracts/generated/docs/shared_schema_package_phase4_reference.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_007_shared_schema_package.md")
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_04_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
BUILD_PLAN_PATH = str(SUB_PLAN)
TECH_STACK_PATH = str(TECH_STACK)
RUST_OUTPUT = str(RUST_PROJECTION)
GENERATED_DOCS_PATH = str(GENERATED_DOCS)
TYPESCRIPT_OUTPUT_ROOT = "packages/schemas/admin_ui/generated"
PROTOBUF_OUTPUT_ROOT = "packages/schemas/overrid_contracts/protobuf/internal"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase4.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase4_generated_hand_edit.invalid.json"
    ): "schema.generated_output_hand_edited",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase4_rust_output_missing_validator.invalid.json"
    ): "rust.validator_entrypoint",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase4_docs_missing_trace.invalid.json"
    ): "schema.docs_trace_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase4_typescript_source_truth.invalid.json"
    ): "schema.typescript_source_authority",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase4_protobuf_public_only.invalid.json"
    ): "schema.protobuf_public_only",
}

REQUIRED_CONTRACTS = {
    "phase4_generation_toolchain",
    "phase4_toolchain_decision",
    "phase4_generation_command",
    "phase4_projection_output",
    "phase4_docs_projection",
    "phase4_typescript_projection",
    "phase4_protobuf_projection",
    "phase4_reproducibility",
}
REQUIRED_SOURCE_INPUTS = {CANONICAL_SCHEMA, MANIFEST_PATH, BUILD_PLAN_PATH, TECH_STACK_PATH}
REQUIRED_DOC_SECTIONS = {
    "Object Families",
    "Required Fields",
    "Privacy And Redaction",
    "Reason Codes",
    "Compatibility And Authority",
}
REQUIRED_TYPESCRIPT_GATES = {
    "rust_projection_validated",
    "golden_fixtures_stable",
    "docs_trace_validated",
    "compatibility_checks_stable",
}
REQUIRED_PROTOBUF_BLOCKERS = {
    "command_payloads",
    "signed_payloads",
    "manifests",
    "policy_declarations",
    "fixtures",
    "docs_examples",
    "api_errors",
    "reason_codes",
    "audit_records",
}
REQUIRED_FAILURE_REASONS = {
    "schema.generation_not_reproducible",
    "schema.generated_output_hand_edited",
    "schema.docs_trace_missing",
    "schema.typescript_source_authority",
    "schema.protobuf_public_only",
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
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase4_generation_toolchain" in props, "schema missing Phase 4 root property")
    for def_name in [
        "phase4_generation_toolchain",
        "phase4_toolchain_decision",
        "phase4_generation_command",
        "phase4_projection_output",
        "phase4_docs_projection",
        "phase4_typescript_projection",
        "phase4_protobuf_projection",
        "phase4_reproducibility",
    ]:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    layout_roots = (
        defs.get("source_layout", {})
        .get("properties", {})
        .get("generated_output_roots", {})
        .get("items", {})
        .get("enum", [])
    )
    assert_true("packages/schemas/overrid_contracts/generated/docs" in layout_roots, "generated docs root missing")

    decision = defs["phase4_toolchain_decision"].get("properties", {})
    assert_true(decision.get("source_of_truth", {}).get("const") == "json_schema", "JSON Schema authority drifted")
    assert_true(decision.get("rust_first", {}).get("const") is True, "Rust-first flag missing")
    assert_true(
        decision.get("hand_edited_generated_files_allowed", {}).get("const") is False,
        "hand-edited generated files are allowed",
    )
    assert_true(
        decision.get("protobuf_public_authority_allowed", {}).get("const") is False,
        "Protobuf public authority is allowed",
    )


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase4")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase4")
    assert_true(entry.get("schema_family") == "shared-schema-package", "wrong schema family")
    assert_true(entry.get("schema_version") == SUPPORTED_SCHEMA, "wrong schema version")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "wrong canonical schema path")
    assert_true(entry.get("source_of_truth") == "json_schema", "JSON Schema must be source of truth")
    assert_true(entry.get("rust_first_validation") is True, "Rust-first validation flag missing")
    assert_true(
        entry.get("rust_projection", {}).get("validator_entrypoint")
        == "SharedSchemaPhase4GenerationContract::canonical().validate()",
        "Rust validator entrypoint drifted",
    )
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "manifest contract list drifted")
    assert_true(set(entry.get("source_hash_inputs", [])) >= REQUIRED_SOURCE_INPUTS, "source hash inputs drifted")

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

    assert_true(not validate_phase4_record({"schema_family": "shared-schema-package", "schema_version": SUPPORTED_SCHEMA, "phase4_generation_toolchain": entry}), "manifest Phase 4 metadata should be valid")


def validate_phase4_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") != "shared-schema-package":
        errors.append("schema.family")
    if record.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.version")

    phase4 = record.get("phase4_generation_toolchain", {})
    if not isinstance(phase4, dict):
        return ["phase4.missing"]

    decision = phase4.get("toolchain_decision") or phase4.get("generation_toolchain") or {}
    if (
        decision.get("toolchain_name") != "rust-json-schema-projection-v0"
        or decision.get("canonical_source") != CANONICAL_SCHEMA
        or decision.get("source_of_truth") != "json_schema"
        or decision.get("rust_first") is not True
        or decision.get("typescript_second") is not True
        or decision.get("generated_outputs_non_authoritative") is not True
    ):
        errors.append("schema.typescript_source_authority")
    if decision.get("hand_edited_generated_files_allowed") is not False:
        errors.append("schema.generated_output_hand_edited")
    if decision.get("protobuf_public_authority_allowed") is not False:
        errors.append("schema.protobuf_public_only")

    commands = phase4.get("generation_commands", [])
    if not isinstance(commands, list) or len(commands) < 2:
        errors.append("schema.generation_not_reproducible")
    else:
        command_text = "\n".join(str(command.get("command", "")) for command in commands if isinstance(command, dict))
        if "cargo test -p overrid-contracts shared_schema_phase4" not in command_text:
            errors.append("schema.generation_not_reproducible")
        if "scripts/validate_shared_schema_package_phase4.py" not in command_text:
            errors.append("schema.docs_trace_missing")
        for command in commands:
            if not isinstance(command, dict):
                errors.append("schema.generation_not_reproducible")
                continue
            if command.get("deterministic") is not True:
                errors.append("schema.generation_not_reproducible")
            if not command.get("source_inputs") or not command.get("output_paths"):
                errors.append("schema.generation_not_reproducible")
            if "." not in str(command.get("dry_run_reason_code", "")):
                errors.append("schema.generation_not_reproducible")

    if set(phase4.get("source_hash_inputs", [])) < REQUIRED_SOURCE_INPUTS:
        errors.append("schema.generation_not_reproducible")

    rust_outputs = phase4.get("rust_outputs", [])
    rust_output = next((item for item in rust_outputs if isinstance(item, dict) and item.get("target") == "rust"), None)
    if not rust_output:
        errors.append("rust.validator_entrypoint")
    else:
        if rust_output.get("path") != RUST_OUTPUT or rust_output.get("source_schema") != CANONICAL_SCHEMA:
            errors.append("rust.validator_entrypoint")
        if rust_output.get("non_authoritative") is not True:
            errors.append("schema.generated_output_hand_edited")
        if "SharedSchemaPhase4GenerationContract" not in str(rust_output.get("validator_entrypoint", "")):
            errors.append("rust.validator_entrypoint")
        if rust_output.get("contains_redaction_metadata") is not True or rust_output.get("contains_reason_code_metadata") is not True:
            errors.append("rust.validator_entrypoint")

    docs_projection = phase4.get("docs_projection", {})
    if (
        docs_projection.get("output_path") != GENERATED_DOCS_PATH
        or docs_projection.get("source_to_doc_trace") is not True
        or set(docs_projection.get("required_sections", [])) < REQUIRED_DOC_SECTIONS
        or docs_projection.get("requires_descriptions") is not True
        or docs_projection.get("requires_examples") is not True
        or docs_projection.get("requires_privacy_class") is not True
        or docs_projection.get("requires_reason_code_links") is not True
    ):
        errors.append("schema.docs_trace_missing")

    ts_projection = phase4.get("typescript_web_projection", {})
    if (
        ts_projection.get("status") != "blocked_until_rust_and_fixtures_stable"
        or ts_projection.get("source_schema") != CANONICAL_SCHEMA
        or ts_projection.get("output_root") != TYPESCRIPT_OUTPUT_ROOT
        or ts_projection.get("browser_safe_redaction") is not True
        or ts_projection.get("source_of_truth_allowed") is not False
        or set(ts_projection.get("blocked_until", [])) < REQUIRED_TYPESCRIPT_GATES
    ):
        errors.append("schema.typescript_source_authority")

    protobuf_projection = phase4.get("protobuf_projection", {})
    if (
        protobuf_projection.get("status") != "internal_only_when_owning_sds_approves"
        or protobuf_projection.get("scope") != "compact_internal_service_rpc_event_contracts"
        or protobuf_projection.get("output_root") != PROTOBUF_OUTPUT_ROOT
        or protobuf_projection.get("json_schema_source_required") is not True
        or protobuf_projection.get("public_object_definition_allowed") is not False
        or set(protobuf_projection.get("public_payload_families_blocked", [])) < REQUIRED_PROTOBUF_BLOCKERS
    ):
        errors.append("schema.protobuf_public_only")

    reproducibility = phase4.get("reproducibility", {})
    if (
        reproducibility.get("deterministic") is not True
        or reproducibility.get("source_to_output_trace_required") is not True
        or reproducibility.get("generated_diff_required") is not True
        or reproducibility.get("hand_edit_policy") != "prohibited"
    ):
        errors.append("schema.generated_output_hand_edited")
    if set(reproducibility.get("failure_reason_codes", [])) < REQUIRED_FAILURE_REASONS:
        errors.append("schema.generation_not_reproducible")

    return sorted(set(errors))


def check_fixtures() -> None:
    for fixture_path in VALID_FIXTURES:
        errors = validate_phase4_record(load_json(fixture_path))
        assert_true(not errors, f"{fixture_path} should be valid but had errors: {errors}")

    for fixture_path, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase4_record(load_json(fixture_path))
        assert_true(
            expected_error in errors,
            f"{fixture_path} should fail with {expected_error}; got {errors}",
        )


def check_text_surfaces() -> None:
    sub_plan = read_text(SUB_PLAN)
    for item in ["**4.1 ", "**4.2 ", "**4.3 ", "**4.4 ", "**4.5 "]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "Rust-first generation pipeline",
        "Rust generated package outputs",
        "Generated reference documentation",
        "TypeScript/web generation plan",
        "Binary projection rules",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for path in [SCHEMAS_README, CONTRACTS_README]:
        text = read_text(path)
        assert_contains(text, "Phase 4", path)
        assert_contains(text, "Protobuf", path)
        assert_contains(text, "TypeScript/web", path)

    generated_docs = read_text(GENERATED_DOCS)
    for expected in [
        "Source of truth: `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json`",
        "source-to-doc trace metadata",
        "Object Families",
        "Reason Codes",
        "Compatibility And Authority",
        "non_authoritative_projection",
    ]:
        assert_contains(generated_docs, expected, GENERATED_DOCS)

    rust_projection = read_text(RUST_PROJECTION)
    for expected in [
        "SharedSchemaPhase4GenerationContract",
        "SharedSchemaPhase4ProjectionOutput",
        "SharedSchemaPhase4DocsProjection",
        "SharedSchemaPhase4TypeScriptProjection",
        "SharedSchemaPhase4ProtobufProjection",
        "REQUIRED_SHARED_SCHEMA_PHASE4_REASON_CODES",
        "ProtobufPublicAuthority",
    ]:
        assert_contains(rust_projection, expected, RUST_PROJECTION)

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 4", PHASE_PLAN)
    assert_contains(phase_plan, "Canonical JSON Schema remains the contract authority", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 4 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)

    suite_validator = read_text(SUITE_VALIDATOR)
    assert_contains(
        suite_validator,
        'Path("scripts/validate_shared_schema_package_phase4.py")',
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
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase4"])
    print("Shared Schema Package Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
