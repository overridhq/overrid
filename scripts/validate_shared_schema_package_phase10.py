#!/usr/bin/env python3
"""Validate Shared Schema Package Phase 10 closure and handoff contracts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any
from urllib.parse import unquote


REPO_ROOT = Path(__file__).resolve().parents[1]

SCHEMA = Path("packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json")
MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
CONTRACTS_README = Path("packages/schemas/overrid_contracts/README.md")
SCHEMAS_README = Path("packages/schemas/README.md")
RUST_PROJECTION = Path("packages/schemas/overrid_contracts/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_007_shared_schema_package.md")
SDS = Path("docs/sds/foundation/shared_schema_package.md")
SERVICE_CATALOG = Path("docs/service_catalog/foundation/shared_schema_package.md")
MASTER_PLAN = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
PHASE_PLAN = Path("docs/planning/shared_schema_package_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/shared_schema_package_phase_10_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

SUPPORTED_SCHEMA = "shared-schema-package.v0.1"
CANONICAL_SCHEMA = str(SCHEMA)
MANIFEST_PATH = str(MANIFEST)
RUST_OUTPUT = str(RUST_PROJECTION)
VALIDATOR_ENTRYPOINT = "SharedSchemaPhase10ValidationHandoffContract::canonical().validate()"

VALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/valid/shared_schema_package_phase10.valid.json"),
]

INVALID_FIXTURES = {
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase10_structure_missing.invalid.json"
    ): "schema.phase10_structure_check_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase10_tech_stack_drift.invalid.json"
    ): "schema.phase10_tech_stack_alignment_drift",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase10_master_link_missing.invalid.json"
    ): "schema.phase10_master_or_source_alignment_missing",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase10_handoff_authority_drift.invalid.json"
    ): "schema.phase10_handoff_authority_drift",
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/"
        "shared_schema_package_phase10_projection_authority.invalid.json"
    ): "schema.phase10_rust_projection_authority_drift",
}

REQUIRED_STRUCTURE_CHECKS = {
    "title_prefix",
    "attached_sds_link",
    "phase_headings_1_through_10",
    "work_item_design_output_validation_fields",
    "exit_gate_present",
    "local_markdown_links",
}
REQUIRED_TECH_STACK_CHECKS = {
    "canonical_json_schema_authority",
    "rust_first_generation",
    "typescript_web_second",
    "protobuf_internal_only",
    "strict_sensitive_validation",
    "no_conventional_cloud_product_boundary",
}
REQUIRED_MASTER_PLAN_CHECKS = {
    "phase_0_through_13_order_unchanged",
    "sds7_phase0_foundation_package",
    "downstream_expansion_owner_gated",
    "master_crosswalk_link_preserved",
}
REQUIRED_SOURCE_ALIGNMENT_DOCS = {
    str(SDS),
    str(SERVICE_CATALOG),
    str(SUB_PLAN),
    str(MASTER_PLAN),
    str(CROSSWALK),
    str(TECH_STACK),
    str(PHASE_PLAN),
    str(PHASE_PROGRESS),
}
REQUIRED_DOWNSTREAM_CONSUMERS = {
    "control_plane_services",
    "execution_services",
    "trust_policy_services",
    "accounting_services",
    "data_storage_namespace_services",
    "adapters",
    "sdk",
    "cli",
    "admin_developer_ui",
    "native_apps",
    "mobile",
    "ai_gateway_services",
    "docdex_rag",
}
REQUIRED_FORBIDDEN_ASSUMPTIONS = {
    "PostgreSQL",
    "Redis",
    "S3",
    "MinIO",
    "NATS",
    "Kafka",
    "Vault",
    "blockchain",
    "NFT",
    "pricing",
    "revenue",
    "customer-count",
}
REQUIRED_CONTRACTS = {
    "phase10_validation_handoff_contract",
    "phase10_structure_check",
    "phase10_tech_stack_alignment_check",
    "phase10_master_plan_alignment_check",
    "phase10_downstream_handoff_rule",
    "phase10_rust_projection",
}
REQUIRED_SOURCE_INPUTS = {
    CANONICAL_SCHEMA,
    MANIFEST_PATH,
    str(SUB_PLAN),
    str(MASTER_PLAN),
    str(CROSSWALK),
    str(TECH_STACK),
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


def phase10_payload(record: dict[str, Any]) -> dict[str, Any]:
    payload = record.get("phase10_validation_handoff_contract", record)
    return payload if isinstance(payload, dict) else {}


def check_required_files() -> None:
    for path in [
        SCHEMA,
        MANIFEST,
        CONTRACTS_README,
        SCHEMAS_README,
        RUST_PROJECTION,
        SUB_PLAN,
        SDS,
        SERVICE_CATALOG,
        MASTER_PLAN,
        CROSSWALK,
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
    assert_true("Phase 10" in schema.get("title", ""), "schema title missing Phase 10")
    props = schema.get("properties", {})
    defs = schema.get("$defs", {})
    assert_true("phase10_validation_handoff_contract" in props, "schema missing Phase 10 root property")
    for def_name in REQUIRED_CONTRACTS | {"phase10_source_alignment_document"}:
        assert_true(def_name in defs, f"schema missing $defs.{def_name}")

    assert_true(
        REQUIRED_STRUCTURE_CHECKS <= set(defs["phase10_structure_check"].get("enum", [])),
        "schema Phase 10 structure enum drifted",
    )
    assert_true(
        REQUIRED_TECH_STACK_CHECKS <= set(defs["phase10_tech_stack_alignment_check"].get("enum", [])),
        "schema Phase 10 tech-stack enum drifted",
    )
    assert_true(
        REQUIRED_MASTER_PLAN_CHECKS <= set(defs["phase10_master_plan_alignment_check"].get("enum", [])),
        "schema Phase 10 master-plan enum drifted",
    )
    assert_true(
        REQUIRED_SOURCE_ALIGNMENT_DOCS <= set(defs["phase10_source_alignment_document"].get("enum", [])),
        "schema Phase 10 source-doc enum drifted",
    )
    handoff = defs["phase10_downstream_handoff_rule"]
    assert_true(handoff.get("additionalProperties") is False, "Phase 10 handoff rule must reject unknown fields")
    assert_true(
        REQUIRED_DOWNSTREAM_CONSUMERS
        <= set(handoff.get("properties", {}).get("consumer_family", {}).get("enum", [])),
        "schema Phase 10 downstream consumer enum drifted",
    )
    for field in [
        "owning_service_authority_retained",
        "generated_contract_consumption_required",
    ]:
        assert_true(
            handoff.get("properties", {}).get(field, {}).get("const") is True,
            f"schema Phase 10 handoff {field} must be true",
        )
    for field in [
        "runtime_authority_allowed",
        "audit_finality_allowed",
        "policy_truth_allowed",
        "accounting_finality_allowed",
        "storage_truth_allowed",
        "secret_custody_allowed",
    ]:
        assert_true(
            handoff.get("properties", {}).get(field, {}).get("const") is False,
            f"schema Phase 10 handoff {field} must be false",
        )

    contract = defs["phase10_validation_handoff_contract"]
    contract_props = contract.get("properties", {})
    assert_true(contract.get("additionalProperties") is False, "Phase 10 contract must reject unknown fields")
    assert_true(
        set(contract.get("required", []))
        >= {
            "structure_checks",
            "tech_stack_alignment_checks",
            "master_plan_alignment_checks",
            "source_alignment_documents",
            "downstream_handoff_rules",
            "forbidden_assumption_terms",
            "source_hash_inputs",
            "rust_projection",
        },
        "Phase 10 contract required fields drifted",
    )
    assert_true(
        contract_props["forbidden_assumption_terms"].get("minItems") >= len(REQUIRED_FORBIDDEN_ASSUMPTIONS),
        "Phase 10 forbidden assumptions minItems is too loose",
    )
    assert_true(
        contract_props["source_hash_inputs"].get("minItems") >= len(REQUIRED_SOURCE_INPUTS),
        "Phase 10 source inputs minItems is too loose",
    )


def validate_phase10_record(record: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if record.get("schema_family") not in (None, "shared-schema-package"):
        errors.append("schema.family")
    if record.get("schema_version") not in (None, SUPPORTED_SCHEMA):
        errors.append("schema.version")

    payload = phase10_payload(record)
    if not payload:
        errors.append("schema.phase10_contract_missing")
        return errors

    structure_checks = set(payload.get("structure_checks", []))
    if not REQUIRED_STRUCTURE_CHECKS <= structure_checks:
        errors.append("schema.phase10_structure_check_missing")

    tech_checks = set(payload.get("tech_stack_alignment_checks", []))
    forbidden_terms = set(payload.get("forbidden_assumption_terms", []))
    if not REQUIRED_TECH_STACK_CHECKS <= tech_checks or not REQUIRED_FORBIDDEN_ASSUMPTIONS <= forbidden_terms:
        errors.append("schema.phase10_tech_stack_alignment_drift")

    master_checks = set(payload.get("master_plan_alignment_checks", []))
    source_docs = set(payload.get("source_alignment_documents", []))
    source_inputs = set(payload.get("source_hash_inputs", []))
    if (
        not REQUIRED_MASTER_PLAN_CHECKS <= master_checks
        or not REQUIRED_SOURCE_ALIGNMENT_DOCS <= source_docs
        or not REQUIRED_SOURCE_INPUTS <= source_inputs
    ):
        errors.append("schema.phase10_master_or_source_alignment_missing")

    handoff_rules = payload.get("downstream_handoff_rules", [])
    rule_consumers = {rule.get("consumer_family") for rule in handoff_rules if isinstance(rule, dict)}
    if handoff_rules and not REQUIRED_DOWNSTREAM_CONSUMERS <= rule_consumers:
        errors.append("schema.phase10_handoff_authority_drift")
    for rule in handoff_rules:
        if not isinstance(rule, dict):
            errors.append("schema.phase10_handoff_authority_drift")
            continue
        if (
            rule.get("consumer_family") not in REQUIRED_DOWNSTREAM_CONSUMERS
            or rule.get("owning_service_authority_retained") is not True
            or not rule.get("schema_request_path")
            or rule.get("generated_contract_consumption_required") is not True
            or rule.get("runtime_authority_allowed") is not False
            or rule.get("audit_finality_allowed") is not False
            or rule.get("policy_truth_allowed") is not False
            or rule.get("accounting_finality_allowed") is not False
            or rule.get("storage_truth_allowed") is not False
            or rule.get("secret_custody_allowed") is not False
        ):
            errors.append("schema.phase10_handoff_authority_drift")

    rust_projection = payload.get("rust_projection")
    if isinstance(rust_projection, dict):
        if (
            rust_projection.get("path") != RUST_OUTPUT
            or rust_projection.get("validator_entrypoint") != VALIDATOR_ENTRYPOINT
            or rust_projection.get("non_authoritative") is not True
        ):
            errors.append("schema.phase10_rust_projection_authority_drift")
    else:
        errors.append("schema.phase10_rust_projection_authority_drift")

    return errors


def check_manifest(manifest: dict[str, Any]) -> None:
    entry = manifest.get("shared_schema_package_phase10")
    assert_true(isinstance(entry, dict), "codegen manifest missing shared_schema_package_phase10")
    assert_true(entry.get("source_of_truth") == "json_schema", "Phase 10 source of truth drifted")
    assert_true(entry.get("rust_first_validation") is True, "Phase 10 must remain Rust-first")
    assert_true(entry.get("canonical_schema") == CANONICAL_SCHEMA, "Phase 10 canonical schema drifted")

    rust = entry.get("rust_projection", {})
    assert_true(rust.get("path") == RUST_OUTPUT, "Phase 10 Rust output path drifted")
    assert_true(rust.get("validator_entrypoint") == VALIDATOR_ENTRYPOINT, "Phase 10 validator entrypoint drifted")
    assert_true(rust.get("non_authoritative") is True, "Phase 10 Rust projection became authority")
    assert_true(REQUIRED_CONTRACTS <= set(entry.get("contracts", [])), "Phase 10 contracts list incomplete")
    assert_true(REQUIRED_STRUCTURE_CHECKS <= set(entry.get("structure_checks", [])), "Phase 10 structure checks incomplete")
    assert_true(
        REQUIRED_TECH_STACK_CHECKS <= set(entry.get("tech_stack_alignment_checks", [])),
        "Phase 10 tech-stack checks incomplete",
    )
    assert_true(
        REQUIRED_MASTER_PLAN_CHECKS <= set(entry.get("master_plan_alignment_checks", [])),
        "Phase 10 master-plan checks incomplete",
    )
    assert_true(
        REQUIRED_SOURCE_ALIGNMENT_DOCS <= set(entry.get("source_alignment_documents", [])),
        "Phase 10 source alignment docs incomplete",
    )
    fixtures = entry.get("fixtures", {})
    assert_true(str(VALID_FIXTURES[0]) in fixtures.get("valid", []), "Phase 10 valid fixture missing from manifest")
    for fixture in INVALID_FIXTURES:
        assert_true(str(fixture) in fixtures.get("invalid", []), f"Phase 10 invalid fixture missing: {fixture}")
    assert_true(not validate_phase10_record(entry), "manifest Phase 10 metadata should be valid")


def check_fixtures() -> None:
    for fixture in VALID_FIXTURES:
        errors = validate_phase10_record(load_json(fixture))
        assert_true(not errors, f"{fixture} should be valid, got {errors}")
    for fixture, expected_error in INVALID_FIXTURES.items():
        errors = validate_phase10_record(load_json(fixture))
        assert_true(expected_error in errors, f"{fixture} missing expected error {expected_error}; got {errors}")


def check_sub_plan_structure() -> None:
    sub_plan = read_text(SUB_PLAN)
    assert_true(sub_plan.startswith("# SUB BUILD PLAN #7 - Shared Schema Package"), "sub-plan title prefix drifted")
    assert_contains(sub_plan, "SDS #7: Shared Schema Package", SUB_PLAN)
    for phase in range(1, 11):
        assert_contains(sub_plan, f"## Phase {phase}:", SUB_PLAN)
    for item in range(1, 6):
        marker = f"- **10.{item} "
        assert_contains(sub_plan, marker, SUB_PLAN)
        start = sub_plan.index(marker)
        next_marker = f"- **10.{item + 1} " if item < 5 else "## Alignment Review"
        end = sub_plan.index(next_marker, start)
        section = sub_plan[start:end]
        for label in ["Design:", "Output:", "Validation:"]:
            assert_contains(section, label, SUB_PLAN)
    assert_contains(sub_plan, "## Exit Gate", SUB_PLAN)


def check_local_markdown_links(paths: list[Path]) -> None:
    link_pattern = re.compile(r"\[[^\]]+\]\(([^)]+)\)")
    for source in paths:
        text = read_text(source)
        for raw_target in link_pattern.findall(text):
            target = raw_target.strip().split()[0]
            if target.startswith("#") or "://" in target or target.startswith("mailto:"):
                continue
            target = unquote(target.split("#", 1)[0].split("?", 1)[0])
            if not target:
                continue
            resolved = (REPO_ROOT / source.parent / target).resolve()
            assert_true(resolved.exists(), f"{source} has missing local Markdown link: {raw_target}")


def check_docs_and_planning() -> None:
    check_sub_plan_structure()
    check_local_markdown_links([SUB_PLAN, SDS, SERVICE_CATALOG, MASTER_PLAN, CROSSWALK])

    tech_stack = read_text(TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)
    assert_contains(tech_stack, "TypeScript/web", TECH_STACK)
    assert_contains(tech_stack, "Protobuf", TECH_STACK)

    contracts_readme = read_text(CONTRACTS_README)
    assert_contains(contracts_readme, "Shared Schema Package Phase 10", CONTRACTS_README)
    assert_contains(contracts_readme, VALIDATOR_ENTRYPOINT, CONTRACTS_README)
    schemas_readme = read_text(SCHEMAS_README)
    assert_contains(schemas_readme, "Shared Schema Package Phase 10", SCHEMAS_README)
    assert_contains(schemas_readme, "downstream handoff", SCHEMAS_README)

    sds = read_text(SDS)
    service_catalog = read_text(SERVICE_CATALOG)
    for source_text, source_path in [(sds, SDS), (service_catalog, SERVICE_CATALOG)]:
        assert_contains(source_text, "sub_build_plan_007_shared_schema_package.md", source_path)
        assert_contains(source_text, "not a deployed microservice", source_path)

    master = read_text(MASTER_PLAN)
    crosswalk = read_text(CROSSWALK)
    assert_contains(master, "SDS #7: [Shared Schema Package]", MASTER_PLAN)
    assert_contains(master, "First build point remains Phase 0", MASTER_PLAN)
    assert_contains(crosswalk, "| SDS #7 |", CROSSWALK)
    assert_contains(crosswalk, "Phase 0: Foundation", CROSSWALK)
    assert_contains(crosswalk, "downstream schema expansion gated by owning service phases", CROSSWALK)

    phase_indexes = []
    for phase in range(14):
        marker = f"## Phase {phase}:"
        assert_contains(master, marker, MASTER_PLAN)
        phase_indexes.append(master.index(marker))
    assert_true(phase_indexes == sorted(phase_indexes), "master Phase 0 through Phase 13 order changed")

    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #7 Phase 10", PHASE_PLAN)
    assert_contains(phase_plan, "downstream handoff", PHASE_PLAN)
    assert_contains(phase_progress, "Shared Schema Package Phase 10 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)


def check_forbidden_assumption_contexts() -> None:
    allowed_markers = [
        "no ",
        "not ",
        "must not",
        "does not",
        "reject",
        "forbidden",
        "drift",
        "assumption",
        "without",
        "except",
        "replacing",
    ]
    for source in [SUB_PLAN, SDS, SERVICE_CATALOG, PHASE_PLAN, TECH_STACK]:
        in_explicit_non_choices = False
        for line_no, line in enumerate(read_text(source).splitlines(), start=1):
            if source == TECH_STACK:
                if line.startswith("## "):
                    in_explicit_non_choices = line.strip() == "## Explicit Non-Choices"
            lower = line.lower()
            for term in REQUIRED_FORBIDDEN_ASSUMPTIONS:
                pattern = re.compile(rf"(?<![A-Za-z0-9_-]){re.escape(term)}(?![A-Za-z0-9_-])", re.IGNORECASE)
                if pattern.search(line):
                    assert_true(
                        in_explicit_non_choices or any(marker in lower for marker in allowed_markers),
                        f"{source}:{line_no} mentions forbidden assumption term without rejection context: {term}",
                    )


def check_rust_projection() -> None:
    source = read_text(RUST_PROJECTION)
    for expected in [
        "PHASE10_CANONICAL_SCHEMA_SOURCE",
        "PHASE10_VALIDATOR_SCRIPT",
        "REQUIRED_SHARED_SCHEMA_PHASE10_STRUCTURE_CHECKS",
        "REQUIRED_SHARED_SCHEMA_PHASE10_TECH_STACK_CHECKS",
        "REQUIRED_SHARED_SCHEMA_PHASE10_DOWNSTREAM_CONSUMERS",
        "SharedSchemaPhase10DownstreamHandoffRule",
        "SharedSchemaPhase10ValidationHandoffContract",
        "SharedSchemaPhase10ContractError",
        VALIDATOR_ENTRYPOINT,
    ]:
        assert_contains(source, expected, RUST_PROJECTION)


def check_suite_wiring() -> None:
    suite = read_text(SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_shared_schema_package_phase10.py")', SUITE_VALIDATOR)


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
    check_forbidden_assumption_contexts()
    check_rust_projection()
    check_suite_wiring()
    run(["cargo", "test", "-p", "overrid-contracts", "shared_schema_phase10"])
    print("Shared Schema Package Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
