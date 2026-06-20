#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 7 evidence-view artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
POLICY_EVIDENCE_TS = SRC_ROOT / "policy_evidence_views.ts"

VALID_EVIDENCE = SHELL_ROOT / "fixtures/valid/policy_evidence_phase7.valid.json"
INVALID_PRIVATE_PAYLOAD = SHELL_ROOT / "fixtures/invalid/policy_evidence_private_payload.invalid.json"
INVALID_MISSING_REF = SHELL_ROOT / "fixtures/invalid/policy_evidence_missing_ref.invalid.json"
INVALID_BREAK_GLASS = SHELL_ROOT / "fixtures/invalid/policy_evidence_break_glass_enabled.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_07_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

REQUIRED_ROUTES = {
    "/admin/policy-decisions",
    "/admin/nodes",
    "/admin/workloads",
    "/admin/disputes",
    "/admin/capabilities",
}
REQUIRED_POLICY_CASES = {
    "denied_egress",
    "insufficient_trust",
    "quota_exhaustion",
    "package_trust_failure",
    "wrong_tenant",
    "budget_precheck_failure",
}
REQUIRED_VERIFICATION_STATES = {"verified", "degraded", "challenged", "expired", "disputed", "untrusted"}
REQUIRED_VERIFICATION_SUBJECT_KINDS = {"node", "provider", "workload"}
REQUIRED_INCIDENT_CONTRACTS = {
    "overgate.signed_break_glass_command",
    "overkey.break_glass_expiry",
    "overguard.break_glass_policy",
    "overwatch.break_glass_receipt",
}
REQUIRED_POLICY_LINK_KINDS = {"policy", "matched_rule", "input_fact", "audit"}
REQUIRED_STABLE_PREFIXES = {
    "audit",
    "benchmark",
    "challenge",
    "correction",
    "decision",
    "dispute",
    "evidence",
    "fact",
    "incident",
    "node",
    "package",
    "policy",
    "provider",
    "refund",
    "rule",
    "settlement",
    "trust",
    "workload",
}

FORBIDDEN_EVIDENCE_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
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
        for pattern in FORBIDDEN_EVIDENCE_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def stable_ref(ref: str) -> bool:
    if ":" not in ref:
        return False
    prefix, value = ref.split(":", 1)
    return bool(prefix in REQUIRED_STABLE_PREFIXES and value and not private_content_paths(ref))


