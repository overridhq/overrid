#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 8 accounting-view artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
ACCOUNTING_VIEWS_TS = SRC_ROOT / "accounting_usage_views.ts"

VALID_ACCOUNTING = SHELL_ROOT / "fixtures/valid/accounting_usage_phase8.valid.json"
INVALID_PRIVATE_PAYLOAD = SHELL_ROOT / "fixtures/invalid/accounting_private_payload.invalid.json"
INVALID_PRICING = SHELL_ROOT / "fixtures/invalid/accounting_pricing_assumption.invalid.json"
INVALID_MUTATION_BLOCKCHAIN = SHELL_ROOT / "fixtures/invalid/accounting_mutation_blockchain.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_08_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

REQUIRED_ROUTES = {"/admin/usage", "/admin/ledger", "/admin/receipts", "/admin/capabilities"}
REQUIRED_USAGE_DIMENSIONS = {"CPU-ORU", "GPU-ORU", "STOR-ORU", "NET-ORU", "MEM-ORU", "DATA-ORU"}
REQUIRED_USAGE_GROUPINGS = {
    "tenant",
    "actor",
    "workload",
    "app",
    "provider",
    "resource_class",
    "time_window",
    "trace_id",
}
REQUIRED_LEDGER_STATES = {
    "available",
    "reserved",
    "held",
    "spent",
    "earned",
    "sponsored",
    "refunded_corrected",
    "expired_revoked",
    "disputed",
}
REQUIRED_ACCOUNTING_ROLES = {
    "platform_owner",
    "tenant_owner",
    "tenant_admin",
    "support_viewer",
    "product_integrator",
    "incident_responder",
    "accounting_viewer",
}
REQUIRED_RIGHTS_FIELDS = {
    "resource_rights_refs",
    "storage_binding_refs",
    "namespace_binding_refs",
    "route_binding_refs",
    "entitlement_refs",
}
REQUIRED_STABLE_PREFIXES = {
    "actor",
    "app",
    "asset",
    "audit",
    "correction",
    "entitlement",
    "grant",
    "invoice",
    "ledger",
    "namespace",
    "oru",
    "overasset",
    "overbill",
    "overgrant",
    "overmeter",
    "payment_provider",
    "payout_hold",
    "provider",
    "purpose",
    "receipt",
    "refund",
    "resource",
    "rights",
    "route",
    "seal_ledger",
    "settlement",
    "storage",
    "tenant",
    "time_window",
    "trace",
    "usage",
    "workload",
}

FORBIDDEN_PRIVATE_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bdecrypted\b", re.IGNORECASE),
    re.compile(r"\bprompt\b", re.IGNORECASE),
    re.compile(r"\bkey[_ -]?material\b", re.IGNORECASE),
    re.compile(r"/Users/|/home/|[A-Za-z]:\\"),
]

FORBIDDEN_ASSUMPTION_PATTERNS = [
    re.compile(r"\bprice[_ -]?per\b", re.IGNORECASE),
    re.compile(r"\bpricing[_ -]?model\b", re.IGNORECASE),
    re.compile(r"\bcustomer[_ -]?count\b", re.IGNORECASE),
    re.compile(r"\bmarket[_ -]?volume\b", re.IGNORECASE),
    re.compile(r"\brevenue[_ -]?projection\b", re.IGNORECASE),
    re.compile(r"\bblockchain\b", re.IGNORECASE),
    re.compile(r"\bnft\b", re.IGNORECASE),
    re.compile(r"\btokenized[_ -]?ownership\b", re.IGNORECASE),
]
ALLOWED_ASSUMPTION_KEYS = {"no_pricing_assumptions", "blockchain_ownership_model", "nft_ownership_model"}


def assert_contains(content: str, snippet: str, path: Path) -> None:
    if snippet not in content:
        raise AssertionError(f"{path} is missing expected text: {snippet}")


def iter_strings_and_keys(value: Any, path: str = "$") -> list[tuple[str, str, bool]]:
    if isinstance(value, str):
        return [(path, value, False)]
    if isinstance(value, list):
        strings: list[tuple[str, str, bool]] = []
        for index, item in enumerate(value):
            strings.extend(iter_strings_and_keys(item, f"{path}[{index}]"))
        return strings
    if isinstance(value, dict):
        strings = []
        for key, item in value.items():
            strings.append((f"{path}.{key}<key>", str(key), True))
            strings.extend(iter_strings_and_keys(item, f"{path}.{key}"))
        return strings
    return []


