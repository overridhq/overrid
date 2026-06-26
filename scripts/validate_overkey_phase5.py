#!/usr/bin/env python3
"""Validate Overkey Phase 5 rotation, revocation, cache invalidation, and break-glass wiring."""

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
VALID_FIXTURE = Path("packages/overkey/fixtures/valid/phase5_lifecycle.valid.json")
INVALID_FIXTURE = Path("packages/overkey/fixtures/invalid/phase5_break_glass_denials.invalid.json")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_009_overkey.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overkey_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/overkey_phase_05_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "raw_api_key=",
    "api_key=",
    "private_key=",
    "-----begin",
    "seed_phrase=",
    "password=",
)

PHASE5_ROTATION_FIELDS = {
    "predecessor_credential_id",
    "predecessor_key_id",
    "predecessor_key_version",
    "successor_credential_id",
    "successor_key_id",
    "successor_key_version",
    "grace_window_seconds",
    "rotation_state",
    "initiated_by",
    "reason_code",
    "activation_at",
    "evidence_refs",
    "revocation_epoch",
    "propagation_status",
}

PHASE5_REVOCATION_FIELDS = {
    "revoked_by",
    "effective_at",
    "affected_command_classes",
    "incident_refs",
    "evidence_refs",
    "expected_current_status",
    "revocation_epoch",
    "break_glass",
    "idempotency_key",
    "propagation_status",
}

