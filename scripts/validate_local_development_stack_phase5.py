#!/usr/bin/env python3
"""Validate Local Development Stack Phase 5 backing stubs."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LOCAL_STACK_CARGO = Path("packages/local_stack/Cargo.toml")
LOCAL_STACK_LIB = Path("packages/local_stack/src/lib.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_004_local_development_stack.md")
PHASE_PLAN = Path("docs/planning/local_development_stack_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/local_development_stack_phase_05_progress.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_FILES = [
    LOCAL_STACK_CARGO,
    LOCAL_STACK_LIB,
    SUB_PLAN,
    PHASE_PLAN,
    PHASE_PROGRESS,
    TECH_STACK,
    SUITE_VALIDATOR,
]

SUPPORTED_SCHEMA = "local-development-stack.v0.1"
BACKING_PHASE_GATE = "phase_5_embedded_state_queue_store"
STATE_KINDS = {
    "tenant",
    "identity",
    "key",
    "manifest",
    "event",
    "fixture_metadata",
    "reset_marker",
    "schema_version",
}
COMPATIBILITY_SURFACES = {
    "local_state",
    "overqueue_job",
    "overstore_artifact_manifest",
    "fixture_manifest",
    "service_endpoint",
}
FORBIDDEN_PRODUCT_MARKERS = (
    "postgresql",
    "mysql",
    "mongodb",
    "dynamodb",
    "redis",
    "kafka",
    "nats",
    "rabbitmq",
    "s3",
    "minio",
    "bucket",
)
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


def check_required_files() -> None:
    for path in REQUIRED_FILES:
        assert_true((REPO_ROOT / path).is_file(), f"missing required file: {path}")


def check_docs_and_source() -> None:
    cargo = read_text(LOCAL_STACK_CARGO)
    source = read_text(LOCAL_STACK_LIB)
    sub_plan = read_text(SUB_PLAN)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    tech_stack = read_text(TECH_STACK)
    suite = read_text(SUITE_VALIDATOR)

    assert_contains(cargo, 'blake3 = "1.5"', LOCAL_STACK_CARGO)
    assert_contains(sub_plan, "## Phase 5: Embedded State, Overqueue, And Overstore Stubs", SUB_PLAN)
    for item in ["**5.1", "**5.2", "**5.3", "**5.4", "**5.5"]:
        assert_contains(sub_plan, item, SUB_PLAN)
    for expected in [
        "Overbase-shaped embedded state records",
        "Overqueue-shaped local job records",
        "BLAKE3/content hash refs",
        "local diagnostic/audit query records",
        "schema-version compatibility gates",
        "scripts/validate_local_development_stack_phase5.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Local Development Stack Phase 5 Progress", PHASE_PROGRESS)
    assert_contains(tech_stack, "Overrid-shaped local stubs", TECH_STACK)
    assert_contains(
        suite,
        'Path("scripts/validate_local_development_stack_phase5.py")',
        SUITE_VALIDATOR,
    )

    for expected in [
        "LOCAL_STACK_PHASE5_BACKING_GATE",
        "LocalStateRecord",
        "LocalQueueJobRecord",
        "LocalArtifactManifest",
        "LocalAuditQueryRecord",
        "SchemaCompatibilityGate",
        "render_local_state_records_json",
        "render_queue_job_records_json",
        "render_artifact_manifests_json",
        "render_audit_query_records_json",
        "render_schema_compatibility_gates_json",
        "blake3::hash",
        "external_database_semantics: false",
        "external_object_store_boundary: false",
        "production_overwatch_authority: false",
        "local_stack.schema_version_incompatible",
    ]:
        assert_contains(source, expected, LOCAL_STACK_LIB)


def check_successful_cli_evidence() -> None:
    status = run_cli_json(["dev", "status", "--json"])
    assert_true(status["ok"] is True, "dev status should pass")
    result = status["result"]
    assert_true(result["backing_phase_gate"] == BACKING_PHASE_GATE, "phase 5 gate drifted")

    local_state_records = result["local_state_records"]
    assert_true({record["record_kind"] for record in local_state_records} == STATE_KINDS, "state kinds drifted")
    for record in local_state_records:
        assert_true(record["schema_version"] == SUPPORTED_SCHEMA, "state schema version drifted")
        assert_true(record["local_only"] is True and record["test_only"] is True, "state markers missing")
        assert_true(record["external_database_semantics"] is False, "state uses database semantics")
        assert_true(record["storage_boundary"] == "overbase_shaped_embedded_state", "state boundary drifted")
        assert_true(
            record["contract_ref"].startswith(("overbase://", "local-state://")),
            "state contract ref is not Overrid-shaped",
        )

    queue_jobs = result["queue_job_records"]
    assert_true(
        sum(job["idempotency_key"] == "idem:local_stack:phase5:smoke" for job in queue_jobs) == 2,
        "duplicate idempotency case missing",
    )
    assert_true(
        any(job["state"] == "duplicate_suppressed" and job["terminal"] is True for job in queue_jobs),
        "duplicate suppression missing",
    )
    assert_true(
        any(job["state"] == "retry_scheduled" and job["retry_count"] == 1 for job in queue_jobs),
        "retry metadata missing",
    )
    assert_true(
        any(
            job["state"] == "dead_letter"
            and job["dead_letter_reason_code"] == "local_stack.job_timeout"
            and job["terminal"] is True
            for job in queue_jobs
        ),
        "dead-letter record missing",
    )
    assert_true(
        all(job["schema_version"] == SUPPORTED_SCHEMA and job["local_only"] and job["test_only"] for job in queue_jobs),
        "queue markers or schema versions drifted",
    )

    artifacts = result["artifact_manifests"]
    assert_true(len(artifacts) == 1, "expected one deterministic local artifact manifest")
    artifact = artifacts[0]
    assert_true(artifact["hash_algorithm"] == "BLAKE3", "artifact hash algorithm drifted")
    assert_true(len(artifact["content_hash"]) == 64, "BLAKE3 hash should be 64 hex chars")
    assert_true(all(character in "0123456789abcdef" for character in artifact["content_hash"]), "hash is not lowercase hex")
    assert_true(artifact["content_address"].endswith(artifact["content_hash"]), "content address does not include hash")
    assert_true(artifact["filesystem_backed"] is True, "artifact should be filesystem-backed local stub")
    assert_true(artifact["external_object_store_boundary"] is False, "artifact modeled as external object store")
    assert_true(artifact["reset_safe"] is True, "artifact reset safety missing")
    assert_true(artifact["local_only"] is True and artifact["test_only"] is True, "artifact markers missing")

    audit_queries = result["audit_query_records"]
    assert_true(audit_queries, "audit query records missing")
    assert_true(
        all(
            record["local_diagnostic_event"] is True
            and record["production_overwatch_authority"] is False
            and record["contains_raw_secret"] is False
            and record["redaction_summary"] == "secret_free"
            for record in audit_queries
        ),
        "audit query records must remain local diagnostics only",
    )
    assert_true(
        any(record["service_id"] == "service:overqueue_jobs" for record in audit_queries),
        "overqueue audit query record missing",
    )

    gates = result["schema_compatibility_gates"]
    assert_true({gate["surface"] for gate in gates} == COMPATIBILITY_SURFACES, "compatibility surfaces drifted")
    assert_true(
        all(
            gate["compatible"] is True
            and gate["schema_version"] == SUPPORTED_SCHEMA
            and gate["blocks_start"] is True
            and gate["blocks_seed"] is True
            and gate["blocks_smoke"] is True
            for gate in gates
        ),
        "compatibility gates are not all passing blockers",
    )

    service_ids = {health["service_id"] for health in result["service_health"]}
    for expected in ["service:overqueue_jobs", "service:overstore_stub", "service:event_audit"]:
        assert_true(expected in service_ids, f"{expected} health missing")

    dependency_status = status["diagnostic_bundle"]["dependency_status"]
    for expected in [
        "overbase_local_state_contract_ready",
        "overqueue_local_jobs_contract_ready",
        "overstore_artifact_stub_ready",
        "local_diagnostic_events_not_authoritative_overwatch",
        "schema_compatibility_gates_passed",
    ]:
        assert_true(expected in dependency_status, f"dependency status missing {expected}")

    rendered = json.dumps(result).lower()
    for marker in FORBIDDEN_PRODUCT_MARKERS:
        assert_true(marker not in rendered, f"local-stack Phase 5 output contains forbidden product marker {marker}")
    for marker in RAW_SECRET_MARKERS:
        assert_true(marker not in rendered, f"local-stack Phase 5 output exposes {marker}")


def check_schema_incompatibility_blocks_start_seed_and_smoke() -> None:
    for command in ["start", "seed", "smoke"]:
        output = run_cli_json(
            ["dev", command, "--json", "--profile", "local-stale-local-state-schema"],
            expect_code=None,
        )
        assert_true(output["ok"] is False, f"dev {command} should fail closed")
        assert_true(output["reason_code"] == "local_stack.schema_version_incompatible", "schema block reason drifted")
        assert_true(output["exit_class"] == "schema", "schema block exit class drifted")
        assert_true("blocked" in output["lifecycle"]["states"], "blocked lifecycle state missing")
        assert_true("starting" not in output["lifecycle"]["states"], "schema block should happen before startup")
        error = output["error"]
        assert_true(error["backing_phase_gate"] == BACKING_PHASE_GATE, "error phase 5 gate missing")
        gates = error["schema_compatibility_gates"]
        assert_true(
            any(
                gate["surface"] == "local_state"
                and gate["compatible"] is False
                and gate["schema_version"] == "local-development-stack.v99.0"
                for gate in gates
            ),
            "local_state incompatible gate missing",
        )
        assert_true(
            any(
                check["check_id"] == "doctor:schema_outputs"
                and check["state"] == "failed"
                and check["reason_code"] == "local_stack.schema_version_incompatible"
                for check in error["doctor_checks"]
            ),
            "schema doctor failure missing",
        )


def check_cargo_tests() -> None:
    run(["cargo", "test", "-p", "overrid-local-stack"])
    run(["cargo", "test", "-p", "overrid-cli", "dev"])


def main() -> int:
    checks = [
        check_required_files,
        check_docs_and_source,
        check_cargo_tests,
        check_successful_cli_evidence,
        check_schema_incompatibility_blocks_start_seed_and_smoke,
    ]
    try:
        for check in checks:
            check()
    except AssertionError as error:
        print(f"Local Development Stack Phase 5 validation failed: {error}", file=sys.stderr)
        return 1

    print("Local Development Stack Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
