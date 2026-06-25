#!/usr/bin/env python3
"""Validate Overgate Phase 7 audit, degraded-WAL, metrics, and ops artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LIB = Path("packages/overgate/src/lib.rs")
AUDIT = Path("packages/overgate/src/audit.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
SERVICE = Path("packages/overgate/src/service.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_PHASE7_FIXTURE = Path("packages/overgate/fixtures/valid/phase7_command.valid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_07_progress.md")
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

ORDERED_PHASE7_EVENTS = [
    "overgate.request_received",
    "overgate.signature_verified",
    "overgate.idempotency_reserved",
    "overgate.command_accepted",
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

    for expected in (
        "pub mod audit;",
        "AuditStore",
        "Phase7AuditEvidence",
        "EmergencyWalStatus",
        "GridOperationsChecklist",
    ):
        assert_contains(lib, expected, LIB)

    assert_contains(suite, 'Path("scripts/validate_overgate_phase7.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 7: Audit, Observability, Degraded Mode, And Grid-Ready Operations",
        SUB_PLAN,
    )
    assert_contains(sub_plan, "Overwatch-compatible ingress events", SUB_PLAN)
    assert_contains(sub_plan, "emergency audit WAL", SUB_PLAN)
    assert_contains(
        tech_stack,
        "Overwatch as the authoritative audit/evidence layer",
        TECH_STACK,
    )
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 7", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 7 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact graphs before edits", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    audit = read(AUDIT)
    routes = read(ROUTES)
    service = read(SERVICE)
    errors = read(ERRORS)

    for expected in (
        "PHASE7_AUDIT_CLIENT_REF",
        "PHASE7_EVENT_TRANSITION_MAP_REF",
        "PHASE7_EMERGENCY_WAL_REF",
        "PHASE7_METRIC_POLICY_REF",
        "PHASE7_GRID_OPERATIONS_REF",
        "AuditStore",
        "AuditGuardInput",
        "AuditDecision",
        "Phase7AuditInput",
        "Phase7AuditEvidence",
        "AuditEventRecord",
        "EventTransition",
        "EmergencyWalConfig",
        "EmergencyWalEntry",
        "EmergencyWalStatus",
        "MetricsTraceSummary",
        "MetricLabelPolicy",
        "GridOperationsChecklist",
        "guard_before_acceptance",
        "record_acceptance",
        "event_transition_map",
        "overgate.request_received",
        "overgate.signature_verified",
        "overgate.schema_denied",
        "overgate.tenant_denied",
        "overgate.idempotency_reserved",
        "overgate.idempotency_replayed",
        "overgate.idempotency_conflict",
        "overgate.rate_limited",
        "overgate.command_accepted",
        "overgate.command_forwarded",
        "overgate.forwarding_failed",
        "none_rust_owned_local_wal",
        "fsync_before_side_effect: true",
        "degraded_until_replayed_to_overwatch",
        "private_data_in_labels: false",
        "raw_payload_in_labels: false",
        "secrets_in_labels: false",
        "tenant.suspend",
        "ledger",
        "accounting",
        "policy.override",
        "break_glass",
    ):
        assert_contains(audit, expected, AUDIT)

    for expected in (
        "AuditGuardInput::from_envelope",
        "Phase7AuditInput::from_parts",
        "record_acceptance",
        "phase7_audit",
        "PHASE7_RESPONSE_SCHEMA_VERSION",
        "phase7_accepted_command_returns_overwatch_compatible_audit_evidence",
        "phase7_overwatch_unavailable_fails_closed_for_high_risk_commands",
        "phase7_emergency_wal_allows_low_risk_phase1_mutation_only",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "pub audit: AuditStore",
        "with_dependencies_and_audit",
    ):
        assert_contains(service, expected, SERVICE)

    for expected in (
        "audit_fail_closed",
        "overgate.audit_fail_closed",
        "overwatch_unavailable_fail_closed_phase7",
        "emergency_audit_wal",
    ):
        assert_contains(errors, expected, ERRORS)


def validate_fixture() -> None:
    fixture = load_json(VALID_PHASE7_FIXTURE)
    if fixture["schema_version"] != "overgate.phase7.local_fixture.v0":
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} has wrong schema_version")
    if fixture["fixture_id"] != "fixture:overgate_phase7_audit_ops_allowed":
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} has wrong fixture_id")

    envelope = fixture["command_envelope"]
    for field in REQUIRED_ENVELOPE_FIELDS:
        if field not in envelope:
            raise AssertionError(f"{VALID_PHASE7_FIXTURE} command_envelope missing {field}")
    if envelope["command_type"] != "overgate.phase7.tenant.profile_update":
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} command_type is wrong")
    if envelope["schema_version"] != "shared-schema-package.v0.1":
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} command schema_version is wrong")
    if envelope["privacy_class"] != "tenant_private":
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} privacy_class is wrong")

    expected = fixture["expected_response"]
    expected_pairs = {
        "schema_version": "overgate.phase6.response.v0",
        "reason_code": "overgate.command_accepted_phase6",
        "phase7_evidence_state": "phase7_audit_evidence_recorded",
        "emergency_wal_enabled": False,
        "hash_chain_verified": True,
        "metric_policy_ref": "metric_label_policy:overgate:phase7",
        "grid_operations_ref": "grid_operations:overgate:phase7",
        "external_log_dependency": "none_overwatch_contract_only",
        "raw_private_payload_stored": False,
        "raw_secret_stored": False,
    }
    for key, expected_value in expected_pairs.items():
        if expected.get(key) != expected_value:
            raise AssertionError(f"{VALID_PHASE7_FIXTURE} expected_response.{key} is wrong")
    if expected.get("ordered_event_types") != ORDERED_PHASE7_EVENTS:
        raise AssertionError(f"{VALID_PHASE7_FIXTURE} ordered_event_types are wrong")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "phase7_accepted_command_returns_overwatch_compatible_audit_evidence",
        "phase7_overwatch_unavailable_fails_closed_for_high_risk_commands",
        "phase7_emergency_wal_allows_low_risk_phase1_mutation_only",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing required tests: {', '.join(missing)}")

    for expected in (
        "Phase 7 Audit, Observability, Degraded Mode, And Grid-Ready Operations",
        "overwatch.audit.v0",
        "event_transition_map:overgate:phase7",
        "overgate.audit_fail_closed",
        "emergency audit WAL",
        "degraded_until_replayed_to_overwatch",
        "Metric labels",
        "fixtures/valid/phase7_command.valid.json",
    ):
        if expected == "Metric labels":
            assert_contains(readme.lower(), "metrics and trace", README)
        else:
            assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE7_FIXTURE):
        text = read(path) if path.suffix != ".json" else json.dumps(load_json(path)).lower()
        lowered = text.lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in lowered:
                raise AssertionError(f"{path} contains raw secret marker {marker!r}")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_fixture()
        validate_tests_and_readme()
    except AssertionError as error:
        print(f"validate_overgate_phase7 failed: {error}", file=sys.stderr)
        return 1
    print("Overgate Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
