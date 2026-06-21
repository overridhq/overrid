#!/usr/bin/env python3
"""Validate SDK Phase 4 command pipeline, idempotency, retry, and error gates."""

from __future__ import annotations

from pathlib import Path
import re
import subprocess
import sys


REPO_ROOT = Path(__file__).resolve().parents[1]

SUB_PLAN = Path("docs/build_plan/sub_build_plan_006_sdk.md")
TECH_STACK = Path("docs/overrid_tech_stack_choice.md")
PHASE_PLAN = Path("docs/planning/sdk_phase_04_plan.md")
PHASE_PROGRESS = Path("docs/planning/sdk_phase_04_progress.md")
SDK_LIB = Path("packages/sdk/src/lib.rs")
SDK_COMMAND = Path("packages/sdk/src/command.rs")
SDK_README = Path("packages/sdk/README.md")
VALIDATION_WRAPPER = Path("scripts/validate_overrid.py")

REQUIRED_PHASE4_SYMBOLS = [
    "pub mod command;",
    "pub use command::*;",
    "SDK_PHASE4_CAPABILITY_PROFILE",
    "SDK_PHASE4_COMMAND_ROUTE",
    "SDK_PHASE4_IN_FLIGHT_RETRY_RETENTION_MS",
    "SDK_PHASE4_PHASE1_TERMINAL_RETENTION_MS",
    "SDK_PHASE4_WORKLOAD_REF_RETENTION_MS",
    "SDK_PHASE4_SECURITY_SENSITIVE_RETENTION_MS",
    "SdkCommandPayload",
    "canonical_payload",
    "SdkCommandBuildInput",
    "SdkCommandEnvelope",
    "build_command",
    "SdkOvergateSubmission",
    "prepare_overgate_submission",
    "SdkOvergateResponseStatus",
    "SdkOvergateResponse",
    "decode_overgate_submission",
    "SdkCommandLifecycleState",
    "SdkIdempotencyCachePolicy",
    "phase4_idempotency_policy",
    "phase4_idempotency_policy_with_deadline",
    "phase4_in_flight_retry_retention_ms",
    "SdkIdempotencyCacheDecision",
    "evaluate_idempotency_cache",
    "clear_phase4_idempotency_cache",
    "SdkRetrySignal",
    "SdkRetryClassification",
    "classify_phase4_retry",
    "SdkServiceErrorInput",
    "decode_stable_overrid_error",
    "IdempotencyConflict",
    "TraceMismatch",
    "InvalidLifecycleTransition",
]

REQUIRED_PHASE4_TESTS = [
    "phase4_command_builder_requires_fields_and_canonicalizes_payload",
    "phase4_overgate_submission_preserves_refs_and_route",
    "phase4_idempotency_cache_bounds_replay_conflict_and_cleanup",
    "phase4_retry_classifier_only_retries_safe_or_explicit_signals",
    "phase4_error_decoder_preserves_stable_service_fields",
    "phase4_lifecycle_requires_explicit_service_response_for_terminal_states",
]

REQUIRED_RESPONSE_STATES = [
    "Accepted",
    "Completed",
    "Denied",
    "Duplicate",
    "Retryable",
    "TerminalFailure",
]

