#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 4 operator shell artifacts."""

from __future__ import annotations

from pathlib import Path
import re
import sys
from typing import Any

from validate_admin_ui_phase2 import assert_no_private_content, load_json, read_text


SHELL_ROOT = Path("packages/admin_ui_shell")
SHELL_README = SHELL_ROOT / "README.md"
SHELL_TSCONFIG = SHELL_ROOT / "tsconfig.json"
SRC_ROOT = SHELL_ROOT / "src"
CONTRACTS_TS = SRC_ROOT / "contracts.ts"
INDEX_TS = SRC_ROOT / "index.ts"
OVERGATE_CLIENT_TS = SRC_ROOT / "overgate_client.ts"
SESSION_CONTEXT_TS = SRC_ROOT / "session_context.ts"
OPERATOR_SHELL_TS = SRC_ROOT / "operator_shell.ts"
REDACTION_TS = SRC_ROOT / "redaction_primitives.ts"
VIEW_PRESETS_TS = SRC_ROOT / "view_presets.ts"

VALID_PRESET = SHELL_ROOT / "fixtures/valid/operator_view_preset.valid.json"
INVALID_PRESET_SECRET = SHELL_ROOT / "fixtures/invalid/operator_view_preset_secret.invalid.json"
INVALID_PRESET_CROSS_TENANT = SHELL_ROOT / "fixtures/invalid/operator_view_preset_cross_tenant.invalid.json"
INVALID_PRESET_UNKNOWN_PANEL = SHELL_ROOT / "fixtures/invalid/operator_view_preset_unknown_panel.invalid.json"

MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
SCHEMA_README = Path("packages/schemas/admin_ui/README.md")
SUITE_VALIDATOR = Path("scripts/validate_admin_ui.py")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_04_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
SDS = Path("docs/sds/foundation/admin_developer_ui.md")
SERVICE_CATALOG = Path("docs/service_catalog/foundation/admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

SOURCE_FILES = [
    CONTRACTS_TS,
    INDEX_TS,
    OVERGATE_CLIENT_TS,
    SESSION_CONTEXT_TS,
    OPERATOR_SHELL_TS,
    REDACTION_TS,
    VIEW_PRESETS_TS,
]

REQUIRED_PANELS = [
    "overview",
    "tenants",
    "identities",
    "keys",
    "nodes",
    "packages",
    "workloads",
    "queue",
    "policy",
    "usage",
    "ledger",
    "disputes",
    "receipts",
    "diagnostics",
]

REQUIRED_SESSION_STATES = [
    "uninitialized",
    "loading",
    "loaded",
    "failed",
    "stale",
    "permission_denied",
]

FORBIDDEN_DIRECT_URL_LITERAL = re.compile(r"['\"](?:postgres|mysql|redis|s3|vault|file)://", re.IGNORECASE)
FORBIDDEN_PRESET_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bdecrypted\b", re.IGNORECASE),
    re.compile(r"\bprompt\b", re.IGNORECASE),
    re.compile(r"/Users/|/home/|[A-Za-z]:\\"),
]


def assert_contains(content: str, snippet: str, path: Path) -> None:
    if snippet not in content:
        raise AssertionError(f"{path} is missing expected text: {snippet}")


def assert_no_direct_url_literals(path: Path) -> None:
    content = read_text(path)
    if FORBIDDEN_DIRECT_URL_LITERAL.search(content):
        raise AssertionError(f"{path} must not contain direct service URL literals")


def validate_source_files() -> None:
    for path in [SHELL_README, SHELL_TSCONFIG, *SOURCE_FILES]:
        read_text(path)

    contracts = read_text(CONTRACTS_TS)
    for snippet in [
        "admin_ui_contracts",
        "admin_read_api_contracts",
        "AdminSessionContext",
        "AdminListResponse",
        "AdminCapabilitiesResponse",
        "AdminErrorResponse",
    ]:
        assert_contains(contracts, snippet, CONTRACTS_TS)

    index = read_text(INDEX_TS)
    for module in [
        "./contracts",
        "./operator_shell",
        "./overgate_client",
        "./redaction_primitives",
        "./session_context",
        "./view_presets",
    ]:
        assert_contains(index, module, INDEX_TS)


def validate_overgate_client() -> None:
    content = read_text(OVERGATE_CLIENT_TS)
    required_snippets = [
        "AdminReadSignature",
        "assertOvergateBaseUrl",
        "DISALLOWED_BASE_URL_PROTOCOLS",
        "DISALLOWED_SERVICE_HOST_HINTS",
        "Admin UI base URL must not target a core service directly",
        "x-overrid-trace-id",
        "x-overrid-schema-version",
        "x-overrid-signature-refs",
        "x-overrid-idempotency-key",
        "activeTenantId",
        "actorId",
        "buildSignedAdminReadOptions",
        "buildAdminRequest",
        "decodeAdminError",
        "collectPaginatedSummaries",
        "next_cursor_ref",
        "/admin/workloads/{id}/timeline",
        "Admin UI Overgate client requires an explicit transport adapter",
    ]
    for snippet in required_snippets:
        assert_contains(content, snippet, OVERGATE_CLIENT_TS)
    assert_no_direct_url_literals(OVERGATE_CLIENT_TS)