def private_content_paths(payload: Any) -> list[str]:
    matches: list[str] = []
    for path, value, _is_key in iter_strings_and_keys(payload):
        for pattern in FORBIDDEN_PRIVATE_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def assumption_paths(payload: Any) -> list[str]:
    matches: list[str] = []
    for path, value, is_key in iter_strings_and_keys(payload):
        if is_key and value in ALLOWED_ASSUMPTION_KEYS:
            continue
        for pattern in FORBIDDEN_ASSUMPTION_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def stable_ref(ref: str) -> bool:
    if ":" not in ref:
        return False
    prefix, value = ref.split(":", 1)
    return bool(prefix in REQUIRED_STABLE_PREFIXES and value and not private_content_paths(ref))


def validate_source_files() -> None:
    source = read_text(ACCOUNTING_VIEWS_TS)
    for snippet in [
        "REQUIRED_USAGE_DIMENSIONS",
        "REQUIRED_USAGE_GROUPINGS",
        "REQUIRED_LEDGER_STATES",
        "REQUIRED_ACCOUNTING_ROLES",
        "REQUIRED_ACCOUNTING_DEPENDENCIES",
        "PHASE8_ACCOUNTING_ROUTES",
        "buildPhase8AccountingWorkspace",
        "buildUsageRollupPanel",
        "buildLedgerReadViews",
        "buildReceiptInvoiceViews",
        "buildGrantVisibility",
        "buildRightsVisibility",
        "buildAccountingAccessMatrix",
        "checkAccountingRefConsistency",
        "hasRequiredUsageDimensionCoverage",
        "hasRequiredLedgerStateCoverage",
        "containsForbiddenAccountingAssumption",
        "directStorageAccess: false",
        "directLedgerMutation: false",
        "noPricingAssumptions: true",
        "blockchainOwnershipModel: false",
        "nftOwnershipModel: false",
        "editableByUi: false",
        "encodesPricingAssumption: false",
        "observed_only",
        "settled_accounting",
        "payment_provider",
        "payout_hold",
        "purpose_scope",
        "storage_binding",
        "namespace_binding",
        "route_binding",
        "entitlement",
    ]:
        assert_contains(source, snippet, ACCOUNTING_VIEWS_TS)
    if source.count('assertRoute(response.path, "/admin/capabilities");') < 2:
        raise AssertionError(
            f"{ACCOUNTING_VIEWS_TS} must route-check both grant and rights visibility responses"
        )
    for forbidden in ["React.", "extends React", "fetch(", "localStorage", "indexedDB", "postgres", "redis", "s3"]:
        if forbidden in source:
            raise AssertionError(f"{ACCOUNTING_VIEWS_TS} must stay framework-neutral and Overgate-bound: {forbidden}")

    index = read_text(INDEX_TS)
    assert_contains(index, "./accounting_usage_views", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_ACCOUNTING)
    private_paths = private_content_paths(payload)
    if private_paths:
        raise AssertionError(f"{VALID_ACCOUNTING} contains private accounting content at {private_paths}")
    assumption_matches = assumption_paths(payload)
    if assumption_matches:
        raise AssertionError(f"{VALID_ACCOUNTING} contains pricing/blockchain assumptions at {assumption_matches}")

    if payload.get("schema_version") != "operator-accounting-usage.v0.1":
        raise AssertionError(f"{VALID_ACCOUNTING} must use the Phase 8 accounting schema marker")
    for field in ["read_only", "uses_overgate_only", "no_pricing_assumptions"]:
        if payload.get(field) is not True:
            raise AssertionError(f"{VALID_ACCOUNTING} must set {field}=true")
    for field in [
        "direct_storage_access",
        "direct_overwatch_connection",
        "direct_ledger_mutation",
        "blockchain_ownership_model",
        "nft_ownership_model",
    ]:
        if payload.get(field) is not False:
            raise AssertionError(f"{VALID_ACCOUNTING} must set {field}=false")
    if set(payload.get("routes", [])) != REQUIRED_ROUTES:
        raise AssertionError(f"{VALID_ACCOUNTING} routes must cover {sorted(REQUIRED_ROUTES)}")
    if set(payload.get("required_usage_dimensions", [])) != REQUIRED_USAGE_DIMENSIONS:
        raise AssertionError(f"{VALID_ACCOUNTING} required usage dimensions are incomplete")
    if set(payload.get("required_usage_groupings", [])) != REQUIRED_USAGE_GROUPINGS:
        raise AssertionError(f"{VALID_ACCOUNTING} required usage groupings are incomplete")
    if set(payload.get("required_ledger_states", [])) != REQUIRED_LEDGER_STATES:
        raise AssertionError(f"{VALID_ACCOUNTING} required ledger states are incomplete")

    usage_rollups = payload.get("usage_rollups", [])
    if {rollup.get("dimension") for rollup in usage_rollups} != REQUIRED_USAGE_DIMENSIONS:
        raise AssertionError(f"{VALID_ACCOUNTING} usage rollups must cover every ORU dimension")
    covered_groupings = {grouping for rollup in usage_rollups for grouping in rollup.get("groupings", [])}
    if not REQUIRED_USAGE_GROUPINGS.issubset(covered_groupings):
        raise AssertionError(f"{VALID_ACCOUNTING} usage rollups must cover every grouping")
    settlement_states = {rollup.get("settlement_state") for rollup in usage_rollups}
    if not {"observed_only", "settled", "held", "disputed"}.issubset(settlement_states):
        raise AssertionError(f"{VALID_ACCOUNTING} must distinguish observed, settled, held, and disputed usage")
    for rollup in usage_rollups:
        for field in ["rollup_ref", "tenant_id", "trace_id", "observed_usage_refs", "timeline_refs", "evidence_links"]:
            if not rollup.get(field):
                raise AssertionError(f"{VALID_ACCOUNTING} usage rollup missing {field}: {rollup}")
        for ref in [
            rollup.get("rollup_ref"),
            *rollup.get("observed_usage_refs", []),
            *rollup.get("settled_accounting_refs", []),
            *rollup.get("timeline_refs", []),
        ]:
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_ACCOUNTING} usage ref is not stable: {ref}")

    ledger_entries = payload.get("ledger_entries", [])
    if {entry.get("state") for entry in ledger_entries} != REQUIRED_LEDGER_STATES:
        raise AssertionError(f"{VALID_ACCOUNTING} ledger entries must cover every required state")
    for entry in ledger_entries:
        if entry.get("editable_by_ui") is not False:
            raise AssertionError(f"{VALID_ACCOUNTING} ledger entries must be read-only: {entry}")
        if not entry.get("immutable_ledger_refs"):
            raise AssertionError(f"{VALID_ACCOUNTING} ledger entries must cite immutable refs: {entry}")
        for ref in [entry.get("account_ref"), *entry.get("ledger_refs", []), *entry.get("immutable_ledger_refs", [])]:
            if not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_ACCOUNTING} ledger ref is not stable: {ref}")

    billing_documents = payload.get("billing_documents", [])
    if not billing_documents:
        raise AssertionError(f"{VALID_ACCOUNTING} must include receipt/invoice documents")
    if not any(document.get("payment_provider_refs") for document in billing_documents):
        raise AssertionError(f"{VALID_ACCOUNTING} must include payment-provider refs")
    if not any(document.get("refund_refs") for document in billing_documents):
        raise AssertionError(f"{VALID_ACCOUNTING} must include refund refs")
    if not any(document.get("correction_refs") for document in billing_documents):
        raise AssertionError(f"{VALID_ACCOUNTING} must include correction refs")
    if not any(document.get("payout_hold_refs") for document in billing_documents):
        raise AssertionError(f"{VALID_ACCOUNTING} must include payout-hold refs")
    for document in billing_documents:
        if document.get("encodes_pricing_assumption") is not False:
            raise AssertionError(f"{VALID_ACCOUNTING} billing documents must not encode pricing assumptions: {document}")
        for ref in [
            document.get("document_ref"),
            *document.get("receipt_refs", []),
            *document.get("invoice_refs", []),
            *document.get("payment_provider_refs", []),
            *document.get("refund_refs", []),
            *document.get("correction_refs", []),
            *document.get("payout_hold_refs", []),
        ]:
            if ref and not stable_ref(str(ref)):
                raise AssertionError(f"{VALID_ACCOUNTING} billing ref is not stable: {ref}")

    grant_cases = payload.get("grant_cases", [])
    if not grant_cases:
        raise AssertionError(f"{VALID_ACCOUNTING} must include grant cases")
    for case in grant_cases:
        for field in [
            "grant_scope_refs",
            "sponsored_allocation_refs",
            "purpose_scope_refs",
            "expiration_refs",
            "correction_refs",
        ]:
            if not case.get(field):
                raise AssertionError(f"{VALID_ACCOUNTING} grant case missing {field}: {case}")
        if case.get("read_only") is not True:
            raise AssertionError(f"{VALID_ACCOUNTING} grant cases must be read-only: {case}")

    rights_cases = payload.get("rights_cases", [])
    if not rights_cases:
        raise AssertionError(f"{VALID_ACCOUNTING} must include rights cases")
    for case in rights_cases:
        for field in REQUIRED_RIGHTS_FIELDS:
            if not case.get(field):
                raise AssertionError(f"{VALID_ACCOUNTING} rights case missing {field}: {case}")
        if case.get("blockchain_ownership_model") is not False or case.get("nft_ownership_model") is not False:
            raise AssertionError(f"{VALID_ACCOUNTING} rights cases must reject blockchain/NFT ownership: {case}")

    access_matrix = payload.get("accounting_access_matrix", [])
    if {entry.get("role") for entry in access_matrix} != REQUIRED_ACCOUNTING_ROLES:
        raise AssertionError(f"{VALID_ACCOUNTING} accounting access matrix must cover required roles")
    for entry in access_matrix:
        if entry.get("role") != "platform_owner" and entry.get("cross_tenant_access") is not False:
            raise AssertionError(f"{VALID_ACCOUNTING} only platform_owner may have cross-tenant accounting visibility")
        if not entry.get("allowed_panels"):
            raise AssertionError(f"{VALID_ACCOUNTING} accounting access entry must name allowed panels: {entry}")


