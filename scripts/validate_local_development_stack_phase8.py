#!/usr/bin/env python3
"""Validate Local Development Stack Phase 8 simulator and smoke guarantees."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_08_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_LIB,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

PHASE8_GATE = "phase_8_node_simulator_smoke_harness"
EXPECTED_HOOK_IDS = [
    "harness_hook:start",
    "harness_hook:status",
    "harness_hook:reset",
    "harness_hook:seed",
    "harness_hook:smoke",
    "harness_hook:logs",
    "harness_hook:health_snapshots",
    "harness_hook:event_export",
    "harness_hook:artifact_collection",
]
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


def envelope_body(payload: dict[str, Any]) -> dict[str, Any]:
    body = payload.get("result") or payload.get("error")
    assert_true(isinstance(body, dict), "CLI envelope did not contain a result or error body")
    return body


def assert_secret_free(payload: dict[str, Any], context: str) -> None:
    rendered = json.dumps(payload, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"{context} exposes raw secret marker {marker}")

    body = envelope_body(payload)
    for event in body.get("lifecycle_events", []):
        assert_true(event["contains_raw_secret"] is False, f"{context} event exposes a raw secret")
        assert_true(event["redaction_summary"] == "secret_free", f"{context} event redaction drifted")
        assert_true(event["local_only"] is True and event["test_only"] is True, f"{context} event scope drifted")


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_docs_and_source() -> None:
    source = read_text(LOCAL_STACK_LIB)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(sub_plan, "## Phase 8: Node Simulator, Smoke Path, And Harness Integration", SUB_PLAN)
    for item in ["**8.1", "**8.2", "**8.3", "**8.4", "**8.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "local Overcell-like node simulator",
        "Phase 0 smoke command evidence",
        "Integration Test Harness hooks",
        "CLI and SDK smoke support records",
        "later simulator expansion rules",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 8 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase8.py")',
        SUITE_VALIDATOR,
    )

    for expected in [
        "LOCAL_STACK_PHASE8_SMOKE_GATE",
        "LOCAL_STACK_PHASE8_SMOKE_FIXTURE_VERSION",
        "LocalNodeSimulatorRecord",
        "LocalPhase0SmokeRecord",
        "LocalHarnessHookRecord",
        "LocalSdkSmokeSupportRecord",
        "LocalSimulatorExpansionRule",
        "node_simulator_records",
        "phase0_smoke_records",
        "harness_hook_records",
        "cli_sdk_smoke_support_records",
        "simulator_expansion_rules",
        "render_node_simulator_records_json",
        "render_phase0_smoke_records_json",
        "render_harness_hook_records_json",
        "render_cli_sdk_smoke_support_records_json",
        "render_simulator_expansion_rules_json",
        "local_stack.phase0_smoke_path_verified",
        "local_stack.node_simulator_fixture_ready",
        PHASE8_GATE,
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack", "phase8"])


def check_node_simulator(status_payload: dict[str, Any]) -> None:
    result = status_payload["result"]
    assert_true(result["smoke_phase_gate"] == PHASE8_GATE, "phase 8 gate drifted")
    records = result["node_simulator_records"]
    assert_true(len(records) == 1, "expected one node simulator record")
    record = records[0]
    assert_true(record["service_id"] == "service:node_agent_simulator", "node simulator service id drifted")
    assert_true(record["health_endpoint_ref"].startswith("http://127.0.0.1:18082/"), "node health endpoint drifted")
    assert_true(record["local_only"] is True and record["test_only"] is True, "node simulator scope drifted")
    for forbidden in [
        "real_hardware_discovery",
        "gpu_runtime_integration",
        "benchmark_publication",
        "installer_update_flow",
        "remote_shell_behavior",
        "provider_eligibility_decision",
    ]:
        assert_true(record[forbidden] is False, f"node simulator enabled forbidden behavior: {forbidden}")

    health = [item for item in result["service_health"] if item["service_id"] == "service:node_agent_simulator"]
    assert_true(health and health[0]["loopback_only"] is True, "node simulator health must be loopback only")


def check_harness_and_support(status_payload: dict[str, Any]) -> None:
    result = status_payload["result"]
    hook_ids = [record["hook_id"] for record in result["harness_hook_records"]]
    for expected in EXPECTED_HOOK_IDS:
        assert_true(expected in hook_ids, f"missing harness hook {expected}")
    assert_true(
        all(
            record["required_for_phase0_smoke"] is True
            and record["public_local_api_only"] is True
            and record["local_only"] is True
            and record["test_only"] is True
            for record in result["harness_hook_records"]
        ),
        "harness hooks must be public local API records",
    )

    surfaces = {record["client_surface"] for record in result["cli_sdk_smoke_support_records"]}
    assert_true({"overrid-cli", "overrid-rust-sdk"}.issubset(surfaces), "CLI/Rust SDK smoke support missing")
    assert_true(
        all(
            record["public_local_api_only"] is True
            and record["avoids_private_storage"] is True
            and record["avoids_simulator_internals"] is True
            for record in result["cli_sdk_smoke_support_records"]
        ),
        "CLI/SDK support must avoid private storage and simulator internals",
    )

    rules = result["simulator_expansion_rules"]
    assert_true(len(rules) >= 5, "later simulator expansion rules missing")
    assert_true(
        all(
            rule["local_test_marker_required"] is True
            and rule["production_contract_shape_required"] is True
            and rule["phase0_responsibility_drift_allowed"] is False
            and rule["blocks_without_owner"] is True
            for rule in rules
        ),
        "simulator expansion rules must block responsibility drift",
    )


def check_phase0_smoke_path() -> None:
    output = run_cli_json(
        [
            "dev",
            "smoke",
            "--json",
            "--trace-id",
            "trace_phase8_validator_smoke",
        ]
    )
    assert_true(output["ok"] is True, "phase 8 smoke should pass")
    result = output["result"]
    assert_true(result["smoke_phase_gate"] == PHASE8_GATE, "phase 8 smoke gate drifted")
    smoke = result["phase0_smoke_records"][0]
    assert_true(smoke["trace_id"] == "trace_phase8_validator_smoke", "smoke trace id drifted")
    assert_true(smoke["signed_noop_admitted"] is True, "signed no-op command not admitted")
    assert_true(smoke["audit_event_write_read"] is True, "audit event roundtrip missing")
    assert_true(smoke["invalid_schema_denied"] is True, "invalid schema denial missing")
    assert_true(smoke["trace_id_propagated"] is True, "trace propagation missing")
    assert_true(smoke["fixture_state_inspected"] is True, "fixture state inspection missing")
    assert_true(smoke["contains_raw_secret"] is False, "smoke record exposes raw secret")
    assert_true(smoke["redacted_artifact_ref"].endswith("/redacted_bundle"), "redacted artifact ref missing")
    assert_true(
        any(reference.endswith("/redacted_bundle") for reference in result["artifact_refs"]),
        "smoke artifact refs missing redacted bundle",
    )
    assert_true(
        "phase0_smoke_path_verified" in output["diagnostic_bundle"]["dependency_status"],
        "smoke dependency status missing",
    )
    assert_secret_free(output, "phase 8 smoke")


def main() -> int:
    check_required_files()
    check_docs_and_source()
    check_cargo_tests()
    status = run_cli_json(["dev", "status", "--json", "--trace-id", "trace_phase8_validator_status"])
    assert_secret_free(status, "phase 8 status")
    check_node_simulator(status)
    check_harness_and_support(status)
    check_phase0_smoke_path()
    print("Local Development Stack Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
