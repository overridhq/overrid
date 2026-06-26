#!/usr/bin/env python3
"""Validate Overkey Phase 7 secret refs, protection classes, and namespace bindings."""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
README = Path("packages/overkey/README.md")
ROUTES = Path("packages/overkey/src/routes.rs")
RECORDS = Path("packages/overkey/src/records.rs")
SCHEMA_RS = Path("packages/overkey/src/schema.rs")
SCHEMA_JSON = Path("packages/schemas/overrid_contracts/v0/overkey_credential.schema.json")
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase7_secret_protection_namespace.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase7_secret_protection_denials.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_07_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_07_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
    "bearer_token=",
)

PHASE7_CREDENTIAL_FIELDS = {
    "environment_scope",
    "endpoint_scope",
    "test_credential",
    "production_bound",
    "protection_evidence_refs",
    "namespace_binding",
    "protected_dependency_states",
    "blocked_state",
    "recovery_hints",
}

PHASE7_SECRET_REF_FIELDS = {
    "secret_class",
    "resolver_service",
    "rotation_policy",
    "allowed_resolver_services",
    "access_audit_refs",
    "dependency_state",
}

PHASE7_DENIAL_REASON_CODES = {
    "overkey.phase7_secret_ref_required",
    "overkey.phase7_secret_resolver_not_allowed",
    "overkey.phase7_test_credential_production_denied",
    "overkey.phase7_protection_class_evidence_required",
    "overkey.phase7_namespace_binding_evidence_required",
    "overkey.phase7_dependency_blocked",
    "auth.phase7_protected_dependency_blocked",
    "auth.phase7_fresh_lookup_required",
}

