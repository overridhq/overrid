#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 10 reliability artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SCRIPT_PATH = Path("scripts/validate_admin_ui_phase10.py")
SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
PHASE10_TS = SRC_ROOT / "phase10_reliability.ts"

VALID_PHASE10 = SHELL_ROOT / "fixtures/valid/phase10_reliability.valid.json"
INVALID_SECURITY = SHELL_ROOT / "fixtures/invalid/phase10_security_leak.invalid.json"
INVALID_RELIABILITY = SHELL_ROOT / "fixtures/invalid/phase10_missing_reliability_case.invalid.json"
INVALID_HANDOFF = SHELL_ROOT / "fixtures/invalid/phase10_handoff_high_risk_enabled.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_10_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_10_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

SCHEMA_VERSION = "operator-phase10-validation.v0.1"
REQUIRED_PRODUCTS = {"docdex", "mcoda", "codali", "sdk", "cli"}
REQUIRED_OUTCOMES = {
    "successful",
    "retryable_failure",
    "final_failure",
    "cancelled",
    "timed_out",
    "policy_denied",
    "budget_exhausted",
    "node_disconnected",
    "disputed_usage",
}
REQUIRED_SECURITY_PROBES = {
    "cross_tenant_access",
    "role_limits",
    "redaction_profiles",
    "encrypted_docdex_rag_metadata",
    "secret_bearing_fields",
    "key_metadata",
    "diagnostic_bundles",
    "raw_prompts",
    "decrypted_snippets",
    "file_paths",
    "query_text",
    "credentials",
    "private_payloads",
    "unredacted_result_contents",
}
REQUIRED_ACCESSIBILITY_CHECKS = {
    "keyboard_navigation",
    "focus_order",
    "screen_reader_labels",
    "long_reason_code_wrapping",
    "stable_table_dimensions",
    "loading_state",
    "empty_state",
    "responsive_behavior",
}
REQUIRED_CONTRACT_CHECKS = {
    "generated_bindings",
    "schema_compatibility",
    "overgate_admin_routes",
    "read_only_mode",
    "action_submission",
    "idempotency",
    "stale_state_blocking",
    "timeline_assembly",
}
REQUIRED_HANDOFF_SURFACES = {
    "system_service_operations",
    "incident_readiness",
    "break_glass_execution",
    "governance_reporting",
    "compliance_views",
}
FORBIDDEN_RENDER_FLAGS = {
    "raw_prompts_rendered",
    "decrypted_snippets_rendered",
    "private_file_paths_rendered",
    "query_text_rendered",
    "key_material_rendered",
    "credentials_rendered",
    "secrets_rendered",
    "private_payloads_rendered",
    "unredacted_result_contents_rendered",
}
REQUIRED_VALIDATION_COMMANDS = {
    "PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_admin_ui_phase10.py",
    "PYTHONDONTWRITEBYTECODE=1 python3 scripts/validate_admin_ui.py",
    "npx --yes -p typescript@5.5.4 tsc -p packages/admin_ui_shell/tsconfig.json",
    "docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid",
}
REQUIRED_STABLE_PREFIXES = {
    "a11y",
    "actor",
    "audit",
    "auth_ref",
    "capability",
    "contract",
    "diagnostic",
    "evidence",
    "field_ref",
    "handoff",
    "key",
    "metadata",
    "path_ref",
    "payload_ref",
    "policy",
    "prompt_ref",
    "query_ref",
    "receipt",
    "redaction",
    "result_ref",
    "route",
    "snippet_ref",
    "tenant",
    "trace",
    "usage",
    "workload",
}
UNSAFE_RENDER_PATTERNS = [
    re.compile(r"\braw prompt text\b", re.IGNORECASE),
    re.compile(r"\bdecrypted snippet\b", re.IGNORECASE),
    re.compile(r"\bquery text\b", re.IGNORECASE),
    re.compile(r"\bkey material\b", re.IGNORECASE),
    re.compile(r"\bcredential value\b", re.IGNORECASE),
    re.compile(r"\bsecret value\b", re.IGNORECASE),
    re.compile(r"\bprivate payload\b", re.IGNORECASE),
    re.compile(r"\bunredacted result contents\b", re.IGNORECASE),
    re.compile(r"/Users/|/home/|[A-Za-z]:\\"),
]


