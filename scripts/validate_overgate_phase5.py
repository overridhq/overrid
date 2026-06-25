#!/usr/bin/env python3
"""Validate Overgate Phase 5 idempotency, retention, and status artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LIB = Path("packages/overgate/src/lib.rs")
IDEMPOTENCY = Path("packages/overgate/src/idempotency.rs")
RETENTION = Path("packages/overgate/src/retention.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
ADMIN = Path("packages/overgate/src/admin.rs")
SERVICE = Path("packages/overgate/src/service.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_PHASE5_FIXTURE = Path("packages/overgate/fixtures/valid/phase5_command.valid.json")
INVALID_CONFLICT_FIXTURE = Path(
    "packages/overgate/fixtures/invalid/phase5_idempotency_conflict.invalid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_05_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret_value",
)

REQUIRED_ENVELOPE_FIELDS = [
    "command_id",
    "command_type",
    "tenant_id",
    "actor_id",
    "trace_id",
    "idempotency_key",
    "credential_id",
    "schema_version",
    "payload_type",
    "request_hash",
    "payload_hash",
    "timestamp",
    "signature_metadata",
    "privacy_class",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> Any:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required JSON fixture: {path}")
    with full_path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def validate_docs_and_wiring() -> None:
    lib = read(LIB)
    suite = read(SUITE_VALIDATOR)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)

    assert_contains(lib, "pub mod idempotency;", LIB)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase5.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 5: Idempotency, Trace Propagation, Status Views, And Retention",
        SUB_PLAN,
    )
    assert_contains(tech_stack, "idempotency keys", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 5", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 5 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact graphs before edits", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    idempotency = read(IDEMPOTENCY)
    routes = read(ROUTES)
    retention = read(RETENTION)
    admin = read(ADMIN)
    service = read(SERVICE)
    errors = read(ERRORS)

    for expected in (
        "IdempotencyStore",
        "IdempotencyRecord",
        "IdempotencyMutation",
        "IdempotencyOutcome",
        "IdempotencyReservationInput",
        "reserve_or_replay",
        "response_digest_ref",
        "private_payload_disclosed: false",
        "credential_material_disclosed: false",
        "overgate.idempotency_reserved",
        "overgate.idempotency_replayed",
        "idempotency.request_hash_conflict",
        "pending_forwarding_phase5",
        "trace_summary",
        "limit_summary",
        "expire_record",
    ):
        assert_contains(idempotency, expected, IDEMPOTENCY)

    for expected in (
        "PHASE5_RESPONSE_SCHEMA_VERSION",
        "overgate.phase5.response.v0",
        "IdempotencyReservationInput::from_envelope",
        "overgate.command_accepted_phase5",
        "overgate.idempotency_replayed",
        "overgate.command_status_phase5",
        "overgate.trace_summary_phase5",
        "overgate.limits_phase5",
        "OvergateError::status_visibility_denied",
        "OvergateError::status_not_found",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "overgate.phase5.classed_hashes_and_refs_only",
        "bodyless_read_trace_cache",
        "low_risk_metadata_write",
        "control_plane_mutation",
        "queue_producing_workload_command",
        "finality_or_rights_command",
        "retention_extension:dispute_ref",
        "retention_extension:retry_ref",
        "retention_extension:incident_ref",
        "retention_extension:finality_ref",
        "overgate.phase5.expire_after_window_unless_extension_refs_hold",
    ):
        assert_contains(retention, expected, RETENTION)

    assert_contains(service, "IdempotencyStore", SERVICE)
    assert_contains(service, "pub idempotency: IdempotencyStore", SERVICE)

    for expected in (
        "IdempotencyRecord",
        "admin_records_for_key",
        "expire_record(&operator_tenant",
        "cross_tenant_idempotency_expire_denied",
        "overgate.admin_idempotency_record_not_found",
        "overgate.admin_idempotency_lookup_phase5",
        "overgate.admin_idempotency_expire_phase5",
        "idempotency_records",
    ):
        assert_contains(admin, expected, ADMIN)

    for expected in (
        "idempotency_conflict",
        "overgate.idempotency_conflict",
        "status_not_found",
        "overgate.status_not_found",
        "status_visibility_denied",
        "overgate.status_visibility_denied",
    ):
        assert_contains(errors, expected, ERRORS)


def validate_fixtures() -> None:
    valid = load_json(VALID_PHASE5_FIXTURE)
    conflict = load_json(INVALID_CONFLICT_FIXTURE)

    envelope = valid["command_envelope"]
    for field in REQUIRED_ENVELOPE_FIELDS:
        if field not in envelope:
            raise AssertionError(f"{VALID_PHASE5_FIXTURE} command_envelope missing {field}")
    if envelope["schema_version"] != "shared-schema-package.v0.1":
        raise AssertionError("Phase 5 valid fixture must use shared-schema-package.v0.1")
    expected = valid["expected_response"]
    expected_pairs = {
        "schema_version": "overgate.phase5.response.v0",
        "reason_code": "overgate.command_accepted_phase5",
        "idempotency_reason_code": "overgate.idempotency_reserved",
        "idempotency_retention_class": "control_plane_mutation",
        "forwarding_state": "pending_forwarding_phase5",
        "response_digest_ref_prefix": "hash:overgate:",
    }
    for key, expected_value in expected_pairs.items():
        if expected.get(key) != expected_value:
            raise AssertionError(f"{VALID_PHASE5_FIXTURE} expected_response.{key} is wrong")

    first = conflict["first_command_envelope"]
    second = conflict["conflicting_command_envelope"]
    for label, envelope in (("first", first), ("conflicting", second)):
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{INVALID_CONFLICT_FIXTURE} {label} envelope missing {field}")
    for field in ("tenant_id", "actor_id", "command_type", "idempotency_key", "credential_id"):
        if first[field] != second[field]:
            raise AssertionError(f"Conflict fixture must keep {field} equal across both requests")
    if first["request_hash"] == second["request_hash"]:
        raise AssertionError("Conflict fixture must change request_hash")
    denial = conflict["expected_denial"]
    if denial["status"] != 409:
        raise AssertionError("Conflict fixture must expect HTTP 409")
    if denial["reason_code"] != "overgate.idempotency_conflict":
        raise AssertionError("Conflict fixture must expect overgate.idempotency_conflict")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "phase5_idempotency_reserves_replays_and_conflicts_by_request_hash",
        "phase5_status_trace_and_limits_are_tenant_filtered",
        "phase5_retention_classes_cover_control_queue_finality_and_extension_refs",
        "phase4_command_admission_denies_credential_actor_tenant_and_service_failures",
        "admin_routes_deny_unsigned_non_operator_and_cross_tenant_requests",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing required tests: {', '.join(missing)}")

    for expected in (
        "Phase 5 Idempotency And Status",
        "overgate.phase5.response.v0",
        "overgate.command_accepted_phase5",
        "overgate.idempotency_replayed",
        "overgate.idempotency_conflict",
        "overgate.command_status_phase5",
        "overgate.trace_summary_phase5",
        "overgate.limits_phase5",
        "fixtures/valid/phase5_command.valid.json",
        "fixtures/invalid/phase5_idempotency_conflict.invalid.json",
    ):
        assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE5_FIXTURE, INVALID_CONFLICT_FIXTURE):
        text = read(path) if path.suffix != ".json" else json.dumps(load_json(path)).lower()
        lowered = text.lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in lowered:
                raise AssertionError(f"{path} contains raw secret marker {marker!r}")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_fixtures()
        validate_tests_and_readme()
    except AssertionError as error:
        print(f"validate_overgate_phase5 failed: {error}", file=sys.stderr)
        return 1
    print("Overgate Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
