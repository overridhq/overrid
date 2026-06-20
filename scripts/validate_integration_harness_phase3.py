#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 3 runner and CLI shell."""

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
SDS = Path("docs/sds/foundation/integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_03_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

HARNESS_MODULES = {
    "src/lib.rs",
    "src/runner.rs",
    "src/manifests.rs",
    "src/fixtures.rs",
    "src/assertions.rs",
    "src/artifacts.rs",
    "src/phase_gate.rs",
}
TEST_COMMANDS = {
    "Integration",
    "Scenario",
    "List",
    "Reset",
    "Artifacts",
}
LIFECYCLE_STATES = {
    "Planned",
    "StackStarting",
    "StackReady",
    "Resetting",
    "Seeding",
    "Running",
    "Asserting",
    "CollectingArtifacts",
    "Passed",
    "Failed",
    "Blocked",
}
FORBIDDEN_STACK_TERMS = {
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
    for term in FORBIDDEN_STACK_TERMS:
        if term in lowered and "must not" not in lowered[max(0, lowered.find(term) - 40) : lowered.find(term)]:
            raise AssertionError(f"{source} appears to introduce forbidden runtime authority: {term}")


def run(command: list[str], cwd: Path = REPO_ROOT, expected_exit: int = 0) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, capture_output=True)
    if result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli(args: list[str], expected_exit: int = 0, cwd: Path = REPO_ROOT) -> dict[str, Any]:
    if cwd == REPO_ROOT:
        command = ["cargo", "run", "-q", "-p", "overrid-cli", "--", *args]
    else:
        command = ["cargo", "run", "-q", "--", *args]
    result = run(command, cwd=cwd, expected_exit=expected_exit)
    if result.stderr:
        raise AssertionError(f"CLI emitted stderr unexpectedly: {' '.join(args)}\n{result.stderr}")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}:\n{result.stdout}") from exc


def validate_required_files() -> None:
    for path in [
        HARNESS / "Cargo.toml",
        CLI / "Cargo.toml",
        CLI / "src/parser.rs",
        CLI / "src/runner.rs",
        SUB_PLAN,
        SDS,
        TECH_STACK,
        PHASE_PLAN,
        PHASE_PROGRESS,
        VALIDATION_WRAPPER,
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"Missing required file: {path}")
    for module in HARNESS_MODULES:
        assert_true((REPO_ROOT / HARNESS / module).is_file(), f"Missing harness module: {module}")


def validate_docs() -> None:
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "Phase 3: Rust Harness Crate, Runner API, And Execution Shell",
        "Harness crate skeleton with runner, manifests, fixtures, assertions, artifacts, and phase-gate modules",
        "overrid test integration",
        "overrid test scenario <name>",
        "overrid test list",
        "overrid test reset",
        "overrid test artifacts <run-id>",
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
        "Complete SUB BUILD PLAN #3 Phase 3",
        "packages/integration_harness",
        "overrid test integration",
        "scripts/validate_integration_harness_phase3.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Status: completed",
        "packages/integration_harness",
        "packages/cli/src/runner.rs",
        "Validation Evidence",
        "cargo test -p overrid-cli",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(wrapper, "scripts/validate_integration_harness_phase3.py", VALIDATION_WRAPPER)


def validate_harness_crate() -> None:
    cargo = read(HARNESS / "Cargo.toml")
    lib_rs = read(HARNESS / "src/lib.rs")
    runner_rs = read(HARNESS / "src/runner.rs")
    manifests_rs = read(HARNESS / "src/manifests.rs")
    phase_gate_rs = read(HARNESS / "src/phase_gate.rs")

    assert_contains(cargo, 'name = "overrid-integration-harness"', HARNESS / "Cargo.toml")
    assert_contains(cargo, 'overrid-contracts = { path = "../schemas/overrid_contracts" }', HARNESS / "Cargo.toml")
    assert_true("node" not in cargo.lower() and "typescript" not in cargo.lower(), "harness crate must not depend on Node/TypeScript")

    for module in ["artifacts", "assertions", "fixtures", "manifests", "phase_gate", "runner"]:
        assert_contains(lib_rs, f"pub mod {module};", HARNESS / "src/lib.rs")

    for state in LIFECYCLE_STATES:
        assert_contains(runner_rs, state, HARNESS / "src/runner.rs")
    for command in TEST_COMMANDS:
        assert_contains(runner_rs, f"HarnessCliCommand::{command}", HARNESS / "src/runner.rs")

    for expected in [
        "HarnessManifestLoader::canonical",
        "LocalStackHarness",
        "safety.non_local_profile",
        "ArtifactLocator",
        "result_json",
        "json_escape",
    ]:
        assert_contains(runner_rs, expected, HARNESS / "src/runner.rs")

    for expected in [
        "SUPPORTED_INTEGRATION_HARNESS_SCHEMA_VERSION",
        "DuplicateScenarioId",
        "MissingFixture",
        "UnsafeField",
        "DEFAULT_MANIFEST_DIR",
    ]:
        assert_contains(manifests_rs, expected, HARNESS / "src/manifests.rs")

    for expected in ["GateClass", "mandatory_gate_classes", "scenario_matches_phase_filter"]:
        assert_contains(phase_gate_rs, expected, HARNESS / "src/phase_gate.rs")

    assert_no_forbidden_runtime_authority(runner_rs, HARNESS / "src/runner.rs")
    assert_no_forbidden_runtime_authority(manifests_rs, HARNESS / "src/manifests.rs")


