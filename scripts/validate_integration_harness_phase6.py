#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 6 scenario step runners."""

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
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_06_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    CONTRACTS / "fixtures/valid/integration_harness_phase6_step_runners.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase6_blocked.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase6_failed.valid.json",
]
INVALID_FIXTURES = [
    CONTRACTS / "fixtures/invalid/scenario_phase6_direct_service_url.invalid.json",
    CONTRACTS / "fixtures/invalid/scenario_phase6_private_storage_read.invalid.json",
    CONTRACTS / "fixtures/invalid/scenario_phase6_unsafe_helper.invalid.json",
    CONTRACTS / "fixtures/invalid/scenario_phase6_cli_failure.invalid.json",
    CONTRACTS / "fixtures/invalid/scenario_phase6_assertion_failure.invalid.json",
]

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
            prefix = lowered[max(0, index - 56) : index]
            if "must not" not in prefix and "not as" not in prefix:
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
        HARNESS / "src/step_runners.rs",
        HARNESS / "src/runner.rs",
        HARNESS / "src/manifests.rs",
        HARNESS / "src/assertions.rs",
        HARNESS / "src/lib.rs",
        CONTRACTS / "codegen_manifest.json",
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
        "Phase 6: Scenario Step Runners, Black-Box Paths, And Fail-Closed Results",
        "Implement CLI step runner",
        "Implement SDK and API step runners",
        "Implement local helper step runner",
        "Implement result and reason-code assertions",
        "Implement blocked versus failed classification",
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
        "Integration Test Harness Phase 6 Implementation Plan",
        "packages/integration_harness/src/step_runners.rs",
        "scripts/validate_integration_harness_phase6.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 6 Progress",
        "Validation Evidence",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(
        wrapper,
        "scripts/validate_integration_harness_phase6.py",
        VALIDATION_WRAPPER,
    )


def validate_codegen_manifest() -> None:
    manifest = load_json(CONTRACTS / "codegen_manifest.json")
    phase6 = manifest.get("integration_harness_phase6")
    assert_true(isinstance(phase6, dict), "codegen manifest missing integration_harness_phase6")
    assert_true(phase6.get("schema_family") == "integration-harness", "wrong Phase 6 schema family")
    assert_true(phase6.get("schema_version") == "integration-harness.v0.1", "wrong Phase 6 schema version")
    assert_true(phase6.get("rust_first_validation") is True, "Phase 6 must remain Rust-first")
    listed_valid = [Path(value) for value in phase6.get("fixtures", {}).get("valid", [])]
    listed_invalid = [Path(value) for value in phase6.get("fixtures", {}).get("invalid", [])]
    assert_true(listed_valid == VALID_FIXTURES, "Phase 6 valid fixture list drifted")
    assert_true(listed_invalid == INVALID_FIXTURES, "Phase 6 invalid fixture list drifted")


def validate_fixture(path: Path, expected_scenario: str, expected_status: str) -> None:
    document = load_json(path)
    fixture = document["fixture_manifest"]
    scenario = document["scenario_manifest"]
    run_record = document["test_run_record"]
    bundle = document["artifact_bundle"]

    assert_true(fixture["test_only"] is True, f"{path} fixture must be test_only")
    assert_true(
        all(key["test_only"] is True and key["raw_key_material_present"] is False for key in fixture["keys"]),
        f"{path} fixture keys must be test-only refs",
    )
    assert_true(scenario["scenario_id"] == expected_scenario, f"{path} scenario id drifted")
    assert_true(run_record["status"] == expected_status, f"{path} run status drifted")
    assert_true(bundle["contains_raw_secret"] is False, f"{path} bundle must reject raw secrets")
    assert_true(bundle["contains_private_key"] is False, f"{path} bundle must reject private keys")
    assert_true(bundle["contains_token"] is False, f"{path} bundle must reject tokens")
    assert_true(bundle["contains_private_payload"] is False, f"{path} bundle must reject private payloads")


def validate_fixtures() -> None:
    validate_fixture(
        VALID_FIXTURES[0],
        "scenario_phase6_step_runners",
        "passed",
    )
    validate_fixture(
        VALID_FIXTURES[1],
        "scenario_phase6_blocked_direct_service",
        "blocked",
    )
    validate_fixture(
        VALID_FIXTURES[2],
        "scenario_phase6_failed_invalid_signature",
        "failed",
    )

    passed_steps = load_json(VALID_FIXTURES[0])["scenario_manifest"]["steps"]
    action_kinds = [step["action_kind"] for step in passed_steps]
    assert_true(
        action_kinds == ["cli", "sdk", "api", "local_helper", "assertion"],
        "Phase 6 passed fixture must cover all step runner kinds in order",
    )

    invalid_expectations = {
        "scenario_phase6_direct_service_url.invalid.json": (
            "safety.direct_service_url",
            "service_url:overgate-admin",
        ),
        "scenario_phase6_private_storage_read.invalid.json": (
            "safety.private_storage_read",
            "storage:private:read",
        ),
        "scenario_phase6_unsafe_helper.invalid.json": (
            "safety.local_helper_not_allowed",
            "helper:production_mutation",
        ),
        "scenario_phase6_cli_failure.invalid.json": (
            "signature.invalid",
            "command:invalid_signature",
        ),
        "scenario_phase6_assertion_failure.invalid.json": (
            "assertion.reason_code_mismatch",
            "assertion:wrong_reason_code",
        ),
    }
    for path in INVALID_FIXTURES:
        text = read(path)
        reason, marker = invalid_expectations[path.name]
        assert_contains(text, reason, path)
        assert_contains(text, marker, path)


