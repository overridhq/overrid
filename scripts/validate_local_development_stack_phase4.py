#!/usr/bin/env python3
"""Validate Local Development Stack Phase 4 loopback/env/secret guards."""

from __future__ import annotations

import json
from pathlib import Path
import socket
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
DEFAULT_FIXTURE = Path(
    "packages/schemas/overrid_contracts/fixtures/valid/"
    "local_development_stack_phase2_default_local.valid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_04_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
ENV_EXAMPLE = Path(".env.example")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_LIB,
    DEFAULT_FIXTURE,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    ENV_EXAMPLE,
    SUITE_VALIDATOR,
]

RESERVED_PORTS = [18080, 18081, 18082, 18083, 18084, 18085]
READY_DOCTOR_CODES = {
    "doctor.rust_toolchain_ready",
    "doctor.repo_layout_ready",
    "doctor.schemas_ready",
    "doctor.ports_available",
    "doctor.file_permissions_ready",
    "doctor.volume_markers_ready",
    "doctor.ci_runner_compatible",
}
RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "api_key=",
    "private key",
    "-----begin",
)


def read_text(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> Any:
    return json.loads(read_text(path))


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


def check_plan_and_docs() -> None:
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(sub_plan, "## Phase 4: Loopback Topology, Env, Ports, And Secret Guards", SUB_PLAN)
    for item in ["**4.1", "**4.2", "**4.3", "**4.4", "**4.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "deterministic port registry",
        "loopback-only binding",
        "redacted local env manifest",
        "test-only secret records",
        "doctor prerequisite checks",
        "scripts/validate_local_development_stack_phase4.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 4 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Loopback-only local stack", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase4.py")',
        SUITE_VALIDATOR,
    )


def check_fixture_source_of_truth() -> None:
    fixture = load_json(DEFAULT_FIXTURE)
    bindings = fixture["port_registry"]["bindings"]
    assert_true([binding["port"] for binding in bindings] == RESERVED_PORTS, "fixture port order drifted")
    assert_true(
        all(binding["bind_host"] == "127.0.0.1" for binding in bindings),
        "fixture ports must bind to loopback",
    )
    assert_true(
        fixture["port_registry"]["collision_policy"] == "fail_before_startup",
        "fixture collision policy drifted",
    )
    assert_true(fixture["local_env_manifest"]["contains_raw_secret"] is False, "fixture env has raw secret")
    assert_true(
        all(record["raw_secret_present"] is False for record in fixture["local_secret_records"]),
        "fixture secret records expose raw values",
    )


def check_env_example() -> None:
    env_example = read_text(ENV_EXAMPLE)
    for expected in ["OVERRID_LOCAL_PROFILE=", "OVERRID_LOCAL_TEST_SECRET_REF="]:
        assert_contains(env_example, expected, ENV_EXAMPLE)
    rendered = env_example.lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f".env.example exposes {marker}")


def check_local_stack_source() -> None:
    source = read_text(LOCAL_STACK_LIB)
    for expected in [
        "LOCAL_STACK_PHASE4_TOPOLOGY_GATE",
        "ReservedPortBinding",
        "LocalEnvVariable",
        "LocalSecretRecord",
        "LocalDoctorCheck",
        "render_port_registry_json",
        "render_env_manifest_json",
        "render_secret_records_json",
        "render_doctor_checks_json",
        "detect_reserved_port_conflicts",
        "render_port_conflicts_json",
        "port_registry:reserved_18080_18085",
        "port_conflicts",
        "OVERRID_LOCAL_TEST_SECRET_REF",
        "doctor.unsafe_env_value",
        "doctor.schemas_stale",
        "doctor.file_permissions_invalid",
        "doctor.rust_toolchain_missing",
        "local_stack_preflight_failed",
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)
    for port in RESERVED_PORTS:
        assert_contains(source, str(port), LOCAL_STACK_LIB)


