#!/usr/bin/env python3
"""Validate Overkey Phase 2 service skeleton, schemas, routes, and fixtures."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE = Path("Cargo.toml")
WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
CRATE_TOML = Path("packages/overkey/Cargo.toml")
README = Path("packages/overkey/README.md")
LIB = Path("packages/overkey/src/lib.rs")
MAIN = Path("packages/overkey/src/main.rs")
SERVICE = Path("packages/overkey/src/service.rs")
ROUTES = Path("packages/overkey/src/routes.rs")
REPOSITORY = Path("packages/overkey/src/repository.rs")
RECORDS = Path("packages/overkey/src/records.rs")
SCHEMA_RS = Path("packages/overkey/src/schema.rs")
DEPENDENCIES = Path("packages/overkey/src/dependencies.rs")
ERRORS = Path("packages/overkey/src/errors.rs")
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase2_local_credential.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase2_raw_secret.invalid.json")
SCHEMA_JSON = Path("packages/schemas/overrid_contracts/v0/overkey_credential.schema.json")
HARNESS_LOCAL_STACK = Path("packages/integration_harness/src/local_stack.rs")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_02_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_02_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_ROUTES = [
    "POST /v1/credentials/api-keys",
    "POST /v1/credentials/signing-keys",
    "POST /v1/credentials/service-accounts",
    "POST /v1/credentials/{credential_id}/rotate",
    "POST /v1/credentials/{credential_id}/revoke",
    "GET /v1/credentials/{credential_id}",
    "POST /v1/verify/signature",
    "POST /v1/verify/api-key",
    "POST /v1/usage/last-used",
    "GET /v1/healthz",
    "GET /v1/readyz",
]

REQUIRED_AXUM_PATHS = [
    '/v1/credentials/api-keys"',
    '/v1/credentials/signing-keys"',
    '/v1/credentials/service-accounts"',
    '/v1/credentials/:credential_id/rotate"',
    '/v1/credentials/:credential_id/revoke"',
    '/v1/credentials/:credential_id"',
    '/v1/verify/signature"',
    '/v1/verify/api-key"',
    '/v1/usage/last-used"',
    '/v1/healthz"',
    '/v1/readyz"',
]

REQUIRED_SCHEMA_OBJECTS = [
    "credential_record",
    "api_key_record",
    "public_key_record",
    "service_account_key",
    "delegation_record",
    "rotation_record",
    "revocation_record",
    "verification_result",
    "secret_ref",
    "api_error",
    "overwatch_event",
]

REQUIRED_SCHEMA_REFS = [
    "schema:overkey:credential_record:v0",
    "schema:overkey:api_key_record:v0",
    "schema:overkey:public_key_record:v0",
    "schema:overkey:service_account_key:v0",
    "schema:overkey:delegation_record:v0",
    "schema:overkey:rotation_record:v0",
    "schema:overkey:revocation_record:v0",
    "schema:overkey:verification_result:v0",
    "schema:overkey:secret_ref:v0",
    "schema:overkey:api_error:v0",
    "schema:overkey:overwatch_event:v0",
]

REQUIRED_DEPENDENCIES = [
    "schema_validation",
    "credential_metadata_repository",
    "overgate_callback",
    "overpass_callback",
    "overtenant_callback",
    "overwatch_event_sink",
    "overvault_secret_ref_resolver",
]

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
)


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


def validate_workspace_and_crate() -> None:
    workspace = read(WORKSPACE)
    workspace_layout = read(WORKSPACE_LAYOUT)
    crate = read(CRATE_TOML)
    lib = read(LIB)
    main = read(MAIN)
    service = read(SERVICE)

    assert_contains(workspace, '"packages/overkey"', WORKSPACE)
    for expected in (
        'name = "overkey"',
        'type = "rust_service_crate"',
        'owner_layer = "runtime_service_contracts"',
        'path = "packages/overkey"',
        'cargo_member = "packages/overkey"',
        'cargo_package = "overrid-overkey"',
        '"cargo test -p overrid-overkey"',
        '"python3 scripts/validate_overkey_phase2.py"',
        'local_stack_participation = "profile_consumer"',
    ):
        assert_contains(workspace_layout, expected, WORKSPACE_LAYOUT)

    assert_contains(crate, 'name = "overrid-overkey"', CRATE_TOML)
    for dependency in ("axum", "tokio", "serde", "serde_json", "tracing", "tracing-subscriber"):
        assert_contains(crate, dependency, CRATE_TOML)
    assert_contains(crate, 'name = "overkey"', CRATE_TOML)
    assert_contains(lib, "#![forbid(unsafe_code)]", LIB)
    for module in ("dependencies", "errors", "records", "repository", "routes", "schema", "service"):
        assert_contains(lib, f"pub mod {module};", LIB)
    assert_contains(main, "OverkeyConfig::from_env()", MAIN)
    assert_contains(main, "tokio::net::TcpListener::bind", MAIN)
    assert_contains(main, "axum::serve", MAIN)
    assert_contains(main, "tracing_subscriber::fmt()", MAIN)
    assert_contains(service, 'service_id: "service:overkey"', SERVICE)
    assert_contains(service, 'bind_addr: "127.0.0.1:18080"', SERVICE)
    assert_contains(service, 'public_base_path: "/overkey"', SERVICE)
    assert_contains(service, "nest(&base_path, route_tree())", SERVICE)


def validate_routes_repository_and_dependencies() -> None:
    routes = read(ROUTES)
    repository = read(REPOSITORY)
    records = read(RECORDS)
    schema_rs = read(SCHEMA_RS)
    dependencies = read(DEPENDENCIES)
    errors = read(ERRORS)

    for route in REQUIRED_ROUTES:
        assert_contains(routes, route, ROUTES)
        assert_contains(read(README), route, README)
    for axum_path in REQUIRED_AXUM_PATHS:
        assert_contains(routes, axum_path, ROUTES)

    for expected in (
        "x-overrid-trace-id",
        "x-overrid-tenant-id",
        "x-overrid-service-account",
        "x-overrid-service-signature",
        "require_json",
        "tenant_from_headers",
        "require_internal_service_account",
        "auth.service_account_required",
        "overkey.phase2.response.v0",
        'uri("/overkey/v1/healthz")',
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "pub trait CredentialMetadataRepository",
        "append_credential",
        "append_status_transition",
        "record_verification",
        "InMemoryCredentialRepository",
        "contains_raw_secret_marker",
        "RepositoryError::RawSecretMaterial",
    ):
        assert_contains(repository, expected, REPOSITORY)

    for record_type in (
        "CredentialRecord",
        "ApiKeyRecord",
        "PublicKeyRecord",
        "ServiceAccountKey",
        "DelegationRecord",
        "RotationRecord",
        "RevocationRecord",
        "VerificationResult",
        "OverwatchEvent",
        "SecretRef",
    ):
        assert_contains(records, f"struct {record_type}", RECORDS)
    for status in ("Pending", "Active", "Rotating", "Suspended", "Revoked", "Expired", "Tombstoned"):
        assert_contains(records, status, RECORDS)

    for schema_ref in REQUIRED_SCHEMA_REFS:
        assert_contains(schema_rs, schema_ref, SCHEMA_RS)
    for schema_constant in (
        "CREDENTIAL_RECORD_SCHEMA_REF",
        "API_KEY_RECORD_SCHEMA_REF",
        "PUBLIC_KEY_RECORD_SCHEMA_REF",
        "SERVICE_ACCOUNT_KEY_SCHEMA_REF",
        "VERIFICATION_RESULT_SCHEMA_REF",
    ):
        assert_contains(routes, schema_constant, ROUTES)
    for schema_object in REQUIRED_SCHEMA_OBJECTS:
        assert_contains(schema_rs, schema_object, SCHEMA_RS)

    for dependency in REQUIRED_DEPENDENCIES:
        assert_contains(dependencies, dependency, DEPENDENCIES)
    assert_contains(dependencies, "DependencyRequirement::Required", DEPENDENCIES)
    assert_contains(dependencies, "DependencyRequirement::Optional", DEPENDENCIES)
    assert_contains(errors, "overkey.raw_secret_material_rejected", ERRORS)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        raise AssertionError(f"{SCHEMA_JSON} must use JSON Schema draft 2020-12")
    defs = schema.get("$defs", {})
    for schema_object in REQUIRED_SCHEMA_OBJECTS + ["credential_status"]:
        if schema_object not in defs:
            raise AssertionError(f"{SCHEMA_JSON} missing $defs entry: {schema_object}")

    credential_required = set(defs["credential_record"]["required"])
    required_fields = {
        "tenant_id",
        "subject_ref",
        "credential_id",
        "credential_class",
        "allowed_uses",
        "status",
        "not_after",
        "algorithm",
        "canonicalization",
        "audit_refs",
        "protection_class",
        "secret_ref",
    }
    if not required_fields.issubset(credential_required):
        missing = sorted(required_fields - credential_required)
        raise AssertionError(f"{SCHEMA_JSON} credential_record missing required fields: {missing}")
    assert_contains(json.dumps(schema), "Ed25519", SCHEMA_JSON)
    assert_contains(json.dumps(schema), "BLAKE3-keyed-lookup", SCHEMA_JSON)
    assert_contains(json.dumps(schema), "^secret://", SCHEMA_JSON)
    assert_contains(json.dumps(schema), "^hash:blake3:", SCHEMA_JSON)

    local_stack_service = valid["local_stack_service"]
    if local_stack_service["service_id"] != "service:overkey":
        raise AssertionError("valid fixture must reference service:overkey")
    if local_stack_service["port_owner_service_id"] != "service:api":
        raise AssertionError("valid fixture must preserve service:api as the port 18080 owner")
    if local_stack_service["bind_host"] != "127.0.0.1" or local_stack_service["port"] != 18080:
        raise AssertionError("valid fixture must keep Overkey on loopback port 18080")
    if local_stack_service["base_path"] != "/overkey":
        raise AssertionError("valid fixture must use /overkey base path")
    if not local_stack_service["local_only"] or not local_stack_service["test_only"]:
        raise AssertionError("valid fixture must be local/test scoped")
    if valid["credential_record"]["secret_ref"]["reference"].startswith("secret://") is False:
        raise AssertionError("valid fixture must use Overvault secret refs instead of secret material")
    if valid["credential_record"]["algorithm"] != "Ed25519":
        raise AssertionError("valid fixture must include Ed25519 credential metadata")
    if valid["harness_scenario_ref"] != "scenario:overkey_phase2_credential_smoke":
        raise AssertionError("valid fixture must expose the phase 2 Overkey smoke scenario ref")
    if valid["direct_secret_material_present"] is not False:
        raise AssertionError("valid fixture must explicitly deny direct secret material")

    for schema_ref in REQUIRED_SCHEMA_REFS:
        if schema_ref not in valid["schema_refs"]:
            raise AssertionError(f"valid fixture missing schema ref: {schema_ref}")
    for dependency in REQUIRED_DEPENDENCIES:
        if dependency not in valid["dependency_refs"]:
            raise AssertionError(f"valid fixture missing dependency ref: {dependency}")
    for fixture_ref in (
        "ed25519_public_key_fixture_ref",
        "blake3_body_hash_ref",
        "api_key_hash_fixture_ref",
        "revocation_fixture_ref",
        "secret_ref_stub",
    ):
        if fixture_ref not in valid:
            raise AssertionError(f"valid fixture missing deterministic fixture ref: {fixture_ref}")

    denial = invalid["expected_denial"]
    if denial["status"] != 400 or denial["reason_code"] != "overkey.raw_secret_material_rejected":
        raise AssertionError("invalid fixture must prove raw secret material denial")
    if invalid["credential_record"]["direct_secret_material_present"] is not True:
        raise AssertionError("invalid fixture must mark direct secret material as present")


def validate_docs_and_suite_wiring() -> None:
    readme = read(README)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)
    harness_local_stack = read(HARNESS_LOCAL_STACK)

    assert_contains(sub_plan, "## Phase 2: Rust Service Skeleton, Schemas, And Record Model", SUB_PLAN)
    assert_contains(tech_stack, "Axum/Tower/Hyper-style Rust HTTP services", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #9 Phase 2", PHASE_PLAN)
    assert_contains(phase_progress, "Overkey Phase 2 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase2.py")', SUITE_VALIDATOR)
    assert_contains(harness_local_stack, '("service:overkey", true)', HARNESS_LOCAL_STACK)

    for expected in (
        "Rust-first service crate",
        "cargo run -p overrid-overkey --bin overkey",
        "service:overkey",
        "service:api",
        "/overkey",
        "CredentialMetadataRepository",
        "append-friendly in-memory stub",
        "never direct key material",
        "service-account headers",
        "packages/schemas/overrid_contracts/v0/overkey_credential.schema.json",
    ):
        assert_contains(readme, expected, README)


def validate_tests_exist() -> None:
    routes = read(ROUTES)
    repository = read(REPOSITORY)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_route_tests = {
        "public_routes_register_and_preserve_trace_json",
        "base_path_routes_to_same_surface",
        "credential_routes_require_json_and_tenant_context",
        "verification_routes_require_internal_service_account_headers",
        "readyz_reports_dependency_matrix",
        "local_fixture_credential_smoke_uses_overkey_base_path",
    }
    missing = sorted(required_route_tests - tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing expected route tests: {', '.join(missing)}")

    for test_name in (
        "appends_lifecycle_history_without_overwriting_record_identity",
        "rejects_raw_private_or_api_key_material",
    ):
        assert_contains(repository, f"fn {test_name}", REPOSITORY)


def validate_secret_safety() -> None:
    for path in (README, VALID_FIXTURE, PHASE_PLAN, PHASE_PROGRESS):
        text = read(path).lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in text:
                raise AssertionError(f"{path} contains raw secret marker: {marker}")


def main() -> int:
    checks = [
        validate_workspace_and_crate,
        validate_routes_repository_and_dependencies,
        validate_schema_and_fixtures,
        validate_docs_and_suite_wiring,
        validate_tests_exist,
        validate_secret_safety,
    ]
    for check in checks:
        try:
            check()
        except AssertionError as exc:
            print(f"Overkey Phase 2 validation failed: {exc}", file=sys.stderr)
            return 1
    print("Overkey Phase 2 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
