#!/usr/bin/env python3
"""Validate Overgate Phase 9 admin views and client ergonomics artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

ADMIN = Path("packages/overgate/src/admin.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
IDEMPOTENCY = Path("packages/overgate/src/idempotency.rs")
PRECHECKS = Path("packages/overgate/src/prechecks.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_FIXTURE = Path("packages/overgate/fixtures/valid/phase9_admin_views.valid.json")
INVALID_FIXTURE = Path("packages/overgate/fixtures/invalid/phase9_admin_protected_records.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_09_progress.md")
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
    sub_plan = read(SUB_PLAN)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)
    readme = read(README)

    assert_contains(
        sub_plan,
        "## Phase 9: Admin Views, Tenant-Isolated Operations, And Client Ergonomics",
        SUB_PLAN,
    )
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 9", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 9 Progress", PHASE_PROGRESS)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase9.py")', SUITE_VALIDATOR)
    for expected in (
        "Phase 9 Admin Views, Tenant-Isolated Operations, And Client Ergonomics",
        "fixtures/valid/phase9_admin_views.valid.json",
        "fixtures/invalid/phase9_admin_protected_records.invalid.json",
        "client_response_shape:overgate:phase9",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    admin = read(ADMIN)
    routes = read(ROUTES)
    idempotency = read(IDEMPOTENCY)
    prechecks = read(PRECHECKS)
    errors = read(ERRORS)

    for expected in (
        "PHASE9_RESPONSE_SCHEMA_VERSION",
        "overgate.admin_ingress_lookup_phase9",
        "overgate.admin_idempotency_lookup_phase9",
        "overgate.admin_idempotency_expire_phase9",
        "overgate.admin_rate_limits_phase9",
        "overgate.admin_idempotency_expiration_refused_phase9",
        "AdminRecordLookup",
        "admin_records_for_request",
        "private_metadata_redacted_phase9",
        "budget_grant_and_tenant_private_values_redacted_phase9",
        "incident_hook_refs",
        "operator_runbook_steps",
        "ClientResponseShape::new",
    ):
        assert_contains(admin, expected, ADMIN)

    for expected in (
        "PHASE9_CLIENT_RESPONSE_SHAPE_REF",
        "ClientResponseShape",
        "free_form_message_required: false",
        "phase9_admin_ingress_lookup_is_tenant_scoped_and_redacted",
        "phase9_idempotency_expiration_refuses_finality_records",
        "phase9_admin_rate_limits_include_quota_diagnostics_and_runbooks",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "AdminRecordLookup",
        "Protected(&'static str)",
        "expiration_protection_reason",
        "active_record_refused_phase9",
        "disputed_record_refused_phase9",
        "incident_linked_record_refused_phase9",
        "retry_linked_record_refused_phase9",
        "finality_protected_record_refused_phase9",
    ):
        assert_contains(idempotency, expected, IDEMPOTENCY)

    for expected in (
        "local_counter_refs",
        "grant_placeholder_refs",
        "denial_reason_distribution",
        "DenialReasonBucket",
    ):
        assert_contains(prechecks, expected, PRECHECKS)

    for expected in (
        "client_contract_ref",
        "free_form_message_required",
        "client_response_shape:overgate:phase9",
    ):
        assert_contains(errors, expected, ERRORS)


def validate_fixtures() -> None:
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    if valid["schema_version"] != "overgate.phase9.local_fixture.v0":
        raise AssertionError(f"{VALID_FIXTURE} has wrong schema_version")
    expected = valid["expected_response"]
    expected_pairs = {
        "schema_version": "overgate.phase9.response.v0",
        "client_contract_ref": "client_response_shape:overgate:phase9",
        "ingress_reason_code": "overgate.admin_ingress_lookup_phase9",
        "idempotency_lookup_reason_code": "overgate.admin_idempotency_lookup_phase9",
        "idempotency_expire_reason_code": "overgate.admin_idempotency_expire_phase9",
        "rate_limits_reason_code": "overgate.admin_rate_limits_phase9",
        "free_form_message_required": False,
        "incident_hooks_required": True,
        "runbook_steps_required": True,
        "tenant_scoped": True,
    }
    for key, expected_value in expected_pairs.items():
        if expected.get(key) != expected_value:
            raise AssertionError(f"{VALID_FIXTURE} expected_response.{key} is wrong")

    routes = set(valid["admin_routes"])
    for route in (
        "GET /v1/admin/ingress/{request_id}",
        "GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}",
        "POST /v1/admin/idempotency/{record_id}/expire",
        "GET /v1/admin/rate-limits",
    ):
        if route not in routes:
            raise AssertionError(f"{VALID_FIXTURE} missing route {route}")

    if invalid["schema_version"] != "overgate.phase9.invalid_fixture.v0":
        raise AssertionError(f"{INVALID_FIXTURE} has wrong schema_version")
    cases = {case["case_id"]: case for case in invalid["invalid_cases"]}
    required_denials = {
        "cross_tenant_ingress_lookup": "cross_tenant_ingress_lookup_denied",
        "finality_protected_expiration": "finality_protected_record_refused_phase9",
        "incident_linked_expiration": "incident_linked_record_refused_phase9",
        "active_record_expiration": "active_record_refused_phase9",
    }
    for case_id, denial in required_denials.items():
        case = cases.get(case_id)
        if not case:
            raise AssertionError(f"{INVALID_FIXTURE} missing invalid case {case_id}")
        if case.get("expected_denial") != denial:
            raise AssertionError(f"{INVALID_FIXTURE} {case_id} denial is wrong")

    for path in (VALID_FIXTURE, INVALID_FIXTURE):
        lower = read(path).lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in lower:
                raise AssertionError(f"{path} contains raw secret marker {marker!r}")


def main() -> int:
    validate_docs_and_wiring()
    validate_rust_sources()
    validate_fixtures()
    print("Overgate Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as exc:
        print(f"Overgate Phase 9 validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)
