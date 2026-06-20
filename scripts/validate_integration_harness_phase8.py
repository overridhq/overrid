#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 8 artifacts and flake evidence."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

HARNESS = Path("packages/integration_harness")
CONTRACTS = Path("packages/schemas/overrid_contracts")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_08_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")
SCHEMA = CONTRACTS / "v0/integration_harness.schema.json"

VALID_FIXTURES = [
    CONTRACTS / "fixtures/valid/integration_harness_phase8_artifact_bundle.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase8_flake_detection.valid.json",
]
INVALID_FIXTURES = [
    CONTRACTS / "fixtures/invalid/artifact_raw_secret.invalid.json",
    CONTRACTS / "fixtures/invalid/artifact_sensitive_material.invalid.json",
]

EXPECTED_SCENARIOS = {
    "scenario_phase8_artifact_bundle": {
        "fixture": VALID_FIXTURES[0],
        "fixture_id": "fixture_phase8_artifact_bundle",
        "gate_class": "release_candidate",
        "status": "passed",
        "reason": "run.passed",
        "reason_class": "success",
        "artifact_policy": "release_candidate",
        "retention_class": "release_candidate",
        "assertion": "assertion_phase8_artifact_bundle",
        "manifest_assertion_ref": "assertion:phase8:artifact_bundle",
        "assertion_status": "passed",
        "assertion_reason": "assertion.passed",
        "flake": False,
        "repeated_run_count": 1,
        "timing_variance_ms": 0,
        "marker": None,
        "tolerance_window_used": False,
        "minimum_retention_days": 90,
        "prune_after_days": 365,
        "expected_exit": 0,
        "top_ok": True,
    },
    "scenario_phase8_flake_detection": {
        "fixture": VALID_FIXTURES[1],
        "fixture_id": "fixture_phase8_flake_detection",
        "gate_class": "regression",
        "status": "failed",
        "reason": "flake.unstable_event_ordering",
        "reason_class": "assertion",
        "artifact_policy": "failure_evidence",
        "retention_class": "failure_evidence",
        "assertion": "assertion_phase8_unstable_event_ordering",
        "manifest_assertion_ref": "assertion:phase8:unstable_event_ordering",
        "assertion_status": "failed",
        "assertion_reason": "flake.unstable_event_ordering",
        "flake": True,
        "repeated_run_count": 3,
        "timing_variance_ms": 125,
        "marker": "unstable_event_ordering",
        "tolerance_window_used": True,
        "minimum_retention_days": 30,
        "prune_after_days": 90,
        "expected_exit": 11,
        "top_ok": False,
    },
}

FORBIDDEN_RUNTIME_TERMS = {
    "node.js",
    "typescript runtime",
    "postgresql",
    "redis",
    "s3",
    "minio",
    "nats",
    "kafka",
    "vault",
    "blockchain",
    "nft",
}


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(read(path))


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} missing expected text: {expected}")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_runtime_authority(text: str, source: Path) -> None:
    lowered = text.lower().replace("overvault", "")
    for term in FORBIDDEN_RUNTIME_TERMS:
        if term in lowered:
            index = lowered.find(term)
            prefix = lowered[max(0, index - 80) : index]
            if "do not add" not in prefix and "must not" not in prefix and "not as" not in prefix:
                raise AssertionError(
                    f"{source} appears to introduce forbidden runtime authority: {term}"
                )


def run(
    command: list[str], cwd: Path = REPO_ROOT, expected_exit: int = 0
) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, capture_output=True)
    if result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli(args: list[str], expected_exit: int = 0) -> dict[str, Any]:
    result = run(
        ["cargo", "run", "-q", "-p", "overrid-cli", "--", *args],
        expected_exit=expected_exit,
    )
    if result.stderr:
        raise AssertionError(f"CLI emitted stderr unexpectedly: {' '.join(args)}\n{result.stderr}")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}:\n{result.stdout}") from exc