def check_cli_json_outputs() -> None:
    doctor = run_cli_json(["dev", "doctor", "--json"])
    assert_true(doctor["ok"] is True, "dev doctor should pass for default local profile")
    result = doctor["result"]
    assert_true(
        result["topology_phase_gate"] == "phase_4_loopback_topology",
        "topology phase gate drifted",
    )
    port_bindings = result["port_registry"]["bindings"]
    assert_true([binding["port"] for binding in port_bindings] == RESERVED_PORTS, "CLI ports drifted")
    assert_true(all(binding["loopback_only"] is True for binding in port_bindings), "CLI ports not loopback")
    assert_true(
        any(
            binding["service_id"] == "service:worker"
            and binding["port"] == 18081
            and binding["purpose"] == "worker_health_metrics"
            for binding in port_bindings
        ),
        "worker health/metrics binding missing",
    )
    api_health = next(
        health for health in result["service_health"] if health["service_id"] == "service:api"
    )
    assert_true(api_health["bind_host"] == "127.0.0.1", "api bind host drifted")
    assert_true(api_health["port"] == 18080, "api port drifted")
    assert_true(api_health["loopback_only"] is True, "api health is not loopback")

    env_manifest = result["env_manifest"]
    assert_true(env_manifest["contains_raw_secret"] is False, "env manifest exposes raw secret")
    assert_true(
        env_manifest["example_values_include_raw_secrets"] is False,
        "example env exposes raw secret",
    )
    assert_true(env_manifest["example_env_target"] == "repo://.env.example", "env example target drifted")
    secret_var = next(
        variable
        for variable in env_manifest["variables"]
        if variable["name"] == "OVERRID_LOCAL_TEST_SECRET_REF"
    )
    assert_true(secret_var["redacted"] is True, "secret env variable is not redacted")
    assert_true(secret_var["value_ref"].startswith("secret://"), "secret env variable is not ref-only")

    assert_true(
        all(
            record["local_only"] is True
            and record["test_only"] is True
            and record["raw_secret_present"] is False
            for record in result["secret_records"]
        ),
        "secret records must be local/test ref-only",
    )
    doctor_codes = {check["reason_code"] for check in result["doctor_checks"]}
    assert_true(READY_DOCTOR_CODES.issubset(doctor_codes), "doctor ready codes missing")
    assert_true(all(check["state"] == "ready" for check in result["doctor_checks"]), "doctor should be ready")

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as listener:
        listener.bind(("127.0.0.1", 18080))
        listener.listen(1)
        port_conflict = run_cli_json(["dev", "start", "--json"], expect_code=None)
    assert_true(port_conflict["ok"] is False, "port conflict should fail closed")
    assert_true(port_conflict["reason_code"] == "local_stack.port_conflict", "port conflict reason drifted")
    assert_true("starting" not in port_conflict["lifecycle"]["states"], "port conflict launched services")
    conflicts = port_conflict["error"]["port_conflicts"]
    assert_true(
        any(
            conflict["port"] == 18080
            and conflict["bind_host"] == "127.0.0.1"
            and conflict["reason_code"] == "local_stack.port_conflict"
            for conflict in conflicts
        ),
        "real reserved-port conflict details missing",
    )
    assert_true(
        any(
            check["check_id"] == "doctor:reserved_ports"
            and check["state"] == "failed"
            for check in port_conflict["error"]["doctor_checks"]
        ),
        "port conflict doctor check missing",
    )

    unsafe_env = run_cli_json(
        ["dev", "doctor", "--json", "--profile", "local-unsafe-env"],
        expect_code=None,
    )
    assert_true(unsafe_env["ok"] is False, "unsafe env should fail")
    assert_true(unsafe_env["reason_code"] == "doctor.unsafe_env_value", "unsafe env reason drifted")
    assert_true(
        any(
            check["reason_code"] == "doctor.unsafe_env_value"
            and check["state"] == "failed"
            for check in unsafe_env["error"]["doctor_checks"]
        ),
        "unsafe env doctor failure missing",
    )

    production_profile = run_cli_json(
        ["dev", "start", "--json", "--profile", "production"],
        expect_code=None,
    )
    assert_true(production_profile["ok"] is False, "production profile should be blocked")
    assert_true(
        production_profile["reason_code"] == "profile.not_local_test",
        "production profile reason drifted",
    )

    rendered = json.dumps([doctor, port_conflict, unsafe_env, production_profile]).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"CLI output exposes {marker}")


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack"])
    run(["cargo", "test", "-p", "overrid-cli", "dev"])


def main() -> int:
    checks = [
        check_required_files,
        check_plan_and_docs,
        check_fixture_source_of_truth,
        check_env_example,
        check_local_stack_source,
        check_cargo_tests,
        check_cli_json_outputs,
    ]
    try:
        for check in checks:
            check()
    except AssertionError as error:
        print(f"Local Development Stack Phase 4 validation failed: {error}", file=sys.stderr)
        return 1

    print("Local Development Stack Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