PHASE7_RUST_TESTS = {
    "phase7_secret_refs_store_overvault_metadata_without_raw_material",
    "phase7_production_protection_and_test_credentials_fail_closed",
    "phase7_namespace_binding_requires_policy_owner_storage_evidence",
    "phase7_protected_dependency_fail_closed_and_fresh_lookup_rules",
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
    readme = read(README)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    suite = read(SUITE_VALIDATOR)

    assert_contains(workspace, '"python3 scripts/validate_overkey_phase7.py"', WORKSPACE_LAYOUT)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase7.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 7: Secret References, Protection Classes, And Phase 8 Expansion",
        SUB_PLAN,
    )
    for expected in (
        "Rust-first infrastructure stack",
        "Axum/Tower/Hyper-style Rust HTTP services",
        "Canonical JSON plus JSON Schema",
        "Signed command envelopes",
        "Ed25519 signatures",
        "BLAKE3/content hashes",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in (
        "Complete SUB BUILD PLAN #9 Phase 7",
        "Overvault secret refs",
        "production protection classes",
        "local/test credential isolation",
        "namespace-aware native-app bindings",
        "blocked-state recovery",
        "scripts/validate_overkey_phase7.py",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    for expected in (
        "Overkey Phase 7 Progress",
        "Docdex impact",
        "scripts/validate_overkey_phase7.py",
        "cargo test -p overrid-overkey phase7",
    ):
        assert_contains(phase_progress, expected, PHASE_PROGRESS)
    for expected in (
        "Phase 7 Secret References, Protection Classes, And Phase 8 Expansion",
        "overkey.phase7.response.v0",
        "fixtures/valid/phase7_secret_protection_namespace.valid.json",
        "fixtures/invalid/phase7_secret_protection_denials.invalid.json",
        "python3 scripts/validate_overkey_phase7.py",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    records = read(RECORDS)
    schema_rs = read(SCHEMA_RS)

    for expected in (
        "OVERKEY_PHASE7_RESPONSE_SCHEMA_VERSION",
        "OVERKEY_PHASE7_FIXTURE_SCHEMA_VERSION",
        "overkey.phase7.secret_protection_namespace.v0",
    ):
        assert_contains(schema_rs, expected, SCHEMA_RS)

    for expected in (
        "Phase7CredentialControls",
        "SecretRefRequest",
        "phase7_controls_for_enrollment",
        "secret_ref_for_phase7",
        "phase7_secret_ref_is_local_only",
        "strong_phase7_protection_class",
        "validate_phase7_namespace_binding",
        "protected_dependency_states_for_phase7",
        "phase7_verification_dependency_denial",
        "phase7_response_schema_for",
        "phase7_response_reason_for",
        "overkey.phase7_secret_ref_required",
        "overkey.phase7_secret_resolver_not_allowed",
        "overkey.phase7_test_credential_production_denied",
        "overkey.phase7_protection_class_evidence_required",
        "overkey.phase7_namespace_binding_evidence_required",
        "overkey.phase7_dependency_blocked",
        "auth.phase7_protected_dependency_blocked",
        "auth.phase7_fresh_lookup_required",
        "overkey.phase7_credential_controls_recorded",
        "overasset_speculative_asset_created: false",
        "cache_invalidation_state",
        "fresh_overkey_lookup",
        "phase7_production_signing_key_body",
    ):
        assert_contains(routes, expected, ROUTES)

    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    missing_tests = sorted(PHASE7_RUST_TESTS - tests)
    if missing_tests:
        raise AssertionError(f"{ROUTES} missing required Phase 7 tests: {missing_tests}")

    for expected in (
        "pub struct SecretRef",
        "secret_class: String",
        "resolver_service: String",
        "rotation_policy: String",
        "allowed_resolver_services: Vec<String>",
        "access_audit_refs: Vec<String>",
        "dependency_state: String",
        "pub struct NamespaceCredentialBinding",
        "pub struct ProtectedDependencyState",
        "pub struct Phase7CredentialControls",
        "environment_scope: String",
        "endpoint_scope: String",
        "test_credential: bool",
        "production_bound: bool",
        "protection_evidence_refs: Vec<String>",
        "namespace_binding: Option<NamespaceCredentialBinding>",
        "protected_dependency_states: Vec<ProtectedDependencyState>",
        "blocked_state: Option<String>",
        "recovery_hints: Vec<String>",
    ):
        assert_contains(records, expected, RECORDS)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    defs = schema.get("$defs", {})
    credential_required = set(defs.get("credential_record", {}).get("required", []))
    missing_credential_fields = sorted(PHASE7_CREDENTIAL_FIELDS - credential_required)
    if missing_credential_fields:
        raise AssertionError(
            f"{SCHEMA_JSON} credential_record missing Phase 7 required fields: {missing_credential_fields}"
        )

    secret_required = set(defs.get("secret_ref", {}).get("required", []))
    missing_secret_fields = sorted(PHASE7_SECRET_REF_FIELDS - secret_required)
    if missing_secret_fields:
        raise AssertionError(
            f"{SCHEMA_JSON} secret_ref missing Phase 7 required fields: {missing_secret_fields}"
        )

    if "namespace_credential_binding" not in defs:
        raise AssertionError(f"{SCHEMA_JSON} missing namespace_credential_binding")
    if "protected_dependency_state" not in defs:
        raise AssertionError(f"{SCHEMA_JSON} missing protected_dependency_state")

    if valid.get("schema_version") != "overkey.phase7.secret_protection_namespace.v0":
        raise AssertionError("valid Phase 7 fixture has wrong schema_version")
    credential = valid["credential_controls"]["credential_record"]
    if not PHASE7_CREDENTIAL_FIELDS <= set(credential):
        raise AssertionError("valid Phase 7 credential fixture omits required fields")
    if not credential["secret_ref"]["reference"].startswith("secret://"):
        raise AssertionError("Phase 7 secret ref must be a secret:// reference")
    if credential["secret_ref"]["resolver_service"] != "service:overvault":
        raise AssertionError("Phase 7 secret refs must resolve through Overvault")
    if credential["test_credential"] is not False or credential["production_bound"] is not True:
        raise AssertionError("Phase 7 production fixture must not be a test credential")
    if not credential["protection_evidence_refs"]:
        raise AssertionError("Phase 7 production fixture must include protection evidence")
    namespace = credential["namespace_binding"]
    if not namespace["route_refs"] or not namespace["storage_entitlement_refs"]:
        raise AssertionError("Phase 7 namespace fixture must include route and storage evidence")
    controls = valid["credential_controls"]["phase7_controls"]
    for key in (
        "overvault_metadata_only",
        "production_protection_evidence_required",
        "local_test_credentials_loopback_only",
        "namespace_binding_requires_policy_owner_storage_evidence",
        "protected_dependencies_fail_closed",
        "ordinary_verification_requires_fresh_lookup_when_cache_invalidation_unavailable",
    ):
        if controls.get(key) is not True:
            raise AssertionError(f"Phase 7 fixture control flag not true: {key}")
    if controls.get("raw_secret_persisted") is not False:
        raise AssertionError("Phase 7 fixtures must not persist raw secrets")
    if controls.get("overasset_speculative_asset_created") is not False:
        raise AssertionError("Phase 7 fixtures must not create speculative Overasset behavior")

    if invalid.get("schema_version") != "overkey.phase7.secret_protection_denials.v0":
        raise AssertionError("invalid Phase 7 fixture has wrong schema_version")
    reason_codes = {entry["reason_code"] for entry in invalid.get("denials", [])}
    if reason_codes != PHASE7_DENIAL_REASON_CODES:
        raise AssertionError(f"invalid Phase 7 denial reason drift: {sorted(reason_codes)}")
    if any(entry.get("side_effects_allowed") for entry in invalid["denials"]):
        raise AssertionError("Phase 7 denial fixtures must prevent side effects")
    if invalid.get("raw_secret_persisted") is not False:
        raise AssertionError("Phase 7 denial fixture must not persist raw secrets")
    if invalid.get("overasset_speculative_asset_created") is not False:
        raise AssertionError("Phase 7 denial fixture must not create speculative Overasset behavior")

    combined = json.dumps({"valid": valid, "invalid": invalid}, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        if marker in combined:
            raise AssertionError(f"Phase 7 fixtures contain raw secret marker: {marker}")


def run_checks() -> None:
    result = subprocess.run(
        ["cargo", "test", "-p", "overrid-overkey", "phase7"],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        raise AssertionError("cargo test -p overrid-overkey phase7 failed")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_schema_and_fixtures()
        run_checks()
    except AssertionError as error:
        print(f"Overkey Phase 7 validation failed: {error}", file=sys.stderr)
        return 1
    print("Overkey Phase 7 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
