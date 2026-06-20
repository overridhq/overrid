#!/usr/bin/env python3
"""Validate Admin and Developer UI Phase 2 schema contracts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

SCHEMA_PATH = Path("packages/schemas/admin_ui/v0/admin_ui_contracts.schema.json")
MANIFEST_PATH = Path("packages/schemas/admin_ui/codegen_manifest.json")
README_PATH = Path("packages/schemas/admin_ui/README.md")
TS_BINDINGS_PATH = Path("packages/schemas/admin_ui/generated/typescript/admin_ui_contracts.d.ts")
PHASE_PLAN = Path("docs/planning/admin_developer_ui_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/admin_developer_ui_phase_02_progress.md")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_001_admin_developer_ui.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")

VALID_FIXTURES = {
    "admin_session_context.valid.json": "admin_session_context",
    "resource_summary_view.valid.json": "resource_summary_view",
    "job_timeline_successful.valid.json": "job_timeline_view",
    "job_timeline_denied.valid.json": "job_timeline_view",
    "job_timeline_cancelled.valid.json": "job_timeline_view",
    "job_timeline_timed_out.valid.json": "job_timeline_view",
    "job_timeline_disputed.valid.json": "job_timeline_view",
    "admin_action_request.valid.json": "admin_action_request",
    "admin_action_receipt.valid.json": "admin_action_receipt",
    "ui_diagnostic_event.valid.json": "ui_diagnostic_event",
}

RESOURCE_SUMMARY_VARIANTS_FIXTURE = Path(
    "packages/schemas/admin_ui/fixtures/valid/resource_summary_variants.valid.json"
)

INVALID_FIXTURES = {
    "admin_session_context_missing_tenant.invalid.json": "admin_session_context",
    "resource_summary_missing_audit_ref.invalid.json": "resource_summary_view",
    "job_timeline_private_payload.invalid.json": "job_timeline_view",
    "admin_action_request_unsigned.invalid.json": "admin_action_request",
    "admin_action_request_missing_reason.invalid.json": "admin_action_request",
    "admin_action_request_missing_expected_state.invalid.json": "admin_action_request",
    "admin_action_request_empty_policy_refs.invalid.json": "admin_action_request",
    "admin_action_request_unsupported_action.invalid.json": "admin_action_request",
    "ui_diagnostic_event_secret_payload.invalid.json": "ui_diagnostic_event",
    "ui_diagnostic_event_private_path.invalid.json": "ui_diagnostic_event",
}

REQUIRED_CONTRACTS = [
    "admin_session_context",
    "resource_summary_view",
    "job_timeline_view",
    "admin_action_request",
    "admin_action_receipt",
    "ui_diagnostic_event",
]

REQUIRED_TIMELINE_OUTCOMES = {
    "successful",
    "denied",
    "cancelled",
    "timed_out",
    "disputed",
}

REQUIRED_RESOURCE_SUMMARY_KINDS = {
    "tenant",
    "identity",
    "key",
    "node",
    "package",
    "workload",
    "queue_item",
    "lease",
    "usage",
    "receipt",
    "dispute",
    "policy_decision",
}

REQUIRED_TS_EXPORTS = [
    "AdminSessionContext",
    "ResourceSummaryView",
    "JobTimelineView",
    "AdminActionRequest",
    "AdminActionReceipt",
    "UiDiagnosticEvent",
]

FORBIDDEN_TEXT_PATTERNS = [
    re.compile(r"\bpassword\s*=", re.IGNORECASE),
    re.compile(r"\bsecret\b", re.IGNORECASE),
    re.compile(r"\bcredential\b", re.IGNORECASE),
    re.compile(r"\bprivate[_ -]?payload\b", re.IGNORECASE),
    re.compile(r"\bdecrypted\b", re.IGNORECASE),
    re.compile(r"\bprompt\b", re.IGNORECASE),
    re.compile(r"(/Users/|/home/|[A-Za-z]:\\)"),
]


class SchemaValidationError(AssertionError):
    """Raised for local schema validation failures."""


def read_text(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def load_json(path: Path) -> Any:
    try:
        return json.loads(read_text(path))
    except json.JSONDecodeError as error:
        raise AssertionError(f"{path} is invalid JSON: {error}") from error


def resolve_ref(root_schema: dict[str, Any], ref: str) -> dict[str, Any]:
    if not ref.startswith("#/"):
        raise SchemaValidationError(f"Only local JSON Schema refs are supported: {ref}")
    current: Any = root_schema
    for part in ref[2:].split("/"):
        current = current[part]
    if not isinstance(current, dict):
        raise SchemaValidationError(f"Schema ref does not resolve to an object: {ref}")
    return current


def type_matches(expected: str, value: Any) -> bool:
    if expected == "object":
        return isinstance(value, dict)
    if expected == "array":
        return isinstance(value, list)
    if expected == "string":
        return isinstance(value, str)
    if expected == "integer":
        return isinstance(value, int) and not isinstance(value, bool)
    if expected == "number":
        return (isinstance(value, int) or isinstance(value, float)) and not isinstance(value, bool)
    if expected == "boolean":
        return isinstance(value, bool)
    raise SchemaValidationError(f"Unsupported schema type: {expected}")


def validate_schema(schema: dict[str, Any], value: Any, root_schema: dict[str, Any], path: str = "$") -> None:
    if "$ref" in schema:
        validate_schema(resolve_ref(root_schema, schema["$ref"]), value, root_schema, path)
        return

    if "const" in schema and value != schema["const"]:
        raise SchemaValidationError(f"{path} expected const {schema['const']!r}, got {value!r}")

    if "enum" in schema and value not in schema["enum"]:
        raise SchemaValidationError(f"{path} expected one of {schema['enum']!r}, got {value!r}")

    expected_type = schema.get("type")
    if expected_type:
        if isinstance(expected_type, list):
            if not any(type_matches(item, value) for item in expected_type):
                raise SchemaValidationError(f"{path} expected one of types {expected_type!r}")
        elif not type_matches(expected_type, value):
            raise SchemaValidationError(f"{path} expected type {expected_type}, got {type(value).__name__}")

    if isinstance(value, str):
        if "minLength" in schema and len(value) < schema["minLength"]:
            raise SchemaValidationError(f"{path} must be at least {schema['minLength']} characters")
        if "pattern" in schema and re.fullmatch(schema["pattern"], value) is None:
            raise SchemaValidationError(f"{path} does not match pattern {schema['pattern']!r}: {value!r}")

    if isinstance(value, (int, float)) and not isinstance(value, bool):
        if "minimum" in schema and value < schema["minimum"]:
            raise SchemaValidationError(f"{path} must be >= {schema['minimum']}")

    if isinstance(value, list):
        if "minItems" in schema and len(value) < schema["minItems"]:
            raise SchemaValidationError(f"{path} must contain at least {schema['minItems']} item(s)")
        if schema.get("uniqueItems") is True:
            seen: set[str] = set()
            for index, item in enumerate(value):
                stable_item = json.dumps(item, sort_keys=True)
                if stable_item in seen:
                    raise SchemaValidationError(f"{path}[{index}] duplicates an earlier item")
                seen.add(stable_item)
        item_schema = schema.get("items")
        if item_schema:
            for index, item in enumerate(value):
                validate_schema(item_schema, item, root_schema, f"{path}[{index}]")

    if isinstance(value, dict):
        required = schema.get("required", [])
        for key in required:
            if key not in value:
                raise SchemaValidationError(f"{path} is missing required field {key!r}")

        properties = schema.get("properties", {})
        if schema.get("additionalProperties") is False:
            extra = sorted(set(value) - set(properties))
            if extra:
                raise SchemaValidationError(f"{path} contains unsupported field(s): {', '.join(extra)}")

        for key, child_schema in properties.items():
            if key in value:
                validate_schema(child_schema, value[key], root_schema, f"{path}.{key}")


def iter_strings(value: Any, path: str = "$") -> list[tuple[str, str]]:
    if isinstance(value, str):
        return [(path, value)]
    if isinstance(value, list):
        strings: list[tuple[str, str]] = []
        for index, item in enumerate(value):
            strings.extend(iter_strings(item, f"{path}[{index}]"))
        return strings
    if isinstance(value, dict):
        strings = []
        for key, item in value.items():
            strings.extend(iter_strings(item, f"{path}.{key}"))
        return strings
    return []


def assert_no_private_content(payload: Any, source: Path) -> None:
    for path, value in iter_strings(payload):
        if ".hidden_fields[" in path:
            continue
        for pattern in FORBIDDEN_TEXT_PATTERNS:
            if pattern.search(value):
                raise SchemaValidationError(f"{source} contains diagnostic-private text at {path}: {value!r}")


def validate_schema_catalog(schema: dict[str, Any]) -> None:
    defs = schema.get("$defs", {})
    for contract in REQUIRED_CONTRACTS:
        if contract not in defs:
            raise AssertionError(f"{SCHEMA_PATH} is missing $defs.{contract}")
        contract_schema = defs[contract]
        if contract_schema.get("additionalProperties") is not False:
            raise AssertionError(f"{SCHEMA_PATH} $defs.{contract} must reject additional properties")
        if "schema_version" not in contract_schema.get("required", []):
            raise AssertionError(f"{SCHEMA_PATH} $defs.{contract} must require schema_version")

    schema_index = {
        "schema_family": "admin_ui",
        "schema_version": "admin-ui.v0.1",
        "contracts": REQUIRED_CONTRACTS,
    }
    validate_schema(schema, schema_index, schema)

    duplicate_contract_index = {
        "schema_family": "admin_ui",
        "schema_version": "admin-ui.v0.1",
        "contracts": [*REQUIRED_CONTRACTS, REQUIRED_CONTRACTS[0]],
    }
    try:
        validate_schema(schema, duplicate_contract_index, schema)
    except SchemaValidationError:
        pass
    else:
        raise AssertionError(f"{SCHEMA_PATH} must reject duplicate contract names")


def fixture_path(kind: str, name: str) -> Path:
    return Path("packages/schemas/admin_ui/fixtures") / kind / name


def validate_valid_fixtures(schema: dict[str, Any]) -> None:
    outcomes: set[str] = set()
    resource_kinds: set[str] = set()
    for fixture_name, contract_name in VALID_FIXTURES.items():
        path = fixture_path("valid", fixture_name)
        payload = load_json(path)
        validate_schema(schema["$defs"][contract_name], payload, schema, str(path))
        assert_no_private_content(payload, path)
        if contract_name == "resource_summary_view":
            resource_kinds.add(payload["kind"])
        if contract_name == "job_timeline_view":
            outcomes.add(payload["outcome"])
            if not payload["nodes"]:
                raise AssertionError(f"{path} must include at least one timeline node")
            node_kinds = {node["kind"] for node in payload["nodes"]}
            if "gap" in node_kinds and payload["outcome"] == "successful":
                raise AssertionError(f"{path} successful timeline must not rely on a gap node")

    if outcomes != REQUIRED_TIMELINE_OUTCOMES:
        raise AssertionError(f"Timeline fixtures cover {sorted(outcomes)}, expected {sorted(REQUIRED_TIMELINE_OUTCOMES)}")

    variant_payloads = load_json(RESOURCE_SUMMARY_VARIANTS_FIXTURE)
    if not isinstance(variant_payloads, list):
        raise AssertionError(f"{RESOURCE_SUMMARY_VARIANTS_FIXTURE} must be a JSON array")
    for index, payload in enumerate(variant_payloads):
        validate_schema(schema["$defs"]["resource_summary_view"], payload, schema, f"{RESOURCE_SUMMARY_VARIANTS_FIXTURE}[{index}]")
        assert_no_private_content(payload, RESOURCE_SUMMARY_VARIANTS_FIXTURE)
        resource_kinds.add(payload["kind"])

    if resource_kinds != REQUIRED_RESOURCE_SUMMARY_KINDS:
        raise AssertionError(
            f"Resource summary fixtures cover {sorted(resource_kinds)}, "
            f"expected {sorted(REQUIRED_RESOURCE_SUMMARY_KINDS)}"
        )


def validate_invalid_fixtures(schema: dict[str, Any]) -> None:
    for fixture_name, contract_name in INVALID_FIXTURES.items():
        path = fixture_path("invalid", fixture_name)
        payload = load_json(path)
        try:
            validate_schema(schema["$defs"][contract_name], payload, schema, str(path))
            assert_no_private_content(payload, path)
        except SchemaValidationError:
            continue
        raise AssertionError(f"{path} unexpectedly passed validation")


def validate_projection_and_manifest() -> None:
    manifest = load_json(MANIFEST_PATH)
    if manifest.get("source_of_truth") != "json_schema":
        raise AssertionError(f"{MANIFEST_PATH} must keep JSON Schema as source_of_truth")
    if manifest.get("typescript_projection", {}).get("non_authoritative") is not True:
        raise AssertionError(f"{MANIFEST_PATH} must mark TypeScript projection as non-authoritative")
    if manifest.get("canonical_schema") != str(SCHEMA_PATH):
        raise AssertionError(f"{MANIFEST_PATH} canonical schema path is not {SCHEMA_PATH}")

    bindings = read_text(TS_BINDINGS_PATH)
    if "Generated from packages/schemas/admin_ui/v0/admin_ui_contracts.schema.json" not in bindings:
        raise AssertionError(f"{TS_BINDINGS_PATH} must identify its schema source")
    if "Do not treat this projection as authoritative" not in bindings:
        raise AssertionError(f"{TS_BINDINGS_PATH} must mark the projection as non-authoritative")
    for exported_name in REQUIRED_TS_EXPORTS:
        if f"interface {exported_name}" not in bindings:
            raise AssertionError(f"{TS_BINDINGS_PATH} is missing interface {exported_name}")
    for required_snippet in [
        "export type NonEmptyArray<T> = [T, ...T[]];",
        "role_bindings: NonEmptyArray<RoleBinding>;",
        "signature_refs: NonEmptyArray<OverridRef>;",
        "policy_refs: NonEmptyArray<PolicyRef>;",
        "audit_refs: NonEmptyArray<AuditRef>;",
    ]:
        if required_snippet not in bindings:
            raise AssertionError(f"{TS_BINDINGS_PATH} is missing expected projection detail: {required_snippet}")


def validate_docs() -> None:
    readme = read_text(README_PATH)
    phase_plan = read_text(PHASE_PLAN)
    phase_progress = read_text(PHASE_PROGRESS)
    sub_plan = read_text(SUB_PLAN)
    tech_stack = read_text(TECH_STACK)

    required_text = {
        README_PATH: [
            "Phase 2 Admin and Developer UI contract artifacts",
            "canonical source",
            "resource summary variant matrix",
        ],
        PHASE_PLAN: [
            "Complete the Phase 2 shared-schema contract work",
            "packages/schemas/admin_ui/v0",
            "scripts/validate_admin_ui_phase2.py",
        ],
        PHASE_PROGRESS: [
            "Phase 2 scope",
            "Validation Evidence",
        ],
        SUB_PLAN: [
            "## Phase 2: Shared Admin Schemas And Generated Client Contracts",
            "admin_session_context",
            "resource_summary_view",
            "job_timeline_view",
            "admin_action_request",
            "ui_diagnostic_event",
        ],
        TECH_STACK: [
            "Canonical JSON plus JSON Schema",
            "TypeScript web UI is acceptable",
        ],
    }

    content_by_path = {
        README_PATH: readme,
        PHASE_PLAN: phase_plan,
        PHASE_PROGRESS: phase_progress,
        SUB_PLAN: sub_plan,
        TECH_STACK: tech_stack,
    }
    for path, snippets in required_text.items():
        content = content_by_path[path]
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
        RESOURCE_SUMMARY_VARIANTS_FIXTURE,
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
    print("Admin UI Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
