#!/usr/bin/env python3
"""Validate Local Development Stack Phase 10 closure and handoff evidence."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
SDS = Path("docs/sds/foundation/local_development_stack.md")
SERVICE_CATALOG = Path("docs/service_catalog/foundation/local_development_stack.md")
MASTER_PLAN = Path("docs/build_plan/master_plan.md")
CROSSWALK = Path("docs/build_plan/service_catalog_alignment.md")
BUILD_PROGRESS = Path("docs/build_plan/progress.md")
QUEUE_STATE = Path(".codex55_sds_queue/state.json")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_10_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_LIB,
    SUB_PLAN,
    SDS,
    SERVICE_CATALOG,
    MASTER_PLAN,
    CROSSWALK,
    BUILD_PROGRESS,
    QUEUE_STATE,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

PHASE10_GATE = "phase_10_validation_documentation_handoff"
EXPECTED_SURFACES = {
    "schemas",
    "manifest_loading",
    "port_registry",
    "env_manifest",
    "secret_records",
    "lifecycle",
    "reset",
    "seed",
    "smoke",
    "health",
    "logs",
    "diagnostics",
    "artifacts",
}
EXPECTED_HANDOFF_PHASES = {
    "phase_1_control_plane",
    "phase_2_private_swarm",
    "phase_3_execution_loop",
    "phase_4_policy_trust",
    "phase_5_accounting",
    "phase_6_product_integration",
    "phase_7_grid_resident_backbone",
    "phase_8_data_storage_namespace",
    "phase_9_deployment_release",
    "phase_10_federation_public_interest",
    "phase_11_public_provider_sandbox",
    "phase_12_native_apps",
    "phase_13_governance_compliance",
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


def load_json(path: Path) -> dict[str, Any]:
    return json.loads(read_text(path))


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_contains(text: str, expected: str, source: Path) -> None:
    assert_true(expected in text, f"{source} missing required text: {expected}")


def require_queue_task(tasks: dict[str, Any], task_id: str) -> dict[str, Any]:
    task = tasks.get(task_id)
    assert_true(isinstance(task, dict), f"queue state missing {task_id}")
    assert_true(task.get("task_id") == task_id, f"{task_id} task_id drifted")
    return task


def assert_task_completed(task: dict[str, Any], task_id: str) -> None:
    assert_true(task.get("status") == "complete", f"{task_id} is not complete")
    assert_true(task.get("last_exit_code") == 0, f"{task_id} did not exit cleanly")
    assert_true(task.get("timed_out") is False, f"{task_id} timed out")


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
    assert_true(isinstance(body, dict), "CLI envelope did not contain result or error body")
    return body


def assert_secret_free(payload: dict[str, Any], context: str) -> None:
    rendered = json.dumps(payload, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"{context} exposes raw secret marker {marker}")
    body = envelope_body(payload)
    for event in body.get("lifecycle_events", []):
        assert_true(event["contains_raw_secret"] is False, f"{context} event exposes raw secret")
        assert_true(
            event["redaction_summary"] == "secret_free",
            f"{context} event redaction drifted",
        )
        assert_true(
            event["local_only"] is True and event["test_only"] is True,
            f"{context} event scope drifted",
        )


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_docs_source_and_queue() -> None:
    source = read_text(LOCAL_STACK_LIB)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)
    queue_state = load_json(QUEUE_STATE)

    for expected in [
        "Validate local-stack self-consistency",
        "Validate tech-stack alignment",
        "Validate security and environment separation",
        "Validate documentation links and queue evidence",
        "Prepare downstream phase handoff",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)
    for item in ["**10.1", "**10.2", "**10.3", "**10.4", "**10.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)

    for expected in [
        "Local Development Stack Phase 10 Plan",
        "Complete SUB BUILD PLAN #4 Phase 10",
        "packages/local_stack/src/lib.rs",
        "scripts/validate_local_development_stack_phase10.py",
        "Rust-first",
        "local/test-only",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    for expected in [
        "Local Development Stack Phase 10 Progress",
        "Validation Evidence",
        "004-build-plan",
        "004-phase-10-work",
        "004-phase-10-control",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)
    for expected in [
        "Rust-first infrastructure stack",
        "JSON plus JSON Schema",
        "Native Overqueue durable state and events, not NATS/Kafka/Redis",
        "Native Overstore",
        "Explicit Non-Choices",
        "Local development",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase10.py")',
        SUITE_VALIDATOR,
    )

    tasks = queue_state.get("tasks", {})
    assert_true(isinstance(tasks, dict), "queue state tasks must be an object")
    assert_task_completed(require_queue_task(tasks, "004-build-plan"), "004-build-plan")
    phase_work = require_queue_task(tasks, "004-phase-10-work")
    assert_task_completed(phase_work, "004-phase-10-work")
    assert_true(phase_work.get("phase") == 10, "004-phase-10-work phase drifted")
    assert_true(phase_work.get("stage") == "phase_work", "004-phase-10-work stage drifted")
    phase_control = require_queue_task(tasks, "004-phase-10-control")
    assert_true(phase_control.get("phase") == 10, "004-phase-10-control phase drifted")
    assert_true(
        phase_control.get("stage") == "phase_control",
        "004-phase-10-control stage drifted",
    )
    assert_true(
        phase_control.get("status") in {"running", "complete"},
        "004-phase-10-control must be recorded as running or complete",
    )
    if phase_control.get("status") == "complete":
        assert_true(
            phase_control.get("last_exit_code") == 0,
            "004-phase-10-control did not exit cleanly",
        )

    for expected in [
        "LOCAL_STACK_PHASE10_VALIDATION_GATE",
        "LocalSelfConsistencyValidationRecord",
        "LocalTechStackAlignmentRecord",
        "LocalSecurityEnvironmentSeparationRecord",
        "LocalDocumentationQueueEvidenceRecord",
        "LocalDownstreamHandoffRule",
        "self_consistency_validations",
        "tech_stack_alignment_checks",
        "security_environment_checks",
        "documentation_queue_evidence",
        "downstream_handoff_rules",
        "render_self_consistency_validations_json",
        "render_tech_stack_alignment_checks_json",
        "render_security_environment_checks_json",
        "render_documentation_queue_evidence_json",
        "render_downstream_handoff_rules_json",
        "phase10_self_consistency_validation_covers_full_local_stack_surface",
        "phase10_tech_stack_alignment_rejects_conventional_cloud_boundaries",
        "phase10_security_environment_separation_denies_non_local_and_payment_paths",
        "phase10_documentation_queue_evidence_links_sds4_and_build_plan",
        "phase10_downstream_handoff_rules_preserve_owner_contracts_and_master_order",
        "local_stack_self_consistency_validated",
        "local_stack_tech_stack_alignment_validated",
        "local_stack_security_environment_separation_validated",
        "local_stack_documentation_queue_evidence_ready",
        "local_stack_downstream_handoff_rules_ready",
        PHASE10_GATE,
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack", "phase10"])


def check_self_consistency(result: dict[str, Any], dependency_status: list[str]) -> None:
    assert_true(result["validation_phase_gate"] == PHASE10_GATE, "phase 10 gate drifted")
    records = result["self_consistency_validations"]
    assert_true(len(records) == 1, "expected one self-consistency validation record")
    record = records[0]
    assert_true(EXPECTED_SURFACES.issubset(set(record["surfaces"])), "self-consistency surfaces missing")
    for key in [
        "schemas_checked",
        "manifest_loading_checked",
        "ports_checked",
        "env_generation_checked",
        "secret_records_checked",
        "lifecycle_checked",
        "reset_seed_smoke_checked",
        "health_logs_diagnostics_artifacts_checked",
        "coherent_stack",
        "local_only",
        "test_only",
    ]:
        assert_true(record[key] is True, f"self-consistency {key} must be true")
    assert_true(
        "local_stack_self_consistency_validated" in dependency_status,
        "dependency status missing self-consistency evidence",
    )


def check_tech_stack(result: dict[str, Any], dependency_status: list[str]) -> None:
    records = result["tech_stack_alignment_checks"]
    assert_true(len(records) == 1, "expected one tech-stack alignment record")
    check = records[0]
    for key in [
        "rust_first_local_tooling",
        "json_schema_contracts",
        "overbase_shaped_state",
        "overqueue_shaped_jobs",
        "overstore_shaped_artifacts",
        "local_only",
        "test_only",
    ]:
        assert_true(check[key] is True, f"tech-stack {key} must be true")
    for key in [
        "external_database_boundary",
        "external_queue_boundary",
        "external_object_store_boundary",
        "vault_boundary",
        "blockchain_or_nft_boundary",
        "pricing_revenue_customer_assumptions",
    ]:
        assert_true(check[key] is False, f"tech-stack {key} must be false")
    assert_true(
        "local_stack_tech_stack_alignment_validated" in dependency_status,
        "dependency status missing tech-stack evidence",
    )


def check_security(result: dict[str, Any], dependency_status: list[str]) -> None:
    records = result["security_environment_checks"]
    assert_true(len(records) == 1, "expected one security environment record")
    check = records[0]
    for key in [
        "loopback_binding_enforced",
        "profile_class_checked",
        "reset_marker_required",
        "fixture_credentials_isolated",
        "secret_redaction_verified",
        "diagnostic_redaction_verified",
        "non_local_profile_denied",
        "seed_staging_production_like_denied",
        "federation_public_provider_denied",
        "payment_provider_payout_denied",
        "local_only",
        "test_only",
    ]:
        assert_true(check[key] is True, f"security {key} must be true")
    assert_true(
        "local_stack_security_environment_separation_validated" in dependency_status,
        "dependency status missing security evidence",
    )


def check_docs_evidence(result: dict[str, Any], dependency_status: list[str]) -> None:
    records = result["documentation_queue_evidence"]
    assert_true(len(records) == 1, "expected one docs/queue evidence record")
    evidence = records[0]
    assert_true(evidence["build_plan_task_id"] == "004-build-plan", "build-plan task id drifted")
    for expected in [
        str(SDS),
        str(SERVICE_CATALOG),
        str(SUB_PLAN),
        str(MASTER_PLAN),
        str(CROSSWALK),
        str(BUILD_PROGRESS),
    ]:
        assert_true(expected in evidence["documentation_refs"], f"missing doc ref {expected}")
    assert_true(
        any("004-build-plan" in reference for reference in evidence["queue_task_refs"]),
        "queue evidence missing 004-build-plan",
    )
    assert_true(
        any("004-phase-10-work" in reference for reference in evidence["queue_task_refs"]),
        "queue evidence missing 004-phase-10-work",
    )
    assert_true(
        any("004-phase-10-control" in reference for reference in evidence["queue_task_refs"]),
        "queue evidence missing 004-phase-10-control",
    )
    for key in [
        "markdown_links_checked",
        "queue_marks_004_build_plan_complete",
        "docdex_search_returns_sds4_plan",
        "progress_evidence_recorded",
        "local_only",
        "test_only",
    ]:
        assert_true(evidence[key] is True, f"documentation evidence {key} must be true")
    assert_true(
        "local_stack_documentation_queue_evidence_ready" in dependency_status,
        "dependency status missing documentation evidence",
    )


def check_handoff(result: dict[str, Any], dependency_status: list[str]) -> None:
    records = result["downstream_handoff_rules"]
    assert_true(len(records) == len(EXPECTED_HANDOFF_PHASES), "handoff phase count drifted")
    phases = {record["target_phase"] for record in records}
    assert_true(EXPECTED_HANDOFF_PHASES == phases, "handoff target phases drifted")
    for record in records:
        assert_true(record["owning_contract_required"] is True, "handoff must require owner contract")
        assert_true(record["local_test_marker_required"] is True, "handoff must require local test marker")
        assert_true(record["bypasses_owner_contracts"] is False, "handoff may not bypass owner contracts")
        assert_true(record["moves_master_order"] is False, "handoff may not move master order")
        assert_true(record["weakens_local_test_safety"] is False, "handoff may not weaken local safety")
        assert_true(record["local_only"] is True and record["test_only"] is True, "handoff scope drifted")
        assert_true(len(record["allowed_additions"]) >= 3, "handoff allowed additions too thin")
        assert_true(
            record["diagnostic_artifact_expectation"].startswith(
                "artifact://local_stack/handoff/"
            ),
            "handoff diagnostic artifact expectation drifted",
        )
    assert_true(
        "local_stack_downstream_handoff_rules_ready" in dependency_status,
        "dependency status missing handoff evidence",
    )


def check_cli_status_and_doctor() -> None:
    status = run_cli_json(
        [
            "dev",
            "status",
            "--json",
            "--trace-id",
            "trace_phase10_validator_status",
        ]
    )
    assert_true(status["ok"] is True, "phase 10 status should pass")
    status_result = status["result"]
    status_deps = status["diagnostic_bundle"]["dependency_status"]
    check_self_consistency(status_result, status_deps)
    check_tech_stack(status_result, status_deps)
    check_security(status_result, status_deps)
    check_docs_evidence(status_result, status_deps)
    check_handoff(status_result, status_deps)
    assert_secret_free(status, "phase 10 status")

    doctor = run_cli_json(
        [
            "dev",
            "doctor",
            "--json",
            "--trace-id",
            "trace_phase10_validator_doctor",
        ]
    )
    assert_true(doctor["ok"] is True, "phase 10 doctor should pass")
    doctor_result = doctor["result"]
    doctor_deps = doctor["diagnostic_bundle"]["dependency_status"]
    check_self_consistency(doctor_result, doctor_deps)
    check_tech_stack(doctor_result, doctor_deps)
    check_security(doctor_result, doctor_deps)
    check_docs_evidence(doctor_result, doctor_deps)
    check_handoff(doctor_result, doctor_deps)
    assert_secret_free(doctor, "phase 10 doctor")


def main() -> int:
    check_required_files()
    check_docs_source_and_queue()
    check_cargo_tests()
    check_cli_status_and_doctor()
    print("Local Development Stack Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