REQUIRED_ERROR_FIXTURES = [
    "invalid_signature",
    "duplicate_idempotency_conflict",
    "schema_version_unsupported",
    "revoked_credential",
    "policy_denial",
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


def validate_sub_plan_phase4() -> None:
    text = read(SUB_PLAN)
    phase_4 = section(text, "## Phase 4: Command Pipeline, Idempotency, Retry, And Errors")
    for item in range(1, 6):
        assert_contains(phase_4, f"**4.{item} ", SUB_PLAN)
    for work_item in re.finditer(
        r"- \*\*4\.[1-5] .+?(?=\n- \*\*4\.|\n## Phase 5:)",
        phase_4,
        re.S,
    ):
        item_text = work_item.group(0)
        for field in ("Design:", "Output:", "Validation:"):
            if field not in item_text:
                first_line = item_text.splitlines()[0]
                raise AssertionError(f"{first_line} is missing {field}")
    for expected in [
        "buildCommand(input)",
        "submitCommand(command)",
        "bounded idempotency cache behavior",
        "Retry classifier and request lifecycle state machine",
        "stable error decoding",
    ]:
        assert_contains(phase_4, expected, SUB_PLAN)


def validate_phase_docs() -> None:
    plan = read(PHASE_PLAN)
    progress = read(PHASE_PROGRESS)
    for expected in [
        "Complete the Phase 4 gate",
        "typed `build_command` semantics",
        "deterministic canonical payload",
        "Overgate submission descriptors",
        "bounded idempotency cache policy",
        "retry classification and lifecycle-state validation",
        "stable error decoding",
        "cargo test -p overrid-sdk",
        "Do not infer accepted, completed, duplicate, denied, failed, or retry-wait states",
    ]:
        assert_contains(plan, expected, PHASE_PLAN)
    for expected in [
        "Loaded Docdex profile memory and repo memory",
        "Docdex impact graph",
        "Local delegation was attempted",
        "Saved the repo-scoped Phase 4 session context",
        "Validation Evidence",
    ]:
        assert_contains(progress, expected, PHASE_PROGRESS)


def validate_sdk_code_and_readme() -> None:
    lib = read(SDK_LIB)
    command = read(SDK_COMMAND)
    readme = read(SDK_README)
    combined = f"{lib}\n{command}"

    for expected in REQUIRED_PHASE4_SYMBOLS + REQUIRED_PHASE4_TESTS:
        assert_contains(combined, expected, SDK_COMMAND)
    for expected in REQUIRED_RESPONSE_STATES:
        assert_contains(command, expected, SDK_COMMAND)
    for expected in REQUIRED_ERROR_FIXTURES:
        assert_contains(command, expected, SDK_COMMAND)
    for expected in [
        "x-overrid-idempotency-key",
        "x-overrid-command-timestamp-ms",
        "x-overrid-command-deadline-at-ms",
        "x-overrid-payload-hash",
        "x-overrid-target",
        "command_deadline_at_ms",
        "SDK_PHASE4_CAPABILITY_PROFILE",
        "stable_phase4_hash",
        "BootstrapAcceptanceRecord",
        "SignedCommandEnvelope::new",
        "RetryClass::RetryAfter",
        "without_service_confirmation",
    ]:
        assert_contains(command, expected, SDK_COMMAND)
    for expected in [
        "## Phase 4 Command Pipeline, Idempotency, Retry, And Errors",
        "`build_command()`",
        "`prepare_overgate_submission()`",
        "`decode_overgate_submission()`",
        "`phase4_idempotency_policy()`",
        "`evaluate_idempotency_cache()`",
        "`classify_phase4_retry()`",
        "`decode_stable_overrid_error()`",
        "shorter of the absolute command deadline or the 2-hour SDK cap",
        "security-sensitive terminal digests for 24 hours with no raw payload retention",
    ]:
        assert_contains(readme, expected, SDK_README)
    if "SDK_PHASE4_IN_FLIGHT_RETRY_RETENTION_MS: u64 = 2 * 60 * 60 * 1_000" not in command:
        raise AssertionError(f"{SDK_COMMAND} must cap in-flight retry retention at 2 hours")
    if not re.search(
        r"SDK_PHASE4_SECURITY_SENSITIVE_RETENTION_MS:\s*u64\s*=\s*SDK_PHASE4_PHASE1_TERMINAL_RETENTION_MS",
        command,
    ):
        raise AssertionError(
            f"{SDK_COMMAND} must keep security-sensitive terminal digests for the Phase 1 24-hour window"
        )
    if not re.search(
        r"phase4_idempotency_policy_with_deadline\(\s*command\.command_class,\s*command\.timestamp_ms,\s*command\.command_deadline_at_ms,\s*\)",
        command,
    ):
        raise AssertionError(
            f"{SDK_COMMAND} must evaluate idempotency with the command timestamp and absolute deadline"
        )


def validate_tech_stack_alignment() -> None:
    tech_stack = read(TECH_STACK)
    command = read(SDK_COMMAND)
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
        "vault",
        "typescript",
    ]:
        if forbidden in command.lower():
            raise AssertionError(f"{SDK_COMMAND} contains forbidden stack term: {forbidden}")


def validate_wrapper() -> None:
    wrapper = read(VALIDATION_WRAPPER)
    assert_contains(wrapper, 'Path("scripts/validate_sdk_phase4.py")', VALIDATION_WRAPPER)


def validate_rust_tests() -> None:
    result = run(["cargo", "test", "-p", "overrid-sdk"])
    assert_contains(result.stdout + result.stderr, "test result: ok.", Path("cargo test -p overrid-sdk"))


def main() -> int:
    checks = [
        validate_sub_plan_phase4,
        validate_phase_docs,
        validate_sdk_code_and_readme,
        validate_tech_stack_alignment,
        validate_wrapper,
        validate_rust_tests,
    ]
    for check in checks:
        check()
    print("SDK Phase 4 validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
