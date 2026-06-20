#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 7 golden trace assertions."""

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
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_07_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

VALID_FIXTURES = [
    CONTRACTS / "fixtures/valid/integration_harness_phase7_exact_protocol.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase7_execution_dag.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase7_policy_dispute.valid.json",
    CONTRACTS / "fixtures/valid/integration_harness_phase7_accounting_ledger.valid.json",
]
INVALID_FIXTURES = [
    CONTRACTS / "fixtures/invalid/golden_trace_missing_edge.invalid.json",
    CONTRACTS / "fixtures/invalid/golden_trace_missing_event.invalid.json",
    CONTRACTS / "fixtures/invalid/golden_trace_extra_state_event.invalid.json",
    CONTRACTS / "fixtures/invalid/golden_trace_reordered_exact.invalid.json",
    CONTRACTS / "fixtures/invalid/golden_trace_forbidden_transition.invalid.json",
]

EXPECTED_SCENARIOS = {
    "scenario_phase7_exact_protocol_trace": {
        "fixture": VALID_FIXTURES[0],
        "mode": "exact",
        "assertion": "assertion_phase7_exact_protocol_trace",
        "nodes": [
            "event_signed_command_admitted",
            "event_schema_denial_observed",
            "event_tenant_created",
            "event_identity_created",
            "event_key_created",
            "event_audit_written",
            "event_audit_read",
            "event_idempotency_observed",
            "event_pending_queue_transition",
        ],
    },
    "scenario_phase7_execution_dag_trace": {
        "fixture": VALID_FIXTURES[1],
        "mode": "dag",
        "assertion": "assertion_phase7_execution_dag_trace",
        "nodes": [
            "event_queue_admitted",
            "event_scheduler_dispatched",
            "event_lease_acquired",
            "event_runner_started",
            "event_result_recorded",
            "event_retry_evaluated",
            "event_cancellation_checked",
            "event_timeout_evaluated",
            "event_dead_letter_checked",
            "event_usage_recorded",
            "event_audit_written",
        ],
    },
    "scenario_phase7_policy_dispute_trace": {
        "fixture": VALID_FIXTURES[2],
        "mode": "dag",
        "assertion": "assertion_phase7_policy_dispute_trace",
        "nodes": [
            "event_policy_package_loaded",
            "event_verification_requested",
            "event_verification_decided",
            "event_dispute_window_opened",
            "event_dispute_evidence_recorded",
            "event_policy_audit_written",
        ],
    },
    "scenario_phase7_accounting_ledger_trace": {
        "fixture": VALID_FIXTURES[3],
        "mode": "dag",
        "assertion": "assertion_phase7_accounting_ledger_trace",
        "nodes": [
            "event_usage_metered",
            "event_ledger_debited",
            "event_receipt_written",
            "event_receipt_read",
            "event_ledger_audit_written",
        ],
    },
}

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
            prefix = lowered[max(0, index - 80) : index]
            if "do not add" not in prefix and "must not" not in prefix and "not as" not in prefix:
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
        HARNESS / "src/assertions.rs",
        HARNESS / "src/runner.rs",
        HARNESS / "src/manifests.rs",
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
        "Phase 7: Golden Trace Assertions And Cross-Service Contract Coverage",
        "Implement exact Phase 0/1 protocol traces",
        "Implement partially ordered DAG trace checks",
        "Add execution-loop trace coverage",
        "Add policy, verification, and dispute trace coverage",
        "Add usage, ledger, and receipt trace coverage",
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
        "Integration Test Harness Phase 7 Implementation Plan",
        "packages/integration_harness/src/assertions.rs",
        "scripts/validate_integration_harness_phase7.py",
        "exact Phase 0/1 protocol coverage",
        "usage/ledger/receipt DAG coverage",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 7 Progress",
        "Validation Evidence",
        "packages/integration_harness/src/assertions.rs",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(
        wrapper,
        "scripts/validate_integration_harness_phase7.py",
        VALIDATION_WRAPPER,
    )


def validate_codegen_manifest() -> None:
    manifest = load_json(CONTRACTS / "codegen_manifest.json")
    phase7 = manifest.get("integration_harness_phase7")
    assert_true(isinstance(phase7, dict), "codegen manifest missing integration_harness_phase7")
    assert_true(phase7.get("schema_family") == "integration-harness", "wrong Phase 7 schema family")
    assert_true(phase7.get("schema_version") == "integration-harness.v0.1", "wrong Phase 7 schema version")
    assert_true(phase7.get("rust_first_validation") is True, "Phase 7 must remain Rust-first")
    listed_valid = [Path(value) for value in phase7.get("fixtures", {}).get("valid", [])]
    listed_invalid = [Path(value) for value in phase7.get("fixtures", {}).get("invalid", [])]
    assert_true(listed_valid == VALID_FIXTURES, "Phase 7 valid fixture list drifted")
    assert_true(listed_invalid == INVALID_FIXTURES, "Phase 7 invalid fixture list drifted")
    guardrails = " ".join(phase7.get("guardrails", []))
    for expected in [
        "Golden trace assertions",
        "Execution-loop traces",
        "Policy, verification, dispute, usage, ledger, and receipt traces",
    ]:
        assert_contains(guardrails, expected, CONTRACTS / "codegen_manifest.json")