def validate_session_context() -> None:
    content = read_text(SESSION_CONTEXT_TS)
    for state in REQUIRED_SESSION_STATES:
        assert_contains(content, f'"{state}"', SESSION_CONTEXT_TS)
    for snippet in [
        "createUninitializedSession",
        "createLoadingSession",
        "createLoadedSession",
        "createFailedSession",
        "markSessionStale",
        "SessionContextSource",
        "SessionContextLoadOptions",
        "loadSessionContext",
        "signatureRefs",
        "idempotencyKey",
        "selectEnvironment",
        "hasCapability",
        "canRenderOperationalPanels",
        "admin.session.read",
        "admin.session.load_failed",
        "selected_environment",
    ]:
        assert_contains(content, snippet, SESSION_CONTEXT_TS)


def validate_operator_shell() -> None:
    content = read_text(OPERATOR_SHELL_TS)
    for panel_id in REQUIRED_PANELS:
        assert_contains(content, f'id: "{panel_id}"', OPERATOR_SHELL_TS)
    for snippet in [
        "minTableWidth",
        "minWidth",
        "ariaLabel",
        "defaultFilters",
        "emptyState",
        "loadingState",
        "getPanelsForCapabilities",
        "local:diagnostics",
    ]:
        assert_contains(content, snippet, OPERATOR_SHELL_TS)


def validate_redaction_primitives() -> None:
    content = read_text(REDACTION_TS)
    for snippet in [
        '"redacted"',
        '"unavailable"',
        '"denied"',
        '"stale"',
        '"degraded"',
        "PRIVATE_FIELD_HINTS",
        "key_material",
        "private_payload",
        "renderAdminField",
        "formatReasonCode",
        "formatAuditRef",
        "copySafeDiagnosticRef",
        "admin.redaction.private_field",
    ]:
        assert_contains(content, snippet, REDACTION_TS)


def validate_preset_payload(payload: Any, source: Path, expected_tenant: str = "tenant:seed") -> list[str]:
    errors: list[str] = []
    if not isinstance(payload, dict):
        return ["invalid_shape"]
    if payload.get("schema_version") != "operator-view-preset.v0.1":
        errors.append("schema_version")
    if payload.get("actor_id") != "actor:founder":
        errors.append("actor_scope")
    if payload.get("tenant_id") != expected_tenant:
        errors.append("tenant_scope")
    if not payload.get("view_id"):
        errors.append("view_id")
    panel_order = payload.get("panel_order")
    if not isinstance(panel_order, list) or not panel_order:
        errors.append("panel_order")
    else:
        if len(set(panel_order)) != len(panel_order):
            errors.append("panel_order_duplicate")
        if any(not isinstance(panel_id, str) or panel_id not in REQUIRED_PANELS for panel_id in panel_order):
            errors.append("panel_unknown")
        if source == VALID_PRESET and set(panel_order) != set(REQUIRED_PANELS):
            errors.append("panel_coverage")
    columns_by_panel = payload.get("columns_by_panel")
    filters_by_panel = payload.get("filters_by_panel")
    sort_by_panel = payload.get("sort_by_panel")
    if not isinstance(columns_by_panel, dict):
        errors.append("columns_by_panel")
    elif any(key not in REQUIRED_PANELS for key in columns_by_panel):
        errors.append("panel_key_unknown")
    if not isinstance(filters_by_panel, dict):
        errors.append("filters_by_panel")
    elif any(key not in REQUIRED_PANELS for key in filters_by_panel):
        errors.append("panel_key_unknown")
    if isinstance(sort_by_panel, dict) and any(key not in REQUIRED_PANELS for key in sort_by_panel):
        errors.append("panel_key_unknown")
    if payload.get("density") not in {"compact", "comfortable"}:
        errors.append("density")

    for path, value in iter_preset_strings_and_keys(payload):
        for pattern in FORBIDDEN_PRESET_PATTERNS:
            if pattern.search(value):
                errors.append(f"private_content:{path}")
    return errors


def iter_preset_strings_and_keys(value: Any, path: str = "$") -> list[tuple[str, str]]:
    if isinstance(value, str):
        return [(path, value)]
    if isinstance(value, list):
        strings: list[tuple[str, str]] = []
        for index, item in enumerate(value):
            strings.extend(iter_preset_strings_and_keys(item, f"{path}[{index}]"))
        return strings
    if isinstance(value, dict):
        strings = []
        for key, item in value.items():
            strings.append((f"{path}.{key}<key>", str(key)))
            strings.extend(iter_preset_strings_and_keys(item, f"{path}.{key}"))
        return strings
    return []