def validate_source_files() -> None:
    source = read_text(POLICY_EVIDENCE_TS)
    for snippet in [
        "REQUIRED_POLICY_DENIAL_CASES",
        "REQUIRED_VERIFICATION_STATES",
        "REQUIRED_INCIDENT_BREAK_GLASS_CONTRACTS",
        "PHASE7_EVIDENCE_ROUTES",
        "buildPhase7EvidenceWorkspace",
        "buildPolicyDecisionExplorer",
        "buildVerificationEvidencePanel",
        "buildDisputeCorrectionViews",
        "buildIncidentReadinessViews",
        "checkEvidenceLinkConsistency",
        "hasRequiredPolicyDenialCoverage",
        "hasRequiredVerificationCoverage",
        "containsUnsafeEvidenceText",
        "isVerificationEvidenceSummary",
        "isProviderEvidenceRef",
        "resolveVerificationSubjectKind(item",
        "STABLE_EVIDENCE_PREFIXES",
        "FORBIDDEN_EVIDENCE_TEXT",
        "directLedgerMutation: false",
        "breakGlassExecutionDisabled: true",
        "usesOvergateOnly: true",
        '"/admin/policy-decisions"',
        '"/admin/disputes"',
    ]:
        assert_contains(source, snippet, POLICY_EVIDENCE_TS)
    for forbidden in ["React.", "extends React", "fetch(", "localStorage", "indexedDB", "postgres", "redis", "s3"]:
        if forbidden in source:
            raise AssertionError(f"{POLICY_EVIDENCE_TS} must stay framework-neutral and Overgate-bound: {forbidden}")

    index = read_text(INDEX_TS)
    assert_contains(index, "./policy_evidence_views", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_EVIDENCE)
    private_paths = private_content_paths(payload)
    if private_paths:
        raise AssertionError(f"{VALID_EVIDENCE} contains private evidence content at {private_paths}")

    if payload.get("schema_version") != "operator-policy-evidence.v0.1":
        raise AssertionError(f"{VALID_EVIDENCE} must use the Phase 7 evidence schema marker")
    for field in ["read_only", "uses_overgate_only"]:
        if payload.get(field) is not True:
            raise AssertionError(f"{VALID_EVIDENCE} must set {field}=true")
    for field in ["direct_storage_access", "direct_overwatch_connection", "direct_ledger_mutation"]:
        if payload.get(field) is not False:
            raise AssertionError(f"{VALID_EVIDENCE} must set {field}=false")
    if set(payload.get("routes", [])) != REQUIRED_ROUTES:
        raise AssertionError(f"{VALID_EVIDENCE} routes must cover {sorted(REQUIRED_ROUTES)}")
    if set(payload.get("required_policy_denial_cases", [])) != REQUIRED_POLICY_CASES:
        raise AssertionError(f"{VALID_EVIDENCE} required policy cases are incomplete")
    if set(payload.get("required_verification_states", [])) != REQUIRED_VERIFICATION_STATES:
        raise AssertionError(f"{VALID_EVIDENCE} required verification states are incomplete")

    policy_cases = payload.get("policy_cases", [])
    if {case.get("case") for case in policy_cases} != REQUIRED_POLICY_CASES:
        raise AssertionError(f"{VALID_EVIDENCE} policy cases must cover {sorted(REQUIRED_POLICY_CASES)}")
    for case in policy_cases:
        for field in ["decision_id", "trace_id", "tenant_id", "policy_version_refs", "matched_rule_refs", "input_fact_refs"]:
            if not case.get(field):
                raise AssertionError(f"{VALID_EVIDENCE} policy case missing {field}: {case}")
        link_kinds = {link.get("kind") for link in case.get("evidence_links", [])}
        if not REQUIRED_POLICY_LINK_KINDS.issubset(link_kinds):
            raise AssertionError(f"{VALID_EVIDENCE} policy case lacks required evidence link kinds: {case}")
        for link in case.get("evidence_links", []):
            if not stable_ref(str(link.get("ref", ""))):
                raise AssertionError(f"{VALID_EVIDENCE} policy evidence link is not stable: {link}")

    verification_cases = payload.get("verification_cases", [])
    if {case.get("state") for case in verification_cases} != REQUIRED_VERIFICATION_STATES:
        raise AssertionError(f"{VALID_EVIDENCE} verification states are incomplete")
    verification_subject_kinds = {case.get("subject_kind") for case in verification_cases}
    if not REQUIRED_VERIFICATION_SUBJECT_KINDS.issubset(verification_subject_kinds):
        raise AssertionError(
            f"{VALID_EVIDENCE} must cover node, provider, and workload verification subjects"
        )
    for case in verification_cases:
        for field in [
            "subject_ref",
            "subject_kind",
            "provider_verification_refs",
            "challenge_refs",
            "benchmark_refs",
            "trust_class_refs",
        ]:
            if not case.get(field):
                raise AssertionError(f"{VALID_EVIDENCE} verification case missing {field}: {case}")
        if case["subject_kind"] not in {"node", "provider", "package", "workload"}:
            raise AssertionError(f"{VALID_EVIDENCE} verification case has unsupported subject_kind: {case}")
        if str(case["subject_ref"]).startswith("provider:") and case["subject_kind"] != "provider":
            raise AssertionError(f"{VALID_EVIDENCE} provider refs must remain provider subjects: {case}")
        for ref in [
            case.get("subject_ref"),
            *case.get("provider_verification_refs", []),
            *case.get("challenge_refs", []),
            *case.get("benchmark_refs", []),
            *case.get("trust_class_refs", []),
        ]:
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_EVIDENCE} verification ref is not stable: {ref}")

    dispute_cases = payload.get("dispute_cases", [])
    if not dispute_cases:
        raise AssertionError(f"{VALID_EVIDENCE} must include dispute cases")
    if not any(case.get("hold_settlement_visibility") for case in dispute_cases):
        raise AssertionError(f"{VALID_EVIDENCE} must cover settlement hold visibility")
    if not any(case.get("correction_outcome_refs") for case in dispute_cases):
        raise AssertionError(f"{VALID_EVIDENCE} must cover correction outcome refs")
    if not any(case.get("refund_refs") for case in dispute_cases):
        raise AssertionError(f"{VALID_EVIDENCE} must cover refund refs")
    for case in dispute_cases:
        if case.get("direct_ledger_mutation") is not False:
            raise AssertionError(f"{VALID_EVIDENCE} dispute views must not mutate ledger state: {case}")
        for ref in [
            case.get("case_ref"),
            *case.get("evidence_refs", []),
            *case.get("correction_outcome_refs", []),
            *case.get("refund_refs", []),
            *case.get("settlement_impact_refs", []),
        ]:
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_EVIDENCE} dispute ref is not stable: {ref}")

    incident_cases = payload.get("incident_cases", [])
    if not incident_cases:
        raise AssertionError(f"{VALID_EVIDENCE} must include incident readiness cases")
    for case in incident_cases:
        if case.get("break_glass_execution_disabled") is not True:
            raise AssertionError(f"{VALID_EVIDENCE} break-glass execution must stay disabled: {case}")
        if set(case.get("missing_contracts", [])) != REQUIRED_INCIDENT_CONTRACTS:
            raise AssertionError(f"{VALID_EVIDENCE} incident case must name missing contracts: {case}")
        if not case.get("disabled_action_reason_codes"):
            raise AssertionError(f"{VALID_EVIDENCE} incident case must explain disabled action state: {case}")
        for link in case.get("evidence_links", []):
            if not stable_ref(str(link.get("ref", ""))):
                raise AssertionError(f"{VALID_EVIDENCE} incident evidence link is not stable: {link}")

    for link_case in payload.get("evidence_link_cases", []):
        if link_case.get("expected_ok") is not True:
            raise AssertionError(f"{VALID_EVIDENCE} valid evidence-link case must be expected_ok: {link_case}")
        for ref in link_case.get("refs", []):
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_EVIDENCE} evidence-link ref is not stable: {ref}")


