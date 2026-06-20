#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 5 operational summary artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SRC_ROOT = SHELL_ROOT / "src"
INDEX_TS = SRC_ROOT / "index.ts"
OPERATOR_SHELL_TS = SRC_ROOT / "operator_shell.ts"
OVERGATE_CLIENT_TS = SRC_ROOT / "overgate_client.ts"
OPERATIONAL_PANELS_TS = SRC_ROOT / "operational_summary_panels.ts"

VALID_OPERATIONAL_SUMMARY = SHELL_ROOT / "fixtures/valid/operational_summary_panels.valid.json"
INVALID_CROSS_TENANT = SHELL_ROOT / "fixtures/invalid/operational_summary_cross_tenant.invalid.json"
INVALID_KEY_MATERIAL = SHELL_ROOT / "fixtures/invalid/operational_summary_key_material.invalid.json"
INVALID_UNBOUNDED_QUERY = SHELL_ROOT / "fixtures/invalid/operational_summary_unbounded_query.invalid.json"

SHELL_README = SHELL_ROOT / "README.md"
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_05_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

REQUIRED_PANELS = {"tenants", "identities", "keys", "nodes", "packages", "workloads", "queue"}
REQUIRED_DEPENDENCY_SERVICES = {
    "overgate",
    "overpass",
    "overtenant",
    "overkey",
    "overregistry",
    "overwatch",
    "overqueue",
    "oversched",
    "overlease",
    "overcell",
    "overrun",
}
REQUIRED_NODE_STATES = {"live", "stale", "expired", "draining", "denied", "unverified"}
REQUIRED_PRODUCT_FAMILIES = {"docdex", "mcoda", "codali", "cli", "sdk"}
REQUIRED_PANEL_COLUMN_KEYS = {
    "tenants": {"tenant_id", "role_bindings", "quota_state"},
    "identities": {"identity_type", "role_bindings", "service_account", "system_service", "last_seen_at"},
    "keys": {"key_metadata_ref", "rotation_state", "revocation_state", "last_used_at"},
    "nodes": {
        "health",
        "heartbeat_age",
        "capability_records",
        "trust_class",
        "region",
        "current_leases",
        "verification_state",
        "benchmark_refs",
        "drain_readiness",
        "maintenance_readiness",
    },
    "packages": {"manifest_ref", "package_ref", "verification_state", "product_family"},
    "workloads": {
        "workload_request_ref",
        "manifest_ref",
        "package_ref",
        "queue_state",
        "priority",
        "retry_count",
        "cancellation_eligible",
        "timeout_state",
        "terminal_outcome",
        "stale_age",
    },
    "queue": {
        "queue_state",
        "priority",
        "retry_count",
        "cancellation_eligible",
        "timeout_state",
        "terminal_outcome",
        "stale_age",
    },
}
REQUIRED_ROW_FIELDS = {
    "tenants": {"role_bindings", "quota_state"},
    "identities": {"identity_type", "role_bindings", "service_account", "system_service", "last_seen_at"},
    "keys": {"key_metadata_ref", "rotation_state", "revocation_state", "last_used_at"},
    "nodes": {
        "health",
        "heartbeat_age_ms",
        "capability_records",
        "trust_class",
        "region",
        "current_leases",
        "verification_state",
        "benchmark_refs",
        "drain_readiness",
        "maintenance_readiness",
    },
    "packages": {"manifest_ref", "package_ref", "verification_state", "product_family"},
    "workloads": {
        "workload_request_ref",
        "manifest_ref",
        "package_ref",
        "queue_state",
        "priority",
        "retry_count",
        "cancellation_eligible",
        "timeout_state",
        "terminal_outcome",
        "stale_age_ms",
    },
    "queue": {
        "queue_state",
        "priority",
        "retry_count",
        "cancellation_eligible",
        "timeout_state",
        "terminal_outcome",
        "stale_age_ms",
    },
}

