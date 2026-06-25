#!/usr/bin/env python3
"""Validate Overgate Phase 6 rate-limit, quota, policy, and matrix artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LIB = Path("packages/overgate/src/lib.rs")
PRECHECKS = Path("packages/overgate/src/prechecks.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
ADMIN = Path("packages/overgate/src/admin.rs")
SERVICE = Path("packages/overgate/src/service.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_PHASE6_FIXTURE = Path("packages/overgate/fixtures/valid/phase6_command.valid.json")
INVALID_PHASE6_FIXTURE = Path(
    "packages/overgate/fixtures/invalid/phase6_precheck_denials.invalid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_06_progress.md")
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

COMMAND_CLASSES = [
    "low_risk_read",
    "phase1_control_plane_mutation",
    "queue_producing_workload",
    "policy_heavy",
    "accounting_affecting",
    "storage_namespace",
    "native_app_side_effect",
    "admin",
    "break_glass",
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

    assert_contains(lib, "pub mod prechecks;", LIB)
    assert_contains(lib, "PrecheckLimitSummary, PrecheckOutcome, PrecheckStore", LIB)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase6.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 6: Rate Limits, Quota Prechecks, And Policy Handoff",
        SUB_PLAN,
    )
    assert_contains(tech_stack, "stable reason codes", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 6", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 6 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact graphs before edits", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    prechecks = read(PRECHECKS)
    routes = read(ROUTES)
    admin = read(ADMIN)
    service = read(SERVICE)
    errors = read(ERRORS)

    for expected in (
        "PHASE6_PRECHECK_ADAPTER_ID",
        "PHASE6_COMMAND_CLASS_MATRIX_REF",
        "DEFAULT_BUCKET_CAPACITY: u32 = 2",
        "PrecheckStore",
        "PrecheckOutcome",
        "PrecheckInput",
        "CommandClassAdmission",
        "RateLimitDecision",
        "QuotaPrecheckRecord",
        "PolicyCheckRecord",
        "ClientDenialSurface",
        "PrecheckLimitSummary",
        "precheck_command",
        "policy_dry_run",
        "overgate.rate_limited",
        "rate_limit_reset:overgate:phase6",
        "overgate.quota_precheck_denied",
        "no_balance_mutation: true",
        "no_seal_ledger_entry: true",
        "not_settled_by_overgate",
        "overguard_dry_run_handoff_phase6",
        "overguard.policy.v0",
        "stored_policy_truth_in_overgate: false",
        "validate_command_class_matrix",
        "accepted_quota_refs",
        "precheck_digest_ref",
    ):
        assert_contains(prechecks, expected, PRECHECKS)
    for command_class in COMMAND_CLASSES:
        assert_contains(prechecks, command_class, PRECHECKS)

    for expected in (
        "PHASE6_RESPONSE_SCHEMA_VERSION",
        "overgate.phase6.response.v0",
        "PrecheckInput::from_parts",
        "precheck_command",
        "overgate.command_accepted_phase6",
        "overgate.limits_phase6",
        "overgate.policy_dry_run_phase6",
        "phase6_prechecks",
        "accepted_command_quota_refs",
        "phase6_precheck_digest_ref",
        "client_denial_refs",
        "phase6_rate_limits_deny_before_forwarding_and_reset_by_window",
        "phase6_quota_policy_and_dry_run_surfaces_are_structured",
        "phase6_command_class_matrix_covers_required_classes",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "PrecheckStore",
        "pub prechecks: PrecheckStore",
    ):
        assert_contains(service, expected, SERVICE)

    for expected in (
        "rate_limited",
        "overgate.rate_limited",
        "quota_precheck_denied",
        "overgate.quota_precheck_denied",
        "policy_denied",
        "overgate.policy_denied",
        "command_class_matrix_denied",
        "client_denial_refs",
    ):
        assert_contains(errors, expected, ERRORS)

    for expected in (
        "RateLimitBucketView",
        "overgate.admin_rate_limits_phase6",
        "rate_limits:phase6_operator_view",
        "signed_operator_tenant_scoped_phase6",
        "rate_limit_buckets",
        "quota_precheck_refs",
        "policy_decision_refs",
    ):
        assert_contains(admin, expected, ADMIN)


def validate_fixtures() -> None:
    valid = load_json(VALID_PHASE6_FIXTURE)
    invalid = load_json(INVALID_PHASE6_FIXTURE)

    envelope = valid["command_envelope"]
    for field in REQUIRED_ENVELOPE_FIELDS:
        if field not in envelope:
            raise AssertionError(f"{VALID_PHASE6_FIXTURE} command_envelope missing {field}")
    expected = valid["expected_response"]
    expected_pairs = {
        "schema_version": "overgate.phase6.response.v0",
        "reason_code": "overgate.command_accepted_phase6",
        "precheck_state": "prechecked_before_forwarding_phase6",
        "rate_limit_reason_code": "overgate.rate_limit_allowed_phase6",
        "quota_reason_code": "overgate.quota_precheck_allowed_phase6",
        "policy_reason_code": "overgate.policy_not_required_phase6",
        "command_class": "phase1_control_plane_mutation",
        "settlement_state": "not_settled_by_overgate",
    }
    for key, expected_value in expected_pairs.items():
        if expected.get(key) != expected_value:
            raise AssertionError(f"{VALID_PHASE6_FIXTURE} expected_response.{key} is wrong")

    sequence = invalid["rate_limit_sequence"]
    if len(sequence) != 3:
        raise AssertionError("Phase 6 rate-limit fixture must include a three-command sequence")
    for label, envelope in (("rate", item) for item in sequence):
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{INVALID_PHASE6_FIXTURE} {label} envelope missing {field}")
    timestamps = {item["timestamp"] for item in sequence}
    if len(timestamps) != 1:
        raise AssertionError("Rate-limit sequence must share one deterministic window")

    for key in ("quota_denied_command_envelope", "policy_denied_command_envelope"):
        envelope = invalid[key]
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{INVALID_PHASE6_FIXTURE} {key} missing {field}")

    expected_denials = invalid["expected_denials"]
    required_denials = {
        "rate_limit": (429, "overgate.rate_limited", "rate_limit_reset:overgate:phase6:"),
        "quota": (402, "overgate.quota_precheck_denied", "budget:overgate:phase6:"),
        "policy": (403, "overgate.policy_denied", "policy_decision:overguard:phase6:"),
    }
    for key, (status, reason_code, prefix) in required_denials.items():
        denial = expected_denials[key]
        if denial["status"] != status:
            raise AssertionError(f"{key} denial has wrong status")
        if denial["reason_code"] != reason_code:
            raise AssertionError(f"{key} denial has wrong reason_code")
        if denial["client_denial_ref_prefix"] != prefix:
            raise AssertionError(f"{key} denial has wrong client_denial_ref_prefix")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "phase6_rate_limits_deny_before_forwarding_and_reset_by_window",
        "phase6_quota_policy_and_dry_run_surfaces_are_structured",
        "phase6_command_class_matrix_covers_required_classes",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing required tests: {', '.join(missing)}")

    for expected in (
        "Phase 6 Rate Limits, Quota Prechecks, And Policy Handoff",
        "overgate.phase6.response.v0",
        "overgate.command_accepted_phase6",
        "overgate.rate_limited",
        "overgate.quota_precheck_denied",
        "overgate.policy_denied",
        "overgate.limits_phase6",
        "overgate.policy_dry_run_phase6",
        "overgate.admin_rate_limits_phase6",
        "client_denial_refs",
        "fixtures/valid/phase6_command.valid.json",
        "fixtures/invalid/phase6_precheck_denials.invalid.json",
    ):
        assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE6_FIXTURE, INVALID_PHASE6_FIXTURE):
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
        print(f"validate_overgate_phase6 failed: {error}", file=sys.stderr)
        return 1
    print("Overgate Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
