#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 9 signed-action artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
ADMIN_ACTIONS_TS = SRC_ROOT / "admin_actions.ts"

VALID_ACTIONS = SHELL_ROOT / "fixtures/valid/admin_actions_phase9.valid.json"
INVALID_MISSING_REASON = SHELL_ROOT / "fixtures/invalid/admin_action_missing_reason.invalid.json"
INVALID_UNSIGNED = SHELL_ROOT / "fixtures/invalid/admin_action_unsigned_malformed.invalid.json"
INVALID_HIGH_RISK = SHELL_ROOT / "fixtures/invalid/admin_action_high_risk_reachable.invalid.json"
INVALID_TENANT = SHELL_ROOT / "fixtures/invalid/admin_action_tenant_mismatch.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_09_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_09_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

REQUIRED_ACTIONS = {
    "cancel_workload",
    "retry_workload",
    "pause_node",
    "drain_node",
    "annotate_dispute",
    "request_credential_rotation",
    "acknowledge_receipt",
}
TARGET_KIND_BY_ACTION = {
    "cancel_workload": "workload",
    "retry_workload": "workload",
    "pause_node": "node",
    "drain_node": "node",
    "annotate_dispute": "dispute",
    "request_credential_rotation": "credential",
    "acknowledge_receipt": "receipt",
}
REQUIRED_DISABLED_ACTIONS = {
    "backbone_maintenance",
    "forced_rollback",
    "break_glass_activation",
    "ledger_correction_execution",
    "provider_payout_override",
    "direct_data_repair",
    "raw_key_recovery",
    "annotate_incident",
}
REQUIRED_RECEIPT_OUTCOMES = {
    "accepted",
    "denied",
    "duplicate",
    "stale_expected_state",
    "downstream_failed",
    "applied",
    "completed",
    "failed",
}
REQUIRED_STABLE_PREFIXES = {
    "actor",
    "admin_receipt",
    "audit",
    "command",
    "credential",
    "dispute",
    "node",
    "policy",
    "policy_decision",
    "receipt",
    "signature",
    "signing_flow",
    "signing_provider",
    "tenant",
    "trace",
    "workload",
}
FORBIDDEN_PRIVATE_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential_material\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bdecrypted\b", re.IGNORECASE),
    re.compile(r"\bprompt\b", re.IGNORECASE),
    re.compile(r"\bkey[_ -]?material\b", re.IGNORECASE),
    re.compile(r"/Users/|/home/|[A-Za-z]:\\"),
]


def assert_contains(content: str, snippet: str, path: Path) -> None:
    if snippet not in content:
        raise AssertionError(f"{path} is missing expected text: {snippet}")


def iter_strings_and_keys(value: Any, path: str = "$") -> list[tuple[str, str]]:
    if isinstance(value, str):
        return [(path, value)]
    if isinstance(value, list):
        strings: list[tuple[str, str]] = []
        for index, item in enumerate(value):
            strings.extend(iter_strings_and_keys(item, f"{path}[{index}]"))
        return strings
    if isinstance(value, dict):
        strings = []
        for key, item in value.items():
            strings.append((f"{path}.{key}<key>", str(key)))
            strings.extend(iter_strings_and_keys(item, f"{path}.{key}"))
        return strings
    return []


def private_content_paths(payload: Any) -> list[str]:
    matches: list[str] = []
    for path, value in iter_strings_and_keys(payload):
        for pattern in FORBIDDEN_PRIVATE_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def stable_ref(ref: str) -> bool:
    if ref.startswith("trace_"):
        return True
    if ":" not in ref:
        return False
    prefix, value = ref.split(":", 1)
    return bool(prefix in REQUIRED_STABLE_PREFIXES and value and not private_content_paths(ref))