def validate_view_presets() -> None:
    content = read_text(VIEW_PRESETS_TS)
    for snippet in [
        "OperatorViewPreset",
        "OPERATOR_PANEL_IDS",
        "OPERATOR_PANEL_ID_SET",
        "scopePresetKey",
        "validateOperatorViewPreset",
        "loadScopedPreset",
        "saveScopedPreset",
        "resetScopedPreset",
        "isPresetInScope",
        "isKnownPanelId",
        "hasOnlyKnownPanelKeys",
        "containsForbiddenPresetText",
        "actor_id",
        "tenant_id",
        "admin.preset.scope_mismatch",
        "admin.preset.private_content",
        "admin.preset.panel_unknown",
        "admin.preset.panel_key_unknown",
        "admin.preset.panel_order_duplicate",
    ]:
        assert_contains(content, snippet, VIEW_PRESETS_TS)

    valid = load_json(VALID_PRESET)
    assert_no_private_content(valid, VALID_PRESET)
    valid_errors = validate_preset_payload(valid, VALID_PRESET)
    if valid_errors:
        raise AssertionError(f"{VALID_PRESET} should be valid but failed: {valid_errors}")

    secret_errors = validate_preset_payload(load_json(INVALID_PRESET_SECRET), INVALID_PRESET_SECRET)
    if not any(error.startswith("private_content") for error in secret_errors):
        raise AssertionError(f"{INVALID_PRESET_SECRET} must fail forbidden-content validation")

    cross_tenant_errors = validate_preset_payload(load_json(INVALID_PRESET_CROSS_TENANT), INVALID_PRESET_CROSS_TENANT)
    if "tenant_scope" not in cross_tenant_errors:
        raise AssertionError(f"{INVALID_PRESET_CROSS_TENANT} must fail tenant-scope validation")

    unknown_panel_errors = validate_preset_payload(load_json(INVALID_PRESET_UNKNOWN_PANEL), INVALID_PRESET_UNKNOWN_PANEL)
    if "panel_unknown" not in unknown_panel_errors or "panel_key_unknown" not in unknown_panel_errors:
        raise AssertionError(f"{INVALID_PRESET_UNKNOWN_PANEL} must fail unknown-panel validation")


def validate_manifest_and_docs() -> None:
    manifest = load_json(MANIFEST_PATH)
    entries = [
        entry
        for entry in manifest.get("phase_artifacts", [])
        if entry.get("phase") == "admin_ui_phase4"
    ]
    if len(entries) != 1:
        raise AssertionError(f"{MANIFEST_PATH} must contain one admin_ui_phase4 phase_artifacts entry")
    entry = entries[0]
    if entry.get("package") != str(SHELL_ROOT):
        raise AssertionError(f"{MANIFEST_PATH} Phase 4 package path must be {SHELL_ROOT}")
    if entry.get("validator") != "scripts/validate_admin_ui_phase4.py":
        raise AssertionError(f"{MANIFEST_PATH} Phase 4 validator path is wrong")
    for expected_file in [str(path) for path in SOURCE_FILES]:
        if expected_file not in entry.get("source_files", []):
            raise AssertionError(f"{MANIFEST_PATH} Phase 4 source_files is missing {expected_file}")

    required_text = {
        SHELL_README: [
            "Phase 4 TypeScript operator shell artifacts",
            "Overgate-only request wrapper",
            "Actor/tenant-scoped local preset validation",
        ],
        SCHEMA_README: [
            "Phase 4 TypeScript operator shell artifacts",
            "packages/admin_ui_shell",
            "validate_admin_ui_phase4.py",
        ],
        PHASE_PLAN: [
            "Complete the Phase 4 TypeScript operator shell",
            "packages/admin_ui_shell/src",
            "scripts/validate_admin_ui_phase4.py",
        ],
        PHASE_PROGRESS: [
            "Phase 4 scope",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 4: TypeScript Operator Shell And Session Context",
            "Build the generated-client integration layer",
            "Build saved view preset local state",
        ],
        SDS: [
            "Maintaining local UI state",
            "Redacting sensitive fields before display",
        ],
        SERVICE_CATALOG: [
            "The UI is an Overrid client surface",
            "Overgate admin API",
        ],
        TECH_STACK: [
            "TypeScript web UI is acceptable",
            "Overgate",
            "never become a privileged backdoor",
        ],
        SUITE_VALIDATOR: [
            "scripts/validate_admin_ui_phase4.py",
        ],
    }
    for path, snippets in required_text.items():
        content = read_text(path)
        for snippet in snippets:
            assert_contains(content, snippet, path)


def validate_file_hygiene() -> None:
    checked_paths = [
        SHELL_README,
        SHELL_TSCONFIG,
        VALID_PRESET,
        INVALID_PRESET_SECRET,
        INVALID_PRESET_CROSS_TENANT,
        INVALID_PRESET_UNKNOWN_PANEL,
        MANIFEST_PATH,
        SCHEMA_README,
        PHASE_PLAN,
        PHASE_PROGRESS,
        *SOURCE_FILES,
    ]
    for path in checked_paths:
        content = read_text(path)
        if not content.endswith("\n"):
            raise AssertionError(f"{path} must end with a newline")
        if "\t" in content:
            raise AssertionError(f"{path} must not contain tabs")


def main() -> int:
    validate_source_files()
    validate_overgate_client()
    validate_session_context()
    validate_operator_shell()
    validate_redaction_primitives()
    validate_view_presets()
    validate_manifest_and_docs()
    validate_file_hygiene()
    print("Admin UI Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
