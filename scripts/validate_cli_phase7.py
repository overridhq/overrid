#!/usr/bin/env python3
"""Validate CLI Phase 7 seed private swarm and execution command gates."""

from __future__ import annotations

from pathlib import Path
import json
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CONTRACTS = Path("packages/schemas/overrid_contracts")
SDK = Path("packages/sdk")
CLI = Path("packages/cli")
CONTRACTS_SCHEMA = CONTRACTS / "v0/cli_command.schema.json"
CONTRACTS_MANIFEST = CONTRACTS / "codegen_manifest.json"
NODE_FIXTURE = CLI / "fixtures/valid/phase7_node_states.valid.json"
WORKLOAD_FIXTURE = CLI / "fixtures/valid/phase7_workload_execution_states.valid.json"
SUB_PLAN = Path("docs/build_plan/sub_build_plan_002_cli.md")
SERVICE = Path("docs/service_catalog/foundation/cli.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/cli_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/cli_phase_07_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

PHASE7_CONTRACTS = {
    "node_status_record",
    "workload_execution_state",
    "execution_timeline",
    "execution_log_bundle",
    "execution_result_ref",
    "polling_plan",
    "execution_diagnostic_event",
}

NODE_STATES = {"live", "stale", "expired", "draining", "disabled"}
WORKLOAD_STATES = {
    "scheduled",
    "leased",
    "running",
    "succeeded",
    "failed",
    "cancelled",
    "timed_out",
    "dead_lettered",
}
EXECUTION_OWNER_REFS = {
    "overgate:execution-command",
    "overqueue:workload-state",
    "oversched:scheduler",
    "overlease:lease",
    "overrun:runner",
    "overcell:node-heartbeat",
    "overpack:package",
    "overstore:result-state-ref",
    "overwatch:trace",
}
TERMINAL_REASON_CODES = {
    "succeeded": "result.ref.available",
    "failed": "result.failed",
    "cancelled": "result.cancelled",
    "timed_out": "result.timed_out",
    "dead_lettered": "result.dead_lettered",
}

SECRET_MARKERS = [
    "-----begin",
    "private key",
    "raw_key",
    "secret=",
    "token=",
    "decrypted_payload",
    "private_payload=",
    "raw prompt",
    "private file",
    "node-agent://",
    "s3://",
    "minio://",
]

LOCAL_PROFILE_ARGS = [
    "--profile",
    "local-dev",
    "--environment",
    "local",
    "--endpoint",
    "http://127.0.0.1:18080/overgate",
    "--endpoint-fingerprint",
    "fp_local",
    "--tenant",
    "tenant_local",
    "--actor",
    "actor_local",
    "--credential-namespace",
    "local-dev",
    "--credential-class",
    "fixture",
    "--credential-ref",
    "fixture://local-dev/key-1",
    "--key-id",
    "key-1",
    "--fixture-allowance",
    "local_only",
]

REQUIRED_FILES = [
    CONTRACTS_SCHEMA,
    CONTRACTS_MANIFEST,
    NODE_FIXTURE,
    WORKLOAD_FIXTURE,
    CONTRACTS / "src/lib.rs",
    SDK / "src/lib.rs",
    CLI / "src/parser.rs",
    CLI / "src/runner.rs",
    CLI / "README.md",
    CONTRACTS / "README.md",
    SUB_PLAN,
    SERVICE,
    TECH_STACK,
    PHASE_PLAN,
    PHASE_PROGRESS,
    VALIDATION_WRAPPER,
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(read(path))


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def assert_secret_free(text: str, source: str) -> None:
    lowered = text.lower()
    for marker in SECRET_MARKERS:
        if marker in lowered:
            raise AssertionError(f"{source} leaked forbidden marker: {marker}")


def run(command: list[str]) -> None:
    result = subprocess.run(command, cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError(f"Command failed with exit {result.returncode}: {' '.join(command)}")


def run_capture(command: list[str], expected_exit: int = 0) -> str:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != expected_exit:
        raise AssertionError(
            f"Command failed with exit {result.returncode}, expected {expected_exit}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    if result.stderr:
        raise AssertionError(f"Command emitted stderr unexpectedly: {' '.join(command)}\n{result.stderr}")
    return result.stdout.strip()


def run_cli(args: list[str], expected_exit: int = 0) -> dict[str, Any]:
    output = run_capture(["cargo", "run", "-q", "-p", "overrid-cli", "--", *args], expected_exit)
    assert_secret_free(output, " ".join(args))
    try:
        return json.loads(output)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"CLI output is not JSON for {' '.join(args)}: {output}") from exc


def validate_required_files() -> None:
    for path in REQUIRED_FILES:
        if not (REPO_ROOT / path).is_file():
            raise AssertionError(f"Missing required file: {path}")


def validate_schema_and_manifest() -> None:
    schema = read_json(CONTRACTS_SCHEMA)
    manifest = read_json(CONTRACTS_MANIFEST)
    defs = schema["$defs"]

    actual_contracts = set(schema["properties"]["contracts"]["items"]["enum"])
    missing_contracts = PHASE7_CONTRACTS - actual_contracts
    if missing_contracts:
        raise AssertionError(f"CLI schema is missing Phase 7 contracts: {sorted(missing_contracts)}")

    missing_defs = PHASE7_CONTRACTS - set(defs)
    if missing_defs:
        raise AssertionError(f"CLI schema is missing Phase 7 $defs: {sorted(missing_defs)}")

    if set(defs["node_state"]["enum"]) != NODE_STATES:
        raise AssertionError("node_state enum must cover live/stale/expired/draining/disabled")
    if set(defs["workload_execution_state"]["enum"]) != WORKLOAD_STATES:
        raise AssertionError("workload_execution_state enum must cover planned Phase 7 states")

    node_record = defs["node_status_record"]
    if node_record["properties"]["credential_checked"].get("const") is not True:
        raise AssertionError("node_status_record must require credential checks")
    if node_record["properties"]["direct_node_access"].get("const") is not False:
        raise AssertionError("node_status_record must forbid direct node access")

    logs = defs["execution_log_bundle"]
    if logs["properties"]["redaction_policy"].get("const") != "secret_free_refs_only":
        raise AssertionError("execution_log_bundle must use secret_free_refs_only redaction")
    if logs["properties"]["contains_private_payload"].get("const") is not False:
        raise AssertionError("execution_log_bundle must not contain private payloads")
    if logs["properties"]["direct_node_path_exposed"].get("const") is not False:
        raise AssertionError("execution_log_bundle must not expose direct node paths")

    result = defs["execution_result_ref"]
    if result["properties"]["contains_private_payload"].get("const") is not False:
        raise AssertionError("execution_result_ref must not contain private payloads")
    if result["properties"]["direct_object_store_path_exposed"].get("const") is not False:
        raise AssertionError("execution_result_ref must not expose object-store paths")

    polling = defs["polling_plan"]
    if polling["properties"]["timeout_ms"].get("maximum") != 600000:
        raise AssertionError("polling_plan timeout must stay bounded")
    if polling["properties"]["poll_interval_ms"].get("minimum") != 100:
        raise AssertionError("polling_plan poll interval must be clamped")
    if polling["properties"]["event_stream_preferred"].get("const") is not True:
        raise AssertionError("polling_plan must prefer event streams")
    if polling["properties"]["fallback_polling"].get("const") is not True:
        raise AssertionError("polling_plan must expose fallback polling")

    if set(manifest.get("phase7_contracts", [])) != PHASE7_CONTRACTS:
        raise AssertionError("Manifest phase7_contracts does not match schema extensions")
    fixture_paths = manifest.get("fixtures", {}).get("phase7_valid", [])
    for expected in [str(NODE_FIXTURE), str(WORKLOAD_FIXTURE)]:
        if expected not in fixture_paths:
            raise AssertionError(f"Manifest is missing Phase 7 fixture path: {expected}")
    guardrails = "\n".join(manifest.get("guardrails", []))
    for expected in [
        "Phase 7 node status records",
        "Phase 7 workload execution timelines",
        "Phase 7 log/result retrieval",
    ]:
        if expected not in guardrails:
            raise AssertionError(f"Manifest guardrails missing Phase 7 text: {expected}")


def validate_fixtures() -> None:
    node = read_json(NODE_FIXTURE)
    workload = read_json(WORKLOAD_FIXTURE)
    if set(node["target_refs"].values()) != NODE_STATES:
        raise AssertionError("Node fixture must cover all Phase 7 node states")
    if set(workload["target_refs"].values()) != {"succeeded", "running", "failed", "timed_out", "dead_lettered", "cancelled"}:
        raise AssertionError("Workload fixture must cover Phase 7 terminal and running states")
    if set(workload["required_refs"]) != EXECUTION_OWNER_REFS:
        raise AssertionError("Workload fixture required refs drifted from Phase 7 owner refs")


def validate_rust_phase7_surface() -> None:
    contracts = read(CONTRACTS / "src/lib.rs")
    for expected in [
        "NodeStatusRecord",
        "WorkloadExecutionState",
        "ExecutionTimeline",
        "ExecutionLogBundle",
        "ExecutionResultRef",
        "PollingPlan",
        "ExecutionDiagnosticEvent",
        "result_state_reason_code",
        "result.dead_lettered",
    ]:
        assert_contains(contracts, expected, CONTRACTS / "src/lib.rs")

    parser = read(CLI / "src/parser.rs")
    for expected in [
        "NodeCommand",
        "WorkloadCommand",
        '"node"',
        '"register"',
        '"health"',
        '"logs"',
        '"cancel"',
        '"result"',
        '"follow"',
        "--wait",
        "--timeout",
        "--poll-interval",
        "--follow",
    ]:
        assert_contains(parser, expected, CLI / "src/parser.rs")

    runner = read(CLI / "src/runner.rs")
    for expected in [
        "render_node_result",
        "render_workload_execution_result",
        "node_state_for_ref",
        "workload_execution_state_for_command",
        "polling_plan_for",
        "overcell_node_heartbeat_ref",
        "overpack_package_ref",
        "overstore_result_state_ref",
        "direct_node_access",
        "direct_node_path_exposed",
        "direct_object_store_path_exposed",
        "contains_private_payload",
        "sdk_overgate_contract",
    ]:
        assert_contains(runner, expected, CLI / "src/runner.rs")

    for source_path in [CLI / "src/parser.rs", CLI / "src/runner.rs", SDK / "src/lib.rs"]:
        lowered = read(source_path).lower()
        for forbidden in ["postgres://", "redis://", "nats://", "kafka://", "s3://", "minio://", "vault://", "node-agent://"]:
            if forbidden in lowered:
                raise AssertionError(f"{source_path} contains forbidden direct target: {forbidden}")


def validate_envelope(payload: dict[str, Any], command: str) -> dict[str, Any]:
    if payload["schema_version"] != "cli-command.v0.1":
        raise AssertionError("Envelope schema version changed")
    if payload["ok"] is not True or payload["exit_class"] != "success":
        raise AssertionError(f"{command} did not return success envelope")
    if payload["result"]["command"] != command:
        raise AssertionError(f"Unexpected command result: {payload['result']['command']}")
    if payload["result"]["sdk_target"] != "overgate_only":
        raise AssertionError(f"{command} must use Overgate-only SDK target")
    if payload["diagnostic_bundle"]["redaction_policy"] != "secret_free_refs_only":
        raise AssertionError(f"{command} diagnostic bundle must stay redacted")
    assert_secret_free(json.dumps(payload, sort_keys=True), command)
    return payload["result"]


def assert_timeline_refs(timeline: dict[str, Any]) -> None:
    missing_refs = EXECUTION_OWNER_REFS - set(timeline["owning_service_refs"])
    if missing_refs:
        raise AssertionError(f"execution_timeline omitted owner refs: {sorted(missing_refs)}")
    if timeline["direct_node_access"] is not False:
        raise AssertionError("execution_timeline exposed direct node access")
    service_refs = {event["service_ref"] for event in timeline["diagnostic_events"]}
    for expected in ["oversched:scheduler", "overlease:lease", "overrun:runner", "overcell:node-heartbeat", "overpack:package", "overstore:result-state-ref"]:
        if expected not in service_refs:
            raise AssertionError(f"execution_timeline omitted diagnostic service ref: {expected}")


def validate_node_behavior() -> None:
    fixture = read_json(NODE_FIXTURE)
    register = run_cli(["node", "register", "--json", *LOCAL_PROFILE_ARGS])
    register_result = validate_envelope(register, "node register")
    if register_result["node_status"]["state"] != "live":
        raise AssertionError("node register must render live node status")
    if register_result["signed"] is not True or not register_result["acceptance"]:
        raise AssertionError("node register must render signed acceptance")

    for target_ref, expected_state in fixture["target_refs"].items():
        payload = run_cli(["node", "inspect", "--json", "--target-ref", target_ref, *LOCAL_PROFILE_ARGS])
        result = validate_envelope(payload, "node inspect")
        status = result["node_status"]
        if status["state"] != expected_state:
            raise AssertionError(f"Node state mismatch for {target_ref}: {status['state']}")
        if status["credential_checked"] is not True:
            raise AssertionError("node_status_record must be credential checked")
        if status["direct_node_access"] is not False:
            raise AssertionError("node_status_record must not expose direct node access")
        if not any(ref.startswith("overcell:node:") for ref in status["capability_refs"]):
            raise AssertionError("node_status_record omitted Overcell capability ref")
        if not any(ref.startswith("overregistry:node:") for ref in status["capability_refs"]):
            raise AssertionError("node_status_record omitted Overregistry capability ref")
        if result["signed"] is not False:
            raise AssertionError("node inspect must remain read-only")

    health = run_cli(["node", "health", "--json", "--target-ref", "node_stale", *LOCAL_PROFILE_ARGS])
    health_result = validate_envelope(health, "node health")
    if health_result["node_status"]["state"] != "stale":
        raise AssertionError("node health did not preserve fixture node state")


def validate_workload_behavior() -> None:
    fixture = read_json(WORKLOAD_FIXTURE)

    logs = run_cli(
        [
            "workload",
            "logs",
            "--json",
            "--workload-ref",
            "workload_local",
            "--wait",
            "--timeout",
            "12000",
            "--poll-interval",
            "500",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    logs_result = validate_envelope(logs, "workload logs")
    if logs_result["execution_state"] != "succeeded":
        raise AssertionError("workload logs must render succeeded execution state by default")
    if logs_result["polling_plan"]["wait"] is not True:
        raise AssertionError("workload logs --wait did not flow into polling_plan")
    if logs_result["polling_plan"]["timeout_ms"] != 12000 or logs_result["polling_plan"]["poll_interval_ms"] != 500:
        raise AssertionError("workload logs polling limits did not flow into polling_plan")
    log_bundle = logs_result["execution_logs"]
    if log_bundle["redaction_policy"] != "secret_free_refs_only":
        raise AssertionError("execution_log_bundle redaction drifted")
    if log_bundle["bounded_streaming"] is not True:
        raise AssertionError("execution_log_bundle must be bounded")
    if log_bundle["contains_private_payload"] is not False or log_bundle["direct_node_path_exposed"] is not False:
        raise AssertionError("execution_log_bundle exposed private payload or node path")
    assert_timeline_refs(logs_result["execution_timeline"])

    result_payload = run_cli(["workload", "result", "--json", "--workload-ref", "workload_local", *LOCAL_PROFILE_ARGS])
    result = validate_envelope(result_payload, "workload result")
    result_ref = result["execution_result"]
    if result_ref["contains_private_payload"] is not False:
        raise AssertionError("execution_result_ref exposed private payload")
    if result_ref["direct_object_store_path_exposed"] is not False:
        raise AssertionError("execution_result_ref exposed object-store path")
    if result_ref["authorized_control_plane_ref"] != "overgate:result:workload_local":
        raise AssertionError("execution_result_ref did not use authorized control-plane ref")

    follow_payload = run_cli(["workload", "follow", "--json", "--workload-ref", "workload_local", *LOCAL_PROFILE_ARGS])
    follow = validate_envelope(follow_payload, "workload follow")
    if follow["polling_plan"]["follow"] is not True or follow["polling_plan"]["wait"] is not True:
        raise AssertionError("workload follow must set follow and wait polling flags")
    if follow["execution_logs"] is None or follow["execution_result"] is None:
        raise AssertionError("workload follow must include logs and result refs")

    cancel_payload = run_cli(
        [
            "workload",
            "cancel",
            "--json",
            "--workload-ref",
            "workload_cancelled",
            "--reason",
            "operator requested",
            *LOCAL_PROFILE_ARGS,
        ]
    )
    cancel = validate_envelope(cancel_payload, "workload cancel")
    if cancel["execution_state"] != "cancelled" or cancel["signed"] is not True:
        raise AssertionError("workload cancel must render signed cancelled state")
    if not cancel["acceptance"] or not cancel_payload["audit_refs"]:
        raise AssertionError("workload cancel must render acceptance and audit refs")

    for target_ref, expected_state in fixture["target_refs"].items():
        if target_ref == "workload_local":
            command = "follow"
        elif target_ref == "workload_cancelled":
            command = "status"
        else:
            command = "status"
        payload = run_cli(["workload", command, "--json", "--workload-ref", target_ref, *LOCAL_PROFILE_ARGS])
        result = validate_envelope(payload, f"workload {command}")
        if result["execution_state"] != expected_state:
            raise AssertionError(f"Workload state mismatch for {target_ref}: {result['execution_state']}")
        timeline = result["execution_timeline"]
        if expected_state not in timeline["states"]:
            raise AssertionError(f"execution_timeline omitted state {expected_state} for {target_ref}")
        if expected_state in TERMINAL_REASON_CODES:
            reason_codes = {event["reason_code"] for event in timeline["diagnostic_events"]}
            if TERMINAL_REASON_CODES[expected_state] not in reason_codes:
                raise AssertionError(f"execution_timeline omitted terminal reason for {expected_state}")
        assert_timeline_refs(timeline)
        if command == "status":
            pending = result["synthetic_workload_pending_state"]
            if pending["queue_state"] != "pending" or pending["execution_implied"] is not False:
                raise AssertionError("workload status must preserve synthetic pending compatibility")


def validate_docs_and_wrapper() -> None:
    sub_plan = read(SUB_PLAN)
    for expected in [
        "### Phase 7 Gate Outputs",
        "Node command gate",
        "Execution command gate",
        "Log/result gate",
        "Wait/follow gate",
        "Execution diagnostics gate",
        "scripts/validate_cli_phase7.py",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    service = read(SERVICE)
    for expected in [
        "## Phase 7 Implementation Gates",
        "Node command gate",
        "Execution command gate",
        "Log/result gate",
        "Wait/follow gate",
        "Execution diagnostics gate",
        "scripts/validate_cli_phase7.py",
    ]:
        assert_contains(service, expected, SERVICE)

    cli_readme = read(CLI / "README.md")
    for expected in [
        "Phase 7 seed private swarm support",
        "node_status_record",
        "execution_timeline",
        "execution_log_bundle",
        "execution_result_ref",
        "polling_plan",
    ]:
        assert_contains(cli_readme, expected, CLI / "README.md")

    contracts_readme = read(CONTRACTS / "README.md")
    for expected in [
        "Phase 7 seed private swarm and execution contracts",
        "node_status_record",
        "workload_execution_state",
        "execution_timeline",
        "execution_log_bundle",
        "execution_result_ref",
        "polling_plan",
        "execution_diagnostic_event",
    ]:
        assert_contains(contracts_readme, expected, CONTRACTS / "README.md")

    phase_plan = read(PHASE_PLAN)
    for expected in ["# CLI Phase 07 Plan", "scripts/validate_cli_phase7.py"]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    phase_progress = read(PHASE_PROGRESS)
    for expected in ["# CLI Phase 07 Progress", "Add Phase 7 validator and wrapper wiring"]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_cli_phase7.py")', VALIDATION_WRAPPER)

    tech_stack = read(TECH_STACK)
    assert_contains(tech_stack, "| CLI | Rust CLI using generated contracts", TECH_STACK)


def validate_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-contracts", "-p", "overrid-sdk", "-p", "overrid-cli"])


def main() -> int:
    try:
        validate_required_files()
        validate_schema_and_manifest()
        validate_fixtures()
        validate_rust_phase7_surface()
        validate_docs_and_wrapper()
        validate_node_behavior()
        validate_workload_behavior()
        validate_cargo_tests()
    except AssertionError as exc:
        print(f"CLI Phase 7 validation failed: {exc}", file=sys.stderr)
        return 1
    print("CLI Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