def assert_contains(content: str, snippet: str, path: Path) -> None:
    if snippet not in content:
        raise AssertionError(f"{path} is missing expected text: {snippet}")


def strings(value: Any, path: str = "$") -> list[tuple[str, str]]:
    if isinstance(value, str):
        return [(path, value)]
    if isinstance(value, list):
        result: list[tuple[str, str]] = []
        for index, item in enumerate(value):
            result.extend(strings(item, f"{path}[{index}]"))
        return result
    if isinstance(value, dict):
        result = []
        for key, item in value.items():
            result.extend(strings(item, f"{path}.{key}"))
        return result
    return []


def unsafe_render_paths(value: Any) -> list[str]:
    matches: list[str] = []
    for path, text in strings(value):
        for pattern in UNSAFE_RENDER_PATTERNS:
            if pattern.search(text):
                matches.append(path)
    return matches


def stable_ref(ref: Any) -> bool:
    if not isinstance(ref, str) or not ref:
        return False
    if ref.startswith("trace_"):
        return True
    if ":" not in ref:
        return False
    prefix, value = ref.split(":", 1)
    return bool(prefix in REQUIRED_STABLE_PREFIXES and value)


def audit_ref_is_stable(ref: Any) -> bool:
    if not isinstance(ref, dict):
        return False
    return (
        stable_ref(ref.get("audit_id"))
        and ref.get("source_service") in {
            "overgate",
            "overguard",
            "overmeter",
            "overcell",
            "overclaim",
            "overvault",
            "overwatch",
        }
        and stable_ref(ref.get("trace_id"))
    )


def require_exact_set(payload: dict[str, Any], field: str, expected: set[str], path: Path) -> None:
    actual = set(payload.get(field, []))
    if actual != expected:
        raise AssertionError(f"{path} {field} mismatch: missing={expected - actual}, extra={actual - expected}")


def product_case_readable(item: dict[str, Any]) -> bool:
    return (
        item.get("product") in REQUIRED_PRODUCTS
        and item.get("outcome") in REQUIRED_OUTCOMES
        and stable_ref(item.get("workload_id"))
        and stable_ref(item.get("tenant_id"))
        and stable_ref(item.get("trace_id"))
        and all(audit_ref_is_stable(ref) for ref in item.get("audit_refs", []))
        and all(stable_ref(ref) for ref in item.get("usage_refs", []))
        and all(stable_ref(ref) for ref in item.get("receipt_refs", []))
        and all(stable_ref(ref) for ref in item.get("evidence_refs", []))
        and stable_ref(item.get("diagnostic_bundle_ref"))
        and item.get("readable_audit_trail") is True
        and item.get("readable_usage_trail") is True
        and item.get("readable_receipt_trail") is True
        and item.get("readable_reason_trail") is True
        and bool(item.get("reason_codes"))
    )


def security_probe_safe(item: dict[str, Any]) -> bool:
    return (
        item.get("probe") in REQUIRED_SECURITY_PROBES
        and stable_ref(item.get("redaction_profile_ref"))
        and all(stable_ref(ref) for ref in item.get("safe_refs", []))
        and all(audit_ref_is_stable(ref) for ref in item.get("audit_refs", []))
        and bool(item.get("reason_codes"))
        and all(item.get(flag) is False for flag in FORBIDDEN_RENDER_FLAGS)
    )


