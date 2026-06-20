#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 10 self-check and handoff gates."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CONTRACTS = Path("packages/schemas/overrid_contracts")
HARNESS = Path("packages/integration_harness")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_10_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")
VALID_FIXTURE = (
    CONTRACTS / "fixtures/valid/integration_harness_phase10_handoff.valid.json"
)

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
    "pricing",
    "revenue",
    "customer-count",
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
        search_from = 0
        while True:
            index = lowered.find(term, search_from)
            if index == -1:
                break
            line_start = lowered.rfind("\n", 0, index) + 1
            prefix = lowered[line_start:index]
            allowed_prefixes = (
                "do not add",
                "must not",
                "not as",
                "without adding",
                "no ",
                "denied_terms",
            )
            if not any(marker in prefix for marker in allowed_prefixes):
                raise AssertionError(
                    f"{source} appears to introduce forbidden runtime authority: {term}"
                )
            search_from = index + len(term)


def run(
    command: list[str],
    *,
    expected_exit: int | None = 0,
    cwd: Path = REPO_ROOT,
) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, capture_output=True)
    if expected_exit is not None and result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli(args: list[str], *, expected_exit: int | None = 0) -> dict[str, Any]:
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


def result_payload(envelope: dict[str, Any]) -> dict[str, Any]:
    result = envelope.get("result")
    assert_true(isinstance(result, dict), "CLI envelope result must be an object")
    return result


def coverage_for(result: dict[str, Any], service_id: str) -> dict[str, Any]:
    for report in result.get("coverage_report", []):
        if report.get("service_id") == service_id:
            return report
    raise AssertionError(f"missing coverage report for {service_id}")


def validate_required_files() -> None:
    for path in [
        HARNESS / "src/handoff.rs",
        HARNESS / "src/lib.rs",
        HARNESS / "src/runner.rs",
        HARNESS / "src/manifests.rs",
        HARNESS / "src/artifacts.rs",
        HARNESS / "src/assertions.rs",
        HARNESS / "src/step_runners.rs",
        CONTRACTS / "codegen_manifest.json",
        VALID_FIXTURE,
        SUB_PLAN,
        TECH_STACK,
        PHASE_PLAN,
        PHASE_PROGRESS,
        VALIDATION_WRAPPER,
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"Missing required file: {path}")