FORBIDDEN_SUMMARY_PATTERNS = [
    re.compile(r"\bkey_material\b", re.IGNORECASE),
    re.compile(r"\braw[-_ ]?private[-_ ]?key\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
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
        for pattern in FORBIDDEN_SUMMARY_PATTERNS:
            if pattern.search(value):
                matches.append(path)
    return matches


def validate_source_files() -> None:
    source = read_text(OPERATIONAL_PANELS_TS)
    for snippet in [
        "PHASE5_OPERATIONAL_PANEL_IDS",
        "REQUIRED_DEPENDENCY_SERVICES",
        "REQUIRED_NODE_FIXTURE_STATES",
        "REQUIRED_WORKLOAD_PRODUCT_FAMILIES",
        "createBoundedSummaryQuery",
        "toAdminRequestOptions",
        "buildDependencyHealthStrip",
        "getPanelDependencyStatuses",
        "getAffectedPanelsForDependency",
        "createOperationalSummaryRows",
        "filterTenantVisibleRows",
        "assertTenantScopedSummaries",
        "buildOperationalSummaryPanelState",
        "classifyNodeOperationalState",
        "detectProductWorkloadFamily",
        "hasRequiredProductFixtureCoverage",
        "normalizeDependencyStatus",
        "staleAgeMs",
        "refreshDue",
        'return "stale"',
        "admin.summary.cross_tenant_denied",
        "admin.query.limit_clamped",
        "admin.query.filter_limit_exceeded",
        "findOperatorPanel",
    ]:
        assert_contains(source, snippet, OPERATIONAL_PANELS_TS)

    operator_shell = read_text(OPERATOR_SHELL_TS)
    for snippet in [
        'id: "packages"',
        'route: "/admin/packages"',
        'id: "queue"',
        'route: "/admin/queue-items"',
        "Package manifest and verification summary table",
        "Queue item summary table",
    ]:
        assert_contains(operator_shell, snippet, OPERATOR_SHELL_TS)
    for panel, column_keys in REQUIRED_PANEL_COLUMN_KEYS.items():
        for column_key in column_keys:
            assert_contains(operator_shell, f'key: "{column_key}"', OPERATOR_SHELL_TS)

    overgate_client = read_text(OVERGATE_CLIENT_TS)
    for snippet in [
        "MAX_ADMIN_QUERY_LIMIT",
        "MAX_ADMIN_FILTER_COUNT",
        "Admin UI summary queries must use a bounded server-side limit.",
        "Admin UI summary queries must use bounded filter sets.",
    ]:
        assert_contains(overgate_client, snippet, OVERGATE_CLIENT_TS)

    index = read_text(INDEX_TS)
    assert_contains(index, "./operational_summary_panels", INDEX_TS)


def validate_valid_fixture() -> None:
    payload = load_json(VALID_OPERATIONAL_SUMMARY)
    if private_content_paths(payload):
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} contains private summary content")

    if set(payload.get("panels", [])) != REQUIRED_PANELS:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} must cover Phase 5 panels")

    services = {item.get("service") for item in payload.get("dependency_health", [])}
    if services != REQUIRED_DEPENDENCY_SERVICES:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} dependency services are incomplete: {sorted(services)}")

    route_max_limit = payload.get("route_max_limit")
    filter_max_count = payload.get("filter_max_count")
    if not isinstance(route_max_limit, int) or route_max_limit <= 0:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} must define a positive route_max_limit")
    if not isinstance(filter_max_count, int) or filter_max_count <= 0:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} must define a positive filter_max_count")

    query_panels = {query.get("panel") for query in payload.get("queries", [])}
    if query_panels != REQUIRED_PANELS:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} must include one bounded query for every Phase 5 panel")

    for query in payload.get("queries", []):
        if query.get("panel") not in REQUIRED_PANELS:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} query has unknown panel: {query}")
        if query.get("limit", 0) > route_max_limit:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} query exceeds route limit: {query}")
        if len(query.get("filters", {})) > filter_max_count:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} query exceeds filter limit: {query}")
        if query.get("refresh_window_ms", 0) < 5000:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} query refresh window is unbounded: {query}")
        if query.get("refresh_mode") != "manual_refresh":
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} should prefer manual refresh for Phase 5: {query}")

    active_tenant_id = payload.get("active_tenant_id")
    node_states: set[str] = set()
    product_families: set[str] = set()
    rows_by_panel: dict[str, list[dict[str, Any]]] = {panel: [] for panel in REQUIRED_PANELS}
    for row in payload.get("rows", []):
        panel = row.get("panel")
        if panel in rows_by_panel:
            rows_by_panel[panel].append(row)
        if row.get("tenant_id") != active_tenant_id:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} row leaks tenant scope: {row}")
        if not row.get("audit_refs"):
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} row lacks audit_refs: {row}")
        required_fields = REQUIRED_ROW_FIELDS.get(panel, set())
        missing_fields = sorted(field for field in required_fields if field not in row)
        if missing_fields:
            raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} row for {panel} misses fields {missing_fields}: {row}")
        if row.get("kind") == "node":
            node_states.add(row.get("node_state"))
        if row.get("kind") in {"package", "workload", "queue_item"} and row.get("product_family"):
            product_families.add(row["product_family"])

    empty_panels = sorted(panel for panel, rows in rows_by_panel.items() if not rows)
    if empty_panels:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} has no rows for panels: {empty_panels}")
    if node_states != REQUIRED_NODE_STATES:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} node states {sorted(node_states)} are incomplete")
    if product_families != REQUIRED_PRODUCT_FAMILIES:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} product families {sorted(product_families)} are incomplete")
    identity_types = {row.get("identity_type") for row in rows_by_panel["identities"]}
    if not {"human", "service_account", "system_service"}.issubset(identity_types):
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} identity rows must cover human, service_account, and system_service")

    dependency_case = payload.get("dependency_scope_case", {})
    if dependency_case.get("unavailable_service") != "overqueue":
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} must include overqueue dependency-scope case")
    if set(dependency_case.get("affected_panels", [])) != {"workloads", "queue"}:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} overqueue must affect only workloads and queue")
    if set(dependency_case.get("unaffected_panels", [])) != {"tenants", "identities", "keys", "nodes", "packages"}:
        raise AssertionError(f"{VALID_OPERATIONAL_SUMMARY} overqueue unaffected panel list is wrong")


