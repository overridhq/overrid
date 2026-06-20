#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 3 read API contracts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import (
    SchemaValidationError,
    assert_no_private_content,
    iter_strings,
    load_json,
    read_text,
    validate_schema,
)


SCHEMA_PATH = Path("packages/schemas/admin_ui/v0/admin_read_api_contracts.schema.json")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
README_PATH = Path("packages/schemas/admin_ui/README.md")
TS_BINDINGS_PATH = Path("packages/schemas/admin_ui/generated/typescript/admin_read_api_contracts.d.ts")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_03_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
ADMIN_UI_SDS = Path("docs/sds/foundation/admin_developer_ui.md")
OVERGATE_SDS = Path("docs/sds/control_plane/overgate.md")
SERVICE_CATALOG = Path("docs/service_catalog/foundation/admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

ROUTES_FIXTURE = Path("packages/schemas/admin_ui/fixtures/valid/admin_read_routes.valid.json")

VALID_FIXTURES = {
    "admin_list_response.valid.json": "admin_list_response",
    "admin_timeline_response_partial.valid.json": "admin_timeline_response",
    "admin_authorization_matrix.valid.json": "admin_authorization_matrix",
    "admin_capabilities_response.valid.json": "admin_capabilities_response",
    "admin_error_response.valid.json": "admin_error_response",
}

INVALID_FIXTURES = {
    "admin_read_route_direct_storage.invalid.json": "admin_read_route",
    "admin_read_route_client_actor_scope.invalid.json": "admin_read_route",
    "admin_list_response_unbounded_limit.invalid.json": "admin_list_response",
    "admin_timeline_private_payload.invalid.json": "admin_timeline_response",
    "admin_authorization_matrix_missing_role.invalid.json": "admin_authorization_matrix",
    "admin_capabilities_direct_storage_fallback.invalid.json": "admin_capabilities_response",
    "admin_error_response_missing_trace.invalid.json": "admin_error_response",
}

REQUIRED_CONTRACTS = [
    "admin_read_route",
    "admin_list_response",
    "admin_timeline_response",
    "admin_authorization_matrix",
    "admin_capabilities_response",
    "admin_error_response",
]

LIST_ROUTE_PATHS = {
    "/admin/tenants",
    "/admin/identities",
    "/admin/keys",
    "/admin/nodes",
    "/admin/packages",
    "/admin/workloads",
    "/admin/queue-items",
    "/admin/policy-decisions",
    "/admin/usage",
    "/admin/ledger",
    "/admin/receipts",
    "/admin/disputes",
}

REQUIRED_ROUTE_PATHS = LIST_ROUTE_PATHS | {
    "/admin/workloads/{id}/timeline",
    "/admin/capabilities",
}

RESOURCE_BY_PATH = {
    "/admin/tenants": "tenant",
    "/admin/identities": "identity",
    "/admin/keys": "key",
    "/admin/nodes": "node",
    "/admin/packages": "package",
    "/admin/workloads": "workload",
    "/admin/queue-items": "queue_item",
    "/admin/policy-decisions": "policy_decision",
    "/admin/usage": "usage",
    "/admin/ledger": "ledger",
    "/admin/receipts": "receipt",
    "/admin/disputes": "dispute",
    "/admin/workloads/{id}/timeline": "workload_timeline",
    "/admin/capabilities": "capability",
}

RESPONSE_BY_PATH = {
    **{path: "admin_list_response" for path in LIST_ROUTE_PATHS},
    "/admin/workloads/{id}/timeline": "admin_timeline_response",
    "/admin/capabilities": "admin_capabilities_response",
}

REQUIRED_ROLES = {
    "platform_owner",
    "tenant_owner",
    "tenant_admin",
    "product_integrator",
    "support_viewer",
    "incident_responder",
    "accounting_viewer",
    "service_account",
    "system_service",
}

POLICY_SCOPES = {
    "admin.read.session",
    "admin.read.summary",
    "admin.read.timeline",
    "admin.read.accounting",
    "admin.read.policy",
    "admin.read.incident",
    "admin.read.capability",
}

REQUIRED_POLICY_SCOPE_BY_PATH = {
    **{path: "admin.read.summary" for path in LIST_ROUTE_PATHS},
    "/admin/policy-decisions": "admin.read.policy",
    "/admin/usage": "admin.read.accounting",
    "/admin/ledger": "admin.read.accounting",
    "/admin/receipts": "admin.read.accounting",
    "/admin/disputes": "admin.read.incident",
    "/admin/workloads/{id}/timeline": "admin.read.timeline",
    "/admin/capabilities": "admin.read.capability",
}

REQUIRED_TS_EXPORTS = [
    "AdminReadRoute",
    "AdminListResponse",
    "AdminTimelineResponse",
    "AdminAuthorizationMatrix",
    "AdminCapabilitiesResponse",
    "AdminErrorResponse",
]

DIRECT_STORAGE_PATTERNS = [
    re.compile(r"\b(postgres|mysql|redis|s3|minio|vault|node-agent|seal-ledger)://", re.IGNORECASE),
    re.compile(r"\bdirect[_ -]?storage\b", re.IGNORECASE),
    re.compile(r"\braw[_ -]?service[_ -]?storage\b", re.IGNORECASE),
]


def fixture_path(kind: str, name: str) -> Path:
    return Path("packages/schemas/admin_ui/fixtures") / kind / name


def assert_no_direct_storage_fallback(payload: Any, source: Path) -> None:
    for path, value in iter_strings(payload):
        for pattern in DIRECT_STORAGE_PATTERNS:
            if pattern.search(value):
                raise SchemaValidationError(f"{source} contains direct-storage fallback at {path}: {value!r}")


def validate_schema_catalog(schema: dict[str, Any]) -> None:
    defs = schema.get("$defs", {})
    for helper_def in ["data_class", "policy_scope"]:
        if helper_def not in defs:
            raise AssertionError(f"{SCHEMA_PATH} is missing $defs.{helper_def}")

    for contract in REQUIRED_CONTRACTS:
        if contract not in defs:
            raise AssertionError(f"{SCHEMA_PATH} is missing $defs.{contract}")
        contract_schema = defs[contract]
        if contract_schema.get("additionalProperties") is not False:
            raise AssertionError(f"{SCHEMA_PATH} $defs.{contract} must reject additional properties")
        if "schema_version" not in contract_schema.get("required", []):
            raise AssertionError(f"{SCHEMA_PATH} $defs.{contract} must require schema_version")

    schema_index = {
        "schema_family": "admin_ui_admin_read_api",
        "schema_version": "admin-ui-admin-api.v0.1",
        "contracts": REQUIRED_CONTRACTS,
    }
    validate_schema(schema, schema_index, schema)

    duplicate_contract_index = {
        "schema_family": "admin_ui_admin_read_api",
        "schema_version": "admin-ui-admin-api.v0.1",
        "contracts": [*REQUIRED_CONTRACTS, REQUIRED_CONTRACTS[0]],
    }
    try:
        validate_schema(schema, duplicate_contract_index, schema)
    except SchemaValidationError:
        pass
    else:
        raise AssertionError(f"{SCHEMA_PATH} must reject duplicate contract names")


def validate_route(route: dict[str, Any], source: Path) -> None:
    path = route["path"]
    if route["owning_service"] != "overgate":
        raise AssertionError(f"{source} route {path} must be owned by Overgate")
    if route["resource_kind"] != RESOURCE_BY_PATH[path]:
        raise AssertionError(f"{source} route {path} has wrong resource_kind")
    if route["response_contract"] != RESPONSE_BY_PATH[path]:
        raise AssertionError(f"{source} route {path} has wrong response_contract")
    if route["error_contract"] != "admin_error_response":
        raise AssertionError(f"{source} route {path} must name admin_error_response as its stable error contract")
    if len(route["bounded_filters"]) > 8:
        raise AssertionError(f"{source} route {path} exceeds bounded filter count")

    pagination = route["cursor_pagination"]
    if pagination["max_limit"] < pagination["default_limit"]:
        raise AssertionError(f"{source} route {path} max_limit is below default_limit")
    if path in LIST_ROUTE_PATHS and pagination["supported"] is not True:
        raise AssertionError(f"{source} list route {path} must support cursor pagination")
    if path not in LIST_ROUTE_PATHS and pagination["supported"] is not False:
        raise AssertionError(f"{source} non-list route {path} must not advertise cursor pagination")

    authz = route["server_authorization"]
    if authz["tenant_scoped"] is not True:
        raise AssertionError(f"{source} route {path} must be tenant scoped")
    if authz["actor_scoped"] is not True:
        raise AssertionError(f"{source} route {path} must be actor scoped")
    if authz["denies_cross_tenant"] is not True:
        raise AssertionError(f"{source} route {path} must deny cross-tenant reads")
    if authz["server_side_only"] is not True:
        raise AssertionError(f"{source} route {path} authorization must be server-side only")
    if not set(authz["policy_scopes"]).issubset(POLICY_SCOPES):
        raise AssertionError(f"{source} route {path} includes unknown policy scopes")
    required_scope = REQUIRED_POLICY_SCOPE_BY_PATH[path]
    if required_scope not in authz["policy_scopes"]:
        raise AssertionError(f"{source} route {path} must include policy scope {required_scope}")
    if route["redaction"]["high_risk_fields_fail_closed"] is not True:
        raise AssertionError(f"{source} route {path} must fail closed for high-risk fields")


def validate_route_set(schema: dict[str, Any]) -> None:
    routes = load_json(ROUTES_FIXTURE)
    if not isinstance(routes, list):
        raise AssertionError(f"{ROUTES_FIXTURE} must be a JSON array")

    seen_paths: set[str] = set()
    for index, route in enumerate(routes):
        validate_schema(schema["$defs"]["admin_read_route"], route, schema, f"{ROUTES_FIXTURE}[{index}]")
        assert_no_private_content(route, ROUTES_FIXTURE)
        assert_no_direct_storage_fallback(route, ROUTES_FIXTURE)
        validate_route(route, ROUTES_FIXTURE)
        seen_paths.add(route["path"])

    if seen_paths != REQUIRED_ROUTE_PATHS:
        raise AssertionError(f"{ROUTES_FIXTURE} covers {sorted(seen_paths)}, expected {sorted(REQUIRED_ROUTE_PATHS)}")


def validate_list_response(payload: dict[str, Any], source: Path) -> None:
    if payload["path"] not in LIST_ROUTE_PATHS:
        raise AssertionError(f"{source} must use a list route path")
    if payload["resource_kind"] != RESOURCE_BY_PATH[payload["path"]]:
        raise AssertionError(f"{source} has wrong resource_kind")
    if payload["page"]["limit"] > 200:
        raise AssertionError(f"{source} page limit must stay within server-controlled max")
    if payload["status"] == "degraded" and not payload["degraded_dependencies"]:
        raise AssertionError(f"{source} degraded response must name degraded dependencies")
    for item in payload["items"]:
        if not item["audit_refs"]:
            raise AssertionError(f"{source} item {item['id']} must include audit refs")


def validate_timeline_response(payload: dict[str, Any], source: Path) -> None:
    node_kinds = {node["kind"] for node in payload["nodes"]}
    if payload["status"] == "partial":
        if "gap" not in node_kinds:
            raise AssertionError(f"{source} partial timeline must include a gap node")
        if not payload["partial_dependencies"]:
            raise AssertionError(f"{source} partial timeline must name unavailable dependencies")
    if payload["status"] == "complete" and "gap" in node_kinds:
        raise AssertionError(f"{source} complete timeline must not include gap nodes")


def validate_authorization_matrix(payload: dict[str, Any], source: Path) -> None:
    roles = {entry["role"]: entry for entry in payload["roles"]}
    if set(roles) != REQUIRED_ROLES:
        raise AssertionError(f"{source} covers roles {sorted(roles)}, expected {sorted(REQUIRED_ROLES)}")
    defaults = payload["fail_closed_defaults"]
    if not all(defaults.values()):
        raise AssertionError(f"{source} fail-closed defaults must all be true")
    for role, entry in roles.items():
        if entry["cross_tenant_access"] is not False:
            raise AssertionError(f"{source} role {role} must not allow client-visible cross-tenant access")
        if not set(entry["allowed_routes"]).issubset(REQUIRED_ROUTE_PATHS):
            raise AssertionError(f"{source} role {role} includes an unknown route")
        if not set(entry["policy_scopes"]).issubset(POLICY_SCOPES):
            raise AssertionError(f"{source} role {role} includes an unknown policy scope")
        required_scopes = {REQUIRED_POLICY_SCOPE_BY_PATH[path] for path in entry["allowed_routes"]}
        if not required_scopes.issubset(set(entry["policy_scopes"])):
            raise AssertionError(f"{source} role {role} is missing required policy scopes {sorted(required_scopes)}")


def validate_capabilities_response(payload: dict[str, Any], source: Path) -> None:
    routes = {entry["path"]: entry for entry in payload["routes"]}
    if set(routes) != REQUIRED_ROUTE_PATHS:
        raise AssertionError(f"{source} route statuses cover {sorted(routes)}, expected {sorted(REQUIRED_ROUTE_PATHS)}")

    unavailable = [entry for entry in payload["routes"] if entry["available"] is False]
    for entry in unavailable:
        if not entry["reason_codes"]:
            raise AssertionError(f"{source} unavailable route {entry['path']} must include reason codes")
    if unavailable and not payload["disabled_panels"]:
        raise AssertionError(f"{source} unavailable routes must produce disabled panel status")

    required_flags = {"admin.session.read", "admin.summary.read", "admin.timeline.read"}
    if not required_flags.issubset(set(payload["feature_flags"])):
        raise AssertionError(f"{source} must expose session, summary, and timeline read flags")
    if payload["limits"]["cursor_required"] is not True:
        raise AssertionError(f"{source} must require cursor-based pagination for list routes")


def validate_error_response(payload: dict[str, Any], source: Path) -> None:
    if payload["status"] < 400:
        raise AssertionError(f"{source} error status must be >= 400")
    if "." not in payload["reason_code"]:
        raise AssertionError(f"{source} reason_code must be stable and namespaced")
    if not payload["audit_refs"]:
        raise AssertionError(f"{source} must include audit refs")


def run_contract_validation(contract_name: str, payload: dict[str, Any], source: Path) -> None:
    if contract_name == "admin_read_route":
        validate_route(payload, source)
    elif contract_name == "admin_list_response":
        validate_list_response(payload, source)
    elif contract_name == "admin_timeline_response":
        validate_timeline_response(payload, source)
    elif contract_name == "admin_authorization_matrix":
        validate_authorization_matrix(payload, source)
    elif contract_name == "admin_capabilities_response":
        validate_capabilities_response(payload, source)
    elif contract_name == "admin_error_response":
        validate_error_response(payload, source)
    else:
        raise AssertionError(f"Unhandled contract: {contract_name}")


def validate_valid_fixtures(schema: dict[str, Any]) -> None:
    validate_route_set(schema)
    for fixture_name, contract_name in VALID_FIXTURES.items():
        path = fixture_path("valid", fixture_name)
        payload = load_json(path)
        validate_schema(schema["$defs"][contract_name], payload, schema, str(path))
        assert_no_private_content(payload, path)
        assert_no_direct_storage_fallback(payload, path)
        run_contract_validation(contract_name, payload, path)


def validate_invalid_fixtures(schema: dict[str, Any]) -> None:
    for fixture_name, contract_name in INVALID_FIXTURES.items():
        path = fixture_path("invalid", fixture_name)
        payload = load_json(path)
        try:
            validate_schema(schema["$defs"][contract_name], payload, schema, str(path))
            assert_no_private_content(payload, path)
            assert_no_direct_storage_fallback(payload, path)
            run_contract_validation(contract_name, payload, path)
        except (AssertionError, SchemaValidationError):
            continue
        raise AssertionError(f"{path} unexpectedly passed validation")


def validate_projection_and_manifest() -> None:
    manifest = load_json(MANIFEST_PATH)
    phase3_entries = [
        entry
        for entry in manifest.get("additional_schemas", [])
        if entry.get("phase") == "admin_ui_phase3"
    ]
    if len(phase3_entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase3 additional_schemas entry")
    phase3 = phase3_entries[0]
    if phase3.get("canonical_schema") != str(SCHEMA_PATH):
        raise AssertionError(f"{MANIFEST_PATH} Phase 3 canonical schema path is not {SCHEMA_PATH}")
    if phase3.get("validator") != "scripts/validate_admin_ui_phase3.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 3 validator path is wrong")
    projection = phase3.get("typescript_projection", {})
    if projection.get("source_schema") != str(SCHEMA_PATH):
        raise AssertionError(f"{MANIFEST_PATH} Phase 3 projection source schema is wrong")
    if projection.get("path") != str(TS_BINDINGS_PATH):
        raise AssertionError(f"{MANIFEST_PATH} Phase 3 TypeScript projection path is wrong")
    if projection.get("non_authoritative") is not True:
        raise AssertionError(f"{MANIFEST_PATH} must mark Phase 3 TypeScript projection as non-authoritative")

    bindings = read_text(TS_BINDINGS_PATH)
    if "Generated from packages/schemas/admin_ui/v0/admin_read_api_contracts.schema.json" not in bindings:
        raise AssertionError(f"{TS_BINDINGS_PATH} must identify its schema source")
    if "Do not treat this projection as authoritative" not in bindings:
        raise AssertionError(f"{TS_BINDINGS_PATH} must mark the projection as non-authoritative")
    for exported_name in REQUIRED_TS_EXPORTS:
        if f"interface {exported_name}" not in bindings:
            raise AssertionError(f"{TS_BINDINGS_PATH} is missing interface {exported_name}")
    for required_snippet in [
        "export type AdminResourceKind =",
        "export type AdminDataClass =",
        "export type AdminPolicyScope =",
        "error_contract: \"admin_error_response\";",
        "actor_scoped: boolean;",
        "policy_scopes: NonEmptyArray<AdminPolicyScope>;",
        "roles_allowed: NonEmptyArray<AdminRole>;",
        "audit_refs: NonEmptyArray<AdminAuditRef>;",
        "routes: NonEmptyArray<{",
        "feature_flags: NonEmptyArray<AdminCapabilityFlag>;",
    ]:
        if required_snippet not in bindings:
            raise AssertionError(f"{TS_BINDINGS_PATH} is missing expected projection detail: {required_snippet}")


def validate_docs() -> None:
    required_text = {
        README_PATH: [
            "Phase 3 Overgate-admin read API contract artifacts",
            "admin_read_api_contracts.schema.json",
            "validate_admin_ui_phase3.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 3 Overgate-admin read API contract work",
            "packages/schemas/admin_ui/v0/admin_read_api_contracts.schema.json",
            "scripts/validate_admin_ui_phase3.py",
        ],
        PHASE_PROGRESS: [
            "Phase 3 scope",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 3: Overgate Admin Read API Contracts",
            "GET /admin/*",
            "GET /admin/workloads/{id}/timeline",
            "Admin capabilities response",
        ],
        ADMIN_UI_SDS: [
            "GET /admin/tenants",
            "GET /admin/workloads/{id}/timeline",
            "Error responses must include stable reason codes",
        ],
        OVERGATE_SDS: [
            "Schema validation before side effects",
            "Stable denial responses with reason codes",
            "Admin endpoints must require signed operator or service-account credentials",
        ],
        SERVICE_CATALOG: [
            "Overgate admin API",
            "Read-only view models with server-side tenant, role, and data-class filtering",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "Canonical JSON plus JSON Schema",
        ],
    }

    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            if snippet not in content:
                raise AssertionError(f"{path} is missing expected text: {snippet}")


def validate_file_hygiene() -> None:
    checked_paths = [
        SCHEMA_PATH,
        MANIFEST_PATH,
        README_PATH,
        TS_BINDINGS_PATH,
        PHASE_PLAN,
        PHASE_PROGRESS,
        ROUTES_FIXTURE,
        *[fixture_path("valid", name) for name in VALID_FIXTURES],
        *[fixture_path("invalid", name) for name in INVALID_FIXTURES],
    ]
    for path in checked_paths:
        content = read_text(path)
        if not content.endswith("\n"):
            raise AssertionError(f"{path} must end with a newline")
        if "\t" in content:
            raise AssertionError(f"{path} must not contain tabs")


def main() -> int:
    schema = load_json(SCHEMA_PATH)
    validate_schema_catalog(schema)
    validate_valid_fixtures(schema)
    validate_invalid_fixtures(schema)
    validate_projection_and_manifest()
    validate_docs()
    validate_file_hygiene()
    print("Admin UI Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