def accessibility_check_complete(item: dict[str, Any]) -> bool:
    return (
        item.get("check") in REQUIRED_ACCESSIBILITY_CHECKS
        and item.get("automated") is True
        and item.get("manual_review") is True
        and item.get("stable_dimensions") is True
        and all(stable_ref(ref) for ref in item.get("evidence_refs", []))
        and bool(item.get("reason_codes"))
    )


def contract_check_complete(item: dict[str, Any]) -> bool:
    route = item.get("route", "")
    if not (route == "/admin/actions" or str(route).startswith("/admin/")):
        return False
    if item.get("method") not in {"GET", "POST"}:
        return False
    if item.get("check") == "action_submission" and item.get("method") != "POST":
        return False
    if item.get("check") == "idempotency" and item.get("idempotency_protected") is not True:
        return False
    if item.get("check") == "stale_state_blocking" and item.get("stale_state_protected") is not True:
        return False
    if item.get("check") == "read_only_mode" and item.get("read_only_mode") is not True:
        return False
    return (
        item.get("check") in REQUIRED_CONTRACT_CHECKS
        and item.get("pass") is True
        and item.get("uses_generated_bindings") is True
        and item.get("uses_overgate_route") is True
        and item.get("no_privileged_backdoor") is True
        and all(stable_ref(ref) for ref in item.get("evidence_refs", []))
        and bool(item.get("reason_codes"))
    )


def handoff_surface_safe(item: dict[str, Any]) -> bool:
    return (
        item.get("surface") in REQUIRED_HANDOFF_SURFACES
        and item.get("phase_gate") in {"phase7", "phase13"}
        and item.get("disabled") is True
        and item.get("readiness_only") is True
        and item.get("high_risk_operation_enabled") is False
        and all(stable_ref(ref) for ref in item.get("evidence_refs", []))
        and bool(item.get("reason_codes"))
    )