def validate_cli_wiring() -> None:
    cli_cargo = read(CLI / "Cargo.toml")
    parser = read(CLI / "src/parser.rs")
    runner = read(CLI / "src/runner.rs")

    assert_contains(cli_cargo, 'overrid-integration-harness = { path = "../integration_harness" }', CLI / "Cargo.toml")
    for expected in [
        "pub enum TestCommand",
        "Integration",
        "Scenario",
        "List",
        "Reset",
        "Artifacts",
        '"test" => test_command(tokens)',
        'Command::Test(TestCommand::Integration)',
        'Command::Test(TestCommand::Scenario',
        'Command::Test(TestCommand::Artifacts',
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    for expected in [
        "HarnessRunner",
        "HarnessCliCommand",
        "resolve_repo_root",
        'join("packages/schemas/overrid_contracts/fixtures/valid")',
        "test_command_result",
        "render_harness_error_json",
        "output.result_json()",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")


def validate_cli_behavior() -> None:
    listed = run_cli(["test", "list", "--phase", "0", "--json"])
    assert_true(listed["ok"] is True, "test list should be ok")
    result = listed["result"]
    assert_true(result["schema_version"] == "integration-harness.v0.1", "wrong harness schema version")
    assert_true(result["phase_filter"] == 0, "phase filter not preserved")
    scenario_ids = {scenario["scenario_id"] for scenario in result["scenarios"]}
    assert_true({"scenario_phase0_smoke", "scenario_blocked_dependency"} <= scenario_ids, "phase 0 scenarios missing")

    package_root_list = run_cli(["test", "list", "--phase", "0", "--json"], cwd=REPO_ROOT / CLI)
    assert_true(package_root_list["ok"] is True, "package-root test list should resolve repo root")
    assert_true(package_root_list["result"]["phase_filter"] == 0, "package-root phase filter not preserved")

    blocked = run_cli(
        ["test", "scenario", "scenario_blocked_dependency", "--json"],
        expected_exit=3,
    )
    assert_true(blocked["ok"] is False, "blocked scenario should return ok=false")
    assert_true(blocked["reason_code"] == "dependency.service_unavailable", "wrong blocked scenario reason")
    assert_true(blocked["result"]["status"] == "blocked", "blocked scenario status drifted")
    assert_true("collecting_artifacts" in blocked["result"]["lifecycle"], "blocked path must collect artifacts")
    assert_true(blocked["result"]["artifacts"], "blocked scenario must include artifact refs")

    integration = run_cli(["test", "integration", "--json"])
    assert_true(integration["ok"] is True, "integration should be ok after Phase 4 local-stack hooks")
    assert_true(integration["result"]["command"] == "test integration", "wrong integration command")
    assert_true(integration["result"]["status"] == "passed", "integration should run the Phase 0 smoke path")
    assert_true("collecting_artifacts" in integration["result"]["lifecycle"], "integration must collect artifacts")

    artifacts = run_cli(["test", "artifacts", "run_phase0_smoke", "--json"])
    assert_true(artifacts["ok"] is True, "artifact lookup should be ok")
    assert_true(artifacts["result"]["reason_code"] == "artifact.lookup_ready", "wrong artifact reason")
    assert_true("artifact:bundle:run_phase0_smoke" in json.dumps(artifacts), "artifact bundle ref missing")


def validate_cargo_checks() -> None:
    run(["cargo", "check", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-integration-harness"])
    run(["cargo", "test", "-p", "overrid-cli"])


def main() -> int:
    validate_required_files()
    validate_docs()
    validate_harness_crate()
    validate_cli_wiring()
    validate_cli_behavior()
    validate_cargo_checks()
    print("Integration Test Harness Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