def validate_rust_surfaces() -> None:
    lib_rs = read(HARNESS / "src/lib.rs")
    runner_rs = read(HARNESS / "src/runner.rs")
    manifests_rs = read(HARNESS / "src/manifests.rs")
    assertions_rs = read(HARNESS / "src/assertions.rs")
    step_runners_rs = read(HARNESS / "src/step_runners.rs")

    for expected in [
        "pub mod step_runners;",
        "ScenarioStepExecutionContext",
        "ScenarioStepExecutionReport",
        "ScenarioStepRunner",
    ]:
        assert_contains(lib_rs, expected, HARNESS / "src/lib.rs")

    for expected in [
        "run_cli_step",
        "run_sdk_or_api_step",
        "run_local_helper_step",
        "run_assertion_step",
        "classify_step_results",
        "safety.direct_service_url",
        "safety.private_storage_read",
        "safety.local_helper_not_allowed",
        "dependency.phase_contract_not_ready",
        "signature.invalid",
        "assertion.reason_code_mismatch",
        ":redacted",
    ]:
        assert_contains(step_runners_rs, expected, HARNESS / "src/step_runners.rs")

    for expected in [
        "extract_scenario_steps",
        "expected_result_class",
        "ScenarioStepRef::to_contract_step",
        "step.validate()",
    ]:
        assert_contains(manifests_rs, expected, HARNESS / "src/manifests.rs")

    for expected in [
        "assert_reason_code",
        "assert_terminal_status",
        "$.reason_code",
        "$.status",
    ]:
        assert_contains(assertions_rs, expected, HARNESS / "src/assertions.rs")

    for expected in [
        "ScenarioStepRunner::new().run_scenario",
        "step_results_json",
        "step_results:",
        "assertion_refs:",
        "ArtifactRetentionClass::FailureEvidence",
        "scenario_phase6_step_runners",
        "scenario_phase6_blocked_direct_service",
        "scenario_phase6_failed_invalid_signature",
        "running to asserting is valid",
    ]:
        assert_contains(runner_rs, expected, HARNESS / "src/runner.rs")

    assert_no_forbidden_runtime_authority(step_runners_rs, HARNESS / "src/step_runners.rs")
    assert_no_forbidden_runtime_authority(runner_rs, HARNESS / "src/runner.rs")


def validate_cli_behavior() -> None:
    passed = run_cli(["test", "scenario", "scenario_phase6_step_runners", "--json"])
    assert_true(passed["ok"] is True, "Phase 6 passed scenario should be ok")
    passed_result = passed["result"]
    assert_true(passed_result["status"] == "passed", "Phase 6 passed scenario status drifted")
    assert_true(passed_result["reason_code"] == "run.passed", "Phase 6 passed reason drifted")
    assert_true(len(passed_result["step_results"]) == 5, "Phase 6 passed scenario must emit five steps")
    assert_true(
        [step["action_kind"] for step in passed_result["step_results"]]
        == ["cli", "sdk", "api", "local_helper", "assertion"],
        "Phase 6 CLI output step order drifted",
    )
    assert_true("asserting" in passed_result["lifecycle"], "Passed step scenario must include asserting")
    assert_true(
        any(step["stdout_ref"] and step["stdout_ref"].endswith(":redacted") for step in passed_result["step_results"]),
        "CLI step must emit redacted stdout ref",
    )
    assert_true(
        any(step["payload_ref"] and ":sdk_payload:" in step["payload_ref"] for step in passed_result["step_results"]),
        "SDK step must emit redacted payload ref",
    )
    assert_true(
        any(step["payload_ref"] and ":api_payload:" in step["payload_ref"] for step in passed_result["step_results"]),
        "API step must emit redacted payload ref",
    )

    blocked = run_cli(
        ["test", "scenario", "scenario_phase6_blocked_direct_service", "--json"],
        expected_exit=3,
    )
    assert_true(blocked["ok"] is False, "Blocked Phase 6 scenario should be ok=false")
    assert_true(blocked["reason_code"] == "safety.direct_service_url", "Blocked reason drifted")
    blocked_result = blocked["result"]
    assert_true(blocked_result["status"] == "blocked", "Blocked result status drifted")
    assert_true("running" in blocked_result["lifecycle"], "Blocked step run must include running")
    assert_true("asserting" in blocked_result["lifecycle"], "Blocked step run must include asserting")
    assert_true(
        any(artifact["retention_class"] == "failure_evidence" for artifact in blocked_result["artifacts"]),
        "Blocked step run must retain failure evidence",
    )

    failed = run_cli(
        ["test", "scenario", "scenario_phase6_failed_invalid_signature", "--json"],
        expected_exit=11,
    )
    assert_true(failed["ok"] is False, "Failed Phase 6 scenario should be ok=false")
    assert_true(failed["reason_code"] == "signature.invalid", "Failed reason drifted")
    failed_result = failed["result"]
    assert_true(failed_result["status"] == "failed", "Failed result status drifted")
    assert_true("running" in failed_result["lifecycle"], "Failed step run must include running")
    assert_true("asserting" in failed_result["lifecycle"], "Failed step run must include asserting")
    assert_true(
        any(artifact["retention_class"] == "failure_evidence" for artifact in failed_result["artifacts"]),
        "Failed step run must retain failure evidence",
    )


def validate_cargo_checks() -> None:
    run(["cargo", "check", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_docs()
        validate_codegen_manifest()
        validate_fixtures()
        validate_rust_surfaces()
        validate_cli_behavior()
        validate_cargo_checks()
    except AssertionError as exc:
        print(f"Integration harness Phase 6 validation failed: {exc}", file=sys.stderr)
        return 1

    print("Integration harness Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