def validate_source_files() -> None:
    source = read_text(PHASE10_TS)
    for snippet in [
        "PHASE10_SCHEMA_VERSION",
        "REQUIRED_PHASE10_PRODUCTS",
        "REQUIRED_PHASE10_RELIABILITY_OUTCOMES",
        "REQUIRED_PHASE10_SECURITY_PROBES",
        "REQUIRED_PHASE10_ACCESSIBILITY_CHECKS",
        "REQUIRED_PHASE10_CONTRACT_CHECKS",
        "REQUIRED_PHASE10_HANDOFF_SURFACES",
        "PHASE10_FORBIDDEN_RENDER_FIELDS",
        "Phase10ProductReliabilityCase",
        "Phase10SecurityProbe",
        "Phase10AccessibilityCheck",
        "Phase10ContractCheck",
        "Phase10HandoffSurface",
        "listUnsafePhase10Content",
        "hasRequiredProductCoverage",
        "hasRequiredOutcomeCoverage",
        "securityRenderFlags",
        "isPhase10ProductCaseReadable",
        "isPhase10SecurityProbeSafe",
        "isPhase10AccessibilityCheckComplete",
        "isPhase10ContractCheckComplete",
        "isPhase10HandoffSurfaceSafe",
        "buildPhase10DiagnosticBundle",
        "buildPhase10ValidationReport",
        "directStorageAccess: false",
        "directServiceAccess: false",
        "directOverwatchConnection: false",
        "rustServicesAuthoritative: true",
        "typeScriptClientSurfaceOnly: true",
    ]:
        assert_contains(source, snippet, PHASE10_TS)
    for forbidden in [
        "React.",
        "extends React",
        "fetch(",
        "localStorage",
        "indexedDB",
        "postgres",
        "redis",
        "s3",
        "kafka",
        "nats",
        "blockchain",
        "nft",
    ]:
        if forbidden.lower() in source.lower():
            raise AssertionError(f"{PHASE10_TS} must stay framework-neutral and Overgate-bound: {forbidden}")

    index = read_text(INDEX_TS)
    assert_contains(index, "./phase10_reliability", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_PHASE10)
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise AssertionError(f"{VALID_PHASE10} must use {SCHEMA_VERSION}")
    for field in ["uses_overgate_only", "rust_services_authoritative", "typescript_client_surface_only"]:
        if payload.get(field) is not True:
            raise AssertionError(f"{VALID_PHASE10} must set {field}=true")
    for field in ["direct_storage_access", "direct_service_access", "direct_overwatch_connection"]:
        if payload.get(field) is not False:
            raise AssertionError(f"{VALID_PHASE10} must set {field}=false")

    require_exact_set(payload, "required_products", REQUIRED_PRODUCTS, VALID_PHASE10)
    require_exact_set(payload, "required_outcomes", REQUIRED_OUTCOMES, VALID_PHASE10)
    require_exact_set(payload, "required_security_probes", REQUIRED_SECURITY_PROBES, VALID_PHASE10)
    require_exact_set(payload, "required_accessibility_checks", REQUIRED_ACCESSIBILITY_CHECKS, VALID_PHASE10)
    require_exact_set(payload, "required_contract_checks", REQUIRED_CONTRACT_CHECKS, VALID_PHASE10)
    require_exact_set(payload, "required_handoff_surfaces", REQUIRED_HANDOFF_SURFACES, VALID_PHASE10)

    product_cases = payload.get("product_cases", [])
    readable_cases = [item for item in product_cases if product_case_readable(item)]
    if {item.get("product") for item in readable_cases} != REQUIRED_PRODUCTS:
        raise AssertionError(f"{VALID_PHASE10} must cover every Phase 10 product with readable cases")
    if {item.get("outcome") for item in readable_cases} != REQUIRED_OUTCOMES:
        raise AssertionError(f"{VALID_PHASE10} must cover every Phase 10 reliability outcome")

    security_probes = payload.get("security_probes", [])
    safe_probes = [item for item in security_probes if security_probe_safe(item)]
    if {item.get("probe") for item in safe_probes} != REQUIRED_SECURITY_PROBES:
        raise AssertionError(f"{VALID_PHASE10} must cover every security/redaction probe safely")

    accessibility = payload.get("accessibility_checks", [])
    if {item.get("check") for item in accessibility if accessibility_check_complete(item)} != REQUIRED_ACCESSIBILITY_CHECKS:
        raise AssertionError(f"{VALID_PHASE10} must cover every accessibility/dense-table check")

    contracts = payload.get("contract_checks", [])
    if {item.get("check") for item in contracts if contract_check_complete(item)} != REQUIRED_CONTRACT_CHECKS:
        raise AssertionError(f"{VALID_PHASE10} must cover every integration/contract check")

    handoff = payload.get("handoff_surfaces", [])
    if {item.get("surface") for item in handoff if handoff_surface_safe(item)} != REQUIRED_HANDOFF_SURFACES:
        raise AssertionError(f"{VALID_PHASE10} must keep every Phase 7/13 handoff surface disabled/readiness-only")

    for event in payload.get("diagnostic_events", []):
        if event.get("schema_version") != "admin-ui.v0.1":
            raise AssertionError(f"{VALID_PHASE10} diagnostic events must use admin-ui.v0.1")
        if not stable_ref(event.get("event_id")) or not stable_ref(event.get("trace_id")):
            raise AssertionError(f"{VALID_PHASE10} diagnostic events need stable event and trace refs")
        if not event.get("safe_refs") or not all(stable_ref(ref) for ref in event.get("safe_refs", [])):
            raise AssertionError(f"{VALID_PHASE10} diagnostic events must use stable safe refs")
        if unsafe_render_paths(event.get("message", "")):
            raise AssertionError(f"{VALID_PHASE10} diagnostic event message exposes unsafe content")

    commands = set(payload.get("validation_commands", []))
    if not REQUIRED_VALIDATION_COMMANDS.issubset(commands):
        raise AssertionError(f"{VALID_PHASE10} validation_commands missing {REQUIRED_VALIDATION_COMMANDS - commands}")