def validate_source_files() -> None:
    source = read_text(ADMIN_ACTIONS_TS)
    for snippet in [
        "PHASE9_ACTION_ROUTE",
        "SUPPORTED_ADMIN_ACTIONS",
        "PHASE9_HIGH_RISK_DENYLIST",
        "ACTION_TARGET_KIND",
        "ACTION_RISK_LEVEL",
        "createAdminActionDraft",
        "buildLocalSigningHandoff",
        "buildSignedAdminActionRequest",
        "buildOvergateActionSubmission",
        "assertSubmittableSignedAdminActionRequest",
        "buildAdminActionReceiptPanel",
        "evaluateStaleStateProtection",
        "buildDisabledActionGate",
        "admin.actions.submit",
        "stale_expected_state",
        'blocker.code === "refresh_required"',
        "missing_signature",
        "malformed_signature",
        "directStorageAccess: false",
        "directServiceAccess: false",
        "generatedContractsProjectionOnly: true",
        '"x-overrid-idempotency-key"',
        '"x-overrid-signature-refs"',
    ]:
        assert_contains(source, snippet, ADMIN_ACTIONS_TS)
    for forbidden in ["React.", "extends React", "fetch(", "localStorage", "indexedDB", "postgres", "redis", "s3"]:
        if forbidden in source:
            raise AssertionError(f"{ADMIN_ACTIONS_TS} must stay framework-neutral and Overgate-bound: {forbidden}")

    index = read_text(INDEX_TS)
    assert_contains(index, "./admin_actions", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_ACTIONS)
    private_paths = private_content_paths(payload)
    if private_paths:
        raise AssertionError(f"{VALID_ACTIONS} contains private action content at {private_paths}")

    if payload.get("schema_version") != "operator-admin-actions.v0.1":
        raise AssertionError(f"{VALID_ACTIONS} must use the Phase 9 action schema marker")
    for field in ["uses_overgate_only", "generated_contracts_projection_only"]:
        if payload.get(field) is not True:
            raise AssertionError(f"{VALID_ACTIONS} must set {field}=true")
    for field in ["direct_storage_access", "direct_service_access"]:
        if payload.get(field) is not False:
            raise AssertionError(f"{VALID_ACTIONS} must set {field}=false")
    if payload.get("action_route") != "/admin/actions" or payload.get("action_method") != "POST":
        raise AssertionError(f"{VALID_ACTIONS} must submit signed actions to POST /admin/actions")
    if "admin.actions.submit" not in payload.get("capability_flags", []):
        raise AssertionError(f"{VALID_ACTIONS} must require admin.actions.submit capability")
    if set(payload.get("supported_actions", [])) != REQUIRED_ACTIONS:
        raise AssertionError(f"{VALID_ACTIONS} supported actions must match generated contracts")
    if set(payload.get("disabled_high_risk_actions", [])) != REQUIRED_DISABLED_ACTIONS:
        raise AssertionError(f"{VALID_ACTIONS} disabled high-risk actions are incomplete")

    drafts = payload.get("drafts", [])
    if {draft.get("action_type") for draft in drafts} != REQUIRED_ACTIONS:
        raise AssertionError(f"{VALID_ACTIONS} drafts must cover every supported action")
    for draft in drafts:
        action = draft.get("action_type")
        if not draft.get("reason") or not draft.get("expected_current_state"):
            raise AssertionError(f"{VALID_ACTIONS} draft missing reason or expected state: {draft}")
        if draft.get("tenant_id") != draft.get("visible_active_tenant_id"):
            raise AssertionError(f"{VALID_ACTIONS} draft tenant mismatch: {draft}")
        if draft.get("target_tenant_id") != draft.get("tenant_id"):
            raise AssertionError(f"{VALID_ACTIONS} draft target tenant mismatch: {draft}")
        if draft.get("target", {}).get("target_kind") != TARGET_KIND_BY_ACTION.get(action):
            raise AssertionError(f"{VALID_ACTIONS} target kind mismatch for {action}")
        if draft.get("status") != "ready_for_signing" or draft.get("blockers"):
            raise AssertionError(f"{VALID_ACTIONS} draft must be ready for signing with no blockers: {draft}")
        for field in ["trace_id", "command_id", "idempotency_key"]:
            if not draft.get(field):
                raise AssertionError(f"{VALID_ACTIONS} draft missing {field}: {draft}")
        if not str(draft.get("idempotency_key", "")).startswith("idem_"):
            raise AssertionError(f"{VALID_ACTIONS} idempotency keys must use idem_ prefix")
        if not draft.get("signature_refs") or not all(stable_ref(str(ref)) for ref in draft.get("signature_refs", [])):
            raise AssertionError(f"{VALID_ACTIONS} draft signature refs are invalid: {draft}")
        if not draft.get("policy_refs") or not draft.get("audit_refs"):
            raise AssertionError(f"{VALID_ACTIONS} draft must include policy and audit refs: {draft}")
        if draft.get("refreshed_at_epoch_ms") is None:
            raise AssertionError(f"{VALID_ACTIONS} draft must include refresh evidence")

    handoff = payload.get("signing_handoff", {})
    for field in ["provider_ref", "signing_flow_ref", "signature_refs", "idempotency_key"]:
        if not handoff.get(field):
            raise AssertionError(f"{VALID_ACTIONS} signing handoff missing {field}")
    for field in ["approved", "submit_to_overgate", "unsigned_blocked", "malformed_signature_blocked"]:
        if handoff.get(field) is not True:
            raise AssertionError(f"{VALID_ACTIONS} signing handoff must set {field}=true")

    receipts = payload.get("receipts", [])
    if {receipt.get("outcome") for receipt in receipts} != REQUIRED_RECEIPT_OUTCOMES:
        raise AssertionError(f"{VALID_ACTIONS} receipts must cover every required outcome")
    for receipt in receipts:
        if receipt.get("outcome") != receipt.get("overgate_status"):
            raise AssertionError(f"{VALID_ACTIONS} receipt overgate status must mirror outcome: {receipt}")
        if not receipt.get("affected_refs") or not receipt.get("reason_codes") or not receipt.get("audit_refs"):
            raise AssertionError(f"{VALID_ACTIONS} receipt missing affected refs, reason codes, or audit refs: {receipt}")
        if not any(ref.get("source_service") == "overwatch" for ref in receipt.get("audit_refs", [])):
            raise AssertionError(f"{VALID_ACTIONS} receipt must include Overwatch refs: {receipt}")
        for ref in [receipt.get("receipt_id"), receipt.get("command_id"), *receipt.get("affected_refs", [])]:
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_ACTIONS} receipt ref is not stable: {ref}")

    stale_cases = payload.get("stale_state_cases", [])
    if not stale_cases:
        raise AssertionError(f"{VALID_ACTIONS} must include stale-state cases")
    for case in stale_cases:
        if case.get("expected_current_state") == case.get("observed_current_state"):
            raise AssertionError(f"{VALID_ACTIONS} stale-state case must show changed target state")
        for field in ["blocks_submission", "requires_refresh", "operator_review_required"]:
            if case.get(field) is not True:
                raise AssertionError(f"{VALID_ACTIONS} stale-state case must set {field}=true")
        if case.get("silent_retry_allowed") is not False:
            raise AssertionError(f"{VALID_ACTIONS} stale-state case must block silent retry")

    security = payload.get("security_review", {})
    for field in [
        "route_feature_flag_blocked",
        "direct_component_invocation_blocked",
        "denylist_enforced",
        "incident_annotation_contract_missing",
    ]:
        if security.get(field) is not True:
            raise AssertionError(f"{VALID_ACTIONS} security review must set {field}=true")