def validate_invalid_fixtures() -> None:
    cross_tenant = load_json(INVALID_CROSS_TENANT)
    active_tenant_id = cross_tenant.get("active_tenant_id")
    if not any(row.get("tenant_id") != active_tenant_id for row in cross_tenant.get("rows", [])):
        raise AssertionError(f"{INVALID_CROSS_TENANT} must contain a cross-tenant row")

    key_material = load_json(INVALID_KEY_MATERIAL)
    if not private_content_paths(key_material):
        raise AssertionError(f"{INVALID_KEY_MATERIAL} must fail private key-material validation")

    unbounded_query = load_json(INVALID_UNBOUNDED_QUERY)
    route_max_limit = unbounded_query.get("route_max_limit")
    filter_max_count = unbounded_query.get("filter_max_count")
    if not any(query.get("limit", 0) > route_max_limit for query in unbounded_query.get("queries", [])):
        raise AssertionError(f"{INVALID_UNBOUNDED_QUERY} must exceed route_max_limit")
    if not any(len(query.get("filters", {})) > filter_max_count for query in unbounded_query.get("queries", [])):
        raise AssertionError(f"{INVALID_UNBOUNDED_QUERY} must exceed filter_max_count")
    if not any(query.get("refresh_window_ms", 0) < 5000 for query in unbounded_query.get("queries", [])):
        raise AssertionError(f"{INVALID_UNBOUNDED_QUERY} must include an unsafe refresh window")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase5"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase5 phase_artifacts entry")
    entry = entries[0]
    if entry.get("package") != str(SHELL_ROOT):
        raise AssertionError(f"{MANIFEST_PATH} Phase 5 package path must be {SHELL_ROOT}")
    if entry.get("validator") != "scripts/validate_admin_ui_phase5.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 5 validator path is wrong")
    if str(OPERATIONAL_PANELS_TS) not in entry.get("source_files", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 5 source_files must include {OPERATIONAL_PANELS_TS}")
    if str(VALID_OPERATIONAL_SUMMARY) not in entry.get("fixtures", {}).get("valid", []):
        raise AssertionError(f"{MANIFEST_PATH} Phase 5 valid fixtures must include {VALID_OPERATIONAL_SUMMARY}")

    required_text = {
        SHELL_README: [
            "Phase 5 read-only operational summary panel artifacts",
            "src/operational_summary_panels.ts",
            "scripts/validate_admin_ui_phase5.py",
        ],
        SCHEMA_README: [
            "Phase 5 read-only operational summary panel artifacts",
            "operational summary fixtures",
            "validate_admin_ui_phase5.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 5 read-only operational summary panels",
            "packages/admin_ui_shell/src/operational_summary_panels.ts",
            "scripts/validate_admin_ui_phase5.py",
        ],
        PHASE_PROGRESS: [
            "Phase 5 scope",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 5: Read-Only Operational Summary Panels",
            "Build tenant, identity, and key panels",
            "Build bounded query behavior",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase5.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        INDEX_TS,
        OPERATOR_SHELL_TS,
        OVERGATE_CLIENT_TS,
        OPERATIONAL_PANELS_TS,
        VALID_OPERATIONAL_SUMMARY,
        INVALID_CROSS_TENANT,
        INVALID_KEY_MATERIAL,
        INVALID_UNBOUNDED_QUERY,
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
    print("Admin UI Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
