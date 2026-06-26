#!/usr/bin/env python3
"""Validate Overkey Phase 4 verification helpers, fixtures, and wiring."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

WORKSPACE_LAYOUT = Path("overrid.workspace.toml")
README = Path("packages/overkey/README.md")
ROUTES = Path("packages/overkey/src/routes.rs")
RECORDS = Path("packages/overkey/src/records.rs")
REPOSITORY = Path("packages/overkey/src/repository.rs")
SCHEMA_RS = Path("packages/overkey/src/schema.rs")
SCHEMA_JSON = Path("packages/schemas/overrid_contracts/v0/overkey_credential.schema.json")
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase4_verification.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase4_verification_denials.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_04_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
)

PHASE4_VERIFICATION_FIELDS = {
    "key_id",
    "key_version",
    "subject_ref",
    "allowed_use",
    "command_class",
    "verification_state",
    "reason_code",
    "request_hash_ref",
    "evidence_ref",
    "revocation_epoch",
    "cache_key_ref",
    "retryability",
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

    assert_contains(workspace, '"python3 scripts/validate_overkey_phase4.py"', WORKSPACE_LAYOUT)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase4.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 4: Verification Helpers, Canonicalization, And Overgate Integration",
        SUB_PLAN,
    )
    for expected in (
        "Rust-first infrastructure stack",
        "Ed25519 signatures",
        "BLAKE3/content hashes",
        "Canonical JSON plus JSON Schema",
        "Signed command envelopes",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in (
        "internal signature verification",
        "internal API-key verification",
        "canonicalization/BLAKE3 evidence refs",
        "Overpass/Overtenant dependency checks",
        "Overgate verification contract",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Overkey Phase 4 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact", PHASE_PROGRESS)

    for expected in (
        "Phase 2 through Phase 4",
        "Verification routes are internal-only Phase 4 helpers",
        "overkey.phase4.response.v0",
        "BLAKE3 request/evidence/cache refs",
        "fixtures/valid/phase4_verification.valid.json",
        "fixtures/invalid/phase4_verification_denials.invalid.json",
        "python3 scripts/validate_overkey_phase4.py",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    records = read(RECORDS)
    repository = read(REPOSITORY)
    schema_rs = read(SCHEMA_RS)

    for expected in (
        "OVERKEY_PHASE4_RESPONSE_SCHEMA_VERSION",
        "APPROVED_VERIFICATION_SERVICE_ACCOUNTS",
        "verify_signature_request",
        "verify_api_key_request",
        "verification_data_base",
        "verification_denial_for_common_checks",
        "body_hash_ref_valid",
        "hash:body:blake3:",
        "timestamp: &str",
        "replay_window_id: Option<&str>",
        "verification_result_from_data",
        "json_response_with_schema",
        "auth.signature_verified_phase4",
        "auth.api_key_verified_phase4",
        "auth.command_class_denied",
        "auth.body_hash_mismatch",
        "auth.api_key_hash_mismatch",
        "auth.subject_dependency_denied",
        "request_hash_ref",
        "verification_evidence_ref",
        "cache_guidance",
        "revocation_epoch",
        "raw_secret_persisted",
        "phase4_signature_verification_checks_metadata_and_dependencies",
        "phase4_api_key_verification_never_returns_raw_key_material",
        "phase4_signature_verification_covers_plan_denials",
        "timestamp_changed_request",
        "replay_changed_request",
        "body_mismatch_request",
        "auth.signature_expired",
        "auth.key_version_mismatch",
        "auth.replay_window_required",
        "auth.credential_unknown",
        "auth.credential_not_active",
        "CredentialStatus::Suspended",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "key_id: String",
        "key_version: u32",
        "subject_ref: String",
        "allowed_use: String",
        "command_class: String",
        "verification_state: String",
        "reason_code: String",
        "request_hash_ref: String",
        "evidence_ref: String",
        "revocation_epoch: u64",
        "cache_key_ref: String",
        "retryability: String",
    ):
        assert_contains(records, expected, RECORDS)

    assert_contains(repository, "reason_code: record.reason_code", REPOSITORY)
    assert_contains(schema_rs, "OVERKEY_PHASE4_FIXTURE_SCHEMA_VERSION", SCHEMA_RS)
    assert_contains(schema_rs, "overkey.phase4.response.v0", SCHEMA_RS)
    assert_contains(schema_rs, "overkey.phase4.verification.v0", SCHEMA_RS)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    verification = schema.get("$defs", {}).get("verification_result", {})
    required = set(verification.get("required", []))
    missing = sorted(PHASE4_VERIFICATION_FIELDS - required)
    if missing:
        raise AssertionError(f"{SCHEMA_JSON} verification_result missing Phase 4 fields: {missing}")
    properties = verification.get("properties", {})
    for field in ("body_hash_ref", "request_hash_ref", "evidence_ref", "cache_key_ref"):
        pattern = properties.get(field, {}).get("pattern")
        if pattern != "^hash:.*blake3:":
            raise AssertionError(f"{SCHEMA_JSON} {field} must require typed BLAKE3 refs")
    states = set(properties.get("verification_state", {}).get("enum", []))
    if states != {"verified", "denied", "blocked"}:
        raise AssertionError(f"{SCHEMA_JSON} verification_state enum drifted")

    if valid.get("schema_version") != "overkey.phase4.verification.v0":
        raise AssertionError("valid Phase 4 fixture has wrong schema_version")
    signature = valid["signature_verification"]
    api_key = valid["api_key_verification"]
    if signature["reason_code"] != "auth.signature_verified_phase4":
        raise AssertionError("signature fixture must use the Phase 4 positive reason")
    if api_key["reason_code"] != "auth.api_key_verified_phase4":
        raise AssertionError("API key fixture must use the Phase 4 positive reason")
    if signature["algorithm"] != "Ed25519":
        raise AssertionError("signature fixture must use Ed25519")
    if api_key["algorithm"] != "BLAKE3-keyed-lookup":
        raise AssertionError("API key fixture must use BLAKE3-keyed-lookup")
    for field in ("body_hash_ref", "request_hash_ref", "evidence_ref", "cache_key_ref"):
        if "blake3:" not in signature[field]:
            raise AssertionError(f"signature fixture {field} must be a BLAKE3 ref")
    if api_key["api_key_hash_ref"].startswith("hash:api_key:blake3:") is False:
        raise AssertionError("API key fixture must use a typed BLAKE3 API key hash ref")
    if api_key["raw_secret_persisted"] is not False or signature["raw_secret_persisted"] is not False:
        raise AssertionError("Phase 4 fixtures must prove no raw secret persistence")
    if "raw_api_key" not in api_key["redacted_fields"]:
        raise AssertionError("API key fixture must name raw_api_key as redacted")

    reason_codes = {entry["reason_code"] for entry in invalid.get("denials", [])}
    expected_denials = {
        "auth.command_class_denied",
        "auth.api_key_hash_mismatch",
        "auth.subject_dependency_denied",
        "auth.service_account_not_approved",
    }
    if reason_codes != expected_denials:
        raise AssertionError(f"invalid fixture denial reason drift: {sorted(reason_codes)}")
    if any(entry.get("side_effects_allowed") for entry in invalid["denials"]):
        raise AssertionError("Phase 4 denial fixtures must prevent side effects")

    combined = json.dumps({"valid": valid, "invalid": invalid}, sort_keys=True).lower()
    for marker in RAW_SECRET_MARKERS:
        if marker in combined:
            raise AssertionError(f"Phase 4 fixtures contain raw secret marker: {marker}")


def run_checks() -> None:
    result = subprocess.run(["cargo", "test", "-p", "overrid-overkey"], cwd=REPO_ROOT)
    if result.returncode != 0:
        raise AssertionError("cargo test -p overrid-overkey failed")


def main() -> int:
    checks = [
        validate_docs_and_wiring,
        validate_rust_sources,
        validate_schema_and_fixtures,
        run_checks,
    ]
    for check in checks:
        check()
    print("Overkey Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