def validate_invalid_fixtures() -> None:
    missing = load_json(INVALID_RELIABILITY)
    cases = missing.get("product_cases", [])
    if {case.get("product") for case in cases if product_case_readable(case)} == REQUIRED_PRODUCTS:
        raise AssertionError(f"{INVALID_RELIABILITY} must not satisfy product coverage")
    first_case = cases[0] if cases else {}
    if first_case.get("readable_audit_trail") is not False or first_case.get("audit_refs"):
        raise AssertionError(f"{INVALID_RELIABILITY} must fail audit readability coverage")

    security = load_json(INVALID_SECURITY)
    probe = security.get("security_probes", [{}])[0]
    if all(probe.get(flag) is False for flag in FORBIDDEN_RENDER_FLAGS):
        raise AssertionError(f"{INVALID_SECURITY} must expose forbidden render flags")
    unsafe_paths = unsafe_render_paths(probe.get("unsafe_rendered_examples", []))
    if len(unsafe_paths) < 5:
        raise AssertionError(f"{INVALID_SECURITY} must include multiple unsafe rendered examples")
    if security_probe_safe(probe):
        raise AssertionError(f"{INVALID_SECURITY} must not pass security probe safety")

    handoff = load_json(INVALID_HANDOFF)
    surface = handoff.get("handoff_surfaces", [{}])[0]
    if handoff_surface_safe(surface):
        raise AssertionError(f"{INVALID_HANDOFF} must not pass handoff safety")
    if surface.get("high_risk_operation_enabled") is not True:
        raise AssertionError(f"{INVALID_HANDOFF} must enable a forbidden high-risk surface")
    if surface.get("disabled") is not False or surface.get("readiness_only") is not False:
        raise AssertionError(f"{INVALID_HANDOFF} must show disabled/readiness gates failing")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase10"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase10 phase_artifacts entry")
    entry = entries[0]
    if entry.get("validator") != "scripts/validate_admin_ui_phase10.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 10 validator path is wrong")
    if str(PHASE10_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 10 source_files must include {PHASE10_TS}")
    fixtures = entry.get("fixtures", {})
    if str(VALID_PHASE10) not in fixtures.get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 10 valid fixtures must include {VALID_PHASE10}")
    for invalid_path in [INVALID_SECURITY, INVALID_RELIABILITY, INVALID_HANDOFF]:
        if str(invalid_path) not in fixtures.get("invalid", []):
            raise AssertionError(f"{MANIFEST_PATH} Phase 10 invalid fixtures must include {invalid_path}")

    required_text = {
        SHELL_README: [
            "Phase 10 product reliability, diagnostics, accessibility, contract validation, and handoff artifacts",
            "src/phase10_reliability.ts",
            "scripts/validate_admin_ui_phase10.py",
        ],
        SCHEMA_README: [
            "Phase 10 product reliability, diagnostics, accessibility, contract validation, and handoff artifacts",
            "phase10_reliability.valid.json",
            "validate_admin_ui_phase10.py",
        ],
        PHASE_PLAN: [
            "Implement SUB BUILD PLAN #1 Phase 10",
            "packages/admin_ui_shell/src/phase10_reliability.ts",
            "scripts/validate_admin_ui_phase10.py",
        ],
        PHASE_PROGRESS: [
            "Admin/Developer UI Phase 10 Progress",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 10: Product Reliability, Diagnostics, Accessibility, And Handoff",
            "Run Phase 6 product reliability cases",
            "Prepare Phase 7 and Phase 13 handoff",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase10.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        SCRIPT_PATH,
        INDEX_TS,
        PHASE10_TS,
        VALID_PHASE10,
        INVALID_SECURITY,
        INVALID_RELIABILITY,
        INVALID_HANDOFF,
        SHELL_README,
        SCHEMA_README,
        MANIFEST_PATH,
        SUITE_VALIDATOR,
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
    print("Admin UI Phase 10 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