def validate_invalid_fixtures() -> None:
    private_payload = load_json(INVALID_PRIVATE_PAYLOAD)
    if not private_content_paths(private_payload):
        raise AssertionError(f"{INVALID_PRIVATE_PAYLOAD} must fail private-content validation")

    missing_ref = load_json(INVALID_MISSING_REF)
    cases = missing_ref.get("policy_cases", [])
    if not cases:
        raise AssertionError(f"{INVALID_MISSING_REF} must include an invalid policy case")
    if not any(
        not case.get("evidence_links")
        or not case.get("policy_version_refs")
        or not case.get("matched_rule_refs")
        or not case.get("input_fact_refs")
        for case in cases
    ):
        raise AssertionError(f"{INVALID_MISSING_REF} must contain a policy case missing stable refs")

    break_glass = load_json(INVALID_BREAK_GLASS)
    if break_glass.get("read_only") is not False or break_glass.get("direct_ledger_mutation") is not True:
        raise AssertionError(f"{INVALID_BREAK_GLASS} must violate read-only and ledger-mutation gates")
    if not any(case.get("break_glass_execution_disabled") is False for case in break_glass.get("incident_cases", [])):
        raise AssertionError(f"{INVALID_BREAK_GLASS} must enable break-glass execution")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase7"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase7 phase_artifacts entry")
    entry = entries[0]
    if entry.get("validator") != "scripts/validate_admin_ui_phase7.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 7 validator path is wrong")
    if str(POLICY_EVIDENCE_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 7 source_files must include {POLICY_EVIDENCE_TS}")
    fixtures = entry.get("fixtures", {})
    if str(VALID_EVIDENCE) not in fixtures.get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 7 valid fixtures must include {VALID_EVIDENCE}")
    for invalid_path in [INVALID_PRIVATE_PAYLOAD, INVALID_MISSING_REF, INVALID_BREAK_GLASS]:
        if str(invalid_path) not in fixtures.get("invalid", []):
            raise AssertionError(f"{MANIFEST_PATH} Phase 7 invalid fixtures must include {invalid_path}")

    required_text = {
        SHELL_README: [
            "Phase 7 policy, verification, dispute, and incident evidence artifacts",
            "src/policy_evidence_views.ts",
            "scripts/validate_admin_ui_phase7.py",
        ],
        SCHEMA_README: [
            "Phase 7 policy, verification, dispute, and incident evidence artifacts",
            "policy evidence fixtures",
            "validate_admin_ui_phase7.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 7 policy, verification, dispute, and incident evidence views",
            "packages/admin_ui_shell/src/policy_evidence_views.ts",
            "scripts/validate_admin_ui_phase7.py",
        ],
        PHASE_PROGRESS: [
            "Admin and Developer UI Phase 7 Progress",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 7: Policy, Verification, Dispute, And Incident Evidence Views",
            "Build policy decision explorer",
            "Build evidence-link consistency checks",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase7.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        INDEX_TS,
        POLICY_EVIDENCE_TS,
        VALID_EVIDENCE,
        INVALID_PRIVATE_PAYLOAD,
        INVALID_MISSING_REF,
        INVALID_BREAK_GLASS,
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
    print("Admin UI Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
