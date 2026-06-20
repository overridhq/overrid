#!/usr/bin/env python3
"""Validate Integration Test Harness Phase 5 deterministic fixture library."""

from __future__ import annotations

from pathlib import Path
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

HARNESS = Path("packages/integration_harness")
SUB_PLAN = Path("docs/build_plan/sub_build_plan_003_integration_test_harness.md")
SDS = Path("docs/sds/foundation/integration_test_harness.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/integration_test_harness_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/integration_test_harness_phase_05_progress.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

FORBIDDEN_RUNTIME_TERMS = {
    "node.js",
    "typescript runtime",
    "postgresql",
    "redis",
    "s3",
    "minio",
    "nats",
    "kafka",
    "vault",
    "blockchain",
    "nft",
}


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} missing expected text: {expected}")


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def assert_no_forbidden_runtime_authority(text: str, source: Path) -> None:
    lowered = text.lower()
    for term in FORBIDDEN_RUNTIME_TERMS:
        if term in lowered:
            index = lowered.find(term)
            prefix = lowered[max(0, index - 56) : index]
            if "must not" not in prefix and "not as" not in prefix:
                raise AssertionError(
                    f"{source} appears to introduce forbidden runtime authority: {term}"
                )


def run(command: list[str], cwd: Path = REPO_ROOT) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=cwd, text=True, capture_output=True)
    if result.returncode != 0:
        raise AssertionError(
            f"Command failed with exit {result.returncode}: "
            f"{' '.join(command)}\nstdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def validate_required_files() -> None:
    for path in [
        HARNESS / "src/fixtures.rs",
        HARNESS / "src/lib.rs",
        SUB_PLAN,
        SDS,
        TECH_STACK,
        PHASE_PLAN,
        PHASE_PROGRESS,
        VALIDATION_WRAPPER,
    ]:
        assert_true((REPO_ROOT / path).is_file(), f"Missing required file: {path}")


def validate_docs() -> None:
    sub_plan = read(SUB_PLAN)
    sds = read(SDS)
    tech_stack = read(TECH_STACK)
    phase_plan = read(PHASE_PLAN)
    phase_progress = read(PHASE_PROGRESS)
    wrapper = read(VALIDATION_WRAPPER)

    for expected in [
        "Phase 5: Deterministic Fixture Library And Test-Only Identity Controls",
        "Build core identity and tenant fixtures",
        "Build test key and signing fixtures",
        "Build workload, package, and resource fixtures",
        "Build accounting and policy fixtures",
        "Build fixture drift detection",
    ]:
        assert_contains(sub_plan, expected, SUB_PLAN)

    for expected in [
        "fixture manifests and deterministic id generation",
        "Mark all fixture credentials as `test_only`",
        "Use local/test ORU accounts and ledger fixtures",
        "Reset/reseed produces identical fixture ids across runs",
    ]:
        assert_contains(sds, expected, SDS)

    for expected in [
        "Rust-first infrastructure stack",
        "Cargo workspace with crates for services, shared contracts, test utilities, CLI, and local tooling",
        "JSON plus JSON Schema",
        "not as the core runtime",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)

    for expected in [
        "Integration Test Harness Phase 5 Implementation Plan",
        "typed fixture library",
        "scripts/validate_integration_harness_phase5.py",
    ]:
        assert_contains(phase_plan, expected, PHASE_PLAN)

    for expected in [
        "Integration Test Harness Phase 5 Progress",
        "Validation Evidence",
        "packages/integration_harness/src/fixtures.rs",
    ]:
        assert_contains(phase_progress, expected, PHASE_PROGRESS)

    assert_contains(wrapper, "scripts/validate_integration_harness_phase5.py", VALIDATION_WRAPPER)


def validate_fixture_code() -> None:
    fixtures_rs = read(HARNESS / "src/fixtures.rs")
    lib_rs = read(HARNESS / "src/lib.rs")

    assert_contains(lib_rs, "pub mod fixtures;", HARNESS / "src/lib.rs")

    for expected in [
        "FixtureIdentityKind",
        "TenantFixture",
        "IdentityFixture",
        "RoleBindingFixture",
        "QuotaFixture",
        "FixtureAdmissionOutcome",
        "SigningKeyVariant",
        "TestSigningKeyFixture",
        "is_fixture_local_profile",
        "ResourceCardFixture",
        "WorkloadFixture",
        "AccountingPolicyFixture",
        "FixtureLibrary",
        "FixtureDriftSnapshot",
        "FixtureDriftDifference",
        "FixtureDriftReport",
        "compare_fixture_drift",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "FixtureKey::test_only",
        "signature_ref_only",
        "raw_key_material_present",
        "safety.fixture_key_outside_local",
        "safety.fixture_not_test_only",
        "signature.revoked_key",
        "signature.wrong_tenant",
        "signature.expired_key",
        "signature.invalid",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "fixture.tenant_suspended",
        "fixture.actor_suspended",
        "fixture.quota_exhausted",
        "fixture.role_denied",
        "fixture.identity_accepted",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "phase1_pending_noop",
        "pending_queue_state_only",
        "execution_claimed",
        "fixture.execution_not_available_in_phase1",
        "component:overqueue_durable_state",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "local_oru_account_ref",
        "usage_dimension_ref",
        "receipt_ref",
        "budget_exhausted",
        "external_payment_provider_reachable",
        "provider_payout_reachable",
        "accounting.test_usage_isolated",
        "safety.external_payment_reachable",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "fixture.drift_absent",
        "fixture.drift_detected",
        "fixture.field_drift",
        "fixture.ref_set_drift",
        "$.schema_version",
        "$.generated_refs",
        "$.golden_trace_refs",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    for expected in [
        "phase5_library_emits_schema_checked_test_only_fixture_manifest",
        "identity_and_tenant_variants_have_stable_accept_deny_behavior",
        "signing_key_variants_fail_closed_without_raw_key_material",
        "workload_fixture_keeps_phase1_at_pending_queue_state",
        "accounting_policy_fixture_is_test_usage_and_payment_isolated",
        "fixture_drift_report_uses_stable_diff_fields",
    ]:
        assert_contains(fixtures_rs, expected, HARNESS / "src/fixtures.rs")

    assert_no_forbidden_runtime_authority(fixtures_rs, HARNESS / "src/fixtures.rs")


def validate_rust_behavior() -> None:
    run(["cargo", "check", "-p", "overrid-integration-harness"])
    result = run(["cargo", "test", "-p", "overrid-integration-harness"])
    assert_contains(result.stdout, "34 passed", Path("cargo test -p overrid-integration-harness"))


def main() -> int:
    try:
        validate_required_files()
        validate_docs()
        validate_fixture_code()
        validate_rust_behavior()
    except AssertionError as exc:
        print(f"Integration harness Phase 5 validation failed: {exc}", file=sys.stderr)
        return 1

    print("Integration harness Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
