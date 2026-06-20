#!/usr/bin/env python3
"""Validate Local Development Stack Phase 6 lifecycle orchestration."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_06_progress.md")
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

LIFECYCLE_PHASE_GATE = "phase_6_lifecycle_orchestration"
EXPECTED_STARTUP_ORDER = [
    "service:embedded_state",
    "service:overqueue_jobs",
    "service:overstore_stub",
    "service:event_audit",
    "service:api",
    "service:worker",
    "service:node_agent_simulator",
]
EXPECTED_SHUTDOWN_ORDER = list(reversed(EXPECTED_STARTUP_ORDER))
REQUIRED_SERVICES = set(EXPECTED_STARTUP_ORDER)
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


def event_types(events: list[dict[str, Any]]) -> set[str]:
    return {event["event_type"] for event in events}


def assert_secret_free(payload: dict[str, Any], context: str) -> None:
    rendered = json.dumps(payload, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"{context} exposes raw secret marker {marker}")

    envelope_body = payload.get("result") or payload.get("error") or {}
    events = envelope_body.get("lifecycle_events", [])
    assert_true(events, f"{context} lifecycle events missing")
    for event in events:
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

    assert_contains(sub_plan, "## Phase 6: Lifecycle Orchestration And Health Readiness", SUB_PLAN)
    for item in ["**6.1", "**6.2", "**6.3", "**6.4", "**6.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "dependency-ordered startup graph",
        "reverse dependency shutdown",
        "readiness/liveness checks",
        "bounded wait policy",
        "lifecycle event records",
        "scripts/validate_local_development_stack_phase6.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 6 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase6.py")',
        SUITE_VALIDATOR,
    )

    for expected in [
        "LOCAL_STACK_PHASE6_LIFECYCLE_GATE",
        "DEFAULT_LIFECYCLE_TIMEOUT_MS",
        "DEFAULT_LIFECYCLE_POLL_INTERVAL_MS",
        "LocalServiceLifecycleStep",
        "LocalServiceShutdownReport",
        "LocalReadinessCheck",
        "LocalWaitPolicy",
        "LocalRollbackReport",
        "LocalLifecycleEventRecord",
        "lifecycle_startup_steps_for_mode",
        "shutdown_reports",
        "readiness_checks_for_mode",
        "wait_policy_for_mode",
        "rollback_reports_for_mode",
        "lifecycle_events_for_mode",
        "shutdown_lifecycle_events",
        "render_startup_graph_json",
        "render_shutdown_reports_json",
        "render_readiness_checks_json",
        "render_wait_policy_json",
        "render_rollback_reports_json",
        "render_lifecycle_events_json",
        "local_stack.health_timeout",
        "local_stack.required_service_failed",
        "local_stack.optional_service_degraded",
        "local_stack.partial_start_rollback",
        "local_stack.start_requested",
        "local_stack.service_starting",
        "local_stack.service_ready",
        "local_stack.reset_started",
        "local_stack.seed_started",
        "local_stack.smoke_started",
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack"])
    run(["cargo", "test", "-p", "overrid-cli", "dev"])


def check_clean_start_lifecycle() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--trace-id",
            "trace_phase6_start",
            "--timeout",
            "4500",
            "--poll-interval",
            "250",
        ]
    )
    assert_true(output["ok"] is True, "clean start should pass")
    result = output["result"]
    assert_true(result["lifecycle_phase_gate"] == LIFECYCLE_PHASE_GATE, "phase 6 gate drifted")
    assert_true(
        [step["service_id"] for step in result["startup_graph"]] == EXPECTED_STARTUP_ORDER,
        "startup dependency order drifted",
    )
    assert_true(
        all(
            step["startup_state"] == "ready"
            and step["health_state"] == "ready"
            and step["readiness_state"] == "ready"
            and step["liveness_state"] == "ready"
            and step["reason_code"] == "local_stack.service_ready"
            for step in result["startup_graph"]
        ),
        "clean startup graph is not fully ready",
    )
    assert_true(
        all(
            check["health_state"] == "ready"
            and check["readiness_state"] == "ready"
            and check["liveness_state"] == "ready"
            and check["dependency_state"] == "satisfied"
            and check["stale_schema"] is False
            for check in result["readiness_checks"]
        ),
        "clean readiness checks drifted",
    )
    wait_policy = result["wait_policy"]
    assert_true(wait_policy["timeout_ms"] == 4500, "wait timeout override drifted")
    assert_true(wait_policy["poll_interval_ms"] == 250, "wait poll interval override drifted")
    assert_true(wait_policy["bounded"] is True, "wait policy is not bounded")
    assert_true(wait_policy["no_unbounded_sleep"] is True, "wait policy allows unbounded sleep")
    assert_true(wait_policy["logs_exported_on_timeout"] is False, "clean start should not export timeout logs")

    types = event_types(result["lifecycle_events"])
    for expected in [
        "local_stack.start_requested",
        "local_stack.service_starting",
        "local_stack.service_ready",
    ]:
        assert_true(expected in types, f"clean start lifecycle event missing {expected}")

    service_health = {health["service_id"]: health for health in result["service_health"]}
    for service_id in REQUIRED_SERVICES:
        assert_true(service_health[service_id]["state"] == "ready", f"{service_id} health not ready")

    dependency_status = output["diagnostic_bundle"]["dependency_status"]
    for expected in [
        "dependency_ordered_startup_verified",
        "reverse_shutdown_order_verified",
        "readiness_liveness_checks_ready",
        "bounded_wait_policy_ready",
        "lifecycle_events_secret_free",
    ]:
        assert_true(expected in dependency_status, f"dependency status missing {expected}")
    assert_secret_free(output, "clean start")


def check_already_running_start() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--profile",
            "local-already-running",
            "--trace-id",
            "trace_phase6_already_running",
        ]
    )
    assert_true(output["ok"] is True, "already-running start should pass")
    result = output["result"]
    assert_true(result["reason_code"] == "local_stack.already_running", "already-running reason drifted")
    assert_true(
        all(step["startup_state"] == "already_running" for step in result["startup_graph"]),
        "already-running startup states drifted",
    )
    assert_true(result["rollback_reports"] == [], "already-running start should not roll back")
    assert_secret_free(output, "already-running start")


def check_optional_degraded_start() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--profile",
            "local-degraded-optional",
            "--trace-id",
            "trace_phase6_degraded",
        ]
    )
    assert_true(output["ok"] is True, "optional degraded start should pass")
    result = output["result"]
    assert_true(result["status"] == "degraded", "optional degraded status drifted")
    assert_true(result["reason_code"] == "local_stack.degraded", "optional degraded reason drifted")
    developer_ui = [
        step for step in result["startup_graph"] if step["service_id"] == "service:developer_ui"
    ]
    assert_true(developer_ui, "optional developer UI lifecycle step missing")
    assert_true(developer_ui[0]["required"] is False, "developer UI should be optional")
    assert_true(developer_ui[0]["startup_state"] == "degraded", "developer UI startup state drifted")
    assert_true(
        all(
            step["startup_state"] == "ready"
            for step in result["startup_graph"]
            if step["service_id"] in REQUIRED_SERVICES
        ),
        "required services should remain ready when optional service degrades",
    )
    assert_true(
        any(
            check["service_id"] == "service:developer_ui"
            and check["readiness_state"] == "degraded"
            and check["reason_code"] == "local_stack.optional_service_degraded"
            for check in result["readiness_checks"]
        ),
        "optional degraded readiness evidence missing",
    )
    assert_true(
        "local_stack.service_degraded" in event_types(result["lifecycle_events"]),
        "optional degraded lifecycle event missing",
    )
    assert_secret_free(output, "optional degraded start")


def check_restart_shutdown_order() -> None:
    output = run_cli_json(
        ["dev", "restart", "--json", "--trace-id", "trace_phase6_restart"]
    )
    assert_true(output["ok"] is True, "restart should pass")
    result = output["result"]
    assert_true(
        [report["service_id"] for report in result["shutdown_reports"]]
        == EXPECTED_SHUTDOWN_ORDER,
        "shutdown reverse dependency order drifted",
    )
    assert_true(
        all(report["graceful"] and report["state_preserved"] for report in result["shutdown_reports"]),
        "restart shutdown reports must be graceful and state-preserving",
    )
    types = event_types(result["lifecycle_events"])
    assert_true("local_stack.restart_requested" in types, "restart requested event missing")
    assert_true("local_stack.service_stopped" in types, "service stopped event missing")
    assert_secret_free(output, "restart")


def check_timeout_failure_rolls_back() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--profile",
            "local-health-timeout",
            "--trace-id",
            "trace_phase6_timeout",
            "--timeout",
            "4500",
            "--poll-interval",
            "250",
        ],
        expect_code=None,
    )
    assert_true(output["ok"] is False, "health timeout should fail")
    assert_true(
        output["reason_code"] == "local_stack.backing_services_unavailable",
        "health timeout public reason drifted",
    )
    assert_true("collecting_artifacts" in output["lifecycle"]["states"], "timeout log export state missing")
    assert_true("failed" in output["lifecycle"]["states"], "timeout failed state missing")
    error = output["error"]
    assert_true(error["lifecycle_phase_gate"] == LIFECYCLE_PHASE_GATE, "timeout phase gate missing")
    wait_policy = error["wait_policy"]
    assert_true(wait_policy["timeout_ms"] == 4500, "timeout wait override drifted")
    assert_true(wait_policy["poll_interval_ms"] == 250, "timeout poll override drifted")
    assert_true(wait_policy["bounded"] is True, "timeout wait policy is not bounded")
    assert_true(wait_policy["no_unbounded_sleep"] is True, "timeout wait policy allows unbounded sleep")
    assert_true(wait_policy["reason_code"] == "local_stack.health_timeout", "timeout wait reason drifted")
    assert_true(wait_policy["logs_exported_on_timeout"] is True, "timeout logs were not exported")
    assert_true(
        any(
            step["service_id"] == "service:api"
            and step["startup_state"] == "timeout"
            and step["reason_code"] == "local_stack.health_timeout"
            for step in error["startup_graph"]
        ),
        "API timeout lifecycle step missing",
    )
    assert_true(
        any(
            step["service_id"] in {"service:worker", "service:node_agent_simulator"}
            and step["startup_state"] == "not_started"
            and step["reason_code"] == "local_stack.dependency_wait_blocked"
            for step in error["startup_graph"]
        ),
        "blocked dependent lifecycle steps missing",
    )
    assert_true(error["rollback_reports"], "timeout should emit rollback reports")
    assert_true(
        all(report["state_preserved"] for report in error["rollback_reports"]),
        "timeout rollback must preserve local state",
    )
    assert_true(
        any(reference.startswith("log://local_stack/startup_failure/") for reference in output["audit_refs"]),
        "timeout startup log audit ref missing",
    )
    assert_true("local_stack.failed" in event_types(error["lifecycle_events"]), "timeout failure event missing")
    assert_secret_free(output, "health timeout")


def check_required_failure_rolls_back() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--profile",
            "local-required-failure",
            "--trace-id",
            "trace_phase6_required_failure",
        ],
        expect_code=None,
    )
    assert_true(output["ok"] is False, "required service failure should fail")
    error = output["error"]
    assert_true(
        output["reason_code"] == "local_stack.backing_services_unavailable",
        "required failure public reason drifted",
    )
    assert_true(
        any(
            step["service_id"] == "service:worker"
            and step["startup_state"] == "failed"
            and step["reason_code"] == "local_stack.required_service_failed"
            for step in error["startup_graph"]
        ),
        "required worker failure lifecycle step missing",
    )
    assert_true(error["rollback_reports"], "required failure should emit rollback reports")
    assert_true("local_stack.failed" in event_types(error["lifecycle_events"]), "required failure event missing")
    assert_secret_free(output, "required failure")


def check_stale_schema_blocks_before_start() -> None:
    output = run_cli_json(
        [
            "dev",
            "start",
            "--json",
            "--profile",
            "local-stale-local-state-schema",
            "--trace-id",
            "trace_phase6_stale_schema",
        ],
        expect_code=None,
    )
    assert_true(output["ok"] is False, "stale schema should fail closed")
    assert_true(
        output["reason_code"] == "local_stack.schema_version_incompatible",
        "stale schema reason drifted",
    )
    assert_true("starting" not in output["lifecycle"]["states"], "stale schema should block before startup")
    error = output["error"]
    assert_true(error["lifecycle_phase_gate"] == LIFECYCLE_PHASE_GATE, "stale schema phase gate missing")
    assert_true(
        error["readiness_checks"]
        and all(
            check["stale_schema"] is True
            and check["readiness_state"] == "stale_schema"
            and check["reason_code"] == "local_stack.schema_version_incompatible"
            for check in error["readiness_checks"]
        ),
        "stale schema readiness evidence missing",
    )
    assert_secret_free(output, "stale schema")


def check_smoke_lifecycle_events() -> None:
    output = run_cli_json(["dev", "smoke", "--json", "--trace-id", "trace_phase6_smoke"])
    assert_true(output["ok"] is True, "smoke should pass")
    result = output["result"]
    types = event_types(result["lifecycle_events"])
    for expected in [
        "local_stack.reset_started",
        "local_stack.seed_started",
        "local_stack.seed_completed",
        "local_stack.smoke_started",
        "local_stack.smoke_completed",
    ]:
        assert_true(expected in types, f"smoke lifecycle event missing {expected}")
    assert_secret_free(output, "smoke")


def main() -> int:
    checks = [
        check_required_files,
        check_docs_and_source,
        check_cargo_tests,
        check_clean_start_lifecycle,
        check_already_running_start,
        check_optional_degraded_start,
        check_restart_shutdown_order,
        check_timeout_failure_rolls_back,
        check_required_failure_rolls_back,
        check_stale_schema_blocks_before_start,
        check_smoke_lifecycle_events,
    ]
    try:
        for check in checks:
            check()
    except AssertionError as error:
        print(f"Local Development Stack Phase 6 validation failed: {error}", file=sys.stderr)
        return 1

    print("Local Development Stack Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
