#!/usr/bin/env python3
"""Validate Overkey Phase 3 enrollment, metadata, lifecycle, and fixtures."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
CRATE_TOML = Path("packages/overkey/Cargo.toml")
README = Path("packages/overkey/README.md")
ROUTES = Path("packages/overkey/src/routes.rs")
REPOSITORY = Path("packages/overkey/src/repository.rs")
RECORDS = Path("packages/overkey/src/records.rs")
SCHEMA_RS = Path("packages/overkey/src/schema.rs")
ERRORS = Path("packages/overkey/src/errors.rs")
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase3_enrollment_metadata.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase3_raw_key_diagnostic.invalid.json")
SCHEMA_JSON = Path("packages/schemas/overrid_contracts/v0/overkey_credential.schema.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_03_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
)

PHASE3_REQUIRED_RECORD_FIELDS = {
    "key_id",
    "key_version",
    "created_at",
    "last_used_at",
    "rotation_refs",
    "revocation_refs",
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
    crate = read(CRATE_TOML)
    readme = read(README)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)

    assert_contains(workspace, '"python3 scripts/validate_overkey_phase3.py"', WORKSPACE_LAYOUT)
    assert_contains(crate, 'blake3 = "1.5"', CRATE_TOML)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase3.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 3: Credential Enrollment, Lifecycle APIs, And Metadata Reads",
        SUB_PLAN,
    )
    for expected in (
        "API key hash enrollment",
        "Ed25519 public key enrollment",
        "service-account credential records",
        "caller-visible redacted metadata reads",
        "append-only lifecycle transitions",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Overkey Phase 3 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Validation Evidence", PHASE_PROGRESS)
    assert_contains(tech_stack, "Rust-first infrastructure stack", TECH_STACK)
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)

    for expected in (
        "Phase 2 and Phase 3",
        "BLAKE3 hash refs",
        "Duplicate active tenant-scoped key ids",
        "API key enrollment stores non-secret prefixes",
        "Signing public key enrollment requires Ed25519",
        "Service-account credential enrollment requires signed internal-service headers",
        "Metadata reads are tenant-scoped",
        "Lifecycle transitions are append-only",
        "python3 scripts/validate_overkey_phase3.py",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    repository = read(REPOSITORY)
    records = read(RECORDS)
    schema_rs = read(SCHEMA_RS)
    errors = read(ERRORS)

    for expected in (
        "OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION",
        "SUPPORTED_RESPONSE_SCHEMA_VERSIONS",
        "API_KEY_LOOKUP_HASH_ALGORITHM",
        "BLAKE3-keyed-lookup",
        "API_KEY_HASH_REF_PREFIX",
        "API_KEY_LOOKUP_HASH_CONTEXT",
        "API_KEY_LOOKUP_KEY_REF",
        "blake3::keyed_hash",
        "SIGNING_ALGORITHM",
        "Ed25519",
        "SERVICE_ACCOUNT_ALLOWED_SERVICES",
        "SERVICE_ACCOUNT_ALLOWED_COMMAND_CLASSES",
        "ApiKeyEnrollmentRequest",
        "SigningKeyEnrollmentRequest",
        "ServiceAccountCredentialRequest",
        "LifecycleRequest",
        "LastUsedRequest",
        "api_key_hash_ref",
        "raw_key_discarded",
        "validate_signing_key_request",
        "narrow_service_account_scope",
        "allowed_service_account_scope",
        "safe_metadata_redactions",
        "redacted_fields",
        "tenant_isolated",
        "last_used_at",
        "ROTATION_RECORD_SCHEMA_REF",
        "REVOCATION_RECORD_SCHEMA_REF",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "DuplicateActiveKey",
        "CredentialNotFound",
        "InvalidStatusTransition",
        "BroadServiceAccountScope",
        "UnsignedServiceAccountCall",
        "update_last_used",
        "valid_status_transition",
        "credential.rotation_refs.push",
        "credential.revocation_refs.push",
    ):
        assert_contains(repository, expected, REPOSITORY)

    for expected in (
        "key_id: String",
        "key_version: u32",
        "created_at: String",
        "last_used_at: Option<String>",
        "rotation_refs: Vec<String>",
        "revocation_refs: Vec<String>",
        "raw_key_discarded: bool",
        "allowed_services: Vec<String>",
        "allowed_command_classes: Vec<String>",
        "signed_call_required: bool",
    ):
        assert_contains(records, expected, RECORDS)

    for expected in (
        "OVERKEY_PHASE3_FIXTURE_SCHEMA_VERSION",
        "overkey.phase3.credential_enrollment.v0",
        "OVERKEY_PHASE3_RESPONSE_SCHEMA_VERSION",
        "overkey.phase3.response.v0",
        "raw_api_key=",
    ):
        assert_contains(schema_rs, expected, SCHEMA_RS)

    for expected in (
        "overkey.duplicate_active_key_rejected",
        "overkey.credential_not_found",
        "overkey.invalid_lifecycle_transition",
        "overkey.broad_service_account_scope_rejected",
        "overkey.service_account_signature_required",
        "invalid_enrollment",
    ):
        assert_contains(errors, expected, ERRORS)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    defs = schema.get("$defs", {})
    if "credential_record" not in defs:
        raise AssertionError(f"{SCHEMA_JSON} missing credential_record $defs entry")
    credential_required = set(defs["credential_record"].get("required", []))
    missing = sorted(PHASE3_REQUIRED_RECORD_FIELDS - credential_required)
    if missing:
        raise AssertionError(f"{SCHEMA_JSON} credential_record missing Phase 3 fields: {missing}")

    for object_name, fields in {
        "api_key_record": {"key_version", "created_at", "last_used_at", "lookup_hint_rules", "raw_key_discarded"},
        "public_key_record": {"key_version", "key_fingerprint_ref", "not_before", "not_after", "protection_class"},
        "service_account_key": {"key_version", "allowed_services", "allowed_command_classes", "protection_class", "signed_call_required"},
    }.items():
        required = set(defs.get(object_name, {}).get("required", []))
        missing = sorted(fields - required)
        if missing:
            raise AssertionError(f"{SCHEMA_JSON} {object_name} missing Phase 3 fields: {missing}")

    api_key_hash_pattern = (
        defs.get("api_key_record", {})
        .get("properties", {})
        .get("api_key_hash_ref", {})
        .get("pattern")
    )
    if api_key_hash_pattern != "^hash:api_key:blake3:":
        raise AssertionError(f"{SCHEMA_JSON} api_key_hash_ref must require typed BLAKE3 refs")

    if valid["schema_version"] != "overkey.phase3.credential_enrollment.v0":
        raise AssertionError("valid Phase 3 fixture has wrong schema_version")
    if valid["service_id"] != "service:overkey" or valid["base_path"] != "/overkey":
        raise AssertionError("valid Phase 3 fixture must describe Overkey at /overkey")

    api_key = valid["api_key_enrollment"]
    if api_key["api_key_prefix"].startswith("ovk_") is False:
        raise AssertionError("API key fixture must use a non-secret ovk_ prefix")
    if api_key["api_key_hash_ref"].startswith("hash:api_key:blake3:") is False:
        raise AssertionError("API key fixture must use a typed BLAKE3 API key hash ref")
    if api_key["raw_key_discarded"] is not True or api_key["raw_secret_persisted"] is not False:
        raise AssertionError("API key fixture must prove raw key discard and no raw persistence")

    signing_key = valid["signing_key_enrollment"]
    if signing_key["algorithm"] != "Ed25519":
        raise AssertionError("signing key fixture must use Ed25519")
    if signing_key["canonicalization"] != "overrid.canonical_json.v0":
        raise AssertionError("signing key fixture must use overrid.canonical_json.v0")
    if not signing_key["not_after"] or not signing_key["protection_class"]:
        raise AssertionError("signing key fixture must include expiry and protection class")

    service_account = valid["service_account_credential"]
    if service_account["signed_call_required"] is not True:
        raise AssertionError("service-account fixture must require signed calls")
    if service_account["allowed_services"] == ["*"] or service_account["allowed_command_classes"] == ["*"]:
        raise AssertionError("service-account fixture must not use wildcard scope")
    expected_services = {
        "service:overgate",
        "service:node-agent",
        "service:system-service",
        "service:worker",
        "service:overvault",
        "service:grid-resident",
    }
    expected_command_classes = {
        "command.verify",
        "command.credential.read",
        "command.credential.rotate",
        "command.credential.revoke",
        "command.node.enroll",
        "command.secret.resolve",
        "command.workload.execute",
        "command.system.operate",
    }
    if set(service_account["allowed_service_matrix"]) != expected_services:
        raise AssertionError("service-account fixture must define the Phase 3 allowed-service matrix")
    if set(service_account["allowed_command_class_matrix"]) != expected_command_classes:
        raise AssertionError("service-account fixture must define the Phase 3 command-class matrix")
    if not set(service_account["allowed_services"]).issubset(expected_services):
        raise AssertionError("service-account fixture uses service outside the Phase 3 matrix")
    if not set(service_account["allowed_command_classes"]).issubset(expected_command_classes):
        raise AssertionError("service-account fixture uses command class outside the Phase 3 matrix")
    if service_account["broad_scope_rejected"] is not True:
        raise AssertionError("service-account fixture must prove broad scope rejection")

    metadata_read = valid["metadata_read"]
    if metadata_read["tenant_isolated"] is not True:
        raise AssertionError("metadata read fixture must prove tenant isolation")
    for field in ("raw_api_key", "api_key_lookup_hash_internal", "private_key_material"):
        if field not in metadata_read["redacted_fields"]:
            raise AssertionError(f"metadata read fixture missing redaction field: {field}")

    lifecycle = valid["lifecycle"]
    expected_statuses = {"pending", "active", "rotating", "suspended", "revoked", "expired", "tombstoned"}
    if set(lifecycle["statuses"]) != expected_statuses:
        raise AssertionError("lifecycle fixture must list all Phase 3 statuses")
    if lifecycle["append_only_history"] is not True:
        raise AssertionError("lifecycle fixture must require append-only history")
    for transition in ("active->rotating", "active->revoked", "rotating->active", "revoked->tombstoned"):
        if transition not in lifecycle["valid_transitions"]:
            raise AssertionError(f"lifecycle fixture missing valid transition: {transition}")
    for transition in ("revoked->active", "tombstoned->active"):
        if transition not in lifecycle["invalid_transitions"]:
            raise AssertionError(f"lifecycle fixture missing invalid transition: {transition}")

    denial = invalid["expected_denial"]
    if denial["status"] != 400 or denial["reason_code"] != "overkey.raw_secret_material_rejected":
        raise AssertionError("invalid Phase 3 fixture must prove raw key material denial")
    if invalid["credential_record"]["direct_secret_material_present"] is not True:
        raise AssertionError("invalid Phase 3 fixture must mark direct secret material as present")


def validate_tests_exist() -> None:
    routes = read(ROUTES)
    repository = read(REPOSITORY)
    route_tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_route_tests = {
        "signing_key_enrollment_rejects_duplicate_active_key_ids",
        "service_account_enrollment_requires_signed_narrow_scope",
        "metadata_reads_are_tenant_scoped_and_redacted",
        "last_used_updates_safe_metadata_summary",
    }
    missing = sorted(required_route_tests - route_tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing expected Phase 3 route tests: {', '.join(missing)}")

    for test_name in (
        "rejects_duplicate_active_key_ids_inside_tenant_scope",
        "rejects_invalid_lifecycle_resurrection",
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
        validate_docs_and_wiring,
        validate_rust_sources,
        validate_schema_and_fixtures,
        validate_tests_exist,
        validate_secret_safety,
    ]
    for check in checks:
        try:
            check()
        except AssertionError as exc:
            print(f"Overkey Phase 3 validation failed: {exc}", file=sys.stderr)
            return 1
    print("Overkey Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