def validate_invalid_fixtures() -> None:
    private_payload = load_json(INVALID_PRIVATE_PAYLOAD)
    if not private_content_paths(private_payload):
        raise AssertionError(f"{INVALID_PRIVATE_PAYLOAD} must fail private-content validation")

    pricing = load_json(INVALID_PRICING)
    if pricing.get("no_pricing_assumptions") is not False:
        raise AssertionError(f"{INVALID_PRICING} must disable the no-pricing guard")
    if not assumption_paths(pricing):
        raise AssertionError(f"{INVALID_PRICING} must fail pricing/revenue/customer-count validation")

    mutation = load_json(INVALID_MUTATION_BLOCKCHAIN)
    for field in ["read_only", "uses_overgate_only"]:
        if mutation.get(field) is not False:
            raise AssertionError(f"{INVALID_MUTATION_BLOCKCHAIN} must set {field}=false")
    for field in ["direct_storage_access", "direct_ledger_mutation", "blockchain_ownership_model", "nft_ownership_model"]:
        if mutation.get(field) is not True:
            raise AssertionError(f"{INVALID_MUTATION_BLOCKCHAIN} must set {field}=true")
    if not any(entry.get("editable_by_ui") for entry in mutation.get("ledger_entries", [])):
        raise AssertionError(f"{INVALID_MUTATION_BLOCKCHAIN} must include editable ledger state")
    if not assumption_paths(mutation):
        raise AssertionError(f"{INVALID_MUTATION_BLOCKCHAIN} must fail blockchain/NFT assumption validation")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase8"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase8 phase_artifacts entry")
    entry = entries[0]
    if entry.get("validator") != "scripts/validate_admin_ui_phase8.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 8 validator path is wrong")
    if str(ACCOUNTING_VIEWS_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 8 source_files must include {ACCOUNTING_VIEWS_TS}")
    fixtures = entry.get("fixtures", {})
    if str(VALID_ACCOUNTING) not in fixtures.get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 8 valid fixtures must include {VALID_ACCOUNTING}")
    for invalid_path in [INVALID_PRIVATE_PAYLOAD, INVALID_PRICING, INVALID_MUTATION_BLOCKCHAIN]:
        if str(invalid_path) not in fixtures.get("invalid", []):
            raise AssertionError(f"{MANIFEST_PATH} Phase 8 invalid fixtures must include {invalid_path}")

    required_text = {
        SHELL_README: [
            "Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights artifacts",
            "src/accounting_usage_views.ts",
            "scripts/validate_admin_ui_phase8.py",
        ],
        SCHEMA_README: [
            "Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights artifacts",
            "accounting usage fixtures",
            "validate_admin_ui_phase8.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 8 usage, ORU, Seal Ledger, receipt, grant, and rights views",
            "packages/admin_ui_shell/src/accounting_usage_views.ts",
            "scripts/validate_admin_ui_phase8.py",
        ],
        PHASE_PROGRESS: [
            "Admin and Developer UI Phase 8 Progress",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 8: Usage, ORU, Seal Ledger, Receipt, Grant, And Rights Views",
            "Build usage rollup panels",
            "Build accounting redaction and access controls",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase8.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        INDEX_TS,
        ACCOUNTING_VIEWS_TS,
        VALID_ACCOUNTING,
        INVALID_PRIVATE_PAYLOAD,
        INVALID_PRICING,
        INVALID_MUTATION_BLOCKCHAIN,
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
    print("Admin UI Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
