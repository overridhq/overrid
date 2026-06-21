#!/usr/bin/env python3
"""Validate SDK Phase 5 credential-provider, signing, and security guardrails."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_05_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_05_progress.md")
SDK_CARGO = Path("packages/sdk/Cargo.toml")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_SIGNING = Path("packages/sdk/src/signing.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE5_SYMBOLS = [
    "pub mod signing;",
    "pub use signing::*;",
    "SDK_PHASE5_CAPABILITY_PROFILE",
    "SDK_PHASE5_SIGNING_ALGORITHM",
    "SDK_PHASE5_REDACTION_POLICY",
    "SDK_PHASE5_TEST_SIGNER_MODULE",
    "SdkCredentialProviderKind",
    "FileBackedLocalTest",
    "HostSigningAgentSocket",
    "PlatformKeychain",
    "HardwareBackedDevice",
    "OverkeyCredentialRef",
    "SdkSigningCapability",
    "SdkCredentialProvider",
    "from_config",
    "from_record",
    "credential_record",
    "SdkTestSignerGate",
    "validate_fixture_signer_installation",
    "SdkCanonicalSigningInput",
    "canonical_string",
    "canonical_bytes",
    "phase5_signature_ref",
    "build_canonical_signing_input",
    "SdkSignedOvergateRequest",
    "sign_request",
    "validate_signed_request_invariants",
    "SdkDiagnosticEventKind",
    "SDK_PHASE5_DIAGNOSTIC_EVENTS",
    "SdkRedactedDiagnosticEvent",
    "redacted_diagnostic_event",
    "SdkCredentialLifecycleStatus",
    "SdkCredentialLifecycleDecision",
    "credential_lifecycle_failure",
    "SdkPhase5Error",
]

REQUIRED_PHASE5_TESTS = [
    "phase5_provider_interfaces_describe_capabilities_without_private_material",
    "phase5_canonical_signing_input_is_stable_and_rejects_mutations",
    "phase5_test_signers_are_structurally_separate_from_production",
    "phase5_redacted_diagnostics_never_emit_secrets_or_signatures",
    "phase5_credential_lifecycle_failures_are_terminal_without_explicit_retry",
    "phase5_signing_rejects_mismatched_credentials_and_unsupported_algorithms",
]

REQUIRED_DIAGNOSTIC_EVENTS = [
    "RequestBuilt",
    "RequestSigned",
    "RequestSent",
    "ResponseReceived",
    "RetryScheduled",
    "RequestDenied",
    "RequestFailed",
    "DuplicateResolved",
]

REQUIRED_LIFECYCLE_STATUSES = [
    "Expired",
    "Revoked",
    "Rotated",
    "MissingCredentialRef",
    "MismatchedCredentialId",
    "UnknownCredential",
    "InsufficientCredential",
    "HostSignerUnavailable",
    "RetryProhibitedSigningFailure",
]


def read(path: Path) -> str:
    full_path = REPO_ROOT / path
    if not full_path.is_file():
        raise AssertionError(f"Missing required file: {path}")
    return full_path.read_text(encoding="utf-8")


def run(command: list[str]) -> subprocess.CompletedProcess[str]:
    result = subprocess.run(command, cwd=REPO_ROOT, text=True, capture_output=True)
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        raise AssertionError(f"Command failed: {' '.join(command)}")
    return result


def section(text: str, heading: str, next_heading_level: str = "## ") -> str:
    marker = f"{heading}\n"
    start = text.find(marker)
    if start == -1:
        raise AssertionError(f"Missing heading: {heading}")
    body_start = start + len(marker)
    end = text.find(f"\n{next_heading_level}", body_start)
    if end == -1:
        end = len(text)
    return text[body_start:end]


def assert_contains(text: str, expected: str, source: Path) -> None:
    if expected not in text:
        raise AssertionError(f"{source} is missing expected text: {expected}")


def validate_sub_plan_phase5() -> None:
    text = read(SUB_PLAN)
    phase_5 = section(text, "## Phase 5: Credential Providers, Signing, And Security Guardrails")
    for item in range(1, 6):
        assert_contains(phase_5, f"**5.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*5\.[1-5] .+?(?=\n- \*\*5\.|\n## Phase 6:)",
        phase_5,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "provider traits for file-backed local/test credentials",
        "signRequest(command, credentialRef)",
        "fake signers and fixture credentials",
        "request_built",
        "Credential error mapping",
    ]:
        assert_contains(phase_5, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 5 gate",
        "credential-provider descriptors",
        "canonical signing input helpers",
        "fake/test signers structurally gated",
        "redacted diagnostic event helpers",
        "credential lifecycle failure mapping",
        "cargo test -p overrid-sdk",
        "Do not store private keys, bearer tokens, seed phrases, raw signatures, raw payload secrets, or vault values",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 5 session context",
        "Validation Evidence",
        "cargo test -p overrid-sdk",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    cargo = read(SDK_CARGO)
    lib = read(SDK_LIB)
    signing = read(SDK_SIGNING)
    readme = read(SDK_README)
    combined = f"{lib}\n{signing}"

    for expected in REQUIRED_PHASE5_SYMBOLS + REQUIRED_PHASE5_TESTS:
        assert_contains(combined, expected, SDK_SIGNING)
    for expected in REQUIRED_DIAGNOSTIC_EVENTS + REQUIRED_LIFECYCLE_STATUSES:
        assert_contains(signing, expected, SDK_SIGNING)
    for expected in [
        "stores_private_material: false",
        "stores_bearer_token: false",
        "stores_seed_phrase: false",
        "stores_vault_value: false",
        "payload=redacted",
        "signature=redacted",
        "secret_refs=redacted",
        "credential_expired",
        "credential_revoked",
        "credential_rotated",
        "credential_ref_missing",
        "host_signer_unavailable",
        "signing_failure_retry_prohibited",
        "retry only through explicit overgate correction path with same idempotency key",
    ]:
        assert_contains(signing, expected, SDK_SIGNING)
    for expected in [
        "## Phase 5 Credential Providers, Signing, And Security Guardrails",
        "`SdkCredentialProvider::from_config()`",
        "`SdkCredentialProviderKind`",
        "`phase5_signature_ref()`",
        "`build_canonical_signing_input()`",
        "`sign_request()`",
        "`validate_signed_request_invariants()`",
        "`validate_fixture_signer_installation()`",
        "`SDK_PHASE5_DIAGNOSTIC_EVENTS`",
        "`redacted_diagnostic_event()`",
        "`credential_lifecycle_failure()`",
        "payloads, signatures, and secret refs as redacted",
    ]:
        assert_contains(readme, expected, SDK_README)
    if "[dependencies]\noverrid-contracts" not in cargo:
        raise AssertionError(f"{SDK_CARGO} should remain dependency-light for Phase 5")
    if not re.search(r"sorted_headers\.sort_by", signing):
        raise AssertionError(f"{SDK_SIGNING} must sort signing headers deterministically")
    if "test_fixtures_enabled" not in signing or "ProductionLike" not in signing:
        raise AssertionError(f"{SDK_SIGNING} must gate fixture signers by explicit test config")


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    signing = read(SDK_SIGNING)
    for expected in [
        "| SDKs | Generated Rust SDK first",
        "Signed command envelopes",
        "idempotency keys",
        "trace ids",
        "stable reason codes",
    ]:
        assert_contains(tech_stack, expected, TECH_STACK)
    for forbidden in [
        "postgres",
        "redis",
        "kafka",
        "nats",
        "s3",
        "typescript",
        "blockchain",
        "nft",
        "pricing",
        "revenue",
        "customer-count",
    ]:
        if forbidden in signing.lower():
            raise AssertionError(f"{SDK_SIGNING} contains forbidden stack term: {forbidden}")


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase5.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk"])
    assert_contains(result.stdout + result.stderr, "test result: ok.", Path("cargo test -p overrid-sdk"))


def main() -> int:
    checks = [
        validate_sub_plan_phase5,
        validate_phase_docs,
        validate_sdk_code_and_readme,
        validate_tech_stack_alignment,
        validate_wrapper,
        validate_rust_tests,
    ]
    for check in checks:
        check()
    print("SDK Phase 5 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
