#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 2 schema contract artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]
SCHEMA = Path("packages/schemas/overrid_contracts/v0/integration_harness.schema.json")
MANIFEST = Path("packages/schemas/overrid_contracts/codegen_manifest.json")
README = Path("packages/schemas/overrid_contracts/README.md")
PLAN = Path("docs/planning/integration_test_harness_phase_02_plan.md")
PROGRESS = Path("docs/planning/integration_test_harness_phase_02_progress.md")
VALID_FIXTURE = Path(
    "packages/schemas/overrid_contracts/fixtures/valid/integration_harness_phase2.valid.json"
)
INVALID_FIXTURES = [
    Path("packages/schemas/overrid_contracts/fixtures/invalid/fixture_missing_seed.invalid.json"),
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/fixture_missing_tenant_actor.invalid.json"
    ),
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/fixture_incompatible_version.invalid.json"
    ),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/fixture_non_test_key.invalid.json"),
    Path(
        "packages/schemas/overrid_contracts/fixtures/invalid/scenario_invalid_action_kind.invalid.json"
    ),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/scenario_missing_assertion.invalid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/scenario_unsupported_phase.invalid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/scenario_unsafe_timeout.invalid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/run_missing_status.invalid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/golden_trace_missing_edge.invalid.json"),
    Path("packages/schemas/overrid_contracts/fixtures/invalid/artifact_raw_secret.invalid.json"),
]

SUPPORTED_SCHEMA = "integration-harness.v0.1"
ACTION_KINDS = {"cli", "sdk", "api", "local_helper", "assertion"}
RUN_STATUSES = {"passed", "failed", "blocked"}
FORBIDDEN_STACK_TERMS = {
    "PostgreSQL",
    "Redis",
    "S3",
    "MinIO",
    "NATS",
    "Kafka",
    "Vault",
    "blockchain",
    "NFT",
}


def load_json(path: Path) -> Any:
    with (REPO_ROOT / path).open("r", encoding="utf-8") as handle:
        return json.load(handle)


def read_text(path: Path) -> str:
    return (REPO_ROOT / path).read_text(encoding="utf-8")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_contains(haystack: str, needle: str, path: Path) -> None:
    assert_true(needle in haystack, f"{path} missing required text: {needle}")


def check_required_files() -> None:
    for path in [SCHEMA, MANIFEST, README, PLAN, PROGRESS, VALID_FIXTURE, *INVALID_FIXTURES]:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_schema_surface(schema: dict[str, Any]) -> None:
    assert_true(schema.get("$schema") == "https://json-schema.org/draft/2020-12/schema", "wrong schema draft")
    assert_true(schema.get("$id", "").endswith("/integration_harness.schema.json"), "wrong schema id")
    assert_true(schema.get("additionalProperties") is False, "root schema must reject unknown fields")
    defs = schema.get("$defs", {})
    for name in [
        "schema_version",
        "fixture_manifest",
        "fixture_identity",
        "fixture_key",
        "resource_card_ref",
        "scenario_manifest",
        "scenario_step",
        "test_run_record",
        "assertion_result",
        "golden_trace",
        "artifact_bundle",
    ]:
        assert_true(name in defs, f"schema missing $defs.{name}")

    assert_true(defs["schema_version"].get("const") == SUPPORTED_SCHEMA, "wrong harness schema version")
    assert_true(
        defs["fixture_key"]["properties"]["test_only"].get("const") is True,
        "fixture keys must be test_only",
    )
    assert_true(
        defs["fixture_key"]["properties"]["raw_key_material_present"].get("const") is False,
        "fixture keys must reject raw key material",
    )
    assert_true(
        defs["scenario_step"]["properties"]["timeout_ms"].get("maximum") == 600000,
        "scenario timeout must be bounded",
    )
    assert_true(
        set(defs["scenario_step"]["properties"]["action_kind"].get("enum", [])) == ACTION_KINDS,
        "scenario action kinds drifted",
    )
    assert_true(
        defs["artifact_bundle"]["properties"]["contains_raw_secret"].get("const") is False,
        "artifact bundle must reject raw secrets",
    )
    assert_true(
        defs["artifact_bundle"]["properties"]["contains_private_payload"].get("const") is False,
        "artifact bundle must reject private payloads",
    )