PHASE5_SERVICES = {
    "service:overgate",
    "service:overvault",
    "service:overqueue",
    "service:oversched",
    "service:overcell",
    "service:system-services",
    "service:product-clients",
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

    assert_contains(workspace, '"python3 scripts/validate_overkey_phase5.py"', WORKSPACE_LAYOUT)
    assert_contains(suite, 'Path("scripts/validate_overkey_phase5.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 5: Rotation, Revocation, Cache Invalidation, And Break-Glass",
        SUB_PLAN,
    )
    for expected in (
        "Rust-first infrastructure stack",
        "Canonical JSON plus JSON Schema",
        "Ed25519 signatures",
        "BLAKE3/content hashes",
        "Signed command envelopes",
    ):
        assert_contains(tech_stack, expected, TECH_STACK)
    for expected in (
        "rotation plans",
        "revocation records",
        "verification cache guidance",
        "signed break-glass revocation",
        "propagation-status",
    ):
        assert_contains(phase_plan, expected, PHASE_PLAN)
    assert_contains(phase_progress, "Overkey Phase 5 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact", PHASE_PROGRESS)

    for expected in (
        "Phase 2 through Phase 5",
        "overkey.phase5.response.v0",
        "Signed break-glass revocation",
        "fixtures/valid/phase5_lifecycle.valid.json",
        "fixtures/invalid/phase5_break_glass_denials.invalid.json",
        "python3 scripts/validate_overkey_phase5.py",
    ):
        assert_contains(readme, expected, README)


def validate_rust_sources() -> None:
    routes = read(ROUTES)
    records = read(RECORDS)
    repository = read(REPOSITORY)
    schema_rs = read(SCHEMA_RS)

    for expected in (
        "OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION",
        "PHASE5_PROPAGATION_SERVICES",
        "Phase5LifecycleData",
        "CacheInvalidation",
        "PropagationStatus",
        "AffectedInventory",
        "append_rotation_record",
        "append_revocation_record",
        "let revocation_epoch = current.revocation_epoch + 1",
        "validate_rotation_successor",
        "overkey.rotation_successor_invalid",
        "validate_break_glass_request",
        "auth.break_glass_unsigned",
        "auth.break_glass_wrong_role",
        "auth.break_glass_protection_class_required",
        "auth.break_glass_missing_evidence",
        "auth.break_glass_idempotency_required",
        "tenant:phase5-other",
        "overkey.credential_not_found",
        "overkey.break_glass_revocation_idempotent_replay",
        "phase5_rotation_records_epoch_cache_and_propagation",
        "phase5_break_glass_revocation_requires_signed_idempotent_command",
    ):
        assert_contains(routes, expected, ROUTES)

    for expected in (
        "predecessor_credential_id: String",
        "successor_credential_id: String",
        "grace_window_seconds: u64",
        "revoked_by: String",
        "effective_at: String",
        "affected_command_classes: Vec<String>",
        "break_glass: bool",
        "idempotency_key: Option<String>",
        "propagation_status: Vec<PropagationStatus>",
        "pub struct CacheInvalidation",
        "pub struct AffectedInventory",
    ):
        assert_contains(records, expected, RECORDS)

    for expected in (
        "DuplicateLifecycleRecord",
        "rotations: Vec<RotationRecord>",
        "revocations: Vec<RevocationRecord>",
        "credential.revocation_epoch",
        "rotation_records",
        "revocation_records",
    ):
        assert_contains(repository, expected, REPOSITORY)

    assert_contains(schema_rs, "OVERKEY_PHASE5_RESPONSE_SCHEMA_VERSION", SCHEMA_RS)
    assert_contains(schema_rs, "OVERKEY_PHASE5_FIXTURE_SCHEMA_VERSION", SCHEMA_RS)


def validate_schema_and_fixtures() -> None:
    schema = load_json(SCHEMA_JSON)
    valid = load_json(VALID_FIXTURE)
    invalid = load_json(INVALID_FIXTURE)

    defs = schema.get("$defs", {})
    rotation_required = set(defs.get("rotation_record", {}).get("required", []))
    missing_rotation = sorted(PHASE5_ROTATION_FIELDS - rotation_required)
    if missing_rotation:
        raise AssertionError(f"{SCHEMA_JSON} rotation_record missing Phase 5 fields: {missing_rotation}")

    revocation_required = set(defs.get("revocation_record", {}).get("required", []))
    missing_revocation = sorted(PHASE5_REVOCATION_FIELDS - revocation_required)
    if missing_revocation:
        raise AssertionError(
            f"{SCHEMA_JSON} revocation_record missing Phase 5 fields: {missing_revocation}"
        )

    cache = defs.get("cache_invalidation", {})
    cache_required = set(cache.get("required", []))
    for field in (
        "cache_key_ref",
        "revocation_epoch",
        "max_positive_ttl_seconds",
        "high_risk_max_positive_ttl_seconds",
        "invalidation_event_ref",
    ):
        if field not in cache_required:
            raise AssertionError(f"{SCHEMA_JSON} cache_invalidation missing {field}")
    if cache.get("properties", {}).get("max_positive_ttl_seconds", {}).get("const") != 30:
        raise AssertionError(f"{SCHEMA_JSON} ordinary positive cache TTL must stay 30 seconds")
    if cache.get("properties", {}).get("high_risk_max_positive_ttl_seconds", {}).get("const") != 5:
        raise AssertionError(f"{SCHEMA_JSON} high-risk positive cache TTL must stay 5 seconds")

    if valid.get("schema_version") != "overkey.phase5.lifecycle.v0":
        raise AssertionError("valid Phase 5 fixture has wrong schema_version")
    rotation = valid["rotation_plan"]["rotation_record"]
    revocation = valid["break_glass_revocation"]["revocation_record"]
    if rotation["successor_key_version"] != 2 or rotation["grace_window_seconds"] <= 0:
        raise AssertionError("rotation fixture must include successor key version and grace window")
    if revocation["break_glass"] is not True or not revocation["idempotency_key"]:
        raise AssertionError("break-glass revocation fixture must be idempotent and marked break_glass")
    if revocation["expected_current_status"] != "active":
        raise AssertionError("break-glass revocation fixture must bind expected current state")
    cache_fixture = valid["cache_invalidation"]
    if cache_fixture["max_positive_ttl_seconds"] != 30:
        raise AssertionError("fixture ordinary positive cache TTL must be 30")
    if cache_fixture["high_risk_max_positive_ttl_seconds"] != 5:
        raise AssertionError("fixture high-risk positive cache TTL must be 5")
    services = set(valid["affected_inventory"]["services"])
    if services != PHASE5_SERVICES:
        raise AssertionError(f"Phase 5 affected services drifted: {sorted(services)}")
    for marker in RAW_SECRET_MARKERS:
        if marker.lower() in json.dumps(valid).lower():
            raise AssertionError(f"{VALID_FIXTURE} includes raw secret marker {marker}")

    reason_codes = {entry["reason_code"] for entry in invalid.get("denials", [])}
    expected_denials = {
        "auth.break_glass_unsigned",
        "auth.break_glass_wrong_role",
        "auth.break_glass_protection_class_required",
        "auth.break_glass_missing_evidence",
        "auth.break_glass_idempotency_required",
        "overkey.credential_not_found",
    }
    if reason_codes != expected_denials:
        raise AssertionError(f"invalid fixture denial reason drift: {sorted(reason_codes)}")
    if any(entry.get("side_effects_allowed") for entry in invalid["denials"]):
        raise AssertionError("Phase 5 break-glass denial fixtures must prevent side effects")


def run_targeted_tests() -> None:
    result = subprocess.run(
        [
            "cargo",
            "test",
            "-p",
            "overrid-overkey",
            "phase5",
        ],
        cwd=REPO_ROOT,
    )
    if result.returncode != 0:
        raise AssertionError("cargo test -p overrid-overkey phase5 failed")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_schema_and_fixtures()
        run_targeted_tests()
    except AssertionError as error:
        print(f"Overkey Phase 5 validation failed: {error}", file=sys.stderr)
        return 1
    print("Overkey Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
