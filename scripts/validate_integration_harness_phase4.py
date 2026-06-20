#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 4 local-stack lifecycle hooks."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

HARNESS = Path("packages/integration_harness")
CLI = Path("packages/cli")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_04_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

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


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} missing expected text: {expected}")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_runtime_authority(text: str, source: Path) -> None:
    lowered = text.lower()
    for term in FORBIDDEN_RUNTIME_TERMS:
        if term in lowered:
            index = lowered.find(term)
            prefix = lowered[max(0, index - 48) : index]
            if "must not" not in prefix and "not as" not in prefix:
                raise AssertionError(f"{source} appears to introduce forbidden runtime authority: {term}")


def run(command: list[str], cwd: Path = REPO_ROOT, expected_exit: int = 0) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, capture_output=True)
    if result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli(args: list[str], expected_exit: int = 0) -> dict[str, Any]:
    result = run(["cargo", "run", "-q", "-p", "overrid-cli", "--", *args], expected_exit=expected_exit)
    if result.stderr:
        raise AssertionError(f"CLI emitted stderr unexpectedly: {' '.join(args)}\n{result.stderr}")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}:\n{result.stdout}") from exc


def validate_required_files() -> None:
    for path in [
        HARNESS / "src/local_stack.rs",
        HARNESS / "src/runner.rs",
        HARNESS / "src/lib.rs",
        CLI / "src/runner.rs",
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
        "Phase 4: Local Stack Lifecycle, Reset, Seed, And Health Integration",
        "harness.start_stack(profile)",
        "harness.reset_stack(profile)",
        "harness.seed(fixtures)",
        "diagnostics",
        "Phase 0 smoke",
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
        "Integration Test Harness Phase 4 Implementation Plan",
        "deterministic Rust harness hooks",
        "scripts/validate_integration_harness_phase4.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 4 Progress",
        "Validation Evidence",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(wrapper, "scripts/validate_integration_harness_phase4.py", VALIDATION_WRAPPER)


def validate_local_stack_code() -> None:
    lib_rs = read(HARNESS / "src/lib.rs")
    runner_rs = read(HARNESS / "src/runner.rs")
    local_stack_rs = read(HARNESS / "src/local_stack.rs")
    cli_runner = read(CLI / "src/runner.rs")

    for expected in [
        "pub mod local_stack;",
        "LocalStackHarness",
        "LocalStackReport",
        "ServiceHealthSummary",
        "LOCAL_TEST_STATE_MARKER",
    ]:
        assert_contains(lib_rs, expected, HARNESS / "src/lib.rs")

    for expected in [
        "READY_STACK_COMPONENTS",
        "pub fn start_stack",
        "pub fn reset_stack",
        "pub fn run_phase0_smoke",
        "pub fn diagnostics",
        "service:local_stack",
        "service:overgate",
        "service:overwatch",
        "component:overqueue_durable_state",
        "component:event_log",
        "component:object_artifact_stub",
        "component:node_agent_simulator",
        "ArtifactRetentionClass::PhaseGateEvidence",
        "ArtifactRetentionClass::FailureEvidence",
        "dependency.service_unavailable",
    ]:
        assert_contains(local_stack_rs, expected, HARNESS / "src/local_stack.rs")

    for forbidden in ["std::process", "TcpStream", "Command::new", "tokio", "axum"]:
        assert_true(forbidden not in local_stack_rs, f"local stack facade must stay deterministic: {forbidden}")
    assert_no_forbidden_runtime_authority(local_stack_rs, HARNESS / "src/local_stack.rs")

    for expected in [
        "LocalStackHarness::new",
        "local_stack_lifecycle",
        "service_health",
        "reset_refs",
        "seed_refs",
        "diagnostic_refs",
        "smoke_refs",
        "safety.non_local_profile",
    ]:
        assert_contains(runner_rs, expected, HARNESS / "src/runner.rs")

    for expected in [
        "test_integration_runs_phase0_smoke_with_local_stack_health",
        "test_reset_runs_local_stack_reset_and_seed",
        "test_reset_rejects_non_local_profile",
    ]:
        assert_contains(cli_runner, expected, CLI / "src/runner.rs")


def validate_result_shape(result: dict[str, Any], command: str, expected_status: str) -> None:
    assert_true(result["schema_version"] == "integration-harness.v0.1", "wrong harness schema version")
    assert_true(result["command"] == command, f"wrong command for {command}")
    assert_true(result["status"] == expected_status, f"wrong status for {command}")
    assert_true("stack_ready" in result["lifecycle"], f"{command} did not report stack readiness")
    assert_true("resetting" in result["lifecycle"], f"{command} did not report reset")
    assert_true("seeding" in result["lifecycle"], f"{command} did not report seed")
    assert_true("collecting_artifacts" in result["lifecycle"], f"{command} did not collect artifacts")
    assert_true(result["service_health"], f"{command} missing service health")
    assert_true(result["reset_refs"], f"{command} missing reset refs")
    assert_true(result["seed_refs"], f"{command} missing seed refs")
    assert_true(result["diagnostic_refs"], f"{command} missing diagnostic refs")
    assert_true(result["artifacts"], f"{command} missing artifacts")


def validate_cli_behavior() -> None:
    integration = run_cli(["test", "integration", "--json"])
    assert_true(integration["ok"] is True, "integration should be ok")
    integration_result = integration["result"]
    validate_result_shape(integration_result, "test integration", "passed")
    assert_true(integration_result["reason_code"] == "run.passed", "wrong integration reason")
    assert_true(integration_result["lifecycle"][-1] == "passed", "integration did not end passed")
    assert_true(integration_result["smoke_refs"], "integration missing smoke refs")
    assert_true(
        any(
            health["service_id"] == "service:local_stack" and health["state"] == "ready"
            for health in integration_result["service_health"]
        ),
        "integration missing ready local stack health",
    )
    assert_true(
        any(artifact["retention_class"] == "phase_gate_evidence" for artifact in integration_result["artifacts"]),
        "integration should retain phase gate evidence",
    )

    smoke = run_cli(["test", "scenario", "scenario_phase0_smoke", "--json"])
    assert_true(smoke["ok"] is True, "phase0 smoke scenario should be ok")
    validate_result_shape(smoke["result"], "test scenario", "passed")
    assert_true(smoke["result"]["smoke_refs"], "phase0 smoke scenario missing smoke refs")

    reset = run_cli(["test", "reset", "--json"])
    assert_true(reset["ok"] is True, "reset should be ok")
    validate_result_shape(reset["result"], "test reset", "passed")
    assert_true(reset["result"]["smoke_refs"] == [], "reset should not report smoke refs")

    blocked = run_cli(["test", "scenario", "scenario_blocked_dependency", "--json"], expected_exit=3)
    assert_true(blocked["ok"] is False, "blocked dependency should be ok=false")
    assert_true(blocked["reason_code"] == "dependency.service_unavailable", "wrong blocked reason")
    blocked_result = blocked["result"]
    validate_result_shape(blocked_result, "test scenario", "blocked")
    assert_true(blocked_result["lifecycle"][-1] == "blocked", "blocked dependency did not end blocked")
    assert_true(
        "service:overqueue:unavailable" in blocked_result["dependency_status"],
        "blocked dependency missing service:overqueue unavailable status",
    )
    assert_true(
        any(artifact["retention_class"] == "failure_evidence" for artifact in blocked_result["artifacts"]),
        "blocked dependency should retain failure evidence",
    )

    unsafe_reset = run_cli(
        ["test", "reset", "--profile", "production_like", "--json"],
        expected_exit=3,
    )
    assert_true(unsafe_reset["reason_code"] == "safety.non_local_profile", "unsafe reset reason drifted")
    assert_true(unsafe_reset["result"]["status"] == "blocked", "unsafe reset must be blocked")


def validate_cargo_checks() -> None:
    run(["cargo", "check", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    validate_required_files()
    validate_docs()
    validate_local_stack_code()
    validate_cli_behavior()
    validate_cargo_checks()
    print("Integration Test Harness Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