def validate_required_files() -> None:
    for path in [
        HARNESS / "src/artifacts.rs",
        HARNESS / "src/runner.rs",
        HARNESS / "src/step_runners.rs",
        HARNESS / "src/manifests.rs",
        HARNESS / "src/lib.rs",
        CONTRACTS / "src/lib.rs",
        CONTRACTS / "codegen_manifest.json",
        SCHEMA,
        SUB_PLAN,
        TECH_STACK,
        PHASE_PLAN,
        PHASE_PROGRESS,
        VALIDATION_WRAPPER,
        *VALID_FIXTURES,
        *INVALID_FIXTURES,
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"Missing required file: {path}")


def validate_docs() -> None:
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "Phase 8: Artifact Bundles, Redaction, Reproduction, And Flake Detection",
        "Implement artifact collection",
        "Implement redaction scanner",
        "Implement reproduction commands",
        "Implement flake detection fields",
        "Implement retention classes",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "Rust-first infrastructure stack",
        "Cargo workspace with crates for services, shared contracts, test utilities, CLI, and local tooling",
        "JSON plus JSON Schema",
        "not as the core runtime",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)

    for expected in [
        "Integration Test Harness Phase 8 Implementation Plan",
        "packages/integration_harness/src/artifacts.rs",
        "packages/integration_harness/src/runner.rs",
        "packages/integration_harness/src/step_runners.rs",
        "scripts/validate_integration_harness_phase8.py",
        "artifact collection refs",
        "flake metadata",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 8 Progress",
        "Validation Evidence",
        "scripts/validate_integration_harness_phase8.py",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(
        wrapper,
        "scripts/validate_integration_harness_phase8.py",
        VALIDATION_WRAPPER,
    )


def validate_codegen_manifest() -> None:
    manifest = load_json(CONTRACTS / "codegen_manifest.json")
    phase8 = manifest.get("integration_harness_phase8")
    assert_true(isinstance(phase8, dict), "codegen manifest missing integration_harness_phase8")
    assert_true(phase8.get("schema_family") == "integration-harness", "wrong Phase 8 schema family")
    assert_true(phase8.get("schema_version") == "integration-harness.v0.1", "wrong Phase 8 schema version")
    assert_true(phase8.get("canonical_schema") == str(SCHEMA), "wrong Phase 8 canonical schema")
    assert_true(phase8.get("rust_first_validation") is True, "Phase 8 must remain Rust-first")
    listed_valid = [Path(value) for value in phase8.get("fixtures", {}).get("valid", [])]
    listed_invalid = [Path(value) for value in phase8.get("fixtures", {}).get("invalid", [])]
    assert_true(listed_valid == VALID_FIXTURES, "Phase 8 valid fixture list drifted")
    assert_true(listed_invalid == INVALID_FIXTURES, "Phase 8 invalid fixture list drifted")
    guardrails = " ".join(phase8.get("guardrails", []))
    for expected in [
        "Artifact collection",
        "Redaction scanner evidence",
        "Reproduction commands",
        "Flake metadata",
        "Retention classes",
    ]:
        assert_contains(guardrails, expected, CONTRACTS / "codegen_manifest.json")


def validate_schema_contract() -> None:
    schema = load_json(SCHEMA)
    defs = schema["$defs"]
    bundle = defs["artifact_bundle"]
    bundle_required = set(bundle["required"])
    for expected in [
        "schema_version_refs",
        "assertion_diff_refs",
        "redaction_report_ref",
        "flake_metadata",
        "retention_policy",
    ]:
        assert_true(expected in bundle_required, f"artifact_bundle must require {expected}")
        assert_true(expected in bundle["properties"], f"artifact_bundle missing property {expected}")

    assert_true(
        bundle["properties"]["flake_metadata"]["$ref"] == "#/$defs/flake_metadata",
        "artifact_bundle flake_metadata must use shared definition",
    )
    assert_true(
        bundle["properties"]["retention_policy"]["$ref"] == "#/$defs/retention_policy",
        "artifact_bundle retention_policy must use shared definition",
    )
    assert_true(
        bundle["properties"]["redaction_report_ref"]["$ref"] == "#/$defs/overrid_ref",
        "artifact_bundle redaction_report_ref must be an Overrid ref",
    )
    assert_true(
        defs["test_run_record"]["properties"]["flake_metadata"]["$ref"] == "#/$defs/flake_metadata",
        "test_run_record must expose flake metadata",
    )
    assert_true(
        bundle["properties"]["redaction_summary"]["properties"]["scanner_passed"]["const"] is True,
        "redaction_summary must require passing scanner evidence",
    )


def validate_flake_metadata(
    metadata: dict[str, Any], expectation: dict[str, Any], source: Path | str
) -> None:
    assert_true(
        metadata["repeated_run_count"] == expectation["repeated_run_count"],
        f"{source} repeated_run_count drifted",
    )
    assert_true(
        metadata["timing_variance_ms"] == expectation["timing_variance_ms"],
        f"{source} timing_variance_ms drifted",
    )
    assert_true(
        metadata["unstable_event_ordering"] is expectation["flake"],
        f"{source} unstable_event_ordering drifted",
    )
    assert_true(
        metadata["tolerance_window_used"] is expectation["tolerance_window_used"],
        f"{source} tolerance_window_used drifted",
    )
    markers = metadata["nondeterministic_assertion_markers"]
    if expectation["marker"] is None:
        assert_true(markers == [], f"{source} should not emit nondeterministic markers")
    else:
        assert_true(
            any(expectation["marker"] in marker for marker in markers),
            f"{source} missing nondeterministic marker",
        )


def validate_retention_policy(
    policy: dict[str, Any], expectation: dict[str, Any], source: Path | str
) -> None:
    assert_true(
        policy["retention_class"] == expectation["retention_class"],
        f"{source} retention policy class drifted",
    )
    assert_true(
        policy["minimum_retention_days"] == expectation["minimum_retention_days"],
        f"{source} minimum retention drifted",
    )
    assert_true(
        policy["prune_after_days"] == expectation["prune_after_days"],
        f"{source} prune horizon drifted",
    )
    assert_true(policy["compact_success_summary"] is False, f"{source} must preserve Phase 8 evidence")


def validate_reproduction_command(
    command: str, scenario_id: str, expectation: dict[str, Any], source: Path | str
) -> None:
    expected_prefix = (
        f"overrid test scenario {scenario_id} --profile local "
        f"--fixture {expectation['fixture_id']}"
    )
    assert_true(command.startswith(expected_prefix), f"{source} reproduction command drifted")
    for expected in [
        "--trace-root",
        "--artifact-output target/overrid/integration_harness/artifacts/",
        "--json",
    ]:
        assert_true(expected in command, f"{source} reproduction command missing {expected}")
    lowered = command.lower()
    assert_true("/users/" not in lowered, f"{source} reproduction command leaked user path")
    assert_true("/home/" not in lowered, f"{source} reproduction command leaked home path")


def validate_artifact_bundle(
    bundle: dict[str, Any], scenario_id: str, expectation: dict[str, Any], source: Path | str
) -> None:
    assert_true(bundle["retention_class"] == expectation["retention_class"], f"{source} retention class drifted")
    assert_true(bundle["redaction_summary"]["policy"] == "secret_free_refs_only", f"{source} redaction policy drifted")
    assert_true(bundle["redaction_summary"]["scanner_passed"] is True, f"{source} redaction scanner must pass")
    assert_true(bundle["redaction_summary"]["redacted_fields"], f"{source} redacted fields must be recorded")
    for field in [
        "redacted_log_refs",
        "overwatch_export_refs",
        "cli_output_refs",
        "api_payload_envelope_refs",
        "stack_health_refs",
        "fixture_version_refs",
        "schema_version_refs",
        "assertion_diff_refs",
        "artifact_refs",
    ]:
        assert_true(bundle[field], f"{source} {field} must not be empty")
    assert_true(
        "artifact:schema_version:integration_harness_v0_1" in bundle["schema_version_refs"],
        f"{source} missing schema version evidence",
    )
    assert_true(bundle["redaction_report_ref"].startswith("artifact:redaction_report:"), f"{source} missing report ref")
    if expectation["flake"]:
        assert_true(
            any("unstable_event_ordering" in value for value in bundle["assertion_diff_refs"]),
            f"{source} flake bundles must carry assertion diff refs",
        )
    validate_reproduction_command(bundle["reproduction_command"], scenario_id, expectation, source)
    validate_flake_metadata(bundle["flake_metadata"], expectation, source)
    validate_retention_policy(bundle["retention_policy"], expectation, source)
    for flag in [
        "contains_raw_secret",
        "contains_private_key",
        "contains_token",
        "contains_signature",
        "contains_encrypted_rag_content",
        "contains_private_payload",
        "contains_fixture_key_material",
    ]:
        assert_true(bundle[flag] is False, f"{source} must reject {flag}")


def validate_fixture(scenario_id: str, expectation: dict[str, Any]) -> None:
    path = expectation["fixture"]
    document = load_json(path)
    fixture = document["fixture_manifest"]
    scenario = document["scenario_manifest"]
    run_record = document["test_run_record"]
    assertion = document["assertion_result"]
    trace = document["golden_trace"]
    bundle = document["artifact_bundle"]

    assert_true(fixture["test_only"] is True, f"{path} fixture must be test_only")
    assert_true(fixture["fixture_id"] == expectation["fixture_id"], f"{path} fixture id drifted")
    assert_true(
        all(key["test_only"] is True and key["raw_key_material_present"] is False for key in fixture["keys"]),
        f"{path} fixture keys must be test-only refs",
    )
    assert_true(scenario["scenario_id"] == scenario_id, f"{path} scenario id drifted")
    assert_true(scenario["master_phase"] == 8, f"{path} must be a Phase 8 scenario")
    assert_true(scenario["gate_class"] == expectation["gate_class"], f"{path} gate class drifted")
    assert_true("phase8" in scenario["tags"], f"{path} missing phase8 tag")
    for expected in ["artifact_bundle", "redaction", "reproduction"]:
        assert_true(expected in scenario["tags"], f"{path} missing tag {expected}")
    assert_true("service:local_stack" in scenario["required_services"], f"{path} missing stack service")
    assert_true("collect_artifacts_then_reset" in scenario["cleanup_rules"], f"{path} missing artifact cleanup")
    assert_true(run_record["status"] == expectation["status"], f"{path} run status drifted")
    assert_true(run_record["reason_code"] == expectation["reason"], f"{path} run reason drifted")
    assert_true(run_record["reason_class"] == expectation["reason_class"], f"{path} reason class drifted")
    assert_true(run_record["artifact_policy"] == expectation["artifact_policy"], f"{path} artifact policy drifted")
    assert_true(assertion["assertion_id"] == expectation["assertion"], f"{path} assertion id drifted")
    assert_true(assertion["status"] == expectation["assertion_status"], f"{path} assertion status drifted")
    assert_true(assertion["reason_code"] == expectation["assertion_reason"], f"{path} assertion reason drifted")
    assert_true(trace["assertion_mode"] == "exact", f"{path} Phase 8 trace should be exact")
    assert_true(trace["forbidden_transitions"], f"{path} must record forbidden transitions")
    validate_flake_metadata(run_record["flake_metadata"], expectation, path)
    validate_artifact_bundle(bundle, scenario_id, expectation, path)


def validate_fixtures() -> None:
    for scenario_id, expectation in EXPECTED_SCENARIOS.items():
        validate_fixture(scenario_id, expectation)

    raw_secret = load_json(INVALID_FIXTURES[0])
    raw_bundle = raw_secret["artifact_bundle"]
    assert_true(raw_secret["expected_error"] == "artifact.raw_secret", "raw secret invalid fixture drifted")
    assert_true(raw_bundle["contains_raw_secret"] is True, "raw secret invalid fixture must contain raw secret")
    assert_true(raw_bundle["redaction_summary"]["scanner_passed"] is False, "raw secret scanner must fail")

    sensitive = load_json(INVALID_FIXTURES[1])
    sensitive_bundle = sensitive["artifact_bundle"]
    expected_errors = {
        "artifact.private_key",
        "artifact.token",
        "artifact.signature",
        "artifact.encrypted_rag_content",
        "artifact.private_payload",
        "artifact.fixture_key_material",
    }
    assert_true(
        set(sensitive["expected_errors"]) == expected_errors,
        "sensitive material invalid fixture drifted",
    )
    for flag in [
        "contains_private_key",
        "contains_token",
        "contains_signature",
        "contains_encrypted_rag_content",
        "contains_private_payload",
        "contains_fixture_key_material",
    ]:
        assert_true(sensitive_bundle[flag] is True, f"sensitive material invalid fixture must set {flag}")
    assert_true(sensitive_bundle["redaction_summary"]["scanner_passed"] is False, "sensitive scanner must fail")


def validate_rust_surfaces() -> None:
    artifacts_rs = read(HARNESS / "src/artifacts.rs")
    runner_rs = read(HARNESS / "src/runner.rs")
    step_runners_rs = read(HARNESS / "src/step_runners.rs")
    harness_lib_rs = read(HARNESS / "src/lib.rs")
    contracts_lib_rs = read(CONTRACTS / "src/lib.rs")

    for expected in [
        "pub mod artifacts;",
        "pub mod runner;",
        "pub mod step_runners;",
    ]:
        assert_contains(harness_lib_rs, expected, HARNESS / "src/lib.rs")

    for expected in [
        "ArtifactCollectionRefs",
        "RedactionScanReport",
        "FlakeMetadata",
        "ArtifactRetentionPolicy",
        "with_run_context",
        "retention_class_for_outcome",
        "scan_artifact_values",
        "secret_free_refs_only",
        "target/overrid/integration_harness/artifacts",
        "artifact_summary_adds_reproduction_flake_and_retention_context",
        "redaction_scanner_rejects_forbidden_artifact_material",
    ]:
        assert_contains(artifacts_rs, expected, HARNESS / "src/artifacts.rs")

    for expected in [
        "enrich_artifacts",
        "artifacts_json",
        "redaction_report_json",
        "flake_metadata_json",
        "retention_policy_json",
        "scenario_phase8_artifact_bundle",
        "scenario_phase8_flake_detection",
        "phase8_artifacts_emit_redaction_reproduction_flake_and_retention_metadata",
        "redaction_report",
        "reproduction_command",
        "flake_metadata",
        "retention_policy",
    ]:
        assert_contains(runner_rs, expected, HARNESS / "src/runner.rs")

    for expected in [
        "assertion:unstable_event_ordering",
        "assertion:nondeterministic_marker",
        "flake.unstable_event_ordering",
        "assertion_step_runner_marks_flakes_as_failed_not_successful",
    ]:
        assert_contains(step_runners_rs, expected, HARNESS / "src/step_runners.rs")

    for expected in [
        "RedactionScanReport",
        "FlakeMetadata",
        "ArtifactRetentionPolicy",
        "ArtifactBundle",
        "validates_phase8_flake_redaction_and_retention_contracts",
    ]:
        assert_contains(contracts_lib_rs, expected, CONTRACTS / "src/lib.rs")

    assert_no_forbidden_runtime_authority(artifacts_rs, HARNESS / "src/artifacts.rs")
    assert_no_forbidden_runtime_authority(runner_rs, HARNESS / "src/runner.rs")
    assert_no_forbidden_runtime_authority(step_runners_rs, HARNESS / "src/step_runners.rs")


def validate_cli_artifact_summary(
    artifacts: list[dict[str, Any]], scenario_id: str, expectation: dict[str, Any]
) -> None:
    assert_true(artifacts, f"{scenario_id} must emit artifacts")
    summary = artifacts[0]
    assert_true(summary["retention_class"] == expectation["retention_class"], f"{scenario_id} retention drifted")
    redaction = summary["redaction_report"]
    assert_true(redaction["policy"] == "secret_free_refs_only", f"{scenario_id} redaction policy drifted")
    assert_true(redaction["scanner_passed"] is True, f"{scenario_id} redaction scanner drifted")
    assert_true(redaction["redacted_fields"], f"{scenario_id} redacted fields missing")
    collection = summary["collection_refs"]
    for field in [
        "redacted_log_refs",
        "overwatch_export_refs",
        "cli_output_refs",
        "api_payload_envelope_refs",
        "stack_health_refs",
        "fixture_version_refs",
        "schema_version_refs",
        "assertion_diff_refs",
    ]:
        assert_true(collection[field], f"{scenario_id} collection {field} missing")
    if expectation["flake"]:
        assert_true(
            any("unstable_event_ordering" in value for value in collection["assertion_diff_refs"]),
            f"{scenario_id} missing flake assertion diff ref",
        )
    validate_reproduction_command(summary["reproduction_command"], scenario_id, expectation, scenario_id)
    validate_flake_metadata(summary["flake_metadata"], expectation, scenario_id)
    validate_retention_policy(summary["retention_policy"], expectation, scenario_id)


def validate_cli_behavior() -> None:
    listed = run_cli(["test", "list", "--phase", "8", "--json"])
    listed_ids = [scenario["scenario_id"] for scenario in listed["result"]["scenarios"]]
    for scenario_id in EXPECTED_SCENARIOS:
        assert_true(scenario_id in listed_ids, f"{scenario_id} missing from Phase 8 list")

    for scenario_id, expectation in EXPECTED_SCENARIOS.items():
        output = run_cli(
            ["test", "scenario", scenario_id, "--json"],
            expected_exit=expectation["expected_exit"],
        )
        assert_true(output["ok"] is expectation["top_ok"], f"{scenario_id} top-level ok drifted")
        if not expectation["top_ok"]:
            assert_true(output["reason_code"] == expectation["reason"], f"{scenario_id} top reason drifted")
        result = output["result"]
        assert_true(result["status"] == expectation["status"], f"{scenario_id} status drifted")
        assert_true(result["reason_code"] == expectation["reason"], f"{scenario_id} reason drifted")
        assert_true(
            expectation["manifest_assertion_ref"] in result["assertion_refs"],
            f"{scenario_id} assertion ref missing",
        )
        assert_true("running" in result["lifecycle"], f"{scenario_id} must include running lifecycle")
        assert_true("asserting" in result["lifecycle"], f"{scenario_id} must include asserting lifecycle")
        assert_true(result["step_results"], f"{scenario_id} must emit step results")
        validate_cli_artifact_summary(result["artifacts"], scenario_id, expectation)


def validate_cargo_checks() -> None:
    run(["cargo", "check", "-p", "overrid-contracts"])
    run(["cargo", "check", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-contracts"])
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_docs()
        validate_codegen_manifest()
        validate_schema_contract()
        validate_fixtures()
        validate_rust_surfaces()
        validate_cli_behavior()
        validate_cargo_checks()
    except AssertionError as exc:
        print(f"Integration harness Phase 8 validation failed: {exc}", file=sys.stderr)
        return 1

    print("Integration harness Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
