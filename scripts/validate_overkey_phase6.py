#!/usr/bin/env python3
"""Validate Overkey Phase 6 delegation, policy handoff, usage, and operator controls."""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
README = Path("packages/overkey/README.md")
ROUTES = Path("packages/overkey/src/routes.rs")
RECORDS = Path("packages/overkey/src/records.rs")
REPOSITORY = Path("packages/overkey/src/repository.rs")
SCHEMA_RS = Path("packages/overkey/src/schema.rs")
SCHEMA_JSON = Path("packages/schemas/overrid_contracts/v0/overkey_credential.schema.json")
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase6_delegation_policy_usage.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase6_delegation_denials.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_06_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_06_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
    "bearer_token=",
)

PHASE6_DELEGATION_FIELDS = {
    "delegator_tenant_id",
    "delegate_tenant_id",
    "delegator_ref",
    "delegate_ref",
    "allowed_scopes",
    "allowed_command_classes",
    "revocation_state",
    "evidence_refs",
    "policy_decision_ref",
}

PHASE6_DENIAL_REASON_CODES = {
    "auth.delegation_overgate_required",
    "overkey.delegation_evidence_required",
    "overkey.delegation_delegate_stale",
    "overkey.delegation_expired",
    "overkey.delegation_cross_tenant_denied",
    "overkey.delegation_scope_too_broad",
    "policy.overguard_denied",
    "overkey.broad_service_account_scope_rejected",
    "auth.operator_lifecycle_unsigned",
    "auth.operator_lifecycle_evidence_required",
}