def validate_docs() -> None:
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "Phase 10: Harness Validation, Documentation, And Downstream Handoff",
        "Validate harness self-consistency",
        "Validate alignment with tech-stack choices",
        "Validate security and redaction",
        "Validate documentation links and progress evidence",
        "Prepare downstream service handoff",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "Rust-first infrastructure stack",
        "JSON plus JSON Schema",
        "not as the core runtime",
        "Explicit Non-Choices",
        "It must not become the product boundary for core Overrid primitives.",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)

    for expected in [
        "Integration Test Harness Phase 10 Implementation Plan",
        "packages/integration_harness/src/handoff.rs",
        "integration_harness_phase10_handoff.valid.json",
        "scripts/validate_integration_harness_phase10.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 10 Progress",
        "Validation Evidence",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(
        wrapper,
        "scripts/validate_integration_harness_phase10.py",
        VALIDATION_WRAPPER,
    )
    assert_no_forbidden_runtime_authority(phase_plan, PHASE_PLAN)
    assert_no_forbidden_runtime_authority(phase_progress, PHASE_PROGRESS)


def validate_handoff_rust_surface() -> None:
    handoff = read(HARNESS / "src/handoff.rs")
    lib = read(HARNESS / "src/lib.rs")
    for expected in [
        "phase10_harness_validation_report",
        "HarnessPhase10Report",
        "TechStackAlignmentCheck",
        "SecurityRedactionCheck",
        "DownstreamHandoffRule",
        "PHASE10_SCHEMA_VERSION",
        "10.1",
        "10.2",
        "10.3",
        "10.4",
        "10.5",
    ]:
        assert_contains(handoff, expected, HARNESS / "src/handoff.rs")
    for expected in [
        "pub mod handoff;",
        "phase10_harness_validation_report",
    ]:
        assert_contains(lib, expected, HARNESS / "src/lib.rs")

    for term in FORBIDDEN_RUNTIME_TERMS:
        assert_contains(handoff.lower(), term, HARNESS / "src/handoff.rs")


def validate_codegen_manifest() -> None:
    manifest = load_json(CONTRACTS / "codegen_manifest.json")
    phase10 = manifest.get("integration_harness_phase10")
    assert_true(isinstance(phase10, dict), "codegen manifest missing integration_harness_phase10")
    assert_true(phase10.get("schema_family") == "integration-harness", "wrong Phase 10 schema family")
    assert_true(
        phase10.get("schema_version") == "integration-harness.v0.1",
        "wrong Phase 10 schema version",
    )
    assert_true(
        phase10.get("canonical_schema")
        == "packages/schemas/overrid_contracts/v0/integration_harness.schema.json",
        "wrong Phase 10 canonical schema",
    )
    assert_true(phase10.get("rust_first_validation") is True, "Phase 10 must remain Rust-first")
    listed_valid = [Path(value) for value in phase10.get("fixtures", {}).get("valid", [])]
    assert_true(listed_valid == [VALID_FIXTURE], "Phase 10 valid fixture list drifted")
    listed_invalid = [Path(value) for value in phase10.get("fixtures", {}).get("invalid", [])]
    for expected in [
        CONTRACTS / "fixtures/invalid/artifact_raw_secret.invalid.json",
        CONTRACTS / "fixtures/invalid/artifact_sensitive_material.invalid.json",
        CONTRACTS / "fixtures/invalid/scenario_unsupported_phase.invalid.json",
        CONTRACTS / "fixtures/invalid/scenario_phase6_direct_service_url.invalid.json",
        CONTRACTS / "fixtures/invalid/scenario_phase6_private_storage_read.invalid.json",
    ]:
        assert_true(expected in listed_invalid, f"missing Phase 10 invalid fixture: {expected}")
    guardrails = " ".join(phase10.get("guardrails", []))
    for expected in [
        "Harness self-validation",
        "Tech-stack alignment",
        "Security and redaction checks",
        "Docs and progress evidence",
        "Downstream handoff",
    ]:
        assert_contains(guardrails, expected, CONTRACTS / "codegen_manifest.json")


def validate_phase10_fixture() -> None:
    fixture = load_json(VALID_FIXTURE)
    scenario = fixture["scenario_manifest"]
    run_record = fixture["test_run_record"]
    artifact = fixture["artifact_bundle"]
    redaction = artifact["redaction_summary"]

    assert_true(
        scenario["scenario_id"] == "scenario_phase10_harness_validation_handoff",
        "Phase 10 scenario id drifted",
    )
    assert_true(scenario["master_phase"] == 10, "Phase 10 scenario must belong to master phase 10")
    assert_true(scenario["gate_class"] == "regression", "Phase 10 scenario must use mandatory regression gate")
    for expected in [
        "harness_self_validation",
        "tech_stack_alignment",
        "security_redaction",
        "downstream_handoff",
        "service_contract_coverage",
    ]:
        assert_true(expected in scenario["tags"], f"Phase 10 scenario missing tag: {expected}")
    assert_true(
        scenario["required_services"]
        == ["service:local_stack", "service:overgate", "service:overwatch"],
        "Phase 10 scenario must stay on local Overrid-shaped services",
    )
    assert_true(run_record["status"] == "passed", "Phase 10 run record must pass")
    assert_true(redaction["scanner_passed"] is True, "Phase 10 artifact scanner must pass")
    for field in [
        "contains_raw_secret",
        "contains_private_key",
        "contains_token",
        "contains_private_payload",
        "contains_fixture_key_material",
    ]:
        assert_true(redaction[field] is False, f"Phase 10 redaction flag must stay false: {field}")


def validate_cli_phase10_handoff() -> None:
    envelope = run_cli(
        [
            "test",
            "list",
            "--phase",
            "10",
            "--tag",
            "downstream_handoff",
            "--json",
        ]
    )
    assert_true(envelope["ok"] is True, "phase 10 handoff list should be successful")
    result = result_payload(envelope)
    assert_true(result["phase_filter"] == 10, "phase 10 filter missing from result")
    assert_true(
        [scenario["scenario_id"] for scenario in result["scenarios"]]
        == ["scenario_phase10_harness_validation_handoff"],
        "Phase 10 handoff list should select only the handoff scenario",
    )
    report = result["selection_report"]
    assert_true(report["requested_phase"] == 10, "selection report must retain requested phase")
    assert_true(
        "scenario_phase10_harness_validation_handoff" in report["selected_scenario_ids"],
        "selection report missing Phase 10 scenario",
    )
    assert_true("regression" in report["selected_gate_classes"], "Phase 10 regression gate not selected")
    assert_true(
        coverage_for(result, "service:overgate")["status"] == "covered",
        "overgate coverage should be covered by Phase 10 handoff scenario",
    )
    assert_true(
        coverage_for(result, "service:public_interest_pool")["status"]
        == "missing_required_contract",
        "public-interest pool should remain missing-required-contract evidence",
    )

    scenario_run = run_cli(
        [
            "test",
            "scenario",
            "scenario_phase10_harness_validation_handoff",
            "--phase",
            "10",
            "--json",
        ]
    )
    run_result = result_payload(scenario_run)
    assert_true(scenario_run["ok"] is True, "explicit Phase 10 scenario should pass")
    assert_true(run_result["status"] == "passed", "explicit Phase 10 scenario status must pass")
    assert_true(run_result["artifacts"], "explicit Phase 10 scenario must export artifacts")
    assert_true(
        run_result["artifacts"][0]["redaction_report"]["scanner_passed"] is True,
        "explicit Phase 10 scenario artifacts must pass redaction",
    )

    blocked = run_cli(
        [
            "test",
            "scenario",
            "scenario_phase10_harness_validation_handoff",
            "--profile",
            "public",
            "--json",
        ],
        expected_exit=None,
    )
    blocked_result = result_payload(blocked)
    assert_true(blocked["ok"] is False, "non-local profile Phase 10 scenario must not be ok")
    assert_true(blocked_result["status"] == "blocked", "non-local profile must be blocked")
    assert_true(
        blocked_result["reason_code"] == "safety.non_local_profile",
        "non-local profile reason code drifted",
    )


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-integration-harness", "phase10"])
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    validate_required_files()
    validate_docs()
    validate_handoff_rust_surface()
    validate_codegen_manifest()
    validate_phase10_fixture()
    validate_rust_tests()
    validate_cli_phase10_handoff()
    print("Integration Test Harness Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
