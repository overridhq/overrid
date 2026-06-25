#!/usr/bin/env python3
"""Validate Overgate Phase 3 command envelope, canonicalization, schema, and error gates."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

CRATE_TOML = Path("packages/overgate/Cargo.toml")
README = Path("packages/overgate/README.md")
LIB = Path("packages/overgate/src/lib.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
ENVELOPE = Path("packages/overgate/src/envelope.rs")
CANONICAL = Path("packages/overgate/src/canonical.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
RETENTION = Path("packages/overgate/src/retention.rs")
SCHEMA = Path("packages/overgate/src/schema.rs")
VALID_PHASE2_FIXTURE = Path("packages/overgate/fixtures/valid/phase2_local_command.valid.json")
VALID_PHASE3_FIXTURE = Path("packages/overgate/fixtures/valid/phase3_command.valid.json")
INVALID_MISSING_TENANT = Path("packages/overgate/fixtures/invalid/phase3_missing_tenant.invalid.json")
INVALID_UNKNOWN_FIELD = Path(
    "packages/overgate/fixtures/invalid/phase3_unknown_private_payload.invalid.json"
)
INVALID_RAW_SECRET = Path("packages/overgate/fixtures/invalid/phase3_raw_secret.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_03_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_03_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

REQUIRED_ENVELOPE_FIELDS = [
    "command_id",
    "command_type",
    "tenant_id",
    "actor_id",
    "trace_id",
    "idempotency_key",
    "credential_id",
    "schema_version",
    "payload_type",
    "request_hash",
    "payload_hash",
    "timestamp",
    "signature_metadata",
    "privacy_class",
]

RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret_value",
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


def validate_docs_and_wiring() -> None:
    crate = read(CRATE_TOML)
    lib = read(LIB)
    suite = read(SUITE_VALIDATOR)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)

    assert_contains(crate, 'overrid-contracts = { path = "../schemas/overrid_contracts" }', CRATE_TOML)
    for module in ("canonical", "envelope", "errors", "retention", "schema"):
        assert_contains(lib, f"pub mod {module};", LIB)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase2.py")', SUITE_VALIDATOR)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase3.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 3: Command Envelope, Canonicalization, Schema Validation, And Errors",
        SUB_PLAN,
    )
    assert_contains(tech_stack, "Canonical JSON plus JSON Schema", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 3", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 3 Progress", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    envelope = read(ENVELOPE)
    canonical = read(CANONICAL)
    errors = read(ERRORS)
    retention = read(RETENTION)
    schema = read(SCHEMA)

    for expected in (
        "Bytes",
        "CommandEnvelope::parse_http",
        "validate_command_envelope",
        "CanonicalRequestInput::from_envelope",
        "RetentionDecision::from_envelope",
        "ApiErrorData",
        "overgate.phase3.response.v0",
        "overgate.command_validated_phase3",
        "not_forwarded_phase3_validation_only",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "#[serde(deny_unknown_fields)]",
        "MAX_COMMAND_ENVELOPE_BYTES",
        "SUPPORTED_COMMAND_SCHEMA_VERSION",
        "missing_required_field",
        "unknown_sensitive_field",
        "raw_secret_rejected",
        "shared-schema-package.v0.1",
    ):
        assert_contains(envelope, expected, ENVELOPE)

    for expected in (
        "CANONICALIZATION_VERSION",
        "overgate.canonical.v0.1",
        "method",
        "path",
        "tenant_id",
        "actor_id",
        "idempotency_key",
        "payload_hash",
        "timestamp",
        "credential_id",
        "body_hash",
        "canonical_hash",
    ):
        assert_contains(canonical, expected, CANONICAL)

    for expected in (
        "ApiErrorData",
        "retryability",
        "correction_fields",
        "correction_hint",
        "redacted_diagnostic",
        "schema.parse_malformed_payload",
        "schema.unsupported_version",
        "schema.wrong_privacy_class",
    ):
        assert_contains(errors, expected, ERRORS)

    for expected in (
        "overgate.phase3.hashes_and_refs_only",
        "raw_body_not_retained",
        "private_payloads_redacted",
        "raw_secrets_redacted",
        "credential_material_redacted",
    ):
        assert_contains(retention, expected, RETENTION)

    for expected in (
        "SharedSchemaPhase3Contract::canonical",
        "ensure_supported_shared_schema_package_schema_version",
        "overgate.phase3.shared_schema_adapter",
        "payload_ref_and_payload_hash_validated",
        "strict_unknown_field_rejection",
    ):
        assert_contains(schema, expected, SCHEMA)


def validate_fixtures() -> None:
    valid_phase2 = load_json(VALID_PHASE2_FIXTURE)
    valid_phase3 = load_json(VALID_PHASE3_FIXTURE)
    missing_tenant = load_json(INVALID_MISSING_TENANT)
    unknown_field = load_json(INVALID_UNKNOWN_FIELD)
    raw_secret = load_json(INVALID_RAW_SECRET)

    for fixture, path in (
        (valid_phase2, VALID_PHASE2_FIXTURE),
        (valid_phase3, VALID_PHASE3_FIXTURE),
    ):
        envelope = fixture["command_envelope"]
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{path} command_envelope missing required field: {field}")
        if envelope["schema_version"] != "shared-schema-package.v0.1":
            raise AssertionError(f"{path} must use shared-schema-package.v0.1")
        if not envelope["command_type"].startswith("overgate."):
            raise AssertionError(f"{path} must use an Overgate command type")
        if not envelope["request_hash"].startswith("hash:"):
            raise AssertionError(f"{path} must carry request_hash as a hash ref")
        if not envelope["payload_hash"].startswith("hash:"):
            raise AssertionError(f"{path} must carry payload_hash as a hash ref")
        if envelope["privacy_class"] != "tenant_private":
            raise AssertionError(f"{path} must use tenant_private command privacy class")
        if "payload_ref" in envelope and not envelope["payload_ref"].startswith("fixture://"):
            raise AssertionError(f"{path} must use fixture payload refs for local tests")

    if missing_tenant["expected_denial"]["reason_code"] != "schema.missing_required_field":
        raise AssertionError("missing tenant fixture must expect schema.missing_required_field")
    if "tenant_id" in missing_tenant["command_envelope"]:
        raise AssertionError("missing tenant fixture must omit tenant_id")
    if unknown_field["expected_denial"]["reason_code"] != "schema.unknown_sensitive_field":
        raise AssertionError("unknown private payload fixture must expect schema.unknown_sensitive_field")
    if "private_payload" not in unknown_field["command_envelope"]:
        raise AssertionError("unknown private payload fixture must include private_payload")
    if raw_secret["expected_denial"]["reason_code"] != "schema.raw_secret_rejected":
        raise AssertionError("raw secret fixture must expect schema.raw_secret_rejected")
    if raw_secret["command_envelope"]["payload_ref"] != "raw_secret_value":
        raise AssertionError("raw secret fixture must carry the sentinel marker")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "public_routes_register_and_preserve_trace_json",
        "local_base_path_routes_to_same_public_surface",
        "local_fixture_command_smoke_submits_through_overgate_base_path",
        "readyz_separates_liveness_from_dependency_authority",
        "admin_routes_deny_unsigned_non_operator_and_cross_tenant_requests",
        "command_envelope_errors_are_stable_and_redacted",
        "command_envelope_rejects_wrong_content_type_oversized_and_unsupported",
        "canonicalization_is_deterministic_and_input_sensitive",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"Missing route tests: {', '.join(missing)}")

    for expected in (
        "Phase 3 Command Validation",
        "overgate.phase3.shared_schema_adapter",
        "overgate.canonical.v0.1",
        "overgate.phase3.hashes_and_refs_only",
        "reason_code",
        "retryability",
        "correction_fields",
        "redacted diagnostics",
        "fixtures/valid/phase3_command.valid.json",
    ):
        assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE2_FIXTURE, VALID_PHASE3_FIXTURE, PHASE_PLAN, PHASE_PROGRESS):
        text = read(path).lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in text:
                raise AssertionError(f"{path} contains raw secret marker: {marker}")


def main() -> int:
    validate_docs_and_wiring()
    validate_rust_sources()
    validate_fixtures()
    validate_tests_and_readme()
    print("Overgate Phase 3 validation passed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"Overgate Phase 3 validation failed: {error}", file=sys.stderr)
        raise SystemExit(1)