def validate_invalid_fixtures() -> None:
    missing = load_json(INVALID_MISSING_REASON)
    draft = missing.get("drafts", [{}])[0]
    if draft.get("reason") or draft.get("expected_current_state") or draft.get("policy_refs") or draft.get("audit_refs"):
        raise AssertionError(f"{INVALID_MISSING_REASON} must fail reason/state/policy/audit validation")

    unsigned = load_json(INVALID_UNSIGNED)
    handoff = unsigned.get("signing_handoff", {})
    draft = unsigned.get("drafts", [{}])[0]
    if draft.get("signature_refs"):
        raise AssertionError(f"{INVALID_UNSIGNED} must include an unsigned draft")
    if handoff.get("unsigned_blocked") is not False or handoff.get("malformed_signature_blocked") is not False:
        raise AssertionError(f"{INVALID_UNSIGNED} must show unsigned/malformed signatures are not blocked")
    if not handoff.get("malformed_signature"):
        raise AssertionError(f"{INVALID_UNSIGNED} must mark malformed_signature=true")

    high_risk = load_json(INVALID_HIGH_RISK)
    if high_risk.get("uses_overgate_only") is not False:
        raise AssertionError(f"{INVALID_HIGH_RISK} must disable Overgate-only guard")
    for field in ["direct_storage_access", "direct_service_access"]:
        if high_risk.get(field) is not True:
            raise AssertionError(f"{INVALID_HIGH_RISK} must set {field}=true")
    if not REQUIRED_DISABLED_ACTIONS.intersection(set(high_risk.get("enabled_actions", []))):
        raise AssertionError(f"{INVALID_HIGH_RISK} must enable a forbidden high-risk action")
    security = high_risk.get("security_review", {})
    if any(security.get(field) is True for field in ["route_feature_flag_blocked", "direct_component_invocation_blocked", "denylist_enforced"]):
        raise AssertionError(f"{INVALID_HIGH_RISK} must fail route/direct invocation/denylist guards")

    tenant = load_json(INVALID_TENANT)
    tenant_draft = tenant.get("drafts", [{}])[0]
    if tenant_draft.get("tenant_id") == tenant_draft.get("visible_active_tenant_id"):
        raise AssertionError(f"{INVALID_TENANT} must include visible active tenant mismatch")
    if tenant_draft.get("tenant_id") == tenant_draft.get("target_tenant_id"):
        raise AssertionError(f"{INVALID_TENANT} must include target tenant mismatch")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase9"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase9 phase_artifacts entry")
    entry = entries[0]
    if entry.get("validator") != "scripts/validate_admin_ui_phase9.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 9 validator path is wrong")
    if str(ADMIN_ACTIONS_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 9 source_files must include {ADMIN_ACTIONS_TS}")
    fixtures = entry.get("fixtures", {})
    if str(VALID_ACTIONS) not in fixtures.get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 9 valid fixtures must include {VALID_ACTIONS}")
    for invalid_path in [INVALID_MISSING_REASON, INVALID_UNSIGNED, INVALID_HIGH_RISK, INVALID_TENANT]:
        if str(invalid_path) not in fixtures.get("invalid", []):
            raise AssertionError(f"{MANIFEST_PATH} Phase 9 invalid fixtures must include {invalid_path}")

    required_text = {
        SHELL_README: [
            "Phase 9 signed admin action and receipt artifacts",
            "src/admin_actions.ts",
            "scripts/validate_admin_ui_phase9.py",
        ],
        SCHEMA_README: [
            "Phase 9 signed admin action and receipt artifacts",
            "signed action fixtures",
            "validate_admin_ui_phase9.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 9 signed admin actions and receipts slice",
            "packages/admin_ui_shell/src/admin_actions.ts",
            "scripts/validate_admin_ui_phase9.py",
        ],
        PHASE_PROGRESS: [
            "Admin and Developer UI Phase 9 Progress",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 9: Signed Admin Actions And Receipts",
            "Implement action drafting and confirmation",
            "Keep high-risk actions out of Phase 6",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase9.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        INDEX_TS,
        ADMIN_ACTIONS_TS,
        VALID_ACTIONS,
        INVALID_MISSING_REASON,
        INVALID_UNSIGNED,
        INVALID_HIGH_RISK,
        INVALID_TENANT,
        SHELL_README,
        SCHEMA_README,
        MANIFEST_PATH,
        PHASE_PLAN,
        PHASE_PROGRESS,
    ]
    for path in checked_paths:
        content = read_text(path)
        if not content.endswith("\n"):
            raise AssertionError(f"{path} must end with a newline")
        if "\t" in content:
            raise AssertionError(f"{path} must not contain tabs")


def main() -> int:
    validate_source_files()
    validate_valid_fixture()
    validate_invalid_fixtures()
    validate_manifest_and_docs()
    validate_file_hygiene()
    print("Admin UI Phase 9 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