def validate_fixture(scenario_id: str, expectation: dict[str, Any]) -> None:
    path = expectation["fixture"]
    document = load_json(path)
    fixture = document["fixture_manifest"]
    scenario = document["scenario_manifest"]
    run_record = document["test_run_record"]
    assertion = document["assertion_result"]
    trace = document["golden_trace"]
    bundle = document["artifact_bundle"]

    assert_true(fixture["test_only"] is True, f"{path} fixture must be test_only")
    assert_true(
        all(key["test_only"] is True and key["raw_key_material_present"] is False for key in fixture["keys"]),
        f"{path} fixture keys must be test-only refs",
    )
    assert_true(scenario["scenario_id"] == scenario_id, f"{path} scenario id drifted")
    assert_true(scenario["master_phase"] == 7, f"{path} must be a Phase 7 scenario")
    assert_true("phase7" in scenario["tags"], f"{path} missing phase7 tag")
    assert_true("golden_trace" in scenario["tags"], f"{path} missing golden_trace tag")
    assert_true(run_record["status"] == "passed", f"{path} run status drifted")
    assert_true(assertion["assertion_id"] == expectation["assertion"], f"{path} assertion id drifted")
    assert_true(trace["assertion_mode"] == expectation["mode"], f"{path} trace mode drifted")
    actual_nodes = [node["node_id"] for node in trace["required_nodes"]]
    assert_true(actual_nodes == expectation["nodes"], f"{path} required node list drifted")
    assert_true(len(trace["required_causal_edges"]) >= len(actual_nodes) - 1, f"{path} missing causal edges")
    assert_true(trace["stable_reason_codes"], f"{path} stable reason codes required")
    assert_true(trace["forbidden_transitions"], f"{path} forbidden transitions required")
    assert_true(bundle["contains_raw_secret"] is False, f"{path} bundle must reject raw secrets")
    assert_true(bundle["contains_private_key"] is False, f"{path} bundle must reject private keys")
    assert_true(bundle["contains_token"] is False, f"{path} bundle must reject tokens")
    assert_true(bundle["contains_signature"] is False, f"{path} bundle must reject signatures")
    assert_true(bundle["contains_private_payload"] is False, f"{path} bundle must reject private payloads")


def validate_fixtures() -> None:
    for scenario_id, expectation in EXPECTED_SCENARIOS.items():
        validate_fixture(scenario_id, expectation)

    invalid_expectations = {
        "golden_trace_missing_edge.invalid.json": "golden_trace.edge_missing",
        "golden_trace_missing_event.invalid.json": "golden_trace.event_missing",
        "golden_trace_extra_state_event.invalid.json": "golden_trace.extra_state_event",
        "golden_trace_reordered_exact.invalid.json": "golden_trace.exact_order_mismatch",
        "golden_trace_forbidden_transition.invalid.json": "golden_trace.forbidden_transition",
    }
    for path in INVALID_FIXTURES:
        document = load_json(path)
        assert_true(
            document.get("expected_error") == invalid_expectations[path.name],
            f"{path} expected_error drifted",
        )


def validate_rust_surfaces() -> None:
    assertions_rs = read(HARNESS / "src/assertions.rs")
    runner_rs = read(HARNESS / "src/runner.rs")
    lib_rs = read(HARNESS / "src/lib.rs")

    for expected in [
        "pub mod assertions;",
        "pub mod runner;",
    ]:
        assert_contains(lib_rs, expected, HARNESS / "src/lib.rs")

    for expected in [
        "ObservedTraceEvent",
        "ObservedTrace",
        "GoldenTraceAssertion",
        "phase01_protocol_trace_contract",
        "execution_loop_dag_trace_contract",
        "policy_dispute_dag_trace_contract",
        "accounting_ledger_dag_trace_contract",
        "assert_golden_trace",
        "golden_trace.edge_missing",
        "golden_trace.exact_order_mismatch",
        "golden_trace.reason_code_unstable",
        "golden_trace.schema_version_mismatch",
        "diagnostic_extra",
    ]:
        assert_contains(assertions_rs, expected, HARNESS / "src/assertions.rs")

    for expected in [
        "phase7_golden_trace_assertions",
        "observed_trace_from_template",
        "scenario_phase7_exact_protocol_trace",
        "scenario_phase7_execution_dag_trace",
        "scenario_phase7_policy_dispute_trace",
        "scenario_phase7_accounting_ledger_trace",
        "assertion_phase7_exact_protocol_trace",
        "assertion_phase7_execution_dag_trace",
        "assertion_phase7_policy_dispute_trace",
        "assertion_phase7_accounting_ledger_trace",
    ]:
        assert_contains(runner_rs, expected, HARNESS / "src/runner.rs")

    assert_no_forbidden_runtime_authority(assertions_rs, HARNESS / "src/assertions.rs")
    assert_no_forbidden_runtime_authority(runner_rs, HARNESS / "src/runner.rs")


def validate_cli_behavior() -> None:
    listed = run_cli(["test", "list", "--phase", "7", "--json"])
    listed_ids = [scenario["scenario_id"] for scenario in listed["result"]["scenarios"]]
    for scenario_id in EXPECTED_SCENARIOS:
        assert_true(scenario_id in listed_ids, f"{scenario_id} missing from Phase 7 list")

    for scenario_id, expectation in EXPECTED_SCENARIOS.items():
        output = run_cli(["test", "scenario", scenario_id, "--json"])
        assert_true(output["ok"] is True, f"{scenario_id} should be ok")
        result = output["result"]
        assert_true(result["status"] == "passed", f"{scenario_id} status drifted")
        assert_true(result["reason_code"] == "run.passed", f"{scenario_id} reason drifted")
        assert_true(expectation["assertion"] in result["assertion_refs"], f"{scenario_id} missing golden assertion ref")
        assert_true("asserting" in result["lifecycle"], f"{scenario_id} must include asserting lifecycle")
        assert_true(result["step_results"], f"{scenario_id} must emit step results")


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
        print(f"Integration harness Phase 7 validation failed: {exc}", file=sys.stderr)
        return 1

    print("Integration harness Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