def check_manifest(manifest: dict[str, Any]) -> None:
    harness = manifest.get("integration_harness_phase2")
    assert_true(isinstance(harness, dict), "codegen manifest missing integration_harness_phase2")
    assert_true(harness.get("schema_family") == "integration-harness", "wrong harness schema family")
    assert_true(harness.get("schema_version") == SUPPORTED_SCHEMA, "wrong harness schema version")
    assert_true(harness.get("canonical_schema") == str(SCHEMA), "wrong canonical schema path")
    assert_true(harness.get("source_of_truth") == "json_schema", "JSON Schema must be source of truth")
    assert_true(harness.get("rust_first_validation") is True, "Rust-first validation flag missing")
    for contract in [
        "fixture_manifest",
        "scenario_manifest",
        "test_run_record",
        "assertion_result",
        "golden_trace",
        "artifact_bundle",
    ]:
        assert_true(contract in harness.get("contracts", []), f"manifest missing contract: {contract}")
    listed_invalid = {Path(path) for path in harness.get("fixtures", {}).get("invalid", [])}
    assert_true(listed_invalid == set(INVALID_FIXTURES), "manifest invalid fixture list drifted")


def validate_fixture_manifest(fixture: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if fixture.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    if not fixture.get("deterministic_seed"):
        errors.append("fixture.deterministic_seed_missing")
    if not fixture.get("tenant_ref") or not fixture.get("actor_ref"):
        errors.append("fixture.tenant_actor_missing")
    if fixture.get("test_only") is not True:
        errors.append("safety.fixture_not_test_only")
    for key in fixture.get("keys", []):
        if (
            key.get("test_only") is not True
            or key.get("signature_ref_only") is not True
            or key.get("raw_key_material_present") is not False
        ):
            errors.append("safety.fixture_key_not_test_only")
    return errors


def validate_scenario_manifest(scenario: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if scenario.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    phase = scenario.get("master_phase")
    if not isinstance(phase, int) or phase < 0 or phase > 13:
        errors.append("scenario.phase_unsupported")
    for step in scenario.get("steps", []):
        if step.get("action_kind") not in ACTION_KINDS:
            errors.append("scenario.invalid_action_kind")
        if not step.get("assertion_refs"):
            errors.append("scenario.assertion_missing")
        timeout_ms = step.get("timeout_ms")
        if not isinstance(timeout_ms, int) or timeout_ms <= 0 or timeout_ms > 600000:
            errors.append("scenario.timeout_unsafe")
    return errors


def validate_run_record(run: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if run.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    if run.get("status") not in RUN_STATUSES:
        errors.append("run.status_missing")
    if not run.get("started_at_ms") or not run.get("ended_at_ms"):
        errors.append("run.timing_missing")
    if not run.get("reason_class"):
        errors.append("run.reason_class_missing")
    if not run.get("artifact_policy") or not run.get("artifact_refs"):
        errors.append("run.artifact_policy_missing")
    return errors


def validate_golden_trace(trace: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if trace.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    if trace.get("assertion_mode") == "dag" and not trace.get("required_causal_edges"):
        errors.append("golden_trace.edge_missing")
    if trace.get("assertion_mode") == "exact" and len(trace.get("required_nodes", [])) < 2:
        errors.append("golden_trace.exact_order_missing")
    if not trace.get("stable_reason_codes"):
        errors.append("golden_trace.reason_codes_missing")
    return errors


def validate_artifact_bundle(bundle: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if bundle.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    redaction = bundle.get("redaction_summary", {})
    if (
        bundle.get("contains_raw_secret") is not False
        or bundle.get("contains_private_payload") is not False
        or redaction.get("scanner_passed") is not True
    ):
        errors.append("artifact.raw_secret")
    if not bundle.get("reproduction_command"):
        errors.append("artifact.reproduction_missing")
    if not bundle.get("artifact_refs"):
        errors.append("artifact.refs_missing")
    return errors


def validate_document(document: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if document.get("schema_version") != SUPPORTED_SCHEMA:
        errors.append("schema.incompatible_version")
    if "fixture_manifest" in document:
        errors.extend(validate_fixture_manifest(document["fixture_manifest"]))
    if "scenario_manifest" in document:
        errors.extend(validate_scenario_manifest(document["scenario_manifest"]))
    if "test_run_record" in document:
        errors.extend(validate_run_record(document["test_run_record"]))
    if "golden_trace" in document:
        errors.extend(validate_golden_trace(document["golden_trace"]))
    if "artifact_bundle" in document:
        errors.extend(validate_artifact_bundle(document["artifact_bundle"]))
    if "assertion_result" in document:
        assertion = document["assertion_result"]
        if assertion.get("status") not in RUN_STATUSES:
            errors.append("assertion.status_missing")
        if not assertion.get("reason_code") or not assertion.get("field_path"):
            errors.append("assertion.reason_missing")
    return errors


def check_valid_fixture(document: dict[str, Any]) -> None:
    errors = validate_document(document)
    assert_true(not errors, f"valid integration harness fixture failed: {errors}")
    assert_true(document["fixture_manifest"]["keys"][0]["test_only"] is True, "fixture key is not test_only")
    assert_true(
        document["scenario_manifest"]["steps"][0]["action_kind"] == "cli",
        "valid fixture must include CLI step",
    )
    assert_true(
        document["scenario_manifest"]["steps"][1]["action_kind"] == "assertion",
        "valid fixture must include assertion step",
    )
    assert_true(
        document["test_run_record"]["status"] == "passed",
        "valid fixture must include terminal run status",
    )
    assert_true(
        document["golden_trace"]["required_causal_edges"],
        "valid golden trace must include causal edge",
    )
    assert_true(
        "overrid test scenario" in document["artifact_bundle"]["reproduction_command"],
        "artifact bundle must include reproduction command",
    )


def check_invalid_fixture(path: Path) -> None:
    document = load_json(path)
    expected = document.get("expected_error")
    assert_true(isinstance(expected, str), f"{path} missing expected_error")
    errors = validate_document(document)
    assert_true(expected in errors, f"{path} expected {expected}, got {errors}")


def check_docs_alignment() -> None:
    readme = read_text(README)
    plan = read_text(PLAN)
    progress = read_text(PROGRESS)
    schema_text = read_text(SCHEMA)

    for text_path, text in [(README, readme), (PLAN, plan), (PROGRESS, progress)]:
        assert_contains(text, "Integration Test Harness", text_path)
        assert_contains(text, "Phase 2", text_path)

    assert_contains(readme, "fixture_manifest", README)
    assert_contains(readme, "golden_trace", README)
    assert_contains(plan, "docs/overrid_tech_stack_choice.md", PLAN)
    assert_contains(plan, "JSON Schema", PLAN)
    assert_contains(progress, "Status: completed", PROGRESS)

    for forbidden in FORBIDDEN_STACK_TERMS:
        assert_true(
            forbidden not in schema_text,
            f"{SCHEMA} should not encode forbidden stack boundary: {forbidden}",
        )


def main() -> int:
    check_required_files()
    check_schema_surface(load_json(SCHEMA))
    check_manifest(load_json(MANIFEST))
    check_valid_fixture(load_json(VALID_FIXTURE))
    for fixture in INVALID_FIXTURES:
        check_invalid_fixture(fixture)
    check_docs_alignment()
    print("Integration Test Harness Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
