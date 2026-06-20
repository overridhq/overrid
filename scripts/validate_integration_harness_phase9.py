#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 9 gates and CI selection."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CONTRACTS = Path("packages/schemas/overrid_contracts")
HARNESS = Path("packages/integration_harness")
CLI = Path("packages/cli")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_09_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    CONTRACTS / "fixtures/valid/integration_harness_phase9_control_plane_spine.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase9_ci_blocked.valid.json",
]
INVALID_FIXTURES = [
    CONTRACTS / "fixtures/invalid/scenario_unsupported_phase.invalid.json",
    CONTRACTS / "fixtures/invalid/scenario_missing_assertion.invalid.json",
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
            prefix = lowered[max(0, index - 100) : index]
            allowed_prefixes = ("do not add", "must not", "not as", "without adding")
            if not any(marker in prefix for marker in allowed_prefixes):
                raise AssertionError(
                    f"{source} appears to introduce forbidden runtime authority: {term}"
                )


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


def validate_required_files() -> None:
    for path in [
        HARNESS / "src/phase_gate.rs",
        HARNESS / "src/manifests.rs",
        HARNESS / "src/runner.rs",
        CLI / "src/parser.rs",
        CLI / "src/runner.rs",
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
        "Phase 9: Phase Gates, CI Selection, And Later-Phase Expansion",
        "Implement phase and tag selection",
        "Implement CI smoke suite",
        "Implement service contract coverage reporting",
        "Implement product integration gates",
        "Implement governance and scale hardening gates",
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
        "Integration Test Harness Phase 9 Implementation Plan",
        "packages/integration_harness",
        "packages/cli",
        "phase, service, tag, changed path, required dependency, gate class, and scenario-name filters",
        "scripts/validate_integration_harness_phase9.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 9 Progress",
        "Validation Evidence",
        "Registered Phase 9 integration-harness fixtures",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(
        wrapper,
        "scripts/validate_integration_harness_phase9.py",
        VALIDATION_WRAPPER,
    )
    assert_no_forbidden_runtime_authority(phase_plan, PHASE_PLAN)
    assert_no_forbidden_runtime_authority(phase_progress, PHASE_PROGRESS)


def validate_codegen_manifest() -> None:
    manifest = load_json(CONTRACTS / "codegen_manifest.json")
    phase9 = manifest.get("integration_harness_phase9")
    assert_true(isinstance(phase9, dict), "codegen manifest missing integration_harness_phase9")
    assert_true(phase9.get("schema_family") == "integration-harness", "wrong Phase 9 schema family")
    assert_true(
        phase9.get("schema_version") == "integration-harness.v0.1",
        "wrong Phase 9 schema version",
    )
    assert_true(
        phase9.get("canonical_schema")
        == "packages/schemas/overrid_contracts/v0/integration_harness.schema.json",
        "wrong Phase 9 canonical schema",
    )
    assert_true(phase9.get("rust_first_validation") is True, "Phase 9 must remain Rust-first")
    listed_valid = [Path(value) for value in phase9.get("fixtures", {}).get("valid", [])]
    listed_invalid = [Path(value) for value in phase9.get("fixtures", {}).get("invalid", [])]
    assert_true(listed_valid == VALID_FIXTURES, "Phase 9 valid fixture list drifted")
    assert_true(listed_invalid == INVALID_FIXTURES, "Phase 9 invalid fixture list drifted")
    guardrails = " ".join(phase9.get("guardrails", []))
    for expected in [
        "Phase and tag selection",
        "CI smoke selection",
        "Service contract coverage",
        "Product integration gates",
        "Later deployment",
    ]:
        assert_contains(guardrails, expected, CONTRACTS / "codegen_manifest.json")


def validate_phase9_fixtures() -> None:
    control_plane = load_json(VALID_FIXTURES[0])
    ci_blocked = load_json(VALID_FIXTURES[1])

    assert_true(
        control_plane["scenario_manifest"]["scenario_id"]
        == "scenario_phase1_control_plane_spine",
        "control-plane fixture scenario id drifted",
    )
    assert_true(
        control_plane["scenario_manifest"]["gate_class"] == "contract_spine",
        "control-plane fixture must be mandatory contract spine",
    )
    assert_true(
        "ci_smoke" in control_plane["scenario_manifest"]["tags"],
        "control-plane fixture must be selectable as CI smoke",
    )
    assert_true(
        ci_blocked["scenario_manifest"]["scenario_id"] == "scenario_phase9_ci_blocked",
        "blocked CI fixture scenario id drifted",
    )
    assert_true(
        ci_blocked["scenario_manifest"]["gate_class"] == "extended",
        "blocked CI fixture must remain planned/extended",
    )
    assert_true(
        ci_blocked["test_run_record"]["status"] == "blocked",
        "blocked CI fixture run status drifted",
    )


def result_payload(envelope: dict[str, Any]) -> dict[str, Any]:
    result = envelope.get("result")
    assert_true(isinstance(result, dict), "CLI envelope result must be an object")
    return result


def coverage_for(result: dict[str, Any], service_id: str) -> dict[str, Any]:
    for report in result.get("coverage_report", []):
        if report.get("service_id") == service_id:
            return report
    raise AssertionError(f"missing coverage report for {service_id}")


def validate_cli_phase9_selection() -> None:
    envelope = run_cli(
        [
            "test",
            "list",
            "--phase",
            "9",
            "--service",
            "service:overgate",
            "--tag",
            "control_plane_spine",
            "--changed-path",
            "services/overgate/routes.rs",
            "--required-dependency",
            "fixture:phase9_control_plane_spine",
            "--gate-class",
            "contract_spine",
            "--scenario-name",
            "scenario_phase1_control_plane_spine",
            "--json",
        ]
    )
    assert_true(envelope["ok"] is True, "filtered phase 9 list should be successful")
    result = result_payload(envelope)
    assert_true(result["phase_filter"] == 9, "phase 9 filter missing from result")
    assert_true(
        [scenario["scenario_id"] for scenario in result["scenarios"]]
        == ["scenario_phase1_control_plane_spine"],
        "filtered phase 9 list should select only control-plane spine scenario",
    )
    filters = set(result["selection_report"]["filters"])
    for expected in [
        "phase:9",
        "service:service:overgate",
        "tag:control_plane_spine",
        "changed_path:services/overgate/routes.rs",
        "required_dependency:fixture:phase9_control_plane_spine",
        "gate_class:contract_spine",
        "scenario_name:scenario_phase1_control_plane_spine",
    ]:
        assert_true(expected in filters, f"missing Phase 9 filter summary: {expected}")


def validate_cli_phase9_ci_and_coverage() -> None:
    envelope = run_cli(["test", "integration", "--phase", "9", "--json"])
    assert_true(envelope["ok"] is True, "phase 9 CI smoke should pass")
    result = result_payload(envelope)
    assert_true(result["status"] == "passed", "phase 9 CI smoke status must pass")
    assert_true(
        result["selection_report"]["ci_entrypoint"] == "scenario_phase1_control_plane_spine",
        "phase 9 CI must prefer explicit ci_smoke scenario",
    )
    assert_true(
        result["scenarios"][0]["scenario_id"] == "scenario_phase1_control_plane_spine",
        "phase 9 CI must run bounded control-plane smoke",
    )
    assert_true(
        "scenario_phase9_ci_blocked" in result["selection_report"]["planned_scenario_ids"],
        "extended blocked scenario should remain planned",
    )
    assert_true(
        coverage_for(result, "service:overgate")["status"] == "covered",
        "overgate coverage should be covered by control-plane spine",
    )
    assert_true(
        coverage_for(result, "service:deployment_planner")["status"]
        == "missing_required_contract",
        "phase 9 deployment planner should remain missing-required-contract evidence",
    )
    assert_true(
        "flake.unstable_event_ordering" not in json.dumps(envelope, sort_keys=True),
        "phase 9 CI smoke must not pick the Phase 8 flake scenario",
    )


def validate_cli_planned_and_terminal_classes() -> None:
    envelope = run_cli(
        [
            "test",
            "list",
            "--phase",
            "9",
            "--gate-class",
            "extended",
            "--tag",
            "blocked_outcome",
            "--json",
        ]
    )
    result = result_payload(envelope)
    assert_true(
        [scenario["scenario_id"] for scenario in result["scenarios"]]
        == ["scenario_phase9_ci_blocked"],
        "extended blocked gate should be listed as planned evidence",
    )
    assert_true(result["selection_report"]["selected_count"] == 0, "extended gate selected unexpectedly")
    assert_true(result["selection_report"]["planned_count"] == 1, "extended gate should be planned")
    assert_true(
        result["selection_report"]["planned_scenario_ids"] == ["scenario_phase9_ci_blocked"],
        "planned Phase 9 extended scenario drifted",
    )

    blocked = run_cli(
        ["test", "scenario", "scenario_phase9_ci_blocked", "--json"],
        expected_exit=None,
    )
    blocked_result = result_payload(blocked)
    assert_true(blocked["ok"] is False, "explicit blocked scenario must not be ok")
    assert_true(blocked_result["status"] == "blocked", "blocked scenario status drifted")
    assert_true(
        blocked_result["reason_class"] == "dependency",
        "blocked scenario reason class drifted",
    )

    failed = run_cli(
        ["test", "scenario", "scenario_phase8_flake_detection", "--json"],
        expected_exit=None,
    )
    failed_result = result_payload(failed)
    assert_true(failed["ok"] is False, "explicit failed scenario must not be ok")
    assert_true(failed_result["status"] == "failed", "failed scenario status drifted")
    assert_true(
        failed_result["reason_code"] == "flake.unstable_event_ordering",
        "failed scenario reason code drifted",
    )


def validate_rust_tests() -> None:
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    validate_required_files()
    validate_docs()
    validate_codegen_manifest()
    validate_phase9_fixtures()
    validate_rust_tests()
    validate_cli_phase9_selection()
    validate_cli_phase9_ci_and_coverage()
    validate_cli_planned_and_terminal_classes()
    print("Integration Test Harness Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
