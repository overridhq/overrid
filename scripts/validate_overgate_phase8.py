#!/usr/bin/env python3
"""Validate Overgate Phase 8 forwarding, Overqueue, and product-client artifacts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]

LIB = Path("packages/overgate/src/lib.rs")
FORWARDING = Path("packages/overgate/src/forwarding.rs")
ROUTES = Path("packages/overgate/src/routes.rs")
SERVICE = Path("packages/overgate/src/service.rs")
IDEMPOTENCY = Path("packages/overgate/src/idempotency.rs")
ERRORS = Path("packages/overgate/src/errors.rs")
README = Path("packages/overgate/README.md")
VALID_PHASE8_FIXTURE = Path("packages/overgate/fixtures/valid/phase8_command.valid.json")
INVALID_PHASE8_FIXTURE = Path(
    "packages/overgate/fixtures/invalid/phase8_forwarding_denials.invalid.json"
)
SUB_PLAN = Path("docs/build_plan/sub_build_plan_008_overgate.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/overgate_phase_08_plan.md")
PHASE_PROGRESS = Path("docs/planning/overgate_phase_08_progress.md")
SUITE_VALIDATOR = Path("scripts/validate_overrid.py")

RAW_SECRET_MARKERS = (
    "password=",
    "token=",
    "secret=",
    "api_key=",
    "private key",
    "-----begin",
    "raw_secret_value",
)

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

REQUIRED_CLASSES = [
    "low_risk_read",
    "phase1_control_plane_mutation",
    "queue_producing_workload",
    "policy_heavy",
    "accounting_affecting",
    "storage_namespace",
    "native_app_side_effect",
    "admin",
    "break_glass",
]


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
    lib = read(LIB)
    suite = read(SUITE_VALIDATOR)
    sub_plan = read(SUB_PLAN)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)

    assert_contains(lib, "pub mod forwarding;", LIB)
    assert_contains(suite, 'Path("scripts/validate_overgate_phase8.py")', SUITE_VALIDATOR)
    assert_contains(
        sub_plan,
        "## Phase 8: Forwarding, Overqueue Dispatch, And Downstream Contract Boundaries",
        SUB_PLAN,
    )
    assert_contains(sub_plan, "Phase 1 synchronous forwarding", SUB_PLAN)
    assert_contains(sub_plan, "Overqueue-backed dispatch", SUB_PLAN)
    assert_contains(sub_plan, "downstream target registry", SUB_PLAN)
    assert_contains(tech_stack, "Native Overqueue durable state and events", TECH_STACK)
    assert_contains(phase_plan, "Complete SUB BUILD PLAN #8 Phase 8", PHASE_PLAN)
    assert_contains(phase_progress, "Overgate Phase 8 Progress", PHASE_PROGRESS)
    assert_contains(phase_progress, "Docdex impact graphs before edits", PHASE_PROGRESS)
    assert_contains(phase_progress, "local delegation", PHASE_PROGRESS)


def validate_rust_sources() -> None:
    forwarding = read(FORWARDING)
    routes = read(ROUTES)
    service = read(SERVICE)
    idempotency = read(IDEMPOTENCY)
    errors = read(ERRORS)

    for expected in (
        "PHASE8_FORWARDING_ADAPTER_ID",
        "PHASE8_TARGET_REGISTRY_REF",
        "PHASE8_OVERQUEUE_CONTRACT_REF",
        "PHASE8_PRODUCT_CLIENT_CHECKLIST_REF",
        "ForwardingStore",
        "ForwardingInput",
        "ForwardingOutcome",
        "ForwardingRecord",
        "DownstreamTarget",
        "OverqueueWorkItem",
        "SynchronousCompletion",
        "RetryMetadata",
        "ForwardingStatusProjection",
        "ProductClientFlowChecklist",
        "target_registry",
        "validate_target_registry",
        "forward_after_acceptance",
        "synchronous_phase1_forwarding_phase8",
        "overqueue_durable_dispatch_phase8",
        "durable_pending_work_phase8",
        "failed_after_acceptance_phase8",
        "retry_scheduled_phase8",
        "dead_lettered_phase8",
        "none_native_overqueue_contract_only",
        "direct_downstream_state_write: false",
        "raw_private_payload_written: false",
        "internal_api_bypass_allowed: false",
    ):
        assert_contains(forwarding, expected, FORWARDING)
    for command_class in REQUIRED_CLASSES:
        assert_contains(forwarding, command_class, FORWARDING)

    for expected in (
        "PHASE8_RESPONSE_SCHEMA_VERSION",
        "overgate.phase8.response.v0",
        "PHASE8_COMMAND_ACCEPTED_REASON",
        "ForwardingInput::from_parts",
        "forward_after_acceptance",
        "IdempotencyForwardingProjection",
        "apply_forwarding_projection",
        "phase8_forwarding",
        "overgate.forwarding_failed_after_acceptance",
        "phase8_synchronous_forwarding_completes_narrow_phase1_commands",
        "phase8_overqueue_dispatch_records_durable_pending_work",
        "phase8_target_registry_rejects_unregistered_targets",
        "phase8_failed_after_acceptance_status_preserves_retry_projection",
        "phase8_product_clients_cannot_bypass_overgate_forwarding_contracts",
    ):
        assert_contains(routes, expected, ROUTES)
    if routes.find("record_acceptance") > routes.find("forward_after_acceptance"):
        raise AssertionError(f"{ROUTES} must record Phase 7 audit before Phase 8 forwarding")

    assert_contains(service, "pub forwarding: ForwardingStore", SERVICE)
    assert_contains(service, "forwarding: ForwardingStore::default()", SERVICE)
    assert_contains(idempotency, "IdempotencyForwardingProjection", IDEMPOTENCY)
    assert_contains(idempotency, "apply_forwarding_projection", IDEMPOTENCY)
    assert_contains(errors, "forwarding_target_unregistered", ERRORS)
    assert_contains(errors, "product_client_bypass_denied", ERRORS)


def validate_fixtures() -> None:
    valid = load_json(VALID_PHASE8_FIXTURE)
    invalid = load_json(INVALID_PHASE8_FIXTURE)

    if valid["schema_version"] != "overgate.phase8.local_fixture.v0":
        raise AssertionError(f"{VALID_PHASE8_FIXTURE} has wrong schema_version")
    envelope = valid["command_envelope"]
    for field in REQUIRED_ENVELOPE_FIELDS:
        if field not in envelope:
            raise AssertionError(f"{VALID_PHASE8_FIXTURE} command_envelope missing {field}")
    if envelope["command_type"] != "overgate.phase8.queue.workload.submit":
        raise AssertionError(f"{VALID_PHASE8_FIXTURE} command_type is wrong")
    if envelope["schema_version"] != "shared-schema-package.v0.1":
        raise AssertionError(f"{VALID_PHASE8_FIXTURE} command schema_version is wrong")

    expected = valid["expected_response"]
    expected_pairs = {
        "schema_version": "overgate.phase8.response.v0",
        "reason_code": "overgate.command_accepted_phase8",
        "forwarding_outcome_state": "overqueue_dispatch_recorded_phase8",
        "forwarding_state": "overqueue_pending_phase8",
        "current_state": "pending_overqueue_dispatch",
        "target_registry_ref": "forwarding_target_registry:overgate:phase8",
        "overqueue_contract_ref": "overqueue.dispatch.v0",
        "durable_state": "durable_pending_work_phase8",
        "owner_service": "service:overqueue",
        "dispatch_mode": "overqueue_durable_dispatch_phase8",
        "native_overqueue_boundary": True,
        "external_queue_dependency": "none_native_overqueue_contract_only",
        "direct_downstream_state_write": False,
        "raw_private_payload_written": False,
        "product_client_checklist_ref": "product_client_flows:overgate:phase8",
        "internal_api_bypass_allowed": False,
    }
    for key, expected_value in expected_pairs.items():
        if expected.get(key) != expected_value:
            raise AssertionError(f"{VALID_PHASE8_FIXTURE} expected_response.{key} is wrong")

    for key in ("missing_target_command_envelope", "product_bypass_command_envelope"):
        envelope = invalid[key]
        for field in REQUIRED_ENVELOPE_FIELDS:
            if field not in envelope:
                raise AssertionError(f"{INVALID_PHASE8_FIXTURE} {key} missing {field}")

    denials = invalid["expected_denials"]
    required_denials = {
        "missing_target": (400, "overgate.forwarding_target_unregistered"),
        "product_bypass": (403, "overgate.product_client_bypass_denied"),
    }
    for key, (status, reason_code) in required_denials.items():
        denial = denials[key]
        if denial["status"] != status:
            raise AssertionError(f"{key} denial has wrong status")
        if denial["reason_code"] != reason_code:
            raise AssertionError(f"{key} denial has wrong reason_code")


def validate_tests_and_readme() -> None:
    routes = read(ROUTES)
    readme = read(README)
    tests = set(re.findall(r"#\[tokio::test\]\s+async fn ([a-z0-9_]+)", routes))
    required_tests = {
        "phase8_synchronous_forwarding_completes_narrow_phase1_commands",
        "phase8_overqueue_dispatch_records_durable_pending_work",
        "phase8_target_registry_rejects_unregistered_targets",
        "phase8_failed_after_acceptance_status_preserves_retry_projection",
        "phase8_product_clients_cannot_bypass_overgate_forwarding_contracts",
    }
    missing = sorted(required_tests - tests)
    if missing:
        raise AssertionError(f"{ROUTES} missing required tests: {', '.join(missing)}")

    for expected in (
        "Phase 8 Forwarding, Overqueue Dispatch, And Downstream Contract Boundaries",
        "overgate.phase8.response.v0",
        "forwarding_target_registry:overgate:phase8",
        "overqueue.dispatch.v0",
        "durable pending work",
        "Failed-after-acceptance",
        "Product-client command flows",
        "fixtures/valid/phase8_command.valid.json",
        "fixtures/invalid/phase8_forwarding_denials.invalid.json",
    ):
        assert_contains(readme, expected, README)

    for path in (README, VALID_PHASE8_FIXTURE, INVALID_PHASE8_FIXTURE):
        text = read(path) if path.suffix != ".json" else json.dumps(load_json(path)).lower()
        lowered = text.lower()
        for marker in RAW_SECRET_MARKERS:
            if marker in lowered:
                raise AssertionError(f"{path} contains raw secret marker {marker!r}")


def main() -> int:
    try:
        validate_docs_and_wiring()
        validate_rust_sources()
        validate_fixtures()
        validate_tests_and_readme()
    except AssertionError as error:
        print(f"validate_overgate_phase8 failed: {error}", file=sys.stderr)
        return 1
    print("Overgate Phase 8 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
