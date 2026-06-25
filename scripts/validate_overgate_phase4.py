#!/usr/bin/env python3
"""Validate Overgate Phase 4 credential, actor, tenant, and operator admission gates."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LIB = Path("packages/overgate/src/lib.rs")
ADMISSION = Path("packages/overgate/src/admission.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
ADMIN = Path("packages/overgate/src/admin.rs")
DEPENDENCIES = Path("packages/overgate/src/dependencies.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_PHASE4_FIXTURE = Path("packages/overgate/fixtures/valid/phase4_command.valid.json")
INVALID_REVOKED = Path("packages/overgate/fixtures/invalid/phase4_revoked_credential.invalid.json")
INVALID_ACTOR_DISABLED = Path("packages/overgate/fixtures/invalid/phase4_actor_disabled.invalid.json")
INVALID_TENANT_ROLE = Path("packages/overgate/fixtures/invalid/phase4_tenant_role_denied.invalid.json")
INVALID_SERVICE_BROAD = Path("packages/overgate/fixtures/invalid/phase4_service_account_broad.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_04_progress.md")
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

    assert_contains(lib, "pub mod admission;", LIB)
    assert_contains(lib, "AdmissionContext", LIB)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase4.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 4: Credentials, Signatures, Actor Resolution, And Tenant Authorization",
        SUB_PLAN,
    )
    assert_contains(tech_stack, "Signed command envelopes", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 4", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 4 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact graphs before edits", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    admission = read(ADMISSION)
    routes = read(ROUTES)
    admin = read(ADMIN)
    dependencies = read(DEPENDENCIES)
    errors = read(ERRORS)

    for expected in (
        "PHASE4_ADMISSION_ADAPTER_ID",
        "AdmissionContext",
        "SignatureCheckRecord",
        "ActorResolutionRecord",
        "TenantAuthorizationRecord",
        "ServiceAccountAdmissionRecord",
        "OperatorAdmissionRecord",
        "admit_command",
        "overkey_lite",
        "overpass",
        "overtenant",
        "auth.signature_verified_phase4",
        "auth.actor_resolved_phase4",
        "auth.tenant_authorized_phase4",
        "auth.service_account_admitted_phase4",
        "auth.node_agent_admitted_phase4",
    ):
        assert_contains(admission, expected, ADMISSION)

    for expected in (
        "auth.credential_unknown",
        "auth.signature_expired",
        "auth.signature_replay_window_failed",
        "auth.credential_revoked",
        "auth.credential_rotation_denied",
        "auth.credential_wrong_tenant",
        "auth.key_version_denied",
        "auth.actor_disabled",
        "auth.actor_suspended",
        "auth.actor_deleted",
        "auth.actor_wrong_type",
        "auth.actor_environment_mismatch",
        "auth.tenant_suspended",
        "auth.tenant_membership_denied",
        "auth.tenant_role_denied",
        "auth.service_account_scope_denied",
        "auth.service_account_command_class_denied",
        "auth.node_agent_command_class_denied",
        "auth.service_account_audit_context_required",
        "auth.node_agent_audit_context_required",
        "auth.hardcoded_development_secret_denied",
    ):
        assert_contains(admission, expected, ADMISSION)

    for expected in (
        "PHASE4_RESPONSE_SCHEMA_VERSION",
        "overgate.phase4.response.v0",
        "admit_command",
        "overgate.command_admitted_phase4",
        "not_forwarded_phase4_admission_only",
        "phase4_command_admission_records_signature_actor_tenant_and_service_accounts",
        "phase4_command_admission_denies_credential_actor_tenant_and_service_failures",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "operator_admission_record",
        "overwatch_ready",
        "auth.operator_signature_required",
        "auth.operator_signature_malformed",
        "auth.operator_role_required",
        "auth.cross_tenant_denied",
        "auth.operator_audit_unavailable",
        "admin_audit_ref",
        "signed_operator_tenant_scoped_phase4",
    ):
        assert_contains(admin, expected, ADMIN)

    assert_contains(dependencies, "dependency_ready", DEPENDENCIES)
    for expected in (
        "credential_denied",
        "actor_denied",
        "tenant_denied",
        "service_account_denied",
        "overkey_lite",
        "overpass",
        "overtenant",
    ):
        assert_contains(errors, expected, ERRORS)


def validate_fixtures() -> None:
    valid = load_json(VALID_PHASE4_FIXTURE)
    invalid_cases = [
        (INVALID_REVOKED, "auth.credential_revoked", "overkey_lite", 401),
        (INVALID_ACTOR_DISABLED, "auth.actor_disabled", "overpass", 403),
        (INVALID_TENANT_ROLE, "auth.tenant_role_denied", "overtenant", 403),
        (INVALID_SERVICE_BROAD, "auth.service_account_scope_denied", "overtenant", 403),
    ]

    envelope = valid["command_envelope"]
    for field in REQUIRED_ENVELOPE_FIELDS:
        if field not in envelope:
            raise AssertionError(f"{VALID_PHASE4_FIXTURE} command_envelope missing {field}")
    if envelope["schema_version"] != "shared-schema-package.v0.1":
        raise AssertionError("Phase 4 valid fixture must use shared-schema-package.v0.1")
    if envelope["actor_id"].startswith("service_account:") is False:
        raise AssertionError("Phase 4 valid fixture must exercise service-account admission")
    if envelope["signature_metadata"]["signature_ref"].startswith("signature:") is False:
        raise AssertionError("Phase 4 valid fixture must use typed signature refs")
    expected = valid["expected_response"]
    if expected["reason_code"] != "overgate.command_admitted_phase4":
        raise AssertionError("Phase 4 valid fixture must expect command admission")
    for key, reason in (
        ("signature_reason_code", "auth.signature_verified_phase4"),
        ("actor_reason_code", "auth.actor_resolved_phase4"),
        ("tenant_reason_code", "auth.tenant_authorized_phase4"),
        ("service_account_reason_code", "auth.service_account_admitted_phase4"),
    ):
        if expected[key] != reason:
            raise AssertionError(f"Phase 4 valid fixture has wrong {key}")

    for path, reason_code, dependency_name, status in invalid_cases:
        fixture = load_json(path)
        envelope = fixture["command_envelope"]
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{path} command_envelope missing {field}")
        denial = fixture["expected_denial"]
        if denial["reason_code"] != reason_code:
            raise AssertionError(f"{path} must expect {reason_code}")
        if denial["dependency_name"] != dependency_name:
            raise AssertionError(f"{path} must name dependency {dependency_name}")
        if denial["status"] != status:
            raise AssertionError(f"{path} must expect status {status}")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "phase4_command_admission_records_signature_actor_tenant_and_service_accounts",
        "phase4_command_admission_denies_credential_actor_tenant_and_service_failures",
        "admin_routes_deny_unsigned_non_operator_and_cross_tenant_requests",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"Missing Phase 4 route tests: {', '.join(missing)}")

    for expected in (
        "Phase 4 Admission",
        "overgate.phase4.response.v0",
        "overgate.command_admitted_phase4",
        "not_forwarded_phase4_admission_only",
        "auth.operator_audit_unavailable",
        "fixtures/valid/phase4_command.valid.json",
        "fixtures/invalid/phase4_service_account_broad.invalid.json",
    ):
        assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE4_FIXTURE, PHASE_PLAN, PHASE_PROGRESS):
        text = read(path).lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in text:
                raise AssertionError(f"{path} contains raw secret marker: {marker}")


def main() -> int:
    validate_docs_and_wiring()
    validate_rust_sources()
    validate_fixtures()
    validate_tests_and_readme()
    print("Overgate Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"Overgate Phase 4 validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