PHASE6_RUST_TESTS = {
    "phase6_delegation_requires_overgate_policy_evidence_and_narrow_scope",
    "phase6_service_account_scope_matrix_blocks_adjacent_authority",
    "phase6_last_used_retry_hook_does_not_mutate_accounting",
    "phase6_operator_lifecycle_requires_signed_evidence_and_strong_protection",
}


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
    workspace = read(WORKSPACE_LAYOUT)
    readme = read(README)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)

    assert_contains(workspace, '"python3 scripts/validate_overkey_phase6.py"', WORKSPACE_LAYOUT)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase6.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 6: Delegated Access, Service Accounts, Policy Handoff, And Usage Hooks",
        SUB_PLAN,
    )
    for expected in (
        "Rust-first infrastructure stack",
        "Axum/Tower/Hyper-style Rust HTTP services",
        "Canonical JSON plus JSON Schema",
        "Signed command envelopes",
        "Ed25519 signatures",
        "BLAKE3/content hashes",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in (
        "Complete SUB BUILD PLAN #9 Phase 6",
        "delegated access metadata",
        "service-account scopes",
        "Overguard policy handoff",
        "last-used and usage-relevant event reporting",
        "operator lifecycle controls",
        "scripts/validate_overkey_phase6.py",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    for expected in (
        "Overkey Phase 6 Progress",
        "Docdex impact",
        "scripts/validate_overkey_phase6.py",
    ):
        assert_contains(phase_progress, expected, PHASE_PROGRESS)
    for expected in (
        "Phase 2 through Phase 5",
        "Phase 6 Delegated Access, Service Accounts, Policy Handoff, And Usage Hooks",
        "overkey.phase6.response.v0",
        "fixtures/valid/phase6_delegation_policy_usage.valid.json",
        "fixtures/invalid/phase6_delegation_denials.invalid.json",
        "python3 scripts/validate_overkey_phase6.py",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    records = read(RECORDS)
    repository = read(REPOSITORY)
    schema_rs = read(SCHEMA_RS)

    for expected in (
        "OVERKEY_PHASE6_RESPONSE_SCHEMA_VERSION",
        "OVERKEY_PHASE6_FIXTURE_SCHEMA_VERSION",
    ):
        assert_contains(schema_rs, expected, SCHEMA_RS)

    for expected in (
        "ROUTE_CREATE_DELEGATION",
        "Phase6DelegationData",
        "DelegationRequest",
        "create_delegation",
        "validate_delegation_request",
        "validate_overguard_policy_handoff",
        "policy_handoff_from_decision",
        "overkey.delegation_recorded_phase6",
        "auth.delegation_overgate_required",
        "overkey.delegation_subject_invalid",
        "overkey.delegation_scope_too_broad",
        "overkey.delegation_expiry_required",
        "overkey.delegation_delegate_stale",
        "overkey.delegation_expired",
        "overkey.delegation_cross_tenant_denied",
        "overkey.delegation_evidence_required",
        "policy.overguard_handoff_required",
        "policy.overguard_denied",
        "overkey_policy_truth_stored: false",
        "SERVICE_ACCOUNT_ALLOWED_SERVICES",
        "SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES",
        "PHASE6_DELEGATION_ALLOWED_SCOPES",
        "service_supports_command",
        '"command.accounting.mutate"',
        '"command.rights.mutate"',
        '"command.payout.mutate"',
        '"command.namespace.mutate"',
        '"command.policy.mutate"',
        "UsageData",
        "overkey.usage_recorded_phase6",
        "overkey.usage_update_queued_after_repository_miss",
        "overmeter_event_refs",
        "oru_balance_mutated: false",
        "seal_ledger_mutated: false",
        "validate_operator_lifecycle_request",
        "auth.operator_lifecycle_unsigned",
        "auth.operator_lifecycle_role_required",
        "auth.operator_lifecycle_protection_class_required",
        "auth.operator_lifecycle_evidence_required",
    ):
        assert_contains(routes, expected, ROUTES)

    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    missing_tests = sorted(PHASE6_RUST_TESTS - tests)
    if missing_tests:
        raise AssertionError(f"{ROUTES} missing required Phase 6 tests: {missing_tests}")

    for expected in (
        "delegator_ref: String",
        "delegate_ref: String",
        "allowed_scopes: Vec<String>",
        "allowed_command_classes: Vec<String>",
        "revocation_state: String",
        "evidence_refs: Vec<String>",
        "policy_decision_ref: String",
        "pub struct PolicyHandoff",
        "policy_engine_ref: String",
        "overkey_policy_truth_stored: bool",
    ):
        assert_contains(records, expected, RECORDS)

    for expected in (
        "append_delegation_record",
        "DuplicateDelegationRecord",
        "delegation_records",
    ):
        assert_contains(repository, expected, REPOSITORY)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    delegation_required = set(schema.get("$defs", {}).get("delegation_record", {}).get("required", []))
    missing = sorted(PHASE6_DELEGATION_FIELDS - delegation_required)
    if missing:
        raise AssertionError(f"{SCHEMA_JSON} delegation_record missing Phase 6 fields: {missing}")
    revocation_state_enum = set(
        schema.get("$defs", {})
        .get("delegation_record", {})
        .get("properties", {})
        .get("revocation_state", {})
        .get("enum", [])
    )
    if revocation_state_enum != {"revocable", "revoked", "expired"}:
        raise AssertionError(f"{SCHEMA_JSON} delegation revocation_state enum drifted")

    if valid.get("schema_version") != "overkey.phase6.delegation_policy_usage.v0":
        raise AssertionError("valid Phase 6 fixture has wrong schema_version")
    delegation = valid["delegation"]["delegation_record"]
    if not PHASE6_DELEGATION_FIELDS <= set(delegation):
        raise AssertionError("valid Phase 6 delegation fixture omits required fields")
    if delegation["delegator_ref"] == delegation["delegate_ref"]:
        raise AssertionError("Phase 6 delegation fixture must keep delegator and delegate distinct")
    if delegation["delegator_tenant_id"] != delegation["tenant_id"]:
        raise AssertionError("Phase 6 delegation fixture delegator tenant must match active tenant")
    if delegation["delegate_tenant_id"] != delegation["tenant_id"]:
        raise AssertionError("Phase 6 delegation fixture delegate tenant must match active tenant")
    if delegation["revocation_state"] != "revocable":
        raise AssertionError("Phase 6 delegation fixture must be explicitly revocable")
    if not delegation["evidence_refs"]:
        raise AssertionError("Phase 6 delegation fixture must include evidence refs")
    handoff = valid["delegation"]["policy_handoff"]
    if handoff["policy_engine_ref"] != "service:overguard":
        raise AssertionError("Phase 6 policy handoff must target Overguard")
    if handoff["overkey_policy_truth_stored"] is not False:
        raise AssertionError("Overkey must not store policy truth in Phase 6")

    matrix = {
        entry["service_id"]: set(entry["allowed_command_classes"])
        for entry in valid["service_account_scope_matrix"]
    }
    expected_matrix = {
        "service:overgate": {
            "command.verify",
            "command.credential.read",
            "command.credential.rotate",
            "command.credential.revoke",
        },
        "service:overvault": {"command.secret.resolve"},
        "service:overqueue": {"command.queue.execution_callback"},
        "service:worker": {"command.worker.runtime_callback", "command.workload.execute"},
    }
    for service_id, expected_commands in expected_matrix.items():
        if matrix.get(service_id) != expected_commands:
            raise AssertionError(f"Phase 6 service-account matrix drifted for {service_id}")

    usage = valid["last_used_usage_hook"]
    if usage["oru_balance_mutated"] is not False or usage["seal_ledger_mutated"] is not False:
        raise AssertionError("Phase 6 usage hooks must not mutate ORU or Seal Ledger state")
    if usage["retry_safe_update_queued_on_repository_miss"] is not True:
        raise AssertionError("Phase 6 last-used miss must queue retry-safe update")

    operator = valid["operator_lifecycle_controls"]
    if operator["signed_overgate_command_required"] is not True:
        raise AssertionError("Phase 6 operator lifecycle controls must require signed Overgate")
    if operator["overwatch_evidence_required"] is not True:
        raise AssertionError("Phase 6 operator lifecycle controls must require evidence")

    if invalid.get("schema_version") != "overkey.phase6.delegation_denials.v0":
        raise AssertionError("invalid Phase 6 fixture has wrong schema_version")
    reason_codes = {entry["reason_code"] for entry in invalid.get("denials", [])}
    if reason_codes != PHASE6_DENIAL_REASON_CODES:
        raise AssertionError(f"invalid Phase 6 denial reason drift: {sorted(reason_codes)}")
    if any(entry.get("side_effects_allowed") for entry in invalid["denials"]):
        raise AssertionError("Phase 6 denial fixtures must prevent side effects")

    combined = json.dumps({"valid": valid, "invalid": invalid}, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        if marker in combined:
            raise AssertionError(f"Phase 6 fixtures contain raw secret marker: {marker}")


def run_checks() -> None:
    result = subprocess.run(
        ["cargo", "test", "-p", "overrid-overkey", "phase6"],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        raise AssertionError("cargo test -p overrid-overkey phase6 failed")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_schema_and_fixtures()
        run_checks()
    except AssertionError as error:
        print(f"Overkey Phase 6 validation failed: {error}", file=sys.stderr)
        return 1
    print("Overkey Phase 6 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
