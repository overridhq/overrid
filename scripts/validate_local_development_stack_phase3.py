#!/usr/bin/env python3
"""Validate Local Development Stack Phase 3 Rust crate and CLI surface."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

ROOT_CARGO = Path("Cargo.toml")
LOCAL_STACK_CARGO = Path("packages/local_stack/Cargo.toml")
LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
CLI_CARGO = Path("packages/cli/Cargo.toml")
CLI_PARSER = Path("packages/cli/src/parser.rs")
CLI_RUNNER = Path("packages/cli/src/runner.rs")
DEFAULT_FIXTURE = Path(
    "packages/schemas/overrid_contracts/fixtures/valid/"
    "local_development_stack_phase2_default_local.valid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_03_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    ROOT_CARGO,
    LOCAL_STACK_CARGO,
    LOCAL_STACK_LIB,
    CLI_CARGO,
    CLI_PARSER,
    CLI_RUNNER,
    DEFAULT_FIXTURE,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

DEV_COMMANDS = [
    "start",
    "stop",
    "restart",
    "status",
    "reset",
    "seed",
    "smoke",
    "logs",
    "doctor",
]

PHASE3_WORK_ITEMS = ["**3.1", "**3.2", "**3.3", "**3.4", "**3.5"]
COMMAND_STATES = [
    "planned",
    "prerequisites_checked",
    "starting",
    "ready",
    "resetting",
    "seeding",
    "smoking",
    "collecting_artifacts",
    "stopped",
    "blocked",
    "failed",
    "completed",
]


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


def run(command: list[str], check: bool = True) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(
        command,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if check and result.returncode != 0:
        raise AssertionError(
            f"{' '.join(command)} failed with {result.returncode}\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def run_cli_json(args: list[str], expect_code: int | None = 0) -> dict[str, Any]:
    result = run(["cargo", "run", "-q", "-p", "overrid-cli", "--", *args], check=False)
    if expect_code is not None and result.returncode != expect_code:
        raise AssertionError(
            f"overrid {' '.join(args)} exited {result.returncode}, expected {expect_code}\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise AssertionError(
            f"overrid {' '.join(args)} did not emit JSON: {error}\n{result.stdout}"
        ) from error


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_workspace_wiring() -> None:
    root = read_text(ROOT_CARGO)
    local_cargo = read_text(LOCAL_STACK_CARGO)
    cli_cargo = read_text(CLI_CARGO)

    assert_contains(root, '"packages/local_stack"', ROOT_CARGO)
    assert_contains(local_cargo, 'name = "overrid-local-stack"', LOCAL_STACK_CARGO)
    assert_contains(local_cargo, "overrid-contracts", LOCAL_STACK_CARGO)
    assert_contains(cli_cargo, "overrid-local-stack", CLI_CARGO)


def check_plan_and_docs() -> None:
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(sub_plan, "## Phase 3: Rust Local-Stack Crate And Command Surface", SUB_PLAN)
    for item in PHASE3_WORK_ITEMS:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "Rust local-stack crate boundary",
        "`overrid dev` command handlers",
        "phase-aware capability discovery",
        "local command state tracking",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "packages/local_stack",
        "overrid dev",
        "scripts/validate_local_development_stack_phase3.py",
        "cargo test -p overrid-local-stack",
        "cargo test -p overrid-cli dev",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 3 Progress", PHASE_PROGRESS)

    for expected in [
        "Rust-first infrastructure stack",
        "Overrid-shaped local stubs",
        "Node.js/TypeScript as the core control-plane",
        "Loopback-only local stack",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase3.py")',
        SUITE_VALIDATOR,
    )


def check_local_stack_source() -> None:
    source = read_text(LOCAL_STACK_LIB)

    for expected in [
        "pub struct LocalStackRunner",
        "pub struct LocalStackManifest",
        "pub enum DevCommand",
        "pub enum LocalCommandState",
        "pub fn capabilities_for_phase",
        "LocalStackCommandOutput",
        "result_json",
        "error_json",
        "declared_dependency_ids",
        "node_or_ts_runtime_authority\\\":false",
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)
    for command in DEV_COMMANDS:
        assert_contains(source, f'"dev {command}"', LOCAL_STACK_LIB)
    for state in COMMAND_STATES:
        assert_contains(source, f'"{state}"', LOCAL_STACK_LIB)
    for expected in [
        "phase.local_service_unavailable",
        "profile.not_local_test",
        "local_stack.backing_services_unavailable",
        "redacted_diagnostics_only",
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_cli_source() -> None:
    parser = read_text(CLI_PARSER)
    runner = read_text(CLI_RUNNER)

    assert_contains(runner, "overrid_local_stack", CLI_RUNNER)
    assert_contains(runner, "LocalStackRunner::new(options).run", CLI_RUNNER)
    assert_contains(runner, "render_local_stack_output_json", CLI_RUNNER)
    assert_contains(runner, "LOCAL_STACK_TRACE_ID", CLI_RUNNER)
    for command in DEV_COMMANDS:
        assert_contains(parser, f'"{command}" => Ok(Command::Dev(DevCommand::', CLI_PARSER)
        assert_contains(parser, f'DevCommand::{command.title().replace("-", "")}', CLI_PARSER)


def check_cli_json_outputs() -> None:
    status = run_cli_json(["dev", "status", "--json"])
    assert_true(status["ok"] is True, "dev status should succeed")
    assert_true(status["result"]["node_or_ts_runtime_authority"] is False, "Rust must own runtime authority")
    assert_true(status["result"]["local_only"] is True, "dev status must be local_only")
    assert_true(status["result"]["test_only"] is True, "dev status must be test_only")
    assert_true(status["result"]["profile"] == "local", "dev status should default to local profile")
    assert_true(status["lifecycle"]["terminal_state"] == "completed", "status should complete")
    assert_true(status["capabilities"]["fail_closed"] is False, "phase 0 routes should be available")

    smoke = run_cli_json(["dev", "smoke", "--json", "--trace-id", "trace_phase3_smoke"])
    assert_true(smoke["ok"] is True, "dev smoke should succeed in phase 0")
    assert_true(smoke["result"]["reason_code"] == "local_stack.smoke_passed", "smoke reason drifted")
    assert_true("collecting_artifacts" in smoke["lifecycle"]["states"], "smoke should collect artifacts")
    assert_true(smoke["audit_refs"], "smoke should emit artifact refs")

    blocked = run_cli_json(["dev", "smoke", "--json", "--phase", "2"], expect_code=None)
    assert_true(blocked["ok"] is False, "phase 2 smoke should be blocked in phase 3")
    assert_true(blocked["reason_code"] == "phase.local_service_unavailable", "phase block reason drifted")
    assert_true(blocked["error"]["phase_gate"] == "phase_3_local_development_stack", "phase gate drifted")
    assert_true(blocked["capabilities"]["fail_closed"] is True, "future capabilities should fail closed")

    unsafe_profile = run_cli_json(
        ["dev", "start", "--json", "--profile", "staging"],
        expect_code=None,
    )
    assert_true(unsafe_profile["ok"] is False, "staging profile must be blocked")
    assert_true(unsafe_profile["reason_code"] == "profile.not_local_test", "profile block reason drifted")

    unavailable = run_cli_json(
        ["dev", "start", "--json", "--profile", "local-health-timeout"],
        expect_code=None,
    )
    assert_true(unavailable["ok"] is False, "unavailable backing profile must be blocked")
    assert_true(
        unavailable["reason_code"] == "local_stack.backing_services_unavailable",
        "backing service block reason drifted",
    )
    rendered = json.dumps(unavailable).lower()
    for forbidden in ["password=", "token=", "api_key=", "private key"]:
        assert_true(forbidden not in rendered, f"diagnostics exposed {forbidden}")


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack"])
    run(["cargo", "test", "-p", "overrid-cli", "dev"])


def main() -> int:
    checks = [
        check_required_files,
        check_workspace_wiring,
        check_plan_and_docs,
        check_local_stack_source,
        check_cli_source,
        check_cargo_tests,
        check_cli_json_outputs,
    ]
    try:
        for check in checks:
            check()
    except AssertionError as error:
        print(f"Local Development Stack Phase 3 validation failed: {error}", file=sys.stderr)
        return 1

    print("Local Development Stack Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
